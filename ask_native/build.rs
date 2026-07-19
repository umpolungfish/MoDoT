// Build script: only does work when the `cuda` feature is on. candle's CUDA
// backend links -lcublas -lcublasLt -lcurand -lnvrtc, but this machine has only
// the CUDA runtime under /usr/local/cuda (no math libs). The math libs are
// present as pip `nvidia-*` wheels under ~/.local; `.cudalibs/` holds unversioned
// symlinks to them for the LINKER, and we add an rpath to the real versioned
// .so dirs so the built binary RESOLVES them at runtime without LD_LIBRARY_PATH.
//
// Build scripts don't see the crate's `#[cfg(feature)]`; Cargo exposes features
// as CARGO_FEATURE_<NAME> env vars instead.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if std::env::var("CARGO_FEATURE_CUDA").is_err() {
        return; // CPU build: nothing to wire
    }
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let home = std::env::var("HOME").unwrap_or_default();

    // unversioned symlinks (.cudalibs/libcublas.so → …/libcublas.so.12) for -l resolution
    println!("cargo:rustc-link-search=native={manifest}/.cudalibs");

    // runtime resolution of the real versioned libraries (pip nvidia wheels + cudart)
    let nv = format!("{home}/.local/lib/python3.10/site-packages/nvidia");
    for sub in ["cublas/lib", "curand/lib", "cuda_nvrtc/lib", "cuda_runtime/lib"] {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{nv}/{sub}");
    }
    for p in ["/usr/local/cuda/lib64", "/usr/local/cuda/lib64/stubs"] {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{p}");
    }
}
