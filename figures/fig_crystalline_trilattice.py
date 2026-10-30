#!/usr/bin/env python3
"""
Figure: the SIXTEEN_3 trilattice, rendered as a crystalline lattice.

The algebraic space is the real trilattice built and verified this session
(Shramko, Dunn & Takenaka, "The Trilattice of Constructive Truth Values", J.
Logic and Computation 11(6):761-788, 2001) — the full powerset P({T,F,t,f}),
16 elements, three partial orderings (information ≤_i, truth ≤_t,
constructivity ≤_c). This is NOT a decorative lattice: every vertex position,
every edge, and every axis below is computed directly from that structure,
imported from the canonical implementation (IMSCRIBr/tokens16_3.py), not
redrawn by hand.

Embedding:
  z (vertical)   = |x|, the popcount — the ≤_i information level (0..4),
                   matching the paper's own "five informational levels"
                   (Table 1: sizes 1, 4, 6, 4, 1).
  x (horizontal) = (#T + #t) - (#F + #f)  — the ≤_t truth-order projection.
  y (horizontal) = (#T + #F) - (#t + #f)  — the ≤_c constructivity projection.

Edges are the covering relation of the Boolean lattice B4 (Hasse diagram: an
edge iff one side is the other plus exactly one base value) — 32 edges,
colored by WHICH of the four base values {T,F,t,f} the edge adds, so the
figure reads as a real crystal: four bond directions, exactly as a
tetrahedral (diamond-like) lattice is conventionally drawn.
"""
import sys
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import matplotlib.font_manager as fm
from mpl_toolkits.mplot3d import Axes3D  # noqa: F401
from mpl_toolkits.mplot3d.art3d import Line3DCollection
import numpy as np

sys.path.insert(0, str(Path.home() / "imsgct" / "IMSCRIBr"))
from tokens16_3 import BASE, ALL_16, reg_name  # noqa: E402

plt.rcParams.update({
    "font.family": "serif",
    "font.serif": ["STIX Two Text", "Times New Roman", "DejaVu Serif"],
    "mathtext.fontset": "stix",
    "font.size": 11,
    "axes.linewidth": 0.6,
    "figure.facecolor": "white",
})

BASE_COLOR = {
    "T": "#1B4F72",   # deep blue    — constructive truth
    "F": "#943126",   # deep red     — constructive falsity
    "t": "#1F618D",   # lighter blue — acceptable
    "f": "#B03A2E",   # lighter red  — rejectable
}
BASE_COLOR = {"T": "#1B4F72", "F": "#943126", "t": "#5DADE2", "f": "#EC7063"}

LEVEL_COLOR = {0: "#7F8C8D", 1: "#2E86C1", 2: "#8E44AD", 3: "#D68910", 4: "#27AE60"}


# The constructivity axis is scaled DOWN relative to truth/information — at
# 1:1 scale several nodes and bonds sit nearly collinear from a legible
# camera angle and overlap in projection (e.g. f/Ff, T/Tt). Compressing ≤_c
# alone peels those apart without changing the actual lattice structure or
# which points are distinct — it's a rendering choice, not a re-embedding.
CONSTRUCTIVITY_SCALE = 0.55


def coords(x: frozenset) -> tuple:
    z = len(x)
    tx = (("T" in x) + ("t" in x)) - (("F" in x) + ("f" in x))
    ty = (("T" in x) + ("F" in x)) - (("t" in x) + ("f" in x))
    return (float(tx), float(ty) * CONSTRUCTIVITY_SCALE, float(z))


def covers(x: frozenset, y: frozenset) -> str | None:
    """Return the base value y adds over x if y covers x in B4, else None."""
    if x <= y and len(y) == len(x) + 1:
        return next(iter(y - x))
    return None


def main():
    pts = {x: coords(x) for x in ALL_16}

    edges = []
    for x in ALL_16:
        for y in ALL_16:
            b = covers(x, y)
            if b is not None:
                edges.append((x, y, b))

    fig = plt.figure(figsize=(11, 11), dpi=300)
    ax = fig.add_subplot(111, projection="3d")
    ax.set_facecolor("white")
    fig.patch.set_facecolor("white")
    # mplot3d reserves a large internal margin around the actual data that
    # ordinary subplots_adjust/tight_layout cannot remove, so the crystal
    # renders small-and-centered regardless of figsize. Rather than fight
    # that by shoving the axes rectangle around (tried — it pushes the
    # title/legend partially off-canvas instead of fixing anything), leave
    # the axes in its normal default position and crop the SAVED raster
    # precisely to the real projected pixel bounds of the data, computed
    # below via proj3d — the same transform mplot3d itself uses.

    # Small deterministic jitter so same-level, same-(tx,ty) points that would
    # otherwise coincide (there are none here, B4's covering embedding is
    # already injective) still render legibly; kept at 0 but structured for
    # future denser lattices.
    jitter = {x: (0.0, 0.0, 0.0) for x in ALL_16}

    # ── bonds, colored by which base value they add (the 4 crystal axes) ──
    segs_by_base = {b: [] for b in BASE}
    for x, y, b in edges:
        p0 = np.array(pts[x]) + np.array(jitter[x])
        p1 = np.array(pts[y]) + np.array(jitter[y])
        segs_by_base[b].append((p0, p1))

    axis_label = {"T": "+T bond", "F": "+F bond", "t": "+t bond", "f": "+f bond"}
    for b, segs in segs_by_base.items():
        lc = Line3DCollection(segs, colors=BASE_COLOR[b], linewidths=1.6,
                               alpha=0.85, label=axis_label[b])
        ax.add_collection3d(lc)

    # ── light drop-lines to the z=0 floor, one per vertex ──
    # A standard depth-legibility aid (stem-plot style): without these, a 3D
    # scatter/lattice is genuinely ambiguous about how far "into" or "out of"
    # the screen a point sits. A thin vertical from each vertex straight down
    # to its (x, y, 0) footprint fixes that at a glance, at every level
    # including the floor itself (drawn with zero length there, harmless).
    floor_segs = [(np.array(pts[x]), np.array([pts[x][0], pts[x][1], 0.0])) for x in ALL_16]
    ax.add_collection3d(Line3DCollection(floor_segs, colors="#B5B8BB", linewidths=0.7,
                                          linestyles=(0, (2, 2)), alpha=0.55, zorder=1))

    # ── vertices ──
    for x in ALL_16:
        p = np.array(pts[x]) + np.array(jitter[x])
        lvl = len(x)
        ax.scatter(*p, s=260 if lvl in (0, 4) else 190, color=LEVEL_COLOR[lvl],
                   edgecolors="black", linewidths=0.7, depthshade=True, zorder=5)
        label = reg_name(x)
        ax.text(p[0], p[1], p[2] + 0.16, f"${label}$" if label in ("N", "A") else label,
                fontsize=9.5, ha="center", va="bottom", zorder=6)

    ax.set_xlabel(r"$\leq_t$   (truth order)", labelpad=14, fontsize=12)
    ax.set_ylabel(r"$\leq_c$   (constructivity order)", labelpad=14, fontsize=12)
    ax.set_zlabel(r"$\leq_i$   (information level, $|x|$)", labelpad=10, fontsize=12)
    # mplot3d auto-scales each axis to fill its box from whatever data range
    # it's given — so compressing the y-COORDINATES while leaving the axis to
    # autoscale just re-stretches them back to fill the same box, and the
    # render comes out pixel-identical. The compression only shows up if the
    # axis BOX itself is pinned to a range that does NOT shrink to match —
    # here, the same -2..2 box the truth axis uses, regardless of
    # CONSTRUCTIVITY_SCALE, so a smaller scale visibly buries the points
    # nearer the y=0 center plane instead of being silently re-expanded.
    ax.set_xlim(-2, 2)
    ax.set_ylim(-2, 2)
    ax.set_zlim(0, 4)
    ax.set_zticks([0, 1, 2, 3, 4])
    ax.set_xticks([-2, -1, 0, 1, 2])
    ax.set_yticks([v * CONSTRUCTIVITY_SCALE for v in (-2, -1, 0, 1, 2)])
    ax.view_init(elev=23, azim=38)
    ax.grid(False)
    for pane in (ax.xaxis, ax.yaxis, ax.zaxis):
        pane.pane.set_edgecolor("#dddddd")
        pane.pane.set_alpha(0.25)

    ax.set_title(
        r"$\mathbf{SIXTEEN_3}$ — the trilattice of constructive truth values"
        "\n" r"$P(\{T,F,t,f\})$, three independent orderings, rendered as a crystal",
        fontsize=13, pad=2,
    )

    handles, labels = ax.get_legend_handles_labels()
    ax.legend(handles, labels, loc="upper left", bbox_to_anchor=(0.0, 0.94),
              fontsize=9.5, frameon=False, title="bond = base value added",
              title_fontsize=9.5)

    caption = (
        r"$N=\emptyset$ (no information) $\to$ $A=\{T,F,t,f\}$ (full information). "
        "Vertical axis is the literal subset order (Def. 5.2, Shramko–Dunn–Takenaka "
        "2001); the two horizontal axes are the truth- and constructivity-order "
        "projections of the same 16-element carrier. Edges are the covering relation "
        r"of $B_4$ — 32 bonds, 4 directions, one per base value."
    )
    fig.text(0.5, 0.015, caption, ha="center", va="bottom", fontsize=8.3,
              wrap=True, color="#333333")

    # mplot3d always reserves room around the data for the cube to rotate
    # into view, independent of ax.set_position — the axis PANE fills the
    # frame while the crystal itself stays small in the middle of it.
    # Cropping to non-white pixels doesn't fix this either: the pane edges,
    # tick marks, and axis-label text are themselves non-white and span
    # nearly the whole canvas. The correct fix is to crop to the actual
    # PROJECTED pixel positions of the vertices (computed via mplot3d's own
    # proj3d transform, the same math it uses to place them on screen) —
    # not a guess, the real bounding box of the rendered crystal.
    from mpl_toolkits.mplot3d import proj3d

    fig.canvas.draw()
    xs_px, ys_px = [], []
    for x in ALL_16:
        p = pts[x]
        x2, y2, _ = proj3d.proj_transform(*p, ax.get_proj())
        px, py = ax.transData.transform((x2, y2))
        xs_px.append(px)
        ys_px.append(py)

    out = Path(__file__).parent
    fig.savefig(out / "fig_crystalline_trilattice.pdf", bbox_inches="tight", facecolor="white")

    fig_w_px, fig_h_px = fig.get_size_inches() * fig.dpi
    pad_px = 130  # room for vertex labels, which extend beyond the vertex point itself
    # The legend sits at the upper-LEFT of the axes (bbox_to_anchor x=0), well
    # left of any data point, so the right-side pad (just labels) and the
    # left-side pad (must also clear the legend box) are asymmetric. Likewise
    # the title/legend stack near the figure's top edge and the caption sits
    # near the bottom edge, both far outside the label padding alone.
    x0 = max(0, min(xs_px) - 850)   # clears the legend on the left
    x1 = min(fig_w_px, max(xs_px) + pad_px + 420)  # clears the "(truth order)" axis label on the right
    y0 = max(0, min(ys_px) - pad_px - 1200)   # clears the caption at the bottom
    y1 = min(fig_h_px, max(ys_px) + pad_px + 1150)  # clears the title + legend at the top
    from matplotlib.transforms import Bbox
    crop_bbox = Bbox([[x0 / fig.dpi, (fig_h_px - y1) / fig.dpi],
                       [x1 / fig.dpi, (fig_h_px - y0) / fig.dpi]])
    fig.savefig(out / "fig_crystalline_trilattice.png", bbox_inches=crop_bbox, facecolor="white", dpi=300)

    print(f"wrote {out}/fig_crystalline_trilattice.{{pdf,png}}")


if __name__ == "__main__":
    main()
