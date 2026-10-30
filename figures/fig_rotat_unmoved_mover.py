#!/usr/bin/env python3
"""
Figure: ROTAT, the first op-opcode — the unmoved mover.

ROTAT is the cyclic shift of an arrangement: the ring automorphism, the
Weyl-Heisenberg shift X on Z/dZ. It moves every node of the word and is itself
unmoved — the multiset (the family signature) is invariant under it, and
ROTAT^|w| returns the word to itself. This figure renders exactly that.

Nothing here is drawn by hand: the arrangement, every rotation, the orbit, and
the invariant signature are all computed by the canonical implementation
(IMSCRIBr/tokens.py: rotat, rotat_orbit, signature, token_name).

Left panel  — the word as a ring of colored nodes, with the ROTAT arrow (k->k+1)
              running around the rim. At the still centre: the invariant
              multiset signature, the quantity ROTAT moves everything around but
              never touches. The unmoved point the wheel turns on.
Right panel — the full ROTAT orbit: all |w| rotations, each sharing one
              fingerprint (the same signature), so the orbit is the equivalence
              the canonical classes are read up to. ROTAT^|w| = identity closes
              the ring back onto its own stillness.
"""
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from matplotlib.patches import FancyArrowPatch

sys.path.insert(0, str(Path.home() / "imsgct" / "IMSCRIBr"))
from tokens import rotat, rotat_orbit, signature, token_name  # noqa: E402

# The demonstration word (a full fork block: init, imscribe, split, both
# branches, fuse, fix, anchor). Any word works; this one spans the grammar.
WORD = (0, 5, 6, 8, 9, 7, 11, 1)
N = len(WORD)

# Colour each token by its value, from a stable qualitative map (12 primitives).
CMAP = plt.get_cmap("twilight")
def tcolor(t):
    return CMAP((t % 12 + 0.5) / 12.0)


def ring_positions(n, radius=1.0, phase=np.pi / 2):
    # Positions clockwise from the top, so k->k+1 reads as a clockwise shift.
    ang = phase - 2 * np.pi * np.arange(n) / n
    return radius * np.cos(ang), radius * np.sin(ang), ang


def draw_ring(ax, word, title, show_shift):
    n = len(word)
    xs, ys, ang = ring_positions(n)
    for i, t in enumerate(word):
        ax.scatter([xs[i]], [ys[i]], s=1500, color=tcolor(t),
                   edgecolors="black", linewidths=1.5, zorder=3)
        ax.text(xs[i], ys[i], token_name(t), ha="center", va="center",
                fontsize=7.5, fontweight="bold", zorder=4,
                color="white" if (t % 12) in (0, 1, 10, 11) else "black")
    if show_shift:
        # A curved ROTAT arrow between consecutive rim slots (k -> k+1).
        for i in range(n):
            j = (i + 1) % n
            a = FancyArrowPatch((xs[i] * 0.72, ys[i] * 0.72),
                                (xs[j] * 0.72, ys[j] * 0.72),
                                connectionstyle="arc3,rad=-0.28",
                                arrowstyle="-|>", mutation_scale=13,
                                lw=1.4, color="#c0392b", zorder=2, alpha=0.85)
            ax.add_patch(a)
    ax.set_title(title, fontsize=11, pad=10)
    ax.set_xlim(-1.5, 1.5)
    ax.set_ylim(-1.5, 1.5)
    ax.set_aspect("equal")
    ax.axis("off")


def main():
    orbit = rotat_orbit(WORD)
    sig = signature(WORD)
    # The signature is ROTAT-invariant: verify it on the whole orbit.
    assert all(signature(w) == sig for w in orbit), "signature not ROTAT-invariant"
    assert rotat(WORD, N) == WORD, "ROTAT^|w| is not the identity"

    fig, (axL, axR) = plt.subplots(1, 2, figsize=(13, 6.4))

    # ----- Left: the ring, the ROTAT arrow, the still centre.
    draw_ring(axL, WORD, "ROTAT — every node moves, one thing does not", show_shift=True)
    axL.text(0, 0.18, "invariant", ha="center", va="center",
             fontsize=9, style="italic", color="#333")
    axL.text(0, -0.05, "signature", ha="center", va="center",
             fontsize=11, fontweight="bold", color="black")
    axL.text(0, -0.30, str(sig), ha="center", va="center", fontsize=7.5,
             family="monospace", color="#c0392b", wrap=True)
    axL.text(0, -1.42, "the multiset ROTAT turns everything around,\nand never itself touches",
             ha="center", va="center", fontsize=8.5, style="italic", color="#555")

    # ----- Right: the full orbit stacked, one fingerprint shared by all.
    orbit_ordered = [rotat(WORD, k) for k in range(N)]
    axR.set_title(f"the ROTAT orbit — {N} rotations, one fingerprint",
                  fontsize=11, pad=10)
    for k, w in enumerate(orbit_ordered):
        y = N - 1 - k
        for i, t in enumerate(w):
            axR.scatter([i], [y], s=260, color=tcolor(t),
                        edgecolors="black", linewidths=0.8, zorder=3)
        axR.text(-0.9, y, f"ROTAT$^{{{k}}}$", ha="right", va="center", fontsize=8.5,
                 family="monospace")
    axR.text((N - 1) / 2, -1.15,
             r"all share signature $\Rightarrow$ same class;  ROTAT$^{%d}$ = identity" % N,
             ha="center", va="center", fontsize=9, color="#c0392b")
    axR.set_xlim(-2.2, N - 0.4)
    axR.set_ylim(-1.6, N - 0.4)
    axR.axis("off")

    fig.suptitle("The first op-opcode: the operator that moves the whole word, and is itself unmoved",
                 fontsize=12.5, y=0.99)
    fig.tight_layout(rect=(0, 0, 1, 0.96))

    out = Path(__file__).parent
    fig.savefig(out / "fig_rotat_unmoved_mover.pdf", bbox_inches="tight", facecolor="white")
    fig.savefig(out / "fig_rotat_unmoved_mover.png", bbox_inches="tight",
                facecolor="white", dpi=300)
    print("wrote fig_rotat_unmoved_mover.{pdf,png} | orbit", len(orbit),
          "| signature invariant:", all(signature(w) == sig for w in orbit))


if __name__ == "__main__":
    main()
