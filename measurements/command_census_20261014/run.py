"""Run MoDoT's direct command families sequentially, without inference."""
import json
import os
from pathlib import Path
import signal
import subprocess
import time

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
RING = ["monad", "the_stone", "monad", "the_stone"]
WORD = "⊢∈≻⊤∋⊣"
CASES = [
    ("cycle", ["--cycle", "the_stone", "monad", "--certify"]),
    ("pathway", ["--pathway", "the_stone", "the_stone", "comonad_goedel", "--certify"]),
    ("click", ["--click", "monad", "the_stone"]),
    ("meet", ["--meet", "monad", "the_stone"]),
    ("switch", ["--switch", "monad", "the_stone"]),
    ("excite", ["--excite", "monad"]),
    ("set", ["--set", "monad", "the_stone", "--excite", "--catalyst", "comonad_goedel"]),
    ("scan-mediators", ["--set", "monad", "the_stone", "--excite", "--scan-mediators", "--top", "5"]),
    ("homolyze", ["--homolyze", "monad", "the_stone"]),
    ("annihilate", ["--annihilate", "monad"]),
    ("recalibrate", ["--recalibrate", "monad", "protection"]),
    ("complement", ["--complement", "monad"]),
    ("polymerize", ["--polymerize", *RING, "--props"]),
    ("close", ["--polymerize", "monad", "the_stone", "monad", "--close"]),
    ("modulus", ["--polymerize", *RING, "--modulus"]),
    ("arrange", ["--polymerize", *RING, "--arrange", "--props"]),
    ("forge", ["--forge", *RING, "--register", "modot_demo_cycle_20261014", "--export", str(HERE / "macrocycle.json")]),
    ("recall", ["--recall", "modot_demo_cycle_20261014", "--export", str(HERE / "macrocycle_recalled.json")]),
    ("compare", ["--compare", *RING, "vs", "monad", "the_stone"]),
    ("dope", ["--dope", *RING, "with", "comonad_goedel"]),
    ("fuse", ["--fuse", "monad", "the_stone", "+", "monad", "the_stone"]),
    ("cleave", ["--cleave", *RING]),
    ("anneal", ["--anneal", *RING]),
    ("distill", ["--distill", "monad", "the_stone", "comonad_goedel"]),
    ("fdistill", ["--fdistill", "monad", "the_stone", "comonad_goedel"]),
    ("sublime", ["--sublime", "monad"]),
    ("crystallize", ["--crystallize", *RING]),
    ("cocrystallize", ["--cocrystallize", "monad", "the_stone"]),
    ("seed", ["--seed", *RING, "with", "the_stone"]),
    ("tlc", ["--tlc", "monad", "the_stone", "comonad_goedel"]),
    ("column", ["--column", "monad", "the_stone", "comonad_goedel", "on", "imasm"]),
    ("gradient", ["--gradient", "monad", "the_stone", "on", "imasm", "from", "monad", "to", "the_stone", "--steps", "5"]),
    ("fpt", ["--fpt", "monad", "the_stone", "comonad_goedel"]),
    ("trap", ["--trap", "monad", "the_stone"]),
    ("stain", ["--stain", "the_stone", "monad", "comonad_goedel"]),
    ("star", ["--star", *RING]),
    ("filter", ["--filter", "monad", "the_stone"]),
    ("ascend", ["--ascend", "monad"]),
    ("descend", ["--descend", "monad"]),
    ("broadcast", ["--broadcast", "monad", "--top", "5"]),
    ("phase-reconstruct", ["--phase-reconstruct", *RING]),
    ("calc", ["--calc", "-(3/2 + 1/2)"]),
    ("ringspec", ["--ringspec", "1", "1", "1", "1"]),
    ("riemann-sic", ["--riemann-sic"]),
    ("riemann-hilbert", ["--riemann-hilbert"]),
    ("windings", ["--windings", "656.28nm", "486.13nm"]),
    ("imasm", ["--imasm", "check", WORD]),
    ("rotat", ["--rotat", WORD]),
    ("weight", ["--weight", WORD]),
    ("banked", ["--banked", "⊢∈⊤≺∋⊡⊣"]),
    ("trans", ["--trans", WORD]),
    ("insert", ["--insert", "⊢∈⊤∋≺⊡⊣"]),
    ("frames", ["--frames", WORD]),
    ("kernel", ["--kernel", "ringspec", "1", "1", "1", "1"]),
    ("tool-gm", ["--tool", "gm", "qc", "H"]),
    ("tool-vox", ["--tool", "vox", "verdict", WORD]),
    ("tool-godel", ["--tool", "godel", "encode", "217728509307789974617049079437290125283"]),
    ("lean", ["--lean", str(ROOT.parent / "p4rakernel/p4ramill/Imscribing/Paraconsistent/BelnapSplitFuse.lean")]),
    ("mask", ["--click", "monad", "the_stone", "--mask", "bip39_key_*"]),
]


# Additional successful inputs and controls retained from the direct review.
CASES.extend([('stain_valid', ['--stain', 'chiral', 'monad', 'the_stone', 'comonad_goedel']), ('set_donor', ['--set', 'the_stone', 'monad', '--excite', '--catalyst', 'extrapolation_93_units']), ('annihilate_nontrivial', ['--annihilate', 'the_stone']), ('compare_rings', ['--compare', 'monad', 'the_stone', 'monad', 'the_stone', 'vs', 'monad', 'the_stone', 'monad', 'thermodynamic_entropy']), ('fuse_rings', ['--fuse', 'monad', 'the_stone', 'monad', 'the_stone', '+', 'monad', 'the_stone', 'monad', 'the_stone']), ('cleave_eight', ['--cleave', 'monad', 'the_stone', 'monad', 'the_stone', 'monad', 'the_stone', 'monad', 'the_stone']), ('gradient_reverse', ['--gradient', 'monad', 'the_stone', 'on', 'imasm', 'from', 'the_stone', 'to', 'monad', '--steps', '5']), ('plasma_repaired', ['--plasma', 'monad']), ('catalog_options', ['--click', 'monad', 'the_stone', '--catalog', '/home/mrnob0dy666/imsgct/imscribing_grammar/IG_catalog.json', '--theta', '0.5', '--verbose', '--no-think']), ('help', ['--help']), ('plasma_ring_partner', ['--plasma', 'the_stone'])])


def run(name, args):
    start = time.monotonic()
    child = subprocess.Popen([str(ROOT / "ask"), *args], cwd=ROOT,
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                             text=True, start_new_session=True)
    timed_out = False
    try:
        stdout, stderr = child.communicate(timeout=60)
    except subprocess.TimeoutExpired:
        timed_out = True
        os.killpg(child.pid, signal.SIGTERM)
        try:
            stdout, stderr = child.communicate(timeout=5)
        except subprocess.TimeoutExpired:
            os.killpg(child.pid, signal.SIGKILL)
            stdout, stderr = child.communicate()
    result = dict(name=name, argv=args, exit=child.returncode, timeout=timed_out,
                  seconds=time.monotonic()-start, stdout=stdout, stderr=stderr)
    (HERE / f"{name}.json").write_text(json.dumps(result, indent=2, ensure_ascii=False)+"\n")
    print(f"{name}: exit={child.returncode}, timeout={timed_out}, {result['seconds']:.3f}s", flush=True)
    return result


if __name__ == "__main__":
    results = []
    for name, args in CASES:
        results.append(run(name, args))
        (HERE / "runs.json").write_text(json.dumps(results, indent=2, ensure_ascii=False)+"\n")
