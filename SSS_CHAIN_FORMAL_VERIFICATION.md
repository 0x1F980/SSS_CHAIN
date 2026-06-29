# SSS_CHAIN — formal verification status

## License: GNU GPLv3 Only

## Library (Rust)

| Property | Status | Evidence |
|----------|--------|----------|
| Field arithmetic | Tested | `sss_chain_field` unit tests |
| Lagrange k=2 | Tested | `sss_chain_lagrange` |
| Epoch step roundtrip | Tested | `transition_roundtrip` |
| Link generate/validate | Tested | `sss_chain_link` 10 tests |
| CLI `.ssc` roundtrip | Tested | bin + `cli_integration` |

## Cross-ecosystem Lean

SSS epoch transitions are refined in **ITS-timelock** Lean (`mathematics/stl/`), not duplicated in this repo:

| Theorem | Location |
|---------|----------|
| `transition_invertible` | timelock `Stl/TimeLock.lean` |
| `rust_*_refines_ideal` | timelock `Stl/Refinement/RustModel.lean` |

Link-level ITS underdetermination for **published link chains** is specified in [SSS_CHAIN_mathematics.md](SSS_CHAIN_mathematics.md); machine-checked link-chain proofs are **future work** (optional `mathematics/stl/` here).

## Status summary

- Shared algebra: **proved** in timelock STL + Rust tests in this crate.
- Link API oracle model: **documented**; formal Lean for link chain TBD.
