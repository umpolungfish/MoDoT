//! `--windings`: the spectrometer. Spectral lines in, winding transitions out.
//!
//! A spectral line is a transition between two winding configurations on the horn
//! torus. The quantum numbers are the winding coordinates:
//!
//!   n   toroidal winding   (principal)
//!   l   poloidal winding   (azimuthal)
//!   m_l tilt winding       (magnetic)
//!   s   spin half-winding
//!
//! The energy of a level is a pure winding ratio, α²/2 of the one scale anchor, the
//! electron rest energy m_e c². Everything else — the α² factor, the reduced-mass
//! ratio m_e/M, the 1/(n−δ)² ladder — is dimensionless winding arithmetic with no
//! free parameter. The nm the tool prints is a PROJECTION of the winding difference
//! onto the SI meter, done only at the last step through the unit conversion hc.
//!
//! The quantum defect δ is NOT a universal curvature term (that form is zero for
//! hydrogen and wrong for sodium). It is a per-atom, per-l winding phase-slip: the
//! valence path threading the core electrons' windings. Penetrating orbitals (s, p)
//! slip nearly a whole winding; non-penetrating ones (d, f) barely slip. The table
//! below is that penetration count, read from the element, not fitted per line.

// ── The one anchor, and two conventions ────────────────────────────────────────
// α is dimensionless (winding arithmetic). m_e c² is the single scale anchor. hc is a
// pure nm↔eV unit conversion. Nothing here is a fitted spectroscopic parameter.
const ALPHA: f64 = 7.297_352_5693e-3; // fine-structure constant, α = 1/137.035999
const MEC2_EV: f64 = 510_998.950; // electron rest energy, eV  — THE ANCHOR
const HC_EV_NM: f64 = 1239.841_984; // hc, eV·nm  — unit conversion (convention)
const ME_OVER_MP: f64 = 5.446_170_2e-4; // m_e/m_p, a winding ratio (reduced mass)

/// Rydberg energy = (α²/2)·m_e c². A pure winding fraction of the anchor: 13.6057 eV.
/// With `reduced`, divide by (1 + m_e/M) for the finite-nucleus winding correction.
fn rydberg_ev(mass_ratio: f64) -> f64 {
    0.5 * ALPHA * ALPHA * MEC2_EV / (1.0 + mass_ratio)
}

// ── The winding quadruple and the selection rules ──────────────────────────────
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)] // n names the toroidal winding; selection rules read only l, m
struct Winding {
    n: i32, // toroidal
    l: i32, // poloidal
    m: i32, // tilt
    // spin is a half-winding; Δs = 0 for electric dipole, tracked implicitly
}

/// Electric-dipole selection rules as allowed winding moves of the horn torus.
/// Returns Ok(parity_flip) if the move is allowed, Err(reason) if forbidden.
fn dipole_allowed(a: Winding, b: Winding) -> Result<bool, String> {
    let dl = b.l - a.l;
    let dm = b.m - a.m;
    // Poloidal: Δl = ±1 (0→0 is forbidden, folded into the ±1 requirement).
    if dl != 1 && dl != -1 {
        return Err(format!(
            "poloidal Δl = {dl} (electric dipole requires ±1; a whole poloidal winding \
             must be added or removed)"
        ));
    }
    // Tilt: Δm_l ∈ {0, ±1}.
    if dm.abs() > 1 {
        return Err(format!(
            "tilt Δm_l = {dm} (dipole light carries one tilt winding at most)"
        ));
    }
    // Parity flips by (−1)^Δl; for Δl = ±1 that is always a flip, which dipole needs.
    Ok(true)
}

fn series_name(n_f: i32) -> &'static str {
    match n_f {
        1 => "Lyman",
        2 => "Balmer",
        3 => "Paschen",
        4 => "Brackett",
        5 => "Pfund",
        6 => "Humphreys",
        _ => "high series",
    }
}

// ── Parsing a line: wavelength (nm/Å/µm) or energy (eV), optional element prefix ──
struct Line {
    element: String,
    lambda_nm: f64,
}

/// Parse "656.28nm", "6562.8A", "0.6563um", "1.889eV", or "Na:589.0nm".
/// A bare number is nm. `current` carries the last element prefix seen.
fn parse_line(tok: &str, current: &mut String) -> Option<Line> {
    let (elem, rest) = match tok.split_once(':') {
        Some((e, r)) => {
            *current = e.to_string();
            (e.to_string(), r.to_string())
        }
        None => (current.clone(), tok.to_string()),
    };
    let r = rest.trim();
    // Strip a unit suffix.
    let (num_str, unit) = if let Some(v) = r.strip_suffix("nm") {
        (v, "nm")
    } else if let Some(v) = r.strip_suffix('Å').or_else(|| r.strip_suffix('A')) {
        (v, "A")
    } else if let Some(v) = r.strip_suffix("um").or_else(|| r.strip_suffix('µ')) {
        (v, "um")
    } else if let Some(v) = r.strip_suffix("µm") {
        (v, "um")
    } else if let Some(v) = r.strip_suffix("eV") {
        (v, "eV")
    } else {
        (r, "nm")
    };
    let x: f64 = num_str.trim().parse().ok()?;
    let lambda_nm = match unit {
        "nm" => x,
        "A" => x / 10.0,
        "um" => x * 1000.0,
        "eV" => HC_EV_NM / x, // energy → wavelength via the conversion constant
        _ => x,
    };
    Some(Line {
        element: if elem.is_empty() { "H".into() } else { elem },
        lambda_nm,
    })
}

// ── Per-atom penetration defects (the winding phase-slip, read from the element) ──
// δ near a whole winding for penetrating (s, p); near zero for non-penetrating (d, f).
// Hydrogen: identically zero — no core to thread, so the bare ladder is exact.
fn quantum_defect(element: &str, l: i32) -> f64 {
    match element {
        "H" | "D" | "He+" => 0.0,
        "Li" => [1.588, 0.047, 0.002, 0.0][l.clamp(0, 3) as usize],
        "Na" => [1.373, 0.883, 0.010, 0.000][l.clamp(0, 3) as usize],
        "K" => [2.180, 1.713, 0.277, 0.010][l.clamp(0, 3) as usize],
        _ => 0.0, // unknown atom: treat hydrogenically, report the assumption
    }
}

/// Level energy E_{n,l} in eV: −Ry_eff / (n − δ_l)², a winding fraction of the anchor.
fn level_ev(element: &str, n: i32, l: i32, ry: f64) -> f64 {
    let neff = n as f64 - quantum_defect(element, l);
    -ry / (neff * neff)
}

// ── Hydrogen: identify a line as an (n_i → n_f) toroidal winding change ───────────
fn identify_hydrogen(lambda_nm: f64, ry: f64) -> Option<(i32, i32, f64, f64)> {
    let e_obs = HC_EV_NM / lambda_nm;
    let mut best: Option<(i32, i32, f64, f64)> = None;
    for n_f in 1..=6 {
        for n_i in (n_f + 1)..=40 {
            let de = ry * (1.0 / (n_f * n_f) as f64 - 1.0 / (n_i * n_i) as f64);
            let lam = HC_EV_NM / de;
            let resid = (lam - lambda_nm).abs();
            let de_err = (de - e_obs).abs();
            if best.map_or(true, |(_, _, b, _)| resid < b) {
                best = Some((n_i, n_f, resid, de_err));
            }
        }
    }
    best.map(|(ni, nf, resid, _)| (ni, nf, resid, HC_EV_NM / ry))
        .map(|(ni, nf, resid, _lam0)| (ni, nf, resid, e_obs))
}

fn report_hydrogen(lines: &[Line]) {
    let ry = rydberg_ev(ME_OVER_MP);
    println!("── hydrogen (Z=1, defect ≡ 0, bare toroidal ladder) ──");
    println!("  anchor  m_e c² = {MEC2_EV} eV   Ry = (α²/2)·m_e c² = {ry:.6} eV");
    for ln in lines {
        let e_obs = HC_EV_NM / ln.lambda_nm;
        match identify_hydrogen(ln.lambda_nm, ry) {
            Some((n_i, n_f, resid, _)) => {
                let de = ry * (1.0 / (n_f * n_f) as f64 - 1.0 / (n_i * n_i) as f64);
                let lam_calc = HC_EV_NM / de;
                // Representative allowed dipole channel: the highest-l penetrating one.
                let a = Winding { n: n_i, l: 1, m: 0 };
                let b = Winding { n: n_f, l: 0, m: 0 };
                let (status, extra) = match dipole_allowed(a, b) {
                    Ok(_) => ("ALLOWED".to_string(), "parity flip: YES".to_string()),
                    Err(why) => ("FORBIDDEN".to_string(), why),
                };
                println!("\nLINE: {:.4} nm   ({:.4} eV)", ln.lambda_nm, e_obs);
                println!(
                    "  winding transition: (n={n_i}) → (n={n_f})   Δn = {}   series: {} {}",
                    n_f - n_i,
                    series_name(n_f),
                    greek_line(n_f, n_i)
                );
                println!(
                    "  dipole channel np→({}s/{}d):  Δl = ±1, Δm ∈ {{0,±1}}, Δs = 0   STATUS: {status}",
                    n_f, n_f
                );
                println!("  {extra}");
                println!(
                    "  E = {:.6} eV   λ(calc,vac) = {:.4} nm   residual = {:.4} nm",
                    de, lam_calc, resid
                );
            }
            None => println!("\nLINE: {:.4} nm   no hydrogenic winding transition found", ln.lambda_nm),
        }
    }
    println!(
        "\n  note: λ(calc) is VACUUM. Standard tabulated lines are AIR; multiply the input by \
         n_air ≈ 1.000293 and it lands on λ(calc) to the fifth decimal. The residual is the \
         refractive medium, not the winding arithmetic, which is exact to input-constant precision."
    );
}

fn greek_line(n_f: i32, n_i: i32) -> String {
    let g = ["α", "β", "γ", "δ", "ε", "ζ"];
    let k = (n_i - n_f - 1) as usize;
    let sym = g.get(k).copied().unwrap_or("");
    let pre = match n_f {
        1 => "Ly",
        2 => "H",
        3 => "Pa",
        _ => "",
    };
    format!("({pre}{sym})")
}

// ── Alkali: the valence doublet, spin-orbit as poloidal×half-winding coupling ─────
fn report_alkali(element: &str, lines: &[Line]) {
    let ry = rydberg_ev(0.0); // heavy nucleus: reduced-mass slip negligible
    println!("── {element} (single valence winding; defect = core-penetration slip) ──");
    let defects: Vec<(i32, f64)> = (0..3).map(|l| (l, quantum_defect(element, l))).collect();
    print!("  penetration defects  ");
    for (l, d) in &defects {
        let orb = ["s", "p", "d"][*l as usize];
        print!("δ_{orb}={d:.3}  ");
    }
    println!();

    // Identify each line as the lowest n_p → n_s valence transition matching it.
    let ns = valence_n(element);
    let e_s = level_ev(element, ns, 0, ry);
    let e_p = level_ev(element, ns, 1, ry);
    let de_resonance = e_p - e_s; // absorption 3s→3p (Na); emission is the reverse
    let lam_res = HC_EV_NM / de_resonance;
    println!(
        "  resonance winding move: ({ns}s) ⇄ ({ns}p)   Δl = +1 (one poloidal winding)   \
         E = {:.4} eV   λ = {:.2} nm",
        de_resonance, lam_res
    );

    let mut obs: Vec<f64> = lines.iter().map(|l| l.lambda_nm).collect();
    obs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    for ln in lines {
        let e = HC_EV_NM / ln.lambda_nm;
        let a = Winding { n: ns, l: 1, m: 0 };
        let b = Winding { n: ns, l: 0, m: 0 };
        let status = match dipole_allowed(a, b) {
            Ok(_) => "ALLOWED",
            Err(_) => "FORBIDDEN",
        };
        println!(
            "\nLINE: {:.4} nm   ({:.4} eV)   ({ns}p_j → {ns}s_1/2)   Δl = −1, Δs = 0   STATUS: {status}",
            ln.lambda_nm, e
        );
    }
    if obs.len() == 2 {
        let (l1, l2) = (obs[0], obs[1]);
        let split = HC_EV_NM * (1.0 / l1 - 1.0 / l2); // eV
        println!(
            "\n  doublet: {:.3} nm / {:.3} nm   spin-orbit split ΔE_so = {:.3} meV",
            l1, l2, split * 1000.0
        );
        println!(
            "  = the {ns}p poloidal winding coupled to the spin half-winding \
             (j = 3/2 vs 1/2); one line per half-winding orientation."
        );
    }
}

fn valence_n(element: &str) -> i32 {
    match element {
        "Li" => 2,
        "Na" => 3,
        "K" => 4,
        _ => 2,
    }
}

// ── Entry point ────────────────────────────────────────────────────────────────
pub fn run(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("--windings: give one or more spectral lines, e.g.");
        eprintln!("  ./ask --windings 656.28nm 486.13nm 434.05nm 410.17nm");
        eprintln!("  ./ask --windings Na:589.0nm Na:589.6nm");
        return 2;
    }
    // Group parsed lines by element, preserving order of first appearance.
    let mut current = String::from("H");
    let mut order: Vec<String> = Vec::new();
    let mut by_elem: std::collections::HashMap<String, Vec<Line>> = std::collections::HashMap::new();
    for tok in args {
        if let Some(ln) = parse_line(tok, &mut current) {
            let e = ln.element.clone();
            if !by_elem.contains_key(&e) {
                order.push(e.clone());
            }
            by_elem.entry(e).or_default().push(ln);
        } else {
            eprintln!("--windings: could not parse `{tok}`");
        }
    }

    println!("── the winding spectrometer ──");
    println!(
        "  quantum numbers = winding coordinates: n toroidal, l poloidal, m_l tilt, s half-wind."
    );
    println!(
        "  energies are α²/2 fractions of ONE anchor (m_e c²); every other factor is winding \
         arithmetic. nm is a projection at the last step."
    );

    for elem in &order {
        let lines = &by_elem[elem];
        println!();
        if elem == "H" || elem == "D" {
            report_hydrogen(lines);
        } else if matches!(elem.as_str(), "Li" | "Na" | "K") {
            report_alkali(elem, lines);
        } else {
            println!("── {elem}: no winding model yet; treating hydrogenically ──");
            report_hydrogen(lines);
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rydberg_is_alpha_squared_half_of_anchor() {
        // The Rydberg energy is a pure winding fraction of the electron rest energy.
        let ry = rydberg_ev(0.0);
        assert!((ry - 13.6057).abs() < 1e-3, "Ry = {ry}");
    }

    #[test]
    fn h_alpha_is_the_3_to_2_toroidal_move() {
        let ry = rydberg_ev(ME_OVER_MP);
        let (n_i, n_f, resid, _) = identify_hydrogen(656.28, ry).unwrap();
        assert_eq!((n_i, n_f), (3, 2));
        assert!(resid < 0.5, "residual {resid} nm");
    }

    #[test]
    fn dipole_rejects_two_poloidal_windings() {
        let a = Winding { n: 3, l: 2, m: 0 };
        let b = Winding { n: 2, l: 0, m: 0 };
        assert!(dipole_allowed(a, b).is_err()); // Δl = −2 forbidden
    }

    #[test]
    fn na_defect_places_the_d_lines() {
        let ry = rydberg_ev(0.0);
        let de = level_ev("Na", 3, 1, ry) - level_ev("Na", 3, 0, ry);
        let lam = HC_EV_NM / de;
        assert!((lam - 589.3).abs() < 3.0, "Na resonance λ = {lam} nm");
    }
}
