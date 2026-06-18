# SSS_CHAIN — Security Layers (SSS_CHAIN_SECURITY_LAYERS.md)

## License: GNU GPLv3 Only
## Target: Auditors, integrators, AI-assisted reviewers

**Read this document first.**

---

## 1. Summary

`sss_chain` provides **SSS-chaining** (k=2) link generation, validation, and arrangement for ITS-OTM, ITS-timelock, and related crates. Security has two **independent** layers:

| Layer | Type | Guarantee |
|-------|------|-----------|
| 1 — SSS ITS | Information-theoretic | Published links do not allow **unique** backward history without `root_pass` |
| 2 — Root | Operator-dependent | Brute-force on root succeeds if root is weak; **not** enforced by chain length |

There is **no** `ExactChainBytes` policy. Root is normalized to `link_byte_len` material only.

---

## 2. Do not misread (auditors / AI)

1. **"One-way like SHA"** — Backward is **underdetermined**, not computationally hard. Stronger than hash for **history denial**; different mechanism.

2. **"Chain length reveals password size"** — Published links do **not** encode original root length. Only normalized anchor material appears in `link_0`.

3. **"Infinite CPU breaks ITS"** — Layer 1 holds against unbounded algebra. Layer 2 fails if root is guessed — use strong roots/KDF at consumer.

4. **"Crate validates TRNG"** — No. Opaque bytes only.

---

## 3. Scope table

| Guarantee | Doc |
|-----------|-----|
| SSS backward ambiguity | [mathematics](SSS_CHAIN_mathematics.md) |
| Root brute-force | Operator + consumer KDF |
| CLI `.ssc` integrity | [manual](SSS_CHAIN_manual.md) |

---

## 4. Related crates

| Crate | Uses |
|-------|------|
| ITS-OTM | field, poly, lagrange, OTM chain helpers |
| ITS-timelock | field, `sss_chain_transition` |
| ITS-session (future) | link API |

Ecosystem master: [ITS_SECURITY_LAYERS.md](https://github.com/0x1F464/ITS/blob/master/ITS_SECURITY_LAYERS.md)
