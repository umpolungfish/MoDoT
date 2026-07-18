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
import shlex
import sys
from typing import Any, Dict, List, Optional

# Where the Imscribing Grammar corpus lives.
IG_ROOT = os.environ.get(
    "IG_ROOT", os.path.expanduser("~/imsgct/imscribing_grammar")
)

# Tool name -> ordered positional arg names, for parsing `TOOL: name a b` lines.
# Names with no positional args (or dict/list args) take a trailing JSON/kv blob.
IG_TOOL_ARGS: Dict[str, List[str]] = {
    "lookup_catalog": ["keyword"],
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
    "riemann_xi_info": [],
    "ask_question": ["question"],
    "record_insight": ["text", "plane", "confidence", "systems"],
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
    catalog = SessionCatalog(catalog_path=CATALOG_PATH)
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
    try:
        return dispatcher().dispatch(name, args, iteration)
    except Exception as exc:
        return {"status": "error", "tool": name, "error": str(exc)}


_PRIMITIVE_GLYPHS = {"Ð", "Þ", "Ř", "Φ", "ƒ", "Ç", "Γ", "ɢ", "⊙", "Ħ", "Σ", "Ω"}


def _coerce(name: str, positional: List[str]) -> Dict[str, Any]:
    spec = IG_TOOL_ARGS.get(name)
    if spec is None:
        raise IGToolError(f"unknown IG tool: {name!r}")
    # The door for swapped order: agents emit `primitive_peel Φ <name>` as often as
    # the documented `primitive_peel <name> Φ`, and the first arg then errors as
    # "Unknown system: Φ". When a spec is [name, primitive] and the args arrive
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


def _main(argv: List[str]) -> int:
    """CLI entry so the native (Rust) loop can shell one IG tool per call.

        python3 -m modot.ig_tools call <verb> <arg> <arg> ...
        python3 -m modot.ig_tools names
        python3 -m modot.ig_tools selftest

    `call` prints the tool's real result as JSON and exits nonzero on error, so
    ask_native can capture it exactly as it captures close/forge/ob3ect.
    """
    import json
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
        verb, rest = argv[1], argv[2:]
        if verb not in IG_TOOL_ARGS:
            print(json.dumps({"status": "skip", "reason": f"{verb!r} not an IG catalog tool"}))
            return 3
        result = call(verb, _coerce(verb, rest))
        print(json.dumps(result, ensure_ascii=False, default=str))
        return 0 if result.get("status") not in ("error",) else 1
    print(json.dumps({"status": "error", "error": f"unknown subcommand {argv[0]!r}"}))
    return 2


if __name__ == "__main__":
    sys.exit(_main(sys.argv[1:]))
