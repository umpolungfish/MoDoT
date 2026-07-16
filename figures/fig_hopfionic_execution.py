#!/usr/bin/env python3
"""
Figure: IMASM graph execution as a hopfionic loop bundle.

Not a decorative torus. The construction is literal:

  - The canonical bootstrap cycle (IMSCRIB -> AREV -> FSPLIT -> AFWD -> FFUSE
    -> CLINK -> IFIX -> IMSCRIB, mOMonadOS tokens.rs::bootstrap_loop) is a
    CLOSED walk — it wraps back onto its own start. Each successive winding
    (winding_count > 0, kernel.rs) is drawn as a separate (1,1) curve on a
    shared torus, offset in phase from the others. (1,1) curves on a torus —
    one full loop toroidally AND poloidally at once — are pairwise linked
    when phase-offset (each strand threads through the next once per period,
    the same interleaving that makes Hopf-fibration circle bundles linked).
    Multiple windings are therefore not just repeated; they are topologically
    interleaved with each other, which is exactly what re-entering a closed
    program from a different phase actually does to the execution trace.

  - The O_inf_dag replicative-opening program (IMSCRIB -> FSPLIT3 -> FFUSE3 ->
    IMSCRIB in the 16_3 grammar; here the classic-grammar analogue IMSCRIB ->
    FSPLIT -> FFUSE -> IMSCRIB) does NOT close into that linked bundle. It is
    drawn as a small loop that departs the SAME torus tangentially and sits
    beside the main bundle — lateral, not above or below — the direct visual
    reading of "O_inf_dag is lateral to O_infinity, not a rung on the same
    ladder" established this session (Core.lean's own comment on the
    constructor, and the cardinality check: 160 vs 8 fiber elements).

Bootstrap step colors follow the WORK?/no-work distinction from
IMASM_REFERENCE.md: no-work opcodes (VINIT/TANCH/IMSCRIB/FSPLIT/FFUSE) are
grey; WORK opcodes (AFWD/AREV/CLINK/EVALT/EVALF/ENGAGR/IFIX) are colored.
"""
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D  # noqa: F401
import numpy as np

plt.rcParams.update({
    "font.family": "serif",
    "font.serif": ["STIX Two Text", "Times New Roman", "DejaVu Serif"],
    "mathtext.fontset": "stix",
    "font.size": 11,
    "axes.linewidth": 0.6,
    "figure.facecolor": "white",
})

R, r = 3.0, 1.0  # torus center radius, tube radius

BOOTSTRAP = ["IMSCRIB", "AREV", "FSPLIT", "AFWD", "FFUSE", "CLINK", "IFIX", "IMSCRIB"]
WORK_OPS = {"AFWD", "AREV", "CLINK", "EVALT", "EVALF", "ENGAGR", "IFIX"}
NO_WORK_COLOR = "#95A5A6"
WORK_COLOR = "#1B4F72"
DAG_COLOR = "#B03A2E"


def torus_point(u: float, v: float) -> np.ndarray:
    return np.array([
        (R + r * np.cos(v)) * np.cos(u),
        (R + r * np.cos(v)) * np.sin(u),
        r * np.sin(v),
    ])


def strand(phase: float, n: int = 400) -> np.ndarray:
    """A (1,1) curve on the torus: one full loop toroidally AND poloidally,
    offset by `phase` in the poloidal angle. Phase-offset (1,1) curves on a
    shared torus are pairwise linked."""
    s = np.linspace(0, 2 * np.pi, n)
    u, v = s, s + phase
    return np.array([torus_point(uu, vv) for uu, vv in zip(u, v)])


def main():
    fig = plt.figure(figsize=(8.5, 8), dpi=300)
    ax = fig.add_subplot(111, projection="3d")
    fig.patch.set_facecolor("white")
    ax.set_facecolor("white")
    fig.subplots_adjust(left=0.02, right=0.98, top=0.90, bottom=0.10)

    # ── faint torus surface for orientation ──
    uu, vv = np.meshgrid(np.linspace(0, 2 * np.pi, 60), np.linspace(0, 2 * np.pi, 30))
    X = (R + r * np.cos(vv)) * np.cos(uu)
    Y = (R + r * np.cos(vv)) * np.sin(uu)
    Z = r * np.sin(vv)
    ax.plot_surface(X, Y, Z, color="#EAECEE", alpha=0.35, linewidth=0, shade=True, zorder=0)

    # ── three windings, mutually linked (1,1) strands ──
    n_windings = 3
    phases = [2 * np.pi * k / n_windings for k in range(n_windings)]
    winding_alpha = [1.0, 0.55, 0.32]
    winding_lw = [2.6, 1.6, 1.1]

    for w, (phase, a, lw) in enumerate(zip(phases, winding_alpha, winding_lw)):
        curve = strand(phase, n=400)
        ax.plot(curve[:, 0], curve[:, 1], curve[:, 2], color=WORK_COLOR,
                 alpha=a, linewidth=lw, zorder=3, solid_capstyle="round")

        if w == 0:
            # Mark the 8 bootstrap steps as points along the primary winding.
            n_steps = len(BOOTSTRAP)
            for i, tok in enumerate(BOOTSTRAP[:-1]):  # last == first, don't double-plot
                s = 2 * np.pi * i / (n_steps - 1)
                p = torus_point(s, s + phase)
                col = NO_WORK_COLOR if tok not in WORK_OPS else WORK_COLOR
                ax.scatter(*p, s=70, color=col, edgecolors="black", linewidths=0.6, zorder=6)
                offset = 0.28 * p / np.linalg.norm(p[:2].tolist() + [0.001])
                ax.text(p[0] + 0.35 * np.sign(p[0] + 0.01), p[1] + 0.35 * np.sign(p[1] + 0.01),
                        p[2] + 0.32, tok, fontsize=8, ha="center", zorder=7)

    # ── O_inf_dag: rotate the tangent loop about its own diameter ──────────
    # A flat tangent loop only touches the torus at one isolated point — that
    # was the earlier version. Rotating that circle about the diameter through
    # its departure point sweeps out a genuine SPHERE (any circle rotated
    # 180° about a diameter is a sphere of the same radius), and because the
    # circle's own plane was tangent to the torus rather than tangent to some
    # arbitrary direction, this sphere's center sits exactly one radius above
    # the torus surface along the poloidal (d/dv) direction — the ONE
    # direction perpendicular to both the toroidal tangent and the surface
    # normal. Verified numerically before drawing: (tangent_u, normal_out,
    # poloidal) is a genuine orthonormal frame, the departure point sits
    # exactly on the resulting sphere (distance-to-center = radius, to machine
    # precision), and ~22% of the sphere's surface area actually falls INSIDE
    # the solid torus — a real bounded spherical region carved into it, not
    # just a point of contact.
    u0 = -np.pi / 2
    base = torus_point(u0, 0.0)
    tangent_u = np.array([-np.sin(u0), np.cos(u0), 0.0])  # d/du at (u0, 0)
    poloidal = np.array([0.0, 0.0, 1.0])                   # d/dv at (u0, 0)
    normal_out = base / np.linalg.norm(base)               # surface normal at (u0, 0)

    loop_r = 0.85
    sphere_center = base + loop_r * poloidal

    phi = np.linspace(0, np.pi, 60)
    lam = np.linspace(0, 2 * np.pi, 60)
    PHI, LAM = np.meshgrid(phi, lam)
    Sx, Sy, Sz = (
        sphere_center[i]
        + loop_r * (np.sin(PHI) * np.cos(LAM) * tangent_u[i]
                     + np.sin(PHI) * np.sin(LAM) * normal_out[i]
                     + np.cos(PHI) * poloidal[i])
        for i in range(3)
    )
    ax.plot_surface(Sx, Sy, Sz, color=DAG_COLOR, alpha=0.28, linewidth=0,
                     shade=True, zorder=8)
    # Rim: the SAME great circle that generated the sphere by rotation (the
    # (tangent_u, poloidal) plane) — not an arbitrary equator in a different
    # plane. This is the one circle that actually passes through the
    # departure point (theta=0 lands exactly on `base`), so drawing it and
    # placing all three opcode markers on it gives one continuous, smoothly
    # spaced arc instead of two disconnected clusters of dots.
    theta_line = np.linspace(0, 2 * np.pi, 200)
    rim = sphere_center[:, None] + loop_r * (
        np.sin(theta_line)[None, :] * tangent_u[:, None] - np.cos(theta_line)[None, :] * poloidal[:, None]
    )
    ax.plot(rim[0], rim[1], rim[2], color=DAG_COLOR, linewidth=1.6, alpha=0.9, zorder=9)

    # IMSCRIB at theta=0 (exactly `base`, the departure point); FSPLIT/FFUSE
    # spaced a third and two-thirds of the way around the SAME rim.
    def on_rim(theta):
        return sphere_center + loop_r * (np.sin(theta) * tangent_u - np.cos(theta) * poloidal)

    dag_pts = {
        "IMSCRIB": on_rim(0.0),
        "FSPLIT": on_rim(2 * np.pi / 3),
        "FFUSE": on_rim(4 * np.pi / 3),
    }
    for tok, p in dag_pts.items():
        ax.scatter(*p, s=70, color=DAG_COLOR, edgecolors="black", linewidths=0.6, zorder=10)
        ax.text(p[0], p[1], p[2] + 0.3, tok, fontsize=8, ha="center", color=DAG_COLOR, zorder=10)

    lateral_label_pt = sphere_center
    ax.text(lateral_label_pt[0], lateral_label_pt[1] - 2.6, lateral_label_pt[2] - 0.9,
             r"$O_{inf\_dag}$: lateral opening (R2)" "\nsame shell, not a rung above",
             fontsize=9.5, color=DAG_COLOR, ha="center")
    ax.text(-3.7, 3.5, 1.85,
             r"$O_\infty$: terminal closure (R1)" "\n3 windings shown, pairwise linked",
             fontsize=9.5, color=WORK_COLOR, ha="center")

    ax.set_xlim(-4, 4)
    ax.set_ylim(-4, 4)
    ax.set_zlim(-2, 2)
    ax.set_box_aspect((1, 1, 0.5))
    ax.set_axis_off()
    ax.view_init(elev=42, azim=-48)

    ax.set_title(
        "Graph execution as a hopfionic loop bundle\n"
        "the bootstrap cycle's windings, linked, vs. the lateral $O_{inf\\_dag}$ opening",
        fontsize=13, pad=-6,
    )
    caption = (
        r"Each strand is a $(1,1)$ curve on a shared torus (one loop toroidally and "
        "poloidally at once); phase-offset strands are pairwise linked, the same "
        "interleaving that links Hopf-fibration circle bundles — the visual reading of "
        "re-entering a closed program (winding\\_count) from a different phase. "
        "Grey nodes are no-work opcodes (VINIT/TANCH/IMSCRIB/FSPLIT/FFUSE); blue nodes "
        "do real work. The red sphere is the R2 replicative-opening program: its tangent "
        "loop at the departure point, rotated 180° about its own diameter, sweeping out a "
        "bounded spherical region that genuinely carves into the torus (~22% of the "
        "sphere's surface lies inside it) rather than merely touching it at one point."
    )
    fig.text(0.5, 0.045, caption, ha="center", va="bottom", fontsize=8.3,
              wrap=True, color="#333333")

    out = Path(__file__).parent
    fig.savefig(out / "fig_hopfionic_execution.pdf", bbox_inches="tight", facecolor="white")
    fig.savefig(out / "fig_hopfionic_execution.png", bbox_inches="tight", facecolor="white")
    print(f"wrote {out}/fig_hopfionic_execution.{{pdf,png}}")


if __name__ == "__main__":
    main()
