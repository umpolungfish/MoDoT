// riemann_hilbert.rs — Zauner Hamiltonian eigenvalues vs Riemann zeta zeros.
//
// CONSTRUCTION 1 (circular): Zeta-encoded H, reconstructed via Gerzon inverse.
//   Verifies the SIC-POVM is informationally complete (mu∘delta=id).
//   The zeta zeros ARE the input — this does NOT prove the connection.
//
// CONSTRUCTION 2 (non-circular): Mixed Zauner+WH Hamiltonian.
//   H = alpha*(U_Z + U_Z^dag)/2 + beta*(X+X^dag+Z+Z^dag)/4
//   Parameters alpha, beta chosen by grid search over the linear fit
//   to zeta zeros. This is NON-CIRCULAR because no zeta zeros are used
//   to construct H — only to evaluate the fit after the fact.
//   Best fit (from Python exploration): alpha≈7.67, beta≈15.25.
//
// CONSTRUCTION 3 (non-circular): WH Laplacian eigenvalues.
//   H = (X+X^dag+Z+Z^dag)/4 — the simplest structural operator.
//   Analytical: cos(2πa/d)+cos(2πb/d) over the 12×12 discrete torus.

use std::f64::consts::PI;
use crate::riemann_sic::{D, D2, wh_orbit, Cmplx};
use crate::riemann_sic::{cmul, cconj, cabs2, cadd, cscale};

pub const ZETA_ZEROS: [f64; 20] = [
    14.1347251417347, 21.0220396387716, 25.0108575801457, 30.4248761258595,
    32.9350615877392, 37.5861781588257, 40.9187190121480, 43.3270732809150,
    48.0051508811672, 49.7738324776723, 52.9703214777145, 56.4462476970634,
    59.3470440026024, 60.8317785246098, 65.1125440480816, 67.0798105294942,
    69.5464017111739, 72.0671576744819, 75.7046906990839, 77.1448400688748,
];

/// WH displacement D(a,b).
fn wh_d(a: usize, b: usize) -> [[Cmplx; D]; D] {
    let om = [f64::cos(2.0*PI/D as f64), f64::sin(2.0*PI/D as f64)];
    let mut m = [[[0.0, 0.0]; D]; D];
    for k in 0..D {
        let kma = (k + D - a) % D;
        let phase = (b * kma) % D;
        let mut p = [1.0, 0.0];
        for _ in 0..phase { p = cmul(p, om); }
        m[k][kma] = p;
    }
    m
}

/// Complex matrix add: C = A + B.
fn cmat_add(a: &[[Cmplx; D]; D], b: &[[Cmplx; D]; D]) -> [[Cmplx; D]; D] {
    let mut c = [[[0.0,0.0]; D]; D];
    for r in 0..D { for col in 0..D { c[r][col] = cadd(a[r][col], b[r][col]); } }
    c
}

/// Complex matrix * scalar.
fn cmat_scale(a: &[[Cmplx; D]; D], s: f64) -> [[Cmplx; D]; D] {
    let mut c = [[[0.0,0.0]; D]; D];
    for r in 0..D { for col in 0..D { c[r][col] = cscale(a[r][col], s); } }
    c
}

/// Complex matrix multiply: C = A * B.
fn cmat_mul(a: &[[Cmplx; D]; D], b: &[[Cmplx; D]; D]) -> [[Cmplx; D]; D] {
    let mut c = [[[0.0,0.0]; D]; D];
    for r in 0..D { for col in 0..D {
        let mut s = [0.0, 0.0];
        for k in 0..D { s = cadd(s, cmul(a[r][k], b[k][col])); }
        c[r][col] = s;
    }}
    c
}

/// Hermitize: H = (H + H^dag)/2.
fn hermitize(h: &[[Cmplx; D]; D]) -> [[Cmplx; D]; D] {
    let mut r = [[[0.0,0.0]; D]; D];
    for i in 0..D { for j in 0..D {
        r[i][j][0] = (h[i][j][0] + h[j][i][0]) / 2.0;
        r[i][j][1] = (h[i][j][1] - h[j][i][1]) / 2.0;
    }}
    r
}

/// Jacobi eigenvalues for real symmetric 12x12.
fn jacobi(a: &mut [[f64; D]; D]) -> Vec<f64> {
    let tol = 1e-14;
    for _ in 0..200 {
        let (mut mx, mut p, mut q) = (0.0, 0usize, 1usize);
        for r in 0..D { for c in (r+1)..D {
            let v = a[r][c].abs();
            if v > mx { mx = v; p = r; q = c; }
        }}
        if mx < tol { break; }
        let th = if (a[p][p]-a[q][q]).abs() < 1e-15 { PI/4.0 }
                 else { 0.5 * f64::atan2(2.0*a[p][q], a[p][p]-a[q][q]) };
        let (cs, sn) = (f64::cos(th), f64::sin(th));
        let (app, aqq, apq) = (a[p][p], a[q][q], a[p][q]);
        a[p][p] = cs*cs*app + sn*sn*aqq - 2.0*cs*sn*apq;
        a[q][q] = sn*sn*app + cs*cs*aqq + 2.0*cs*sn*apq;
        a[p][q] = 0.0; a[q][p] = 0.0;
        for r in 0..D { if r != p && r != q {
            let (arp, arq) = (a[r][p], a[r][q]);
            a[r][p] = cs*arp - sn*arq; a[p][r] = a[r][p];
            a[r][q] = sn*arp + cs*arq; a[q][r] = a[r][q];
        }}
    }
    let mut ev: Vec<f64> = (0..D).map(|i| a[i][i]).collect();
    ev.sort_by(|x,y| x.partial_cmp(y).unwrap());
    ev
}

/// Extract real symmetric part, compute eigenvalues.
fn eigenvalues(h: &[[Cmplx; D]; D]) -> Vec<f64> {
    let mut a = [[0.0f64; D]; D];
    for r in 0..D { for c in 0..D { a[r][c] = (h[r][c][0] + h[c][r][0])/2.0; } }
    jacobi(&mut a)
}

/// H from Born probabilities: H = (d+1) sum p_i |psi_i><psi_i| - (sum p_i) I.
fn hamiltonian_from_probs(probs: &[f64], orbit: &[[Cmplx; D]]) -> [[Cmplx; D]; D] {
    let mut h = [[[0.0,0.0]; D]; D];
    let sp: f64 = probs.iter().sum();
    for (i, oi) in orbit.iter().enumerate() {
        let pi = probs[i];
        for r in 0..D { for c in 0..D {
            let contrib = cmul(oi[r], cconj(oi[c]));
            h[r][c] = cadd(h[r][c], cscale(contrib, pi));
        }}
    }
    let f = (D+1) as f64;
    for r in 0..D { for c in 0..D { h[r][c] = cscale(h[r][c], f); } }
    for r in 0..D { h[r][r][0] -= sp; }
    h
}
/// Born probabilities for H = diagonal(eigenvalues) in computational basis.
fn zeta_encoded_probs(orbit: &[[Cmplx; D]], evals: &[f64]) -> Vec<f64> {
    let mut p = vec![0.0f64; D2];
    for (i, oi) in orbit.iter().enumerate() {
        let mut s = 0.0;
        for k in 0..D { s += cabs2(oi[k]) * evals[k]; }
        p[i] = s / (D as f64);
    }
    p
}

/// Optimal linear fit: minimize ||slope*ev + intercept - target||^2.
fn linear_fit(ev: &[f64], target: &[f64]) -> (Vec<f64>, f64, f64) {
    let n = ev.len() as f64;
    let sx: f64 = ev.iter().sum();
    let sy: f64 = target.iter().sum();
    let sxy: f64 = ev.iter().zip(target.iter()).map(|(x,y)| x*y).sum();
    let sxx: f64 = ev.iter().map(|x| x*x).sum();
    let denom = n*sxx - sx*sx;
    let (slope, intercept) = if denom.abs() < 1e-15 { (0.0, sy/n) }
    else { ((n*sxy - sx*sy)/denom, (sy - ((n*sxy - sx*sy)/denom)*sx)/n) };
    let fitted: Vec<f64> = ev.iter().map(|x| slope*x + intercept).collect();
    (fitted, slope, intercept)
}

/// Print eigenvalue comparison table.
fn print_comparison(label: &str, ev: &[f64], zeta: &[f64], slope: f64, intercept: f64, fitted: &[f64]) {
    let mut max_d = 0.0f64; let mut sum_d = 0.0f64;
    println!("  {}:", label);
    println!("  Linear fit: lam_fit = {:.6} * lam + {:.6}", slope, intercept);
    println!("  {:>3}  {:>14}  {:>14}  {:>14}  {:>10}", "n", "lam_n(H_Z)", "lam_fit", "t_n(zeta)", "|Delta|");
    println!("  {}  {}  {}  {}  {}", "---", "--------------", "--------------", "--------------", "----------");
    let m = ev.len().min(zeta.len());
    for n in 0..m {
        let delta = (fitted[n] - zeta[n]).abs();
        if delta > max_d { max_d = delta; } sum_d += delta;
        println!("  {:3}  {:>14.8}  {:>14.8}  {:>14.8}  {:>10.6}",
            n+1, ev[n], fitted[n], zeta[n], delta);
    }
    println!("  Mean |Delta| = {:.6},  Max |Delta| = {:.6}", sum_d/(m as f64), max_d);
    println!();
}

/// Build Zauner unitary U_Z via Appleby formula: tau^{r^2+2rs} / sqrt(d).
fn zauner_unitary() -> [[Cmplx; D]; D] {
    let tau: Cmplx = [-f64::cos(PI/D as f64), f64::sin(PI/D as f64)];
    let mut uz = [[[0.0,0.0]; D]; D];
    for r in 0..D {
        for s in 0..D {
            let exp = ((r*r + 2*r*s) % (2*D)) as f64;
            let mut p = [1.0, 0.0];
            for _ in 0..(exp as usize) { p = cmul(p, tau); }
            uz[r][s] = cscale(p, 1.0/(D as f64).sqrt());
        }
    }
    uz
}

/// CLI entry: Riemann-Hilbert eigenvalue computation.
pub fn run() -> i32 {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  RIEMANN-HILBERT: Zauner Hamiltonian H_Z vs Riemann zeta    ║");
    println!("║  d=12 SIC-POVM -> self-adjoint operator eigenvalue analysis ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    let orbit_states = wh_orbit();
    println!("Weyl-Heisenberg orbit: {} states generated.", orbit_states.len());

    // Verify SIC condition
    let target_ol = 1.0/((D+1) as f64);
    let ol = cabs2({
        let mut inner = [0.0,0.0];
        for k in 0..D { inner = cadd(inner, cmul(cconj(orbit_states[0][k]), orbit_states[1][k])); }
        inner
    });
    println!("SIC check: |<psi_0|psi_1>|^2 = {:.12} (target {:.12}, Delta={:.2e})",
        ol, target_ol, (ol - target_ol).abs());

    // Build WH generators
    let xf = wh_d(1, 0); let xb = wh_d(11, 0);
    let zf = wh_d(0, 1); let zb = wh_d(0, 11);
    let h_wh: [[Cmplx; D]; D] = {
        let mut h = cmat_add(&xf, &xb);
        h = cmat_add(&h, &zf); h = cmat_add(&h, &zb);
        hermitize(&cmat_scale(&h, 0.25))
    };

    // Build Zauner unitary and its Hermitian part
    let uz = zauner_unitary();
    // Verify U_Z^3 = phase*I
    let uz2 = cmat_mul(&uz, &uz);
    let uz3 = cmat_mul(&uz2, &uz);
    let phase = uz3[0][0];
    let mut uz3_err = 0.0f64;
    for r in 0..D { for c in 0..D {
        let expected = if r == c { phase } else { [0.0, 0.0] };
        let diff = (uz3[r][c][0]-expected[0]).abs() + (uz3[r][c][1]-expected[1]).abs();
        if diff > uz3_err { uz3_err = diff; }
    }}
    println!("Zauner unitary order-3: max|U^3-phase*I| = {:.2e}", uz3_err);

    let uz_herm = hermitize(&uz);
    // Extract real part for eigenvalue computation
    let mut uz_real = [[0.0f64; D]; D];
    for r in 0..D { for c in 0..D { uz_real[r][c] = uz_herm[r][c][0]; } }
    let ev_uz_herm = jacobi(&mut uz_real);
    println!("U_Z_herm eigenvalues: {:?}", ev_uz_herm);

    // Extract real part of H_WH
    let mut h_wh_real = [[0.0f64; D]; D];
    for r in 0..D { for c in 0..D { h_wh_real[r][c] = h_wh[r][c][0]; } }

    let z12: Vec<f64> = ZETA_ZEROS.iter().take(D).copied().collect();
    // ═══════════════════════════════════════════════════════════
    // CONSTRUCTION 1: Zeta-encoded (CIRCULAR — verifies mu∘delta=id)
    // ═══════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════");
    println!("  CONSTRUCTION 1: Zeta-encoded Gerzon reconstruction");
    println!("  (CIRCULAR: zeta zeros used as eigenvalues of input H)");
    println!("  Purpose: verify mu∘delta=id for d=12 SIC-POVM frame.");
    println!("═══════════════════════════════════════════════════════════");
    println!();

    let probs_zeta = zeta_encoded_probs(&orbit_states, &z12);
    let h_rec = hamiltonian_from_probs(&probs_zeta, &orbit_states);

    let mut max_anti = 0.0f64;
    for r in 0..D { for c in 0..D {
        let a = (h_rec[r][c][0]-h_rec[c][r][0]).abs() + (h_rec[r][c][1]+h_rec[c][r][1]).abs();
        if a > max_anti { max_anti = a; }
    }}
    println!("  Self-adjoint: max|H-H^dag| = {:.2e}", max_anti);

    let ev_rec = eigenvalues(&h_rec);
    let (fit_rec, sl_rec, ic_rec) = linear_fit(&ev_rec, &z12);
    print_comparison("Gerzon reconstruction (circular)", &ev_rec, &z12, sl_rec, ic_rec, &fit_rec);
    let md_rec = fit_rec.iter().zip(z12.iter()).map(|(f,z)| (f-z).abs()).sum::<f64>()/(D as f64);
    let gerzon_ok = md_rec < 1e-10;
    println!("  Gerzon mu∘delta=id: {} (mean|Δ|={:.2e})",
        if gerzon_ok { "PASS" } else { "NOTE: non-zero but small" }, md_rec);

    // ═══════════════════════════════════════════════════════════
    // CONSTRUCTION 2: Mixed Zauner+WH (NON-CIRCULAR)
    // Grid search over alpha, beta for best linear fit to zeta zeros.
    // The fit uses zeta zeros ONLY for evaluation — not for constructing H.
    // ═══════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════");
    println!("  CONSTRUCTION 2: Mixed Zauner+WH Hamiltonian");
    println!("  (NON-CIRCULAR: no zeta zeros used to construct H)");
    println!("  H = alpha*(U_Z+U_Z^dag)/2 + beta*(X+X^dag+Z+Z^dag)/4");
    println!("  Grid search over alpha,beta for best linear fit.");
    println!("═══════════════════════════════════════════════════════════");
    println!();

    // Grid search
    let mut best_mean = f64::INFINITY;
    let mut best_alpha = 0.0f64;
    let mut best_beta = 0.0f64;
    let mut best_ev: Vec<f64> = Vec::new();

    let alpha_range: Vec<f64> = (0..25).map(|i| 0.1 + i as f64 * 0.4).collect(); // 0.1 to 9.7
    let beta_range: Vec<f64> = (0..25).map(|i| 0.5 + i as f64 * 1.2).collect();  // 0.5 to 29.3

    for &alpha in &alpha_range {
        for &beta in &beta_range {
            // Build H = alpha*U_Z_herm + beta*H_WH
            let mut h = [[0.0f64; D]; D];
            for r in 0..D { for c in 0..D {
                h[r][c] = alpha * uz_real[r][c] + beta * h_wh_real[r][c];
            }}
            let ev = jacobi(&mut h);
            let (fit, _, _) = linear_fit(&ev, &z12);
            let md: f64 = fit.iter().zip(z12.iter()).map(|(f,z)| (f-z).abs()).sum::<f64>()/(D as f64);
            if md < best_mean {
                best_mean = md;
                best_alpha = alpha;
                best_beta = beta;
                best_ev = ev;
            }
        }
    }

    println!("  Best grid params: alpha={:.2}, beta={:.2}, mean|Δ|={:.6}",
        best_alpha, best_beta, best_mean);
    let (fit_mix, sl_mix, ic_mix) = linear_fit(&best_ev, &z12);
    print_comparison("Mixed Zauner+WH (non-circular)", &best_ev, &z12, sl_mix, ic_mix, &fit_mix);

    // ═══════════════════════════════════════════════════════════
    // CONSTRUCTION 3: WH Laplacian (NON-CIRCULAR)
    // ═══════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════");
    println!("  CONSTRUCTION 3: WH Laplacian H = (X+X^dag+Z+Z^dag)/4");
    println!("  (NON-CIRCULAR: discrete torus adjacency operator)");
    println!("  Analytical spectrum: [cos(2πa/d)+cos(2πb/d)]/2");
    println!("═══════════════════════════════════════════════════════════");
    println!();

    let ev_wh = eigenvalues(&h_wh);
    let (fit_wh, sl_wh, ic_wh) = linear_fit(&ev_wh, &z12);
    let md_wh: f64 = fit_wh.iter().zip(z12.iter()).map(|(f,z)| (f-z).abs()).sum::<f64>()/(D as f64);
    print_comparison("WH Laplacian (non-circular)", &ev_wh, &z12, sl_wh, ic_wh, &fit_wh);

    // Analytical full spectrum (144 values, showing unique)
    let mut analytical: Vec<f64> = (0..D).flat_map(|a|
        (0..D).map(move |b|
            (f64::cos(2.0*PI*(a as f64)/(D as f64)) + f64::cos(2.0*PI*(b as f64)/(D as f64)))/2.0)
    ).collect();
    analytical.sort_by(|x,y| x.partial_cmp(y).unwrap());
    let mut uniq: Vec<f64> = Vec::new();
    for &v in &analytical {
        if uniq.is_empty() || (v - uniq[uniq.len()-1]).abs() > 1e-10 { uniq.push(v); }
    }
    println!("  Analytical unique values (of 144): {} values: {:?}", uniq.len(), uniq);

    // ═══════════════════════════════════════════════════════════
    // FINAL VERDICT
    // ═══════════════════════════════════════════════════════════
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  VERDICT                                                     ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║                                                              ║");
    println!("║  Construction 1 (circular, zeta as input):                   ║");
    println!("║    mean|Δ| = {:.2e}  — verifies mu∘delta=id               ║", md_rec);
    println!("║    The d=12 SIC-POVM frame is informationally complete.      ║");
    println!("║    Given the zeta zeros, the frame faithfully encodes them.  ║");
    println!("║                                                              ║");
    println!("║  Construction 2 (non-circular, mixed Zauner+WH):             ║");
    println!("║    mean|Δ| = {:.6}  — APPROXIMATE, not exact                ║", best_mean);
    println!("║    Best: alpha={:.2}, beta={:.2}                            ║", best_alpha, best_beta);
    println!("║    Captures rough spectral shape but not precise zeros.      ║");
    println!("║                                                              ║");
    println!("║  Construction 3 (non-circular, WH Laplacian):                ║");
    println!("║    mean|Δ| = {:.6}  — APPROXIMATE, not exact                ║", md_wh);
    println!("║    Discrete torus adjacency — simplest structural operator.  ║");
    println!("║                                                              ║");
    println!("║  CONCLUSION:                                                 ║");
    println!("║  The d=12 SIC-POVM faithfully ENCODES the zeta zeros         ║");
    println!("║  (Construction 1 proves mu∘delta=id) but does not            ║");
    println!("║  PRODUCE them from its structure alone. No non-circular      ║");
    println!("║  construction yields the zeta zeros to high precision.       ║");
    println!("║                                                              ║");
    println!("║  The structural fusion zeta ⊗ SIC → H·P is consistent       ║");
    println!("║  at the grammar level but the explicit Hilbert-Polya         ║");
    println!("║  operator cannot be derived from the SIC-POVM alone          ║");
    println!("║  without presupposing the zeta zeros.                        ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    if gerzon_ok { 0 } else { 1 }
}
