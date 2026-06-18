# SSS_CHAIN — Vision (SSS_CHAIN_vision.md)

## License: GNU GPLv3 Only
## Target: Architects, cryptographers

---

## Purpose

**SSS_CHAIN** is the shared Rust crate for forward-only Shamir-style **link chaining** used across the ITS ecosystem. It deduplicates field arithmetic, Lagrange interpolation, epoch transitions, and the **link block API** (`sss_chain_generate`, validate, arrange).

Consumers:

- **ITS-OTM** — WC-MAC attestation chains (not the link CLI format)
- **ITS-timelock** — epoch transition algebra
- **ITS-session** (future) — session link chains

---

## Threat model

**Eve (omniscient except root):** knows algorithm + all published links + unbounded CPU.

- **Layer 1:** Cannot uniquely reconstruct or **prove** prior chain state (ITS underdetermination).
- **Layer 2:** Can try root candidates via forward validation; success if root is weak.

Root length is **not** required to match chain size. Passphrase length is **not** leaked on the wire.

---

## Design principles

1. `#![no_std]` library; optional `sss_chain` CLI for operators.
2. Single root policy: normalize/truncate/pad to link material.
3. No Shamir fragment splitting (`hydra_sss`) — chaining only.
4. KDF/passphrase handling stays in ITS-KeyManagement / session consumers.

---

## vs hash chains

Hash chains provide **computational** one-wayness. SSS-chaining provides **information-theoretic backward ambiguity** without relying on hardness assumptions for Layer 1.

---

## Repository

**Crate:** `sss_chain` · **GitHub:** `0x1F980/SSS_CHAIN`
