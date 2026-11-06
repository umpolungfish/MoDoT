# Executable IMASM morphism tape

**Authority:** MoDoT / imasm_core native Rust: check.rs, flow.rs,
imasm16_3.rs. The tape consumes these modules. It does not define a new
operator alphabet, a substitute SIXTEEN_3, or a Python shadow evaluator.

## What runs

Each registered tape program is an explicit directed graph of native IMASM
operators. The graph's edges specify real data flow and the order of the
three FSPLIT arms. The native kernel checks graph valence, ancestry-based
reconnection, and transformation before the program is registered.

Each sample executes that graph on a SIXTEEN_3 value via flow_values.
The resulting tape frame contains:

- the full carrier input and readout, never an implicit FOUR projection;
- the native graph verdict T/N/B/F and closure state;
- a separate IFIX-commit flag for an IFIX upstream of the TANCH readout;
- original source evidence, observation and publication timestamps;
- a monotonically increasing sequence number.

The native B verdict and the B carrier state are different fields and
are never conflated. A B verdict is retained; it does not veto execution.

IMASM's **ROTAT is an operator on compositions**, not a source name or
a classification based on a measurement magnitude. No dynamic ROTAT
mapping is synthesized by this tape implementation.

## Build and test

From the MoDoT checkout:

    cargo test --manifest-path imasm_core/Cargo.toml --lib tape::tests
    cargo test --manifest-path imasm_core/Cargo.toml --bin imasm_tape
    cargo build --release --manifest-path imasm_core/Cargo.toml --bin imasm_tape

The executable is at imasm_core/target/release/imasm_tape.
The supplied GitHub Actions workflow builds and tests it on Linux.

## Native JSONL protocol

Register a three-arm program; the edges are REAL edges rather than a
word that will be rewired by a guessed sequence parser:

    {"kind":"register","id":"coop","nodes":["VINIT","FSPLIT3","EVALT","EVALF","IMSCRIB","FFUSE3","ENGAGR","IFIX","TANCH"],"edges":[[0,1],[1,2],[1,3],[1,4],[2,5],[3,5],[4,5],[5,6],[6,7],[7,8]],"readout":8}

Feed one observation with an explicit original carrier value:

    {"kind":"sample","source":"coop.x","program":"coop","observed_at_ns":1,"available_at_ns":2,"as_of_ns":2,"seed":"TF","payload":{"x":1.25}}

The native result contains a graph verdict B, carrier readout A, and
committed=true, since the actual graph executes ENGAGR and then IFIX.

Compose two declared programs with a declared connection, consuming
actual graph ports:

    {"kind":"compose","id":"joined","left":"left","right":"right","links":[[1,0]]}

The native register step enforces valences after binding.

## Causality and scalability

The interpreter rejects publication before observation, future-at-as-of
samples, and backwards publication timestamps. Equal timestamps are
allowed; they are not deduplicated. Each successfully emitted frame can be
released from RAM while its global sequence and state persist.

The native input parser enables exact floating-point round-tripping.
A real archived value, 1.7787827609827747, is a regression case.

## fin3r archive ingestion

In fin3r, the transport adapter lives in core/imasm_tape.py and the
whole-archive command lives in scripts/imasm_archive_tape.py.

The adapter **requires an explicit morphism graph or source-program
routing**. It does not guess which operator a metric name or numerical
value represents. It preserves the entire underlying reading object,
including nested provenance, for the SIC observer and reproducible replay.

Existing snapshots do not contain a canonical per-sensor IMASM program.
Any universal example graph used for an archive transport test is an
explicitly declared executable example, not a derived claim about the
historical sensor's morphism.

This implementation establishes native executable streaming; it does not
assert a new Lean proof or that raw archived sensor magnitudes alone
determine an IMASM operator.
