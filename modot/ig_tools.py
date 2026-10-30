"""Bridge to the full IG tool corpus.

MoDoT's agent loop already recognizes `TOOL: <name> <args>` lines emitted by the
model, but it only recorded them; it had no backend for the IG catalog tools
(compute_distance, primitive_peel, crystal_decode, zfc_probe, aleph_encode, ...).
Those live in `~/imsgct/imscribing_grammar/IG_inquiry.py` behind a single
`ToolDispatcher.dispatch(name, args, iteration)` entry point.

Rather than re-implement the ~40 tools, this loads the live dispatcher (one
manifold, R∧W∧X) and exposes it to MoDoT. Calling a tool here runs the real IG
code and returns its real result, so a `TOOL:` line grounds instead of narrates.
"""

from __future__ import annotations

import os
from pathlib import Path
import shlex
import sys
from typing import Any, Dict, List, Optional

# Where the Imscribing Grammar corpus lives.
IG_ROOT = os.environ.get(
    "IG_ROOT", os.path.expanduser("~/imsgct/imscribing_grammar")
)

_WORKSPACE = Path(__file__).resolve().parents[2]
ASK_NATIVE_BIN = os.environ.get("ASK_NATIVE_BIN", str(_WORKSPACE / "MoDoT" / "ask.bin"))

# Tool name -> ordered positional arg names, for parsing `TOOL: name a b` lines.
# Names with no positional args (or dict/list args) take a trailing JSON/kv blob.
IG_TOOL_ARGS: Dict[str, List[str]] = {
    "m3": ["command"],
    "m3iosis": ["command"],
    "gm": ["command"],
    "g-momonados": ["command"],
    "godel": ["command"],
    "godel_decode": ["word"],
    "godel_encode": ["n"],
    "godel_check": ["op", "lhs", "rhs", "out"],
    "godel_unbraid": ["word"],
    "godel_braid": ["gamma", "lambda"],
    "godel_relation": ["from", "to"],
    "godel_analyze": ["word", "window"],
    "godel_lte2": ["odd_a", "even_m"],
    "godel_product": ["product", "left", "right"],
    "godel_frame": ["value"],
    "godel_shiab": ["value", "left", "right"],
    "godel_separate": ["semiprime"],
    "godel_factor": ["value"],
    "vox": ["command"],
    "vox_factor": ["n"],
    "vox_factor_glut": ["n"],
    "vox_factor_membrane": ["n"],
    "vox_scout": ["n"],
    "vox_scout_membrane": ["n"],
    "vox_lift": ["path"],
    "vox_verdict": ["word"],
    "vox_circuit": ["module", "mask"],
    "vox_word": ["path"],
    "vox_prime_check": ["n"],
    "vox_coprime": ["base", "n"],
    "vox_morphism_factor": ["word"],
    "vox_construct_carrier": ["operator_word"],
    "vox_extract_factor": ["factor_carrier_word"],
    "vox_self": [],
    "vox_tables": ["path", "symbol"],
    "vox_classify": ["mn", "ops"],
    "lookup_catalog": ["keyword"],
    # A word is a ring and ROTAT is the cyclic shift: the verdict and the
    # topology hold across the orbit, the final register does not. `insert`
    # places a token at every index of every rotation, which is the same
    # question asked of an operation, since appending is insertion at the seam.
    "lattice_cycle": ["word", "insert"],
    # Where the weight moves. The fork is a set and the fuse a union, so a
    # finished walk keeps which values were touched and nothing else.
    "weight_flow": ["word"],
    "list_catalog": [],
    "encode_system": ["name", "description", "tuple"],
    "imscribe_system": ["name", "description", "tuple"],
    "check_imscription": ["name_boundary", "name_bulk"],
    "ouroborics": ["name"],
    "compute_distance": ["name_a", "name_b"],
    "compute_conflict_distance": ["name_holistic", "name_compositional"],
    "compute_meet": ["name_a", "name_b"],
    "compute_join": ["name_a", "name_b"],
    "compute_tensor": ["name_a", "name_b"],
    "containment_boundary": ["name"],
    "find_analogies": ["name"],
    "monad_probe": ["name"],
    "topo_protection_probe": ["name"],
    "consciousness_score": ["name"],
    "project": ["name"],
    "primitive_peel": ["name", "primitive"],
    "principal_decomp": ["name"],
    "retrosynthetic_path": ["name"],
    "emergence_frontier": [],
    "compute_promotions": ["name_source", "name_target"],
    "predict_from_promotions": ["promoted_primitives"],
    "register_promotion_pattern": ["promoted_primitives", "behavior_description", "example_system"],
    "crystal_encode": [],
    "crystal_decode": ["address"],
    "crystal_navigate": ["limit"],
    "crystal_count": [],
    "crystal_tier_census": [],
    "crystal_nearest": ["name"],
    "crystal_tier_gap_ladder": [],
    "quiver_encode": ["name"],
    "domain_info": ["domain"],
    "domain_verify": ["domain"],
    "domain_nearest": ["name"],
    "navigator_info": [],
    "zfc_formula": ["name"],
    "zfc_probe": ["name"],
    "zfc_catalog_probe": ["n"],
    "aleph_encode": ["text"],
    "aleph_distance": ["a", "b"],
    "composite_type": ['names'],
    "jump_path_integral": ['a', 'b'],
    "frobenius_closure_check": ['a', 'b'],
    "signature_manifold": [],
    "paradice_lattice": [],
    "braid_word": ['a', 'b'],
    "universe_jump": ['a', 'b'],
    "paradice_map": [],
    "riemann_xi_info": [],
    "ask_question": ["question"],
    "record_insight": ["text", "plane", "confidence", "systems"],
    # Structural verbs (native ask_native CLI verbs) — shell out to ask.bin
    "click": ["a", "b"],
    "switch": ["a", "b"],
    "excite": ["name"],
    "ascend": ["name"],
    "descend": ["name"],
    "filter": ["names"],
    "phase_reconstruct": ["monomers"],
    "set": ["donor", "acceptor"],
    "homolyze": ["a", "b"],
    "scan": ["donor", "acceptor"],
    "complement": ["name"],
    "cycle": ["catalyst", "substrate"],
    "pathway": ["substrate", "catalysts"],
    "polymerize": ["monomers"],
    "star": ["monomers"],
    "broadcast": ["source"],
    "close": ["monomers"],
    "material": ["monomers"],
    "modulus": ["monomers"],
    "arrange": ["monomers"],
    "forge": ["monomers"],
    "compare": ["set_a", "vs", "set_b"],
    "dope": ["base", "with", "dopant"],
    "fuse": ["ring_a", "plus", "ring_b"],
    "cleave": ["monomers"],
    "anneal": ["monomers"],
    "distill": ["monomers"],
    "fdistill": ["monomers"],
    "sublime": ["name"],
    "crystallize": ["monomers"],
    "cocrystallize": ["a", "b"],
    "seed": ["monomers"],
    "tlc": ["monomers"],
    "column": ["monomers"],
    "fpt": ["monomers"],
    "trap": ["name"],
    "stain": ["reagent", "monomers"],
    "register": ["name", "monomers"],
    "recall": ["name"],
    "imscribe": ["name"],
    "ob3ect": ["description"],
    "imasm": ["command"],
    "calc": ["expression"],
}

# args that must be coerced to int / list when parsed from a bare `TOOL:` line
_INT_ARGS = {"address"}
_LIST_ARGS = {"promoted_primitives"}

_dispatcher = None


class IGToolError(RuntimeError):
    """Raised when the IG tool corpus cannot be loaded or a tool call fails."""


def _deshadow_navigators() -> None:
    """Ensure `navigators` resolves to the package under IG_ROOT, not the stray
    top-level file `navigators/navigators.py`.

    modot.witness_proof puts `<IG_ROOT>/navigators` on sys.path so it can import
    the cl8nk navigator directly; that shadows the `navigators` PACKAGE that
    IG_inquiry imports (`navigators.crystal_navigator`). We put IG_ROOT ahead of
    the shadow and drop a mis-cached file-version `navigators` module so the
    package wins, without removing witness_proof's entry.
    """
    if IG_ROOT in sys.path:
        sys.path.remove(IG_ROOT)
    sys.path.insert(0, IG_ROOT)
    nav = sys.modules.get("navigators")
    nav_file = getattr(nav, "__file__", "") or ""
    if nav is not None and nav_file.replace("\\", "/").endswith("navigators/navigators.py"):
        for mod in [m for m in sys.modules if m == "navigators" or m.startswith("navigators.")]:
            del sys.modules[mod]


def dispatcher():
    """The live IG ToolDispatcher (lazily instantiated, cached)."""
    global _dispatcher
    if _dispatcher is not None:
        return _dispatcher
    _deshadow_navigators()
    try:
        from IG_inquiry import ToolDispatcher, SessionCatalog, CATALOG_PATH
    except Exception as exc:  # pragma: no cover - environment dependent
        raise IGToolError(
            f"cannot import IG_inquiry from {IG_ROOT}: {exc}"
        ) from exc
    catalog = SessionCatalog(catalog_path=os.environ.get("MOMONADOS_CATALOG", CATALOG_PATH))
    _dispatcher = ToolDispatcher(catalog=catalog, question_queue=[], insights=[])
    return _dispatcher


def tool_names() -> List[str]:
    """Every IG tool name this bridge can route."""
    return sorted(IG_TOOL_ARGS)


def available() -> bool:
    """True iff the IG corpus can be loaded."""
    try:
        dispatcher()
        return True
    except IGToolError:
        return False


def call(name: str, args: Dict[str, Any], iteration: int = 0) -> Dict[str, Any]:
    """Run one IG tool by name with an explicit args dict; return its real result."""
    if name in IG_TOOL_ARGS and (name in ("m3", "m3iosis", "gm", "g-momonados", "vox", "godel") or name.startswith(("vox_", "godel_"))):
        positional = [str(args[key]) for key in IG_TOOL_ARGS[name] if key in args]
        positional.extend(str(value) for value in args.get("args", []))
        return parse_and_call(shlex.join([name, *positional]), iteration)
    try:
        return dispatcher().dispatch(name, args, iteration)
    except Exception as exc:
        return {"status": "error", "tool": name, "error": str(exc)}


_WORKSPACE = Path(__file__).resolve().parents[2]
GMOMONADOS_BIN = os.environ.get("GMOMONADOS_BIN", os.environ.get("MOMONADOS_BIN", str(_WORKSPACE / "G-mOMonadOS/target/release/g-momonados")))
VOX_BIN = os.environ.get("VOX_BIN", str(_WORKSPACE / "Vox/target/release/vox"))
GODEL_BIN = os.environ.get("GODEL_BIN", str(_WORKSPACE / "G-mOMonadOS/target/release/godel"))

def _call_binary(binary: str, tool: str, cmd: str, args: List[str], *, prefix=(), env=None) -> Dict[str, Any]:
    import subprocess
    try:
        result = subprocess.run([binary, *prefix, cmd, *args], capture_output=True, text=True, timeout=300, env=env)
        return {"status": "ok" if result.returncode == 0 else "error",
                "tool": f"{tool} {cmd}", "binary": binary,
                "stdout": result.stdout, "stderr": result.stderr, "returncode": result.returncode}
    except subprocess.TimeoutExpired:
        return {"status": "error", "tool": f"{tool} {cmd}", "error": "timeout after 300s"}
    except OSError as exc:
        return {"status": "error", "tool": f"{tool} {cmd}", "error": str(exc)}


def call_m3(cmd: str, args: List[str]) -> Dict[str, Any]:
    override = os.environ.get("M3IOSIS_BIN")
    if override:
        return _call_binary(override, "m3", cmd, args)
    root = _WORKSPACE / "m3iosis"
    python = root / ".venv/bin/python"
    env = os.environ.copy()
    env["PYTHONPATH"] = os.pathsep.join([str(root / "src"), *filter(None, [env.get("PYTHONPATH")])])
    return _call_binary(str(python) if python.is_file() else "python3", "m3", cmd, args,
                        prefix=("-m", "m3iosis.cli"), env=env)


def call_vox(cmd: str, args: List[str]) -> Dict[str, Any]:
    return _call_binary(VOX_BIN, "vox", cmd, args)


def call_godel(cmd: str, args: List[str]) -> Dict[str, Any]:
    return _call_binary(GODEL_BIN, "godel", cmd, args)


def _call_structural_tool(verb: str, args: List[str]) -> Dict[str, Any]:
    """Shell out to ask_native binary for structural verbs.

    Mirrors the Rust `run_structural_tool` in fart.rs.
    """
    # Build CLI flags for each verb (matching Rust run_structural_tool)
    flags: List[str]
    a = lambda i: args[i] if i < len(args) else None

    if verb == "click":
        if a(0) is None:
            return {"status": "error", "tool": verb, "error": "click needs at least 1 argument"}
        flags = ["--click", a(0)]
        if a(1):
            flags.append(a(1))
    elif verb == "switch":
        if a(0) is None or a(1) is None:
            return {"status": "error", "tool": verb, "error": "switch needs 2 arguments"}
        flags = ["--switch", a(0), a(1)]
    elif verb == "excite":
        if a(0) is None:
            return {"status": "error", "tool": verb, "error": "excite needs 1 argument"}
        flags = ["--excite", a(0)]
    elif verb == "ascend":
        if a(0) is None:
            return {"status": "error", "tool": verb, "error": "ascend needs 1 argument"}
        flags = ["--ascend", a(0)]
    elif verb == "descend":
        if a(0) is None:
            return {"status": "error", "tool": verb, "error": "descend needs 1 argument"}
        flags = ["--descend", a(0)]
    elif verb == "star":
        if len(args) < 4:
            return {"status": "error", "tool": verb, "error": "star needs at least 4 arguments"}
        flags = ["--star"] + args
    elif verb == "filter":
        if len(args) < 2:
            return {"status": "error", "tool": verb, "error": "filter needs at least 2 arguments"}
        flags = ["--filter"] + args
    elif verb == "phase_reconstruct":
        if len(args) < 2:
            return {"status": "error", "tool": verb, "error": "phase_reconstruct needs at least 2 arguments"}
        flags = ["--phase-reconstruct"] + args
    elif verb == "set":
        if a(0) is None or a(1) is None:
            return {"status": "error", "tool": verb, "error": "set needs 2 arguments"}
        flags = ["--set", a(0), a(1)]
    elif verb == "homolyze":
        if a(0) is None:
            return {"status": "error", "tool": verb, "error": "homolyze needs at least 1 argument"}
        flags = ["--homolyze", a(0)]
        if a(1):
            flags.append(a(1))
    elif verb == "scan":
        if a(0) is None or a(1) is None:
            return {"status": "error", "tool": verb, "error": "scan needs 2 arguments"}
        flags = ["--set", a(0), a(1), "--scan-mediators"]
    elif verb == "complement":
        if a(0) is None:
            return {"status": "error", "tool": verb, "error": "complement needs 1 argument"}
        flags = ["--complement", a(0)]
    elif verb == "cycle":
        if a(0) is None or a(1) is None:
            return {"status": "error", "tool": verb, "error": "cycle needs 2 arguments"}
        flags = ["--cycle", a(0), a(1)]
    elif verb == "pathway":
        if len(args) < 2:
            return {"status": "error", "tool": verb, "error": "pathway needs at least 2 arguments"}
        flags = ["--pathway"] + args
    elif verb == "polymerize":
        if len(args) < 2:
            return {"status": "error", "tool": verb, "error": "polymerize needs at least 2 arguments"}
        flags = ["--polymerize"] + args
    elif verb == "close":
        if len(args) < 2:
            return {"status": "error", "tool": verb, "error": "close needs at least 2 arguments"}
        flags = ["--polymerize"] + args + ["--close"]
    elif verb == "material":
        if len(args) < 2:
            return {"status": "error", "tool": verb, "error": "material needs at least 2 arguments"}
        flags = ["--polymerize"] + args + ["--props"]
    elif verb == "modulus":
        if len(args) < 2:
            return {"status": "error", "tool": verb, "error": "modulus needs at least 2 arguments"}
        flags = ["--polymerize"] + args + ["--modulus"]
    elif verb == "arrange":
        if len(args) < 2:
            return {"status": "error", "tool": verb, "error": "arrange needs at least 2 arguments"}
        flags = ["--polymerize"] + args + ["--arrange"]
    elif verb == "forge":
        if len(args) < 2:
            return {"status": "error", "tool": verb, "error": "forge needs at least 2 arguments"}
        flags = ["--forge"] + args
    elif verb == "compare":
        # compare format: A B vs X Y Z -> need to parse "vs" separator
        if len(args) < 3:
            return {"status": "error", "tool": verb, "error": "compare needs at least 3 arguments (A B vs X Y...)"}
        flags = ["--compare"] + args
    elif verb == "dope":
        # dope format: A B with C -> need to parse "with" separator
        if len(args) < 3:
            return {"status": "error", "tool": verb, "error": "dope needs at least 3 arguments (A B with C)"}
        flags = ["--dope"] + args
    elif verb == "fuse":
        # fuse format: A B + X Y Z -> need to parse "+" separator
        if len(args) < 5:
            return {"status": "error", "tool": verb, "error": "fuse needs at least 5 arguments (A B + X Y...)"}
        flags = ["--fuse"] + args
    elif verb == "cleave":
        if len(args) < 4:
            return {"status": "error", "tool": verb, "error": "cleave needs at least 4 arguments"}
        flags = ["--cleave"] + args
    elif verb == "anneal":
        if len(args) < 3:
            return {"status": "error", "tool": verb, "error": "anneal needs at least 3 arguments"}
        flags = ["--anneal"] + args
    elif verb == "distill":
        if len(args) < 2:
            return {"status": "error", "tool": verb, "error": "distill needs at least 2 arguments"}
        flags = ["--distill"] + args
    elif verb == "fdistill":
        if len(args) < 2:
            return {"status": "error", "tool": verb, "error": "fdistill needs at least 2 arguments"}
        flags = ["--fdistill"] + args
    elif verb == "sublime":
        if a(0) is None:
            return {"status": "error", "tool": verb, "error": "sublime needs 1 argument"}
        flags = ["--sublime", a(0)]
    elif verb == "crystallize":
        if len(args) < 2:
            return {"status": "error", "tool": verb, "error": "crystallize needs at least 2 arguments"}
        flags = ["--crystallize"] + args
    elif verb == "cocrystallize":
        if a(0) is None or a(1) is None:
            return {"status": "error", "tool": verb, "error": "cocrystallize needs 2 arguments"}
        flags = ["--cocrystallize", a(0), a(1)]
    elif verb == "seed":
        if len(args) < 3:
            return {"status": "error", "tool": verb, "error": "seed needs at least 3 arguments"}
        flags = ["--seed"] + args
    elif verb == "tlc":
        if len(args) < 2:
            return {"status": "error", "tool": verb, "error": "tlc needs at least 2 arguments"}
        flags = ["--tlc"] + args
    elif verb == "column":
        if len(args) < 2:
            return {"status": "error", "tool": verb, "error": "column needs at least 2 arguments"}
        flags = ["--column"] + args
    elif verb == "fpt":
        if len(args) < 2:
            return {"status": "error", "tool": verb, "error": "fpt needs at least 2 arguments"}
        flags = ["--fpt"] + args
    elif verb == "trap":
        if a(0) is None:
            return {"status": "error", "tool": verb, "error": "trap needs at least 1 argument"}
        flags = ["--trap", a(0)]
        if a(1):
            flags.append(a(1))
    elif verb == "stain":
        if len(args) < 2:
            return {"status": "error", "tool": verb, "error": "stain needs at least 2 arguments"}
        flags = ["--stain"] + args
    elif verb == "register":
        if len(args) < 2:
            return {"status": "error", "tool": verb, "error": "register needs at least 2 arguments (name + monomers)"}
        flags = ["--register", a(0)] + args[1:]
    elif verb == "recall":
        if a(0) is None:
            return {"status": "error", "tool": verb, "error": "recall needs 1 argument"}
        flags = ["--recall", a(0)]
    elif verb == "imscribe":
        # imscribe is special - it runs the generate pipeline, not ask.bin
        # For now, delegate to the IG catalog imscribe tool
        return {"status": "skip", "reason": "imscribe not implemented in structural bridge; use IG catalog imscribe_system"}
    elif verb == "ob3ect":
        # ob3ect runs the Auto-Designer pipeline
        return {"status": "skip", "reason": "ob3ect not implemented in structural bridge; runs external auto.py"}
    elif verb == "imasm":
        # imasm composes opcodes - could shell to ask.bin --imasm
        if len(args) < 1:
            return {"status": "error", "tool": verb, "error": "imasm needs at least 1 argument"}
        flags = ["--imasm"] + args
    elif verb == "calc":
        if len(args) < 1:
            return {"status": "error", "tool": verb, "error": "calc needs an expression"}
        flags = ["--calc"] + args
    else:
        return {"status": "error", "tool": verb, "error": f"unhandled structural verb: {verb}"}

    return _call_binary(ASK_NATIVE_BIN, "ask", "", flags)


_PRIMITIVE_GLYPHS = {"⊢", "⊣", "≻", "≺", "⋈", "⊤", "∈", "∋", "⊙", "⊥", "⊞", "⊡"}


def _coerce(name: str, positional: List[str]) -> Dict[str, Any]:
    spec = IG_TOOL_ARGS.get(name)
    if spec is None:
        raise IGToolError(f"unknown IG tool: {name!r}")
    # The door for swapped order: agents emit `primitive_peel < <name>` as often as
    # the documented `primitive_peel <name> <`, and the first arg then errors as
    # "Unknown system: <". When a spec is [name, primitive] and the args arrive
    # primitive-first, put them the right way round instead of erroring.
    if (
        spec[:2] == ["name", "primitive"]
        and len(positional) >= 2
        and positional[0] in _PRIMITIVE_GLYPHS
        and positional[1] not in _PRIMITIVE_GLYPHS
    ):
        positional = [positional[1], positional[0]] + list(positional[2:])
    args: Dict[str, Any] = {}
    for i, key in enumerate(spec):
        if i >= len(positional):
            break
        if key in _LIST_ARGS:
            args[key] = positional[i:]  # rest of the line is the list
            break
        val: Any = positional[i]
        if key in _INT_ARGS:
            try:
                val = int(val)
            except ValueError:
                pass
        args[key] = val
    return args


def parse_and_call(line: str, iteration: int = 0) -> Dict[str, Any]:
    """Run a bare `TOOL: name arg1 arg2` (or `name arg1 arg2`) line.

    This matches the convention MoDoT's agent already emits, so the agent's
    `_act` can route recognized IG tools straight through here.
    """
    body = line.strip()
    for prefix in ("TOOL:", "ACTION:"):
        if body.upper().startswith(prefix):
            body = body[len(prefix):].strip()
            break
    parts = shlex.split(body)
    if not parts:
        return {"status": "error", "error": "empty tool line"}
    name, positional = parts[0], parts[1:]
    # MoDoT jump tools: dispatch locally
    _MODO_JUMP_TOOLS = {'paradice_lattice', 'composite_type', 'frobenius_closure_check', 'braid_word', 'paradice_map', 'universe_jump', 'signature_manifold', 'jump_path_integral'}
    if name in _MODO_JUMP_TOOLS:
        try:
            from modot.jump_tools import dispatch as jump_dispatch
            args = _coerce(name, positional)
            return jump_dispatch(name, args)
        except Exception as exc:
            return {"status": "error", "tool": name, "error": str(exc)}
    if name in ("m3", "m3iosis"):
        return call_m3(positional[0] if positional else "--help", positional[1:])
    if name in ("gm", "g-momonados"):
        return _call_binary(GMOMONADOS_BIN, "gm", positional[0] if positional else "help", positional[1:])
    # Godel tools - handle directly
    if name in ("godel", "godel_decode", "godel_encode", "godel_check", "godel_unbraid",
                "godel_braid", "godel_relation", "godel_analyze", "godel_lte2",
                "godel_product", "godel_frame", "godel_shiab", "godel_separate", "godel_factor"):
        if name == "godel":
            cmd = positional[0] if positional else "help"
            cmd_args = positional[1:]
        elif name == "godel_decode":
            cmd = "decode"
            cmd_args = positional
        elif name == "godel_encode":
            cmd = "encode"
            cmd_args = positional
        elif name == "godel_check":
            cmd = "check"
            cmd_args = positional
        elif name == "godel_unbraid":
            cmd = "unbraid"
            cmd_args = positional
        elif name == "godel_braid":
            cmd = "braid"
            cmd_args = positional
        elif name == "godel_relation":
            cmd = "relation"
            cmd_args = positional
        elif name == "godel_analyze":
            cmd = "analyze"
            cmd_args = positional
        elif name == "godel_lte2":
            cmd = "lte2"
            cmd_args = positional
        elif name == "godel_product":
            cmd = "product"
            cmd_args = positional
        elif name == "godel_frame":
            cmd = "frame"
            cmd_args = positional
        elif name == "godel_shiab":
            cmd = "shiab"
            cmd_args = positional
        elif name == "godel_separate":
            cmd = "separate"
            cmd_args = positional
        elif name == "godel_factor":
            cmd = "factor"
            cmd_args = positional
        else:
            cmd = name.replace("godel_", "")
            cmd_args = positional
        return call_godel(cmd, cmd_args)

    # Vox tools - handle directly
    if name in ("vox", "vox_factor", "vox_factor_glut", "vox_factor_membrane", "vox_scout",
                "vox_scout_membrane", "vox_lift", "vox_verdict", "vox_circuit", "vox_word",
                "vox_prime_check", "vox_coprime", "vox_morphism_factor", "vox_construct_carrier",
                "vox_extract_factor", "vox_self", "vox_tables", "vox_classify"):
        # For vox_factor, vox_factor_glut, etc., the command is the tool name without vox_ prefix
        if name == "vox":
            # vox takes the command as first arg
            cmd = positional[0] if positional else "help"
            cmd_args = positional[1:]
        elif name == "vox_factor":
            cmd = "factor"
            cmd_args = positional
        elif name == "vox_factor_glut":
            cmd = "factor-glut"
            cmd_args = positional
        elif name == "vox_factor_membrane":
            cmd = "factor-membrane"
            cmd_args = positional
        elif name == "vox_scout":
            cmd = "scout"
            cmd_args = positional
        elif name == "vox_scout_membrane":
            cmd = "scout-membrane"
            cmd_args = positional
        elif name == "vox_lift":
            cmd = "lift"
            cmd_args = positional
        elif name == "vox_verdict":
            cmd = "verdict"
            cmd_args = positional
        elif name == "vox_circuit":
            cmd = "circuit"
            cmd_args = positional
        elif name == "vox_word":
            cmd = "word"
            cmd_args = positional
        elif name == "vox_prime_check":
            cmd = "prime-check"
            cmd_args = positional
        elif name == "vox_coprime":
            cmd = "coprime"
            cmd_args = positional
        elif name == "vox_morphism_factor":
            cmd = "morphism-factor"
            cmd_args = positional
        elif name == "vox_construct_carrier":
            cmd = "construct-carrier"
            cmd_args = positional
        elif name == "vox_extract_factor":
            cmd = "extract-factor"
            cmd_args = positional
        elif name == "vox_self":
            cmd = "self"
            cmd_args = positional
        elif name == "vox_tables":
            cmd = "tables"
            cmd_args = positional
        elif name == "vox_classify":
            cmd = "classify"
            cmd_args = positional
        else:
            cmd = name.replace("vox_", "")
            cmd_args = positional
        return call_vox(cmd, cmd_args)

    # Structural verbs (native ask_native CLI verbs) — shell out to ask.bin
    _STRUCTURAL_VERBS = {
        "click", "switch", "excite", "ascend", "descend", "filter",
        "phase_reconstruct", "set", "homolyze", "scan", "complement",
        "cycle", "pathway", "polymerize", "star", "broadcast",
        "close", "material", "modulus", "arrange", "forge",
        "compare", "dope", "fuse", "cleave", "anneal",
        "distill", "fdistill", "sublime", "crystallize", "cocrystallize",
        "seed", "tlc", "column", "fpt", "trap", "stain",
        "register", "recall", "imscribe", "ob3ect", "imasm", "calc",
    }
    if name in _STRUCTURAL_VERBS:
        return _call_structural_tool(name, positional)

    if name not in IG_TOOL_ARGS:
        return {"status": "skip", "reason": f"{name!r} not an IG catalog tool"}
    return call(name, _coerce(name, positional), iteration)


def _selftest() -> None:
    assert available(), f"IG corpus not loadable from {IG_ROOT}"
    n = len(tool_names())
    cc = call("crystal_count", {})
    assert cc.get("count") == 17280000, cc
    dec = parse_and_call("TOOL: crystal_decode 0")
    assert dec.get("status") in ("ok", "error"), dec
    print(f"ig_tools: {n} IG catalog tools live from {IG_ROOT}")
    print(f"  crystal_count -> {cc.get('count')}")
    print(f"  crystal_decode 0 -> {str(dec)[:120]}")


def batch_compose(plan_path: str, receipt_path: str) -> Dict[str, Any]:
    """Compose and register a dependency-ordered plan with one catalog commit.

    Coordinates come only from compute_join/meet/tensor. Registration uses the
    existing encode_system tool; its ordinary per-entry persistence is deferred
    until the complete batch has passed, then the same catalog writer commits.
    """
    import hashlib
    import json
    from IG_inquiry import PRIMITIVE_ORDER

    engine = dispatcher()
    catalog = engine.catalog
    path = Path(catalog._catalog_path)
    before_bytes = path.read_bytes()
    before = json.loads(before_bytes)
    before_entries = before.get("imscriptions", []) if isinstance(before, dict) else before
    plan = json.loads(Path(plan_path).read_text(encoding="utf-8"))
    results = []
    expected = {}
    saved_path = catalog._catalog_path
    catalog._catalog_path = None
    try:
        for recipe in plan:
            name = recipe["original"]
            parts = recipe["operands"]
            if not parts or len(parts) < 2:
                raise IGToolError(f"{name}: incomplete composition recipe")
            operation = "meet" if name.startswith("polymer_meet_") else (
                "tensor" if name.startswith("polymer_tensor_") else "join")
            left = parts[0]
            steps = []
            for index, right in enumerate(parts[1:], 1):
                target = name if index == len(parts) - 1 else f"{name}__prefix_{index}"
                result = engine.dispatch(f"compute_{operation}", {"name_a": left, "name_b": right}, 0)
                if result.get("status") != "ok":
                    raise IGToolError(f"{name}: {result}")
                notation = result["result_notation"]
                registered = engine.dispatch("encode_system", {
                    "name": target, "tuple": notation,
                    "description": f"MoDoT compute_{operation}({left}, {right}); batch receipt {receipt_path}",
                }, 0)
                if registered.get("status") not in ("ok", "updated") or registered.get("notation") != notation:
                    raise IGToolError(f"{target}: registration did not preserve tool output: {registered}")
                expected[target] = catalog.get(target).copy()
                steps.append({"target": target, "computation": result, "registration": registered})
                left = target
            results.append({"name": name, "operation": operation, "operands": parts, "steps": steps})
        if path.read_bytes() != before_bytes:
            raise IGToolError("main catalog changed during preparation; batch was not committed")
        # Commit exactly the registered tool outputs while retaining the full
        # source envelope and every unrelated record, including empty names or
        # repeated records which a name-indexed whole-catalog save would fold.
        entries = []
        existing_names = set()
        for entry in before_entries:
            name = entry.get("name")
            existing_names.add(name)
            if name in expected:
                entry = dict(entry)
                entry.update(expected[name])
                entry["description"] = catalog._descriptions[name]
            entries.append(entry)
        for name, values in expected.items():
            if name not in existing_names:
                entries.append({"name": name, "description": catalog._descriptions[name], **values})
        payload = dict(before) if isinstance(before, dict) else entries
        if isinstance(payload, dict):
            payload["imscriptions"] = entries
        pending = path.with_suffix(path.suffix + ".batch.tmp")
        pending.write_text(json.dumps(payload, ensure_ascii=False, indent=2), encoding="utf-8")
        if path.read_bytes() != before_bytes:
            pending.unlink()
            raise IGToolError("main catalog changed before commit; batch was not committed")
        pending.replace(path)
        after = json.loads(path.read_text(encoding="utf-8"))
        entries = after.get("imscriptions", []) if isinstance(after, dict) else after
        readback = {entry["name"]: entry for entry in entries}
        for name, values in expected.items():
            if any(readback.get(name, {}).get(key) != values[key] for key in PRIMITIVE_ORDER):
                raise IGToolError(f"{name}: committed readback differs from the computed tuple")
        if [entry for entry in entries if entry.get("name") not in expected] != [
            entry for entry in before_entries if entry.get("name") not in expected
        ]:
            raise IGToolError("unrelated records changed")
        receipt = {"status": "ok", "catalog": str(path), "requested": len(plan),
                   "reissued": len(results), "registered_results": len(expected),
                   "before_sha256": hashlib.sha256(before_bytes).hexdigest(),
                   "after_sha256": hashlib.sha256(path.read_bytes()).hexdigest(), "results": results}
        Path(receipt_path).write_text(json.dumps(receipt, ensure_ascii=False, indent=2), encoding="utf-8")
        return {key: value for key, value in receipt.items() if key != "results"}
    finally:
        catalog._catalog_path = saved_path


def _main(argv: List[str]) -> int:
    """CLI entry so the native (Rust) loop can shell one IG tool per call.

        python3 -m modot.ig_tools call <verb> <arg> <arg> ...
        python3 -m modot.ig_tools names
        python3 -m modot.ig_tools selftest

    `call` prints the tool's real result as JSON and exits nonzero on error, so
    ask_native can capture it exactly as it captures close/forge/ob3ect.
    """
    import json
    if argv and argv[0] == "batch-compose":
        if len(argv) != 3:
            print(json.dumps({"status": "error", "error": "batch-compose PLAN.json RECEIPT.json"}))
            return 2
        try:
            _deshadow_navigators()
            print(json.dumps(batch_compose(argv[1], argv[2]), ensure_ascii=False))
            return 0
        except Exception as exc:
            print(json.dumps({"status": "error", "error": str(exc)}, ensure_ascii=False))
            return 1
    if not argv or argv[0] in ("selftest", "--selftest"):
        _selftest()
        return 0
    if argv[0] == "names":
        print("\n".join(tool_names()))
        return 0
    if argv[0] == "call":
        if len(argv) < 2:
            print(json.dumps({"status": "error", "error": "call needs a verb"}))
            return 2
        # Use parse_and_call to handle both IG and Vox tools
        line = shlex.join(argv[1:])
        result = parse_and_call(line)
        print(json.dumps(result, ensure_ascii=False, default=str))
        return 0 if result.get("status") not in ("error",) else 1
    print(json.dumps({"status": "error", "error": f"unknown subcommand {argv[0]!r}"}))
    return 2


if __name__ == "__main__":
    sys.exit(_main(sys.argv[1:]))
