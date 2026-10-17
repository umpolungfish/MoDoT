//! Local executable tools, shared by the language loop and --tool.
use std::path::PathBuf;
use std::process::Command;

pub fn kernel_binary() -> String {
    std::env::var("GMOMONADOS_BIN")
        .or_else(|_| std::env::var("MOMONADOS_BIN"))
        .unwrap_or_else(|_| default_binary("G-mOMonadOS", "g-momonados"))
}

fn default_binary(repo: &str, name: &str) -> String {
    // Resolve from the source checkout, independently of the caller's cwd.
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..").join(repo).join("target/release").join(name)
        .to_string_lossy().into_owned()
}

const VOX_COMMANDS: &[&str] = &[
    "factor", "factor-glut", "factor-membrane", "scout", "scout-membrane",
    "lift", "verdict", "circuit", "word", "prime-check", "coprime",
    "morphism-factor", "construct-carrier", "extract-factor", "self", "tables", "classify",
];
const GODEL_COMMANDS: &[&str] = &[
    "decode", "encode", "check", "unbraid", "braid", "relation", "analyze",
    "lte2", "product", "frame", "shiab", "separate", "factor",
];

pub fn recognizes(verb: &str) -> bool {
    matches!(verb, "gm" | "g-momonados" | "vox" | "godel" | "m3" | "m3iosis")
        || verb.strip_prefix("vox_").is_some_and(|v| VOX_COMMANDS.contains(&v.replace('_', "-").as_str()))
        || verb.strip_prefix("godel_").is_some_and(|v| GODEL_COMMANDS.contains(&v))
}

pub fn run(verb: &str, args: &[String]) -> Result<String, String> {
    if !recognizes(verb) { return Err(format!("unknown local binary tool: {verb}")); }
    if matches!(verb, "m3" | "m3iosis") {
        let mut command = m3_command()?;
        if args.is_empty() { command.arg("--help"); }
        return execute(command.args(args));
    }
    let (binary, alias) = if matches!(verb, "gm" | "g-momonados") {
        (kernel_binary(), None)
    } else if verb == "vox" || verb.starts_with("vox_") {
        (std::env::var("VOX_BIN").unwrap_or_else(|_| default_binary("Vox", "vox")),
         verb.strip_prefix("vox_").map(|v| v.replace('_', "-")))
    } else {
        (std::env::var("GODEL_BIN").unwrap_or_else(|_| default_binary("G-mOMonadOS", "godel")),
         verb.strip_prefix("godel_").map(str::to_owned))
    };
    let mut command = Command::new(&binary);
    if let Some(alias) = alias { command.arg(alias); }
    else if args.is_empty() { command.arg("help"); }
    execute(command.args(args))
}

fn m3_command() -> Result<Command, String> {
    if let Ok(binary) = std::env::var("M3IOSIS_BIN") {
        return Ok(Command::new(binary));
    }
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../m3iosis");
    let python = root.join(".venv/bin/python");
    let mut command = Command::new(if python.is_file() { python } else { "python3".into() });
    command.args(["-m", "m3iosis.cli"]);
    let mut paths = vec![root.join("src")];
    if let Some(existing) = std::env::var_os("PYTHONPATH") {
        paths.extend(std::env::split_paths(&existing));
    }
    command.env("PYTHONPATH", std::env::join_paths(paths).map_err(|e| e.to_string())?);
    Ok(command)
}

fn execute(command: &mut Command) -> Result<String, String> {
    let binary = command.get_program().to_string_lossy().into_owned();
    let output = command.output().map_err(|e| format!("cannot execute {binary}: {e}"))?;
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    if !output.status.success() {
        return Err(format!("{binary} exited with {}\n{text}", output.status));
    }
    Ok(text)
}
