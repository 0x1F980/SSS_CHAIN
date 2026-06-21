# SSS_CHAIN — Proof manifest (v1.0.0)

| Concern | Implementation | Layer |
|---------|----------------|-------|
| M31/M61 field arithmetic | `sss_chain_field.rs` | Algebra |
| Lagrange k=2 interpolation | `sss_chain_lagrange.rs` | Algebra |
| Link generate/validate | `sss_chain_link.rs` | Chaining |
| Epoch step forward | `sss_chain_epoch.rs` | Forward secrecy |
| OTM chain helpers | `sss_chain_otm.rs` | Attestation binding |

**Lean (in-repo):** [mathematics/Epoch/StepForward.lean](mathematics/Epoch/StepForward.lean)

**Consumers:** ITS-asymmetric (epoch FS), ITS-OTM, ITS-timelock — all delegate chaining algebra here.

**Tests:** `cargo test` in SSS_CHAIN  
**Lean:** L1 backward underdetermination documented in [SSS_CHAIN_SECURITY_LAYERS.md](SSS_CHAIN_SECURITY_LAYERS.md)
