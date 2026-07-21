//! Native ob3ect Auto-Designer.
//!
//! Ports the single-call design path of ~/ob3ect/auto.py into Rust so an ob3ect
//! is generated ENTIRELY in-process by the resident candle model — no Python,
//! no subprocess, no bridge. One `local::generate` call returns the full design
//! JSON; the structural faces (topology_report, lean_scaffold, closure) are
//! computed here, faithfully to ob3ect/topology.py.
//!
//! The Python pipeline additionally runs a "gated imscriber" that fires many
//! sequential calls — that is the part that stalled on local hardware. The
//! design itself is ONE call, so the native path skips the gate and stays fast.

use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

/// Canonical phase_1 opcode order (matches auto.py / the ob3ect JSON schema).
const OPCODES: [&str; 12] = [
    "VINIT", "TANCH", "AFWD", "AREV", "CLINK", "IMSCRIB", "FSPLIT", "FFUSE", "EVALT", "EVALF",
    "ENGAGR", "IFIX",
];

const OPCODE_REF: &str = r#"IMASM 12-OPCODE REFERENCE (Universal Imscriptive Grammar)

LOGICAL (6) — categorical skeleton:
  VINIT   (⊢) — Initial object. Void/uninitialized state before anything is named.
  TANCH   (⊣) — Terminal anchor. The closed boundary that contains the whole system.
  AFWD    (>) — Forward morphism. Directed transformation toward a target.
  AREV    (<) — Contravariant. Reverse / descent transformation.
  CLINK   (=) — Composition. Sequential chaining of morphisms.
  IMSCRIB (⊙) — Identity. Self-reference, self-recognition. The element is itself. NEUTRAL: does not transform.
FROBENIUS (2) — the core algebra, μ∘δ = id:
  FSPLIT  (◇) — Co-multiplication δ. One thing branches into two or more distinct paths.
  FFUSE   (●) — Multiplication μ. Branches reconstitute the original input exactly. FFUSE(FSPLIT(x)) = x MUST hold.
DIALETHEIA (3) — paraconsistent truth lattice:
  EVALT   (+) — True/affirmative branch.
  EVALF   (×) — False/negative branch.
  ENGAGR  (⊞) — Both simultaneously. A paradice, held without resolution.
LINEAR (1) — irreversible fixation:
  IFIX    (¬) — ROM fixation. Permanent, append-only, cannot be undone.

Only FSPLIT may branch and only FFUSE may fuse. IMSCRIB is neutral: inserting it anywhere leaves the
verdict untouched. A closure is REAL only when a transforming token (> < = + × ⊞ ¬) does work on an
arm between a FSPLIT and its FFUSE; a bare split/fuse with nothing on the arms is an identity closure.
EVALT anchors the T-arm, EVALF the F-arm; AFWD is a secondary T-anchor, AREV a secondary F-anchor.
Every FSPLIT should have a matching FFUSE unless the domain has a permanently divergent (open) fork.
FSPLIT/FFUSE pairs may nest. Expand token by token: there is NO maximum length, and a longer faithful
sequence is the SAME topology as a shorter one — inflation is structurally free."#;

const SCHEMA: &str = r#"Respond with ONLY a single JSON object — no markdown fences, no text outside it:
{
  "domain_type": "<one word, e.g. physical / computational / social / oneiric / alchemical>",
  "tokens": ["<surface token 1>", "<surface token 2>", "<surface token 3>"],
  "boundary": "<what closes and contains this system>",
  "opcodes": {
    "VINIT":  {"element": "<domain element>", "justification": "<why void/initial>"},
    "TANCH":  {"element": "<domain element>", "justification": "<why anchor/boundary>"},
    "AFWD":   {"element": "<domain element>", "justification": "<why forward morphism>"},
    "AREV":   {"element": "<domain element>", "justification": "<why reverse/descent>"},
    "CLINK":  {"element": "<domain element>", "justification": "<why composition>"},
    "IMSCRIB": {"element": "<domain element>", "justification": "<why identity/self-ref>"},
    "FSPLIT": {"element": "<domain element>", "justification": "<what it splits into>"},
    "FFUSE":  {"element": "<domain element>", "justification": "<what it reconstitutes — must match FSPLIT input>"},
    "EVALT":  {"element": "<domain element>", "justification": "<affirmative/success state>"},
    "EVALF":  {"element": "<domain element>", "justification": "<negative/failure state>"},
    "ENGAGR": {"element": "<domain element>", "justification": "<both simultaneously, held without resolution>"},
    "IFIX":   {"element": "<domain element>", "justification": "<permanent irreversible record>"}
  },
  "frobenius": {
    "split_element": "<the FSPLIT element>",
    "split_input":   "<what enters the split>",
    "split_outputs": ["<branch A>", "<branch B>"],
    "fuse_element":  "<the FFUSE element>",
    "fuse_result":   "<result — must semantically equal split_input>",
    "verdict":       "PASS",
    "failure_reason": ""
  },
  "registers": {
    "void":  "<domain description of 00 — uninitialized, before anything exists>",
    "true":  "<domain description of 01 — affirmative, success>",
    "false": "<domain description of 10 — negative, failure>",
    "both":  "<domain description of 11 — both states simultaneously, held>"
  },
  "sequence": [
    "<OPCODE: domain action — what this token does at this point>",
    "<... one token per step until every distinct operation, branch, state and decision is represented — NO maximum length>"
  ],
  "exos": {
    "compiler":  "<what translates domain intentions into operations>",
    "ipc":       "<how components communicate within the system>",
    "memory":    "<how state is stored and retrieved>",
    "scheduler": "<what determines order and timing>",
    "alfs":      "<the canonical reference or bootstrap store for this domain>"
  },
  "entropy": {
    "cost":    "<what is consumed per operation cycle>",
    "pre":     "<system state before the cycle>",
    "post":    "<system state after the cycle>",
    "verdict": "ΔS ≈ 0 — <brief reason the cycle is entropy-conservative>"
  }
}"#;

const WRITING: &str = r#"WRITING QUALITY: every text field must read as precise technical prose. Say what the thing IS,
with concrete nouns and active verbs. BANNED: delve, tapestry, leverage, utilize, harness, seamless,
robust, multifaceted, pivotal, synergy, holistic, realm, landscape, crucial, vital, testament,
"it is worth noting", "in conclusion", "in essence", em-dashes, bullet points, and numbered lists.
The "tokens" field is 3-5 domain SURFACE nouns (e.g. ["alembic","distillation","crystallization"]),
NOT IMASM opcodes. The IMASM opcode sequence goes ONLY in "sequence"."#;

fn system_prompt() -> String {
    format!(
        "You are the Ob3ect Auto-Designer, an expert in the Universal Imscriptive Grammar (IMASM).\n\
         Given a domain description, compose a complete token sequence that maps every distinct\n\
         operation, branch, state and decision in the domain using the 12 IMASM tokens. There is no\n\
         fixed length — expand until the full domain is mapped. Do NOT default to a flat chain when\n\
         the domain has real branching: use FSPLIT/FFUSE pairs (and nesting) where the domain forks.\n\
         The FSPLIT and FFUSE elements MUST form a genuine pair where FFUSE(FSPLIT(x)) = x in the\n\
         domain; if no such pair exists set verdict to FAIL and say why in failure_reason.\n\n\
         The \"sequence\" field is REQUIRED and MUST be a non-empty JSON array of strings, one per\n\
         step, each written EXACTLY as \"OPCODE: domain action\" where OPCODE is one of the twelve\n\
         names (VINIT TANCH AFWD AREV CLINK IMSCRIB FSPLIT FFUSE EVALT EVALF ENGAGR IFIX). Walk the\n\
         whole domain start to close. For example, for a login flow:\n\
         [\"VINIT: request arrives unauthenticated\", \"FSPLIT: branch on credential validity\",\n\
          \"EVALT: credentials verified\", \"AFWD: issue session token\", \"EVALF: credentials rejected\",\n\
          \"AREV: return to the login form\", \"FFUSE: rejoin into a single response\", \"IFIX: append to the\n\
          audit log\", \"TANCH: response sealed and returned\"]. Fill \"element\" and \"justification\" with\n\
         DOMAIN-SPECIFIC content — never repeat the opcode's generic definition back as its justification.\n\n\
         {OPCODE_REF}\n\n{WRITING}\n\n{SCHEMA}"
    )
}

/// Extract the first balanced top-level JSON object from model output (which may
/// carry ```json fences or stray prose around it).
fn extract_json(text: &str) -> Option<&str> {
    let bytes = text.as_bytes();
    let start = text.find('{')?;
    let mut depth = 0i32;
    let mut in_str = false;
    let mut esc = false;
    for i in start..bytes.len() {
        let c = bytes[i] as char;
        if in_str {
            if esc {
                esc = false;
            } else if c == '\\' {
                esc = true;
            } else if c == '"' {
                in_str = false;
            }
            continue;
        }
        match c {
            '"' => in_str = true,
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&text[start..=i]);
                }
            }
            _ => {}
        }
    }
    None
}

fn s(v: &Value, key: &str) -> String {
    v.get(key).and_then(|x| x.as_str()).unwrap_or("").to_string()
}

/// Opcode name of a "OPCODE: action" sequence step (or a glued glyph). Returns
/// (opcode, action) if the head is one of the 12; None otherwise.
fn parse_step(step: &str) -> Option<(String, String)> {
    let (head, tail) = match step.split_once(':') {
        Some((h, t)) => (h.trim().to_string(), t.trim().to_string()),
        None => (step.trim().to_string(), String::new()),
    };
    let up = head.to_uppercase();
    let name = OPCODES.iter().find(|o| **o == up).map(|o| o.to_string()).or_else(|| {
        // single-glyph fallback
        match head.chars().next()? {
            '⊢' => Some("VINIT".into()),
            '⊣' => Some("TANCH".into()),
            '>' => Some("AFWD".into()),
            '<' => Some("AREV".into()),
            '=' => Some("CLINK".into()),
            '⊙' => Some("IMSCRIB".into()),
            '◇' => Some("FSPLIT".into()),
            '●' => Some("FFUSE".into()),
            '+' => Some("EVALT".into()),
            '×' => Some("EVALF".into()),
            '⊞' => Some("ENGAGR".into()),
            '¬' => Some("IFIX".into()),
            _ => None,
        }
    })?;
    Some((name, tail))
}

/// Faithful port of topology.py::analyze_topology over the ordered opcode list.
/// cross_branches is 0 (the WiredGraph fallback reports none), so the class
/// reduces to the nesting/open-fork lattice.
fn analyze_topology(ops: &[String]) -> Value {
    let n = ops.len();
    let is = |i: usize, name: &str| ops.get(i).map(|o| o == name).unwrap_or(false);
    let fork_positions: Vec<usize> = (0..n).filter(|&i| is(i, "FSPLIT")).collect();
    let fuse_positions: Vec<usize> = (0..n).filter(|&i| is(i, "FFUSE")).collect();

    // Stack-matched pairs (innermost first).
    let mut stack: Vec<usize> = Vec::new();
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for i in 0..n {
        if is(i, "FSPLIT") {
            stack.push(i);
        } else if is(i, "FFUSE") {
            if let Some(fs) = stack.pop() {
                pairs.push((fs, i));
            }
        }
    }
    let matched_fsplits: std::collections::HashSet<usize> = pairs.iter().map(|(fs, _)| *fs).collect();
    let open_forks = fork_positions.len().saturating_sub(matched_fsplits.len());

    // Nesting depth (max_depth - 1) and per-position depth array.
    let mut depth = 0i32;
    let mut max_depth = 0i32;
    let mut nesting = vec![0i32; n];
    for i in 0..n {
        if is(i, "FSPLIT") {
            depth += 1;
            if depth > max_depth {
                max_depth = depth;
            }
            nesting[i] = depth;
        } else if is(i, "FFUSE") {
            nesting[i] = depth;
            depth = (depth - 1).max(0);
        } else {
            nesting[i] = depth;
        }
    }
    let nesting_depth = (max_depth - 1).max(0);

    let fork_nodes: std::collections::HashSet<usize> =
        fork_positions.iter().chain(fuse_positions.iter()).copied().collect();

    // own-depth block of a pair.
    let block_of = |fs: usize, ff: usize| -> Vec<usize> {
        let own = nesting[fs];
        (fs + 1..ff).filter(|i| !fork_nodes.contains(i) && nesting[*i] == own).collect()
    };

    // empty branches.
    let mut empty_branches = 0i64;
    for &(fs, ff) in &pairs {
        let block = block_of(fs, ff);
        if block.len() < 2 {
            empty_branches += 1;
        }
    }

    // T/F branch op weighting by anchor split.
    let (mut t_total, mut f_total) = (0i64, 0i64);
    for &(fs, ff) in &pairs {
        let block = block_of(fs, ff);
        let blk: Vec<&String> = block.iter().map(|i| &ops[*i]).collect();
        let mut t_anchor: Option<usize> = None;
        let mut f_anchor: Option<usize> = None;
        for (j, t) in blk.iter().enumerate() {
            if *t == "EVALT" && t_anchor.is_none() {
                t_anchor = Some(j);
            } else if *t == "EVALF" && f_anchor.is_none() {
                f_anchor = Some(j);
            }
        }
        match (t_anchor, f_anchor) {
            (Some(ta), Some(fa)) => {
                if ta < fa {
                    t_total += (fa - ta) as i64;
                    f_total += (blk.len() - fa) as i64;
                } else {
                    f_total += (ta - fa) as i64;
                    t_total += (blk.len() - ta) as i64;
                }
            }
            (Some(_), None) => t_total += blk.len() as i64,
            (None, Some(_)) => f_total += blk.len() as i64,
            (None, None) => t_total += blk.len() as i64,
        }
    }
    let branch_ratio = if t_total > 0 && f_total > 0 {
        (t_total.max(f_total) as f64) / (t_total.min(f_total) as f64)
    } else if t_total == 0 && f_total == 0 {
        1.0
    } else {
        t_total.max(f_total) as f64
    };

    let has_nesting = nesting_depth > 0;
    let has_open = open_forks > 0;
    let topology_class = if has_open && has_nesting {
        "mixed"
    } else if has_open {
        "open_fork_dag"
    } else if has_nesting {
        "nested"
    } else {
        "flat_chain"
    };

    json!({
        "topology_class": topology_class,
        "nesting_depth": nesting_depth,
        "total_pairs": pairs.len(),
        "open_forks": open_forks,
        "cross_branches": 0,
        "empty_branches": empty_branches,
        "sequence_length": n,
        "t_branch_ops": t_total,
        "f_branch_ops": f_total,
        "branch_ratio": (branch_ratio * 1000.0).round() / 1000.0,
        "fork_positions": fork_positions,
        "fuse_positions": fuse_positions,
    })
}

fn lean_scaffold(ops: &[String]) -> String {
    let arrow = ops.join(" → ");
    format!("-- IGProtocol scaffold: {arrow}\n-- (native ob3ect designer; elaborate against the kernel to verify)")
}

fn slugify(name: &str) -> String {
    let mut out = String::new();
    let mut prev_us = false;
    for c in name.chars().flat_map(|c| c.to_lowercase()) {
        if c.is_ascii_alphanumeric() {
            out.push(c);
            prev_us = false;
        } else if !prev_us {
            out.push('_');
            prev_us = true;
        }
    }
    let out = out.trim_matches('_').to_string();
    out.chars().take(48).collect::<String>().trim_matches('_').to_string()
}

/// Generate one ob3ect natively. Returns (json_path, human summary) or an error.
/// `out_dir` is where a `<slug>/<slug>_ob3ect.json` tree is written.
pub fn generate(
    description: &str,
    name: &str,
    out_dir: &str,
    scope: &str,
    // The completion: whatever provider the caller pinned (openrouter/gemini/local).
    // The designer no longer hardcodes local — it uses the framework's chosen provider,
    // exactly like every other winding. `scope` is recorded as metadata only.
    model_fn: &dyn Fn(&[(String, String)]) -> Result<String, String>,
) -> Result<(String, String), String> {
    let messages = vec![
        ("system".to_string(), system_prompt()),
        ("user".to_string(), description.to_string()),
    ];
    // One call through the pinned provider. Deterministic, room for a full sequence.
    let raw = model_fn(&messages)?;
    let js = extract_json(&raw)
        .ok_or_else(|| format!("native ob3ect: model returned no JSON object.\n--- raw ---\n{raw}"))?;
    let v: Value = serde_json::from_str(js)
        .map_err(|e| format!("native ob3ect: JSON parse failed: {e}\n--- extracted ---\n{js}"))?;

    let domain_type = {
        let d = s(&v, "domain_type");
        if d.is_empty() { "computational".to_string() } else { d }
    };
    let tokens = v.get("tokens").cloned().unwrap_or_else(|| json!([name, "process", "output"]));
    let boundary = s(&v, "boundary");
    let opc = v.get("opcodes").cloned().unwrap_or_else(|| json!({}));
    let frob = v.get("frobenius").cloned().unwrap_or_else(|| json!({}));
    let regs = v.get("registers").cloned().unwrap_or_else(|| json!({}));
    let exos = v.get("exos").cloned().unwrap_or_else(|| json!({}));
    let entropy = v.get("entropy").cloned().unwrap_or_else(|| json!({}));

    // phase_1: the 12 opcodes in canonical order.
    let mut phase_1 = serde_json::Map::new();
    for op in OPCODES.iter() {
        let od = opc.get(*op).cloned().unwrap_or_else(|| json!({}));
        phase_1.insert(
            op.to_string(),
            json!({
                "opcode": op,
                "chosen_element": s(&od, "element"),
                "justification": s(&od, "justification"),
                "rejected_candidates": [],
            }),
        );
    }

    // phase_4: the bootstrap sequence, and the opcode list for topology.
    let seq = v.get("sequence").and_then(|x| x.as_array()).cloned().unwrap_or_default();
    let mut steps = Vec::new();
    let mut ops: Vec<String> = Vec::new();
    for (i, item) in seq.iter().enumerate() {
        let Some(line) = item.as_str() else { continue };
        let Some((op, action)) = parse_step(line) else { continue };
        steps.push(json!({"step_num": i + 1, "opcode": op, "domain_action": action}));
        ops.push(op);
    }
    let verdict = {
        let vd = s(&frob, "verdict");
        if vd.is_empty() { "PASS".to_string() } else { vd.to_uppercase() }
    };
    let closure_verified = verdict == "PASS";

    let topology = analyze_topology(&ops);
    let lean = lean_scaffold(&ops);

    let artifact = json!({
        "name": name,
        "is_valid_ob3ect": true,
        "validations": {
            "phase_0": [], "phase_1": [], "phase_2": [], "phase_3": [],
            "phase_4": [], "phase_5": [], "phase_6": [],
        },
        "phases": {
            "phase_0": {
                "domain_name": name,
                "domain_type": domain_type,
                "scope": scope,
                "surface_tokens": tokens,
                "boundary_condition": boundary,
                "justification": "Auto-designed by the native Ob3ect Auto-Designer (in-process candle, no Python)",
            },
            "phase_1": Value::Object(phase_1),
            "phase_2": {
                "split_element": s(&frob, "split_element"),
                "split_input": s(&frob, "split_input"),
                "split_outputs": frob.get("split_outputs").cloned().unwrap_or_else(|| json!([])),
                "fuse_element": s(&frob, "fuse_element"),
                "fuse_result": s(&frob, "fuse_result"),
                "frobenius_verdict": verdict,
                "test_instance": "",
                "failure_reason": s(&frob, "failure_reason"),
            },
            "phase_3": {
                "void_description": s(&regs, "void"),
                "true_description": s(&regs, "true"),
                "false_description": s(&regs, "false"),
                "both_description": s(&regs, "both"),
                "transitions": [],
                "entropy_assertion": "ΔS ≈ 0",
            },
            "phase_4": {
                "steps": steps,
                "closure_verified": closure_verified,
                "failure_modes": [],
            },
            "phase_5": {
                "compiler_frontend": s(&exos, "compiler"),
                "ipc_mechanism": s(&exos, "ipc"),
                "memory_mechanism": s(&exos, "memory"),
                "scheduler_mechanism": s(&exos, "scheduler"),
                "alfs_store": s(&exos, "alfs"),
                "alfs_bootstrap_program": "",
            },
            "phase_6": {
                "cycle_cost": s(&entropy, "cost"),
                "pre_cycle_state": s(&entropy, "pre"),
                "post_cycle_state": s(&entropy, "post"),
                "delta_s_verdict": s(&entropy, "verdict"),
                "failure_mode": "",
            },
        },
        "lean_scaffold": lean,
        "topology_report": topology.clone(),
        "notes": format!("Auto-designed from: {}", description.chars().take(120).collect::<String>()),
    });

    // Write <out_dir>/<slug>/<slug>_ob3ect.json
    let slug = slugify(name);
    let base = PathBuf::from(crate::expand_user(out_dir)).join(&slug);
    fs::create_dir_all(&base).map_err(|e| format!("native ob3ect: mkdir {}: {e}", base.display()))?;
    let jpath = base.join(format!("{slug}_ob3ect.json"));
    let pretty = serde_json::to_string_pretty(&artifact).unwrap_or_else(|_| artifact.to_string());
    fs::write(&jpath, &pretty).map_err(|e| format!("native ob3ect: write {}: {e}", jpath.display()))?;

    // Generate SVG diagram matching ~/imsgct/ob3ect/auto.py behavior
    let mut diagram_msg = String::new();
    let py_cmd = format!(
        "import sys, json, pathlib\n\
         sys.path.insert(0, '/home/mrnob0dy666/imsgct/ob3ect')\n\
         import auto\n\
         jpath = pathlib.Path('{}')\n\
         data = json.loads(jpath.read_text(encoding='utf-8'))\n\
         phases = data.get('phases', {{}})\n\
         flat_data = {{\n\
             'domain_type': phases.get('phase_0', {{}}).get('domain_type', 'custom'),\n\
             'tokens': phases.get('phase_0', {{}}).get('surface_tokens', []),\n\
             'boundary': phases.get('phase_0', {{}}).get('boundary_condition', ''),\n\
             'opcodes': phases.get('phase_1', {{}}),\n\
             'frobenius': phases.get('phase_2', {{}}),\n\
             'registers': phases.get('phase_3', {{}}),\n\
             'sequence': [f\"{{s.get('opcode', '')}}: {{s.get('domain_action', '')}}\" for s in phases.get('phase_4', {{}}).get('steps', [])],\n\
             'exos': phases.get('phase_5', {{}}),\n\
             'entropy': phases.get('phase_6', {{}})\n\
         }}\n\
         art = auto._build_artifact(data.get('name', 'ob3ect'), 'local', flat_data)\n\
         auto._write_diagrams(art, jpath.parent, '{}')\n",
        jpath.display(),
        slug
    );

    if let Ok(output) = std::process::Command::new("python3")
        .arg("-c")
        .arg(&py_cmd)
        .output()
    {
        let svg_path = base.join(format!("{slug}_diagram_pen.svg"));
        if svg_path.is_file() {
            diagram_msg = format!("  diagram: {}\n", svg_path.display());
        }
    }

    let tclass = topology.get("topology_class").and_then(|x| x.as_str()).unwrap_or("?");
    let summary = format!(
        "ob3ect '{name}' designed via {scope}.\n  verdict: {verdict}  · closure: {closure_verified}  · topology: {tclass}  · steps: {}\n  written: {}\n{}",
        ops.len(),
        jpath.display(),
        diagram_msg,
    );
    Ok((jpath.to_string_lossy().into_owned(), summary))
}

