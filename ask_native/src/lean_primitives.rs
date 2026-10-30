//! Primitive construction comes from the elaborated Lean modules.
//!
//! The JSON here is a wire format emitted by Lean on this invocation, never an
//! artifact or wordbook selected as a second source of primitive definitions.
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use imasm_core::classic::Token;

#[derive(Clone, serde::Deserialize)]
struct PrimitiveProgram {
    name: String,
    source: String,
    word: String,
    opcodes: Vec<String>,
    actions: Vec<String>,
    pairs: Vec<(usize, usize)>,
    #[serde(default)]
    dependencies: Vec<String>,
}

#[derive(serde::Deserialize)]
struct ProgramExport {
    schema: u32,
    programs: Vec<PrimitiveProgram>,
}

fn mill_root() -> PathBuf {
    PathBuf::from(crate::expand_user("~/imsgct/p4rakernel/p4ramill"))
}

fn fresh_source(root: &Path, source: &str) -> Result<(), String> {
    let path = Path::new(source);
    if !(source.starts_with("Imscribing/Primitives/Types/")
        || source.starts_with("Imscribing/Ob3ects/GateCheck/"))
        || path.components().any(|p| !matches!(p, std::path::Component::Normal(_)))
        || path.extension().and_then(|p| p.to_str()) != Some("lean")
    {
        return Err(format!("invalid Lean primitive source: {source}"));
    }
    let lean = root.join(path);
    let olean = root.join(".lake/build/lib/lean").join(path.with_extension("olean"));
    let modified = |path: &Path| {
        std::fs::metadata(path).and_then(|m| m.modified())
            .map_err(|e| format!("Lean primitive {}: {e}", path.display()))
    };
    if modified(&lean)? > modified(&olean)? {
        return Err(format!("Lean primitive source changed: {source}; elaborate it before expansion"));
    }
    Ok(())
}

fn load() -> Result<BTreeMap<String, PrimitiveProgram>, String> {
    let root = mill_root();
    // Reject stale imports before starting Lean, rather than returning an old
    // compiled program after its source has changed.
    let directory = root.join("Imscribing/Primitives/Types");
    for entry in std::fs::read_dir(&directory).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.extension().and_then(|p| p.to_str()) == Some("lean")
            && path.file_stem().and_then(|p| p.to_str()).is_some_and(|s| s.starts_with("PrimitiveType"))
        {
            let source = path.strip_prefix(&root).map_err(|e| e.to_string())?.to_string_lossy();
            fresh_source(&root, &source)?;
        }
    }
    let output = std::process::Command::new("lake")
        .args(["env", "lean", "--run", "_scripts/export_primitive_programs.lean"])
        .current_dir(&root).output().map_err(|e| format!("run Lean primitive exporter: {e}"))?;
    if !output.status.success() {
        return Err(format!("Lean primitive exporter failed: {}{}",
            String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr)));
    }
    let export: ProgramExport = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("parse Lean primitive export: {e}"))?;
    if export.schema != 1 || export.programs.is_empty() {
        return Err("unsupported or empty Lean primitive export".into());
    }
    let mut programs = BTreeMap::new();
    for program in export.programs {
        fresh_source(&root, &program.source)?;
        for dependency in &program.dependencies { fresh_source(&root, dependency)?; }
        validate(&program)?;
        let name = program.name.clone();
        if programs.insert(name.clone(), program).is_some() {
            return Err(format!("duplicate Lean primitive {name}"));
        }
    }
    Ok(programs)
}

fn validate(program: &PrimitiveProgram) -> Result<(), String> {
    let ops: Vec<Token> = program.opcodes.iter().map(|op|
        Token::parse(op).ok_or_else(|| format!("unknown Lean opcode {op}")))
        .collect::<Result<_, _>>()?;
    if ops.is_empty() || ops.len() != program.actions.len()
        || ops.iter().map(|op| op.code()).collect::<String>() != program.word
    {
        return Err(format!("incomplete Lean program {}", program.name));
    }
    for &(fork, fuse) in &program.pairs {
        if ops.get(fork) != Some(&Token::Fsplit) || ops.get(fuse) != Some(&Token::Ffuse) {
            return Err(format!("invalid Lean fork/fuse pair in {}", program.name));
        }
    }
    Ok(())
}

static PROGRAMS: OnceLock<Result<BTreeMap<String, PrimitiveProgram>, String>> = OnceLock::new();

fn programs() -> Result<&'static BTreeMap<String, PrimitiveProgram>, String> {
    PROGRAMS.get_or_init(load).as_ref().map_err(Clone::clone)
}

fn key(name: &str) -> String {
    name.trim().trim_start_matches("the_primitive_type_called_")
        .trim_start_matches("the primitive type called ").replace(' ', "_").to_ascii_lowercase()
}

pub(crate) fn expand(name: &str) -> Result<(Vec<Token>, Vec<(usize, usize)>, Vec<String>), String> {
    let name = key(name);
    let program = programs()?.get(&name).ok_or_else(|| format!("no Lean primitive program '{name}'"))?;
    // validate() has already rejected all missing tokens.
    let ops = program.opcodes.iter().map(|op| Token::parse(op).unwrap()).collect();
    let actions = program.opcodes.iter().zip(&program.actions)
        .map(|(op, action)| format!("{op}: {action}")).collect();
    Ok((ops, program.pairs.clone(), actions))
}

pub(crate) fn source(name: &str) -> Result<String, String> {
    let name = key(name);
    let program = programs()?.get(&name).ok_or_else(|| format!("no Lean primitive program '{name}'"))?;
    Ok(mill_root().join(&program.source).display().to_string())
}

pub(crate) fn names() -> Result<Vec<String>, String> {
    Ok(programs()?.keys().cloned().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_native_type_has_an_exported_lean_construction() {
        let exported = programs().expect("Lean primitive export");
        assert_eq!(exported.len(), crate::imasm::GLYPH_TYPE_NAME.len());
        for &(_, name) in crate::imasm::GLYPH_TYPE_NAME {
            let program = exported.get(name).expect(name);
            let (ops, pairs, actions) = expand(name).expect(name);
            assert_eq!(ops.iter().map(|op| op.code()).collect::<String>(), program.word);
            assert_eq!(pairs, program.pairs);
            assert_eq!(actions.len(), ops.len());
            assert!(source(name).unwrap().ends_with(".lean"));
        }
    }

    #[test]
    fn incomplete_export_is_rejected_without_dropping_tokens() {
        let program = PrimitiveProgram {
            name: "control".into(), source: "unused".into(), word: "⊢⊣".into(),
            opcodes: vec!["VINIT".into(), "UNKNOWN".into(), "TANCH".into()],
            actions: vec!["begin".into(), "unknown".into(), "end".into()], pairs: vec![],
            dependencies: vec![],
        };
        assert!(validate(&program).is_err());
    }

    #[test]
    fn source_newer_than_compiled_module_is_rejected() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../measurements/lean_primitive_sources_20261024")
            .join(format!("freshness_control_{}", std::process::id()));
        let source = "Imscribing/Primitives/Types/PrimitiveTypeControl.lean";
        let lean = root.join(source);
        let compiled = root.join(".lake/build/lib/lean/Imscribing/Primitives/Types/PrimitiveTypeControl.olean");
        std::fs::create_dir_all(lean.parent().unwrap()).unwrap();
        std::fs::create_dir_all(compiled.parent().unwrap()).unwrap();
        std::fs::write(&lean, "-- source freshness control").unwrap();
        std::fs::write(&compiled, "compiled freshness control").unwrap();
        let epoch = std::time::SystemTime::UNIX_EPOCH;
        for (path, seconds) in [(&lean, 20), (&compiled, 10)] {
            std::fs::File::options().write(true).open(path).unwrap()
                .set_times(std::fs::FileTimes::new().set_modified(epoch + std::time::Duration::from_secs(seconds)))
                .unwrap();
        }
        assert!(fresh_source(&root, source).unwrap_err().contains("source changed"));
        std::fs::File::options().write(true).open(&compiled).unwrap()
            .set_times(std::fs::FileTimes::new().set_modified(epoch + std::time::Duration::from_secs(30)))
            .unwrap();
        assert!(fresh_source(&root, source).is_ok());
    }
}
