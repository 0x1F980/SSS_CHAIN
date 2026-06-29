# SSS_CHAIN: Shared SSS-Chaining Link API Portal

## GNU General Public License v3.0 Only
Copyright (C) 2026 0x1F980. All rights reserved.

SSS_CHAIN is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, **version 3 of the License only** (not later versions).

**No MIT/Apache dual licensing.** Commercial use is permitted only under GPL-3.0 obligations (source availability, license propagation, and copyleft). See [LICENSE](LICENSE) and [RELICENSING.md](RELICENSING.md).

## Target: Cryptographers, Security Auditors, Integrators

**ITS = Information-Theoretic Secrecy** — [ecosystem definition](https://github.com/0x1F980/ITS-ROUTING/blob/master/ITS_ECOSYSTEM.md#its--information-theoretic-secrecy). SSS_CHAIN is the shared algebra underlying ITS wire, OTM, and timelock layers.

**Read first:** **[SSS_CHAIN_SECURITY_LAYERS.md](SSS_CHAIN_SECURITY_LAYERS.md)** — Layer 1 ITS backward ambiguity vs Layer 2 root secrecy.

Minimal `#![no_std]` Rust crate for **SSS-chaining** link generation, validation, and arrangement (`k=2`). Shared field/epoch/OTM primitives for ITS-OTM, ITS-timelock, and future ITS-session.

**Repository:** `git@github.com:0x1F980/SSS_CHAIN.git`

```bash
git clone git@github.com:0x1F980/SSS_CHAIN.git
cd SSS_CHAIN
cargo test
cargo build --release --bin sss_chain
nix-shell --run "cargo build --release --bin sss_chain"
docker build -t sss-chain:local .
sss_chain demo
```

---

## The 7-Pillar Documentation Architecture

```
                  +----------------------------------------------+
                  |                  README.md                   |
                  |                (This Portal)                 |
                  +----------------------+-----------------------+
                                         |
                  +----------------------v-----------------------+
                  |      SSS_CHAIN_SECURITY_LAYERS.md (#0)       |
                  +----------------------+-----------------------+
                                         |
         +-------------------------------+-------------------------------+
         |                               |                               |
+--------v--------+             +--------v--------+             +--------v--------+
|    Vision       |             |   Mathematics   |             |     Manual      |
|  (Core Design & |             | (Threat Model & |             | (API + CLI +    |
|  Threat Model)  |             |  vs Hash)       |             |  Build)         |
+--------+--------+             +--------+--------+             +--------+--------+
         |                               |                               |
         +-------------------------------+-------------------------------+
         |                               |                               |
+--------v--------+             +--------v--------+             +--------v--------+
| Troubleshooting |             |    Use-Cases    |             |    HEADS_UP     |
+-----------------+             +-----------------+             +-----------------+
```

0. **[Security Layers](SSS_CHAIN_SECURITY_LAYERS.md)** — ITS vs root; no chain-length root policy.
1. **[Vision](SSS_CHAIN_vision.md)** — Purpose, Eve model, ecosystem role.
2. **[Mathematics](SSS_CHAIN_mathematics.md)** — k=2 underdetermination; hash comparison.
3. **[Manual](SSS_CHAIN_manual.md)** — Rust API, `sss_chain` CLI, Docker/Nix, man page.
4. **[Troubleshooting](SSS_CHAIN_troubleshooting.md)** — `.ssc`, arrange, M61 mismatch.
5. **[Use-Cases](SSS_CHAIN_usecase.md)** — Pipes, OTM/timelock integration.
6. **[HEADS_UP](SSS_CHAIN_HEADS_UP.md)** — Root handling, coercion notes.

**Also:** [FORMULAS](SSS_CHAIN_FORMULAS.md) · [FORMAL_VERIFICATION](SSS_CHAIN_FORMAL_VERIFICATION.md)

---

## Ecosystem

| Repository | Role |
|---|---|
| **SSS_CHAIN** (this repo) | Shared SSS chain + field + epoch |
| [ITS-OTM_public_attestation](https://github.com/0x1F980/ITS-OTM_public_attestation) | WC-MAC public attestation |
| [ITS-self_enclosed_timelock](https://github.com/0x1F980/ITS-self_enclosed_timelock) | Time-lock puzzle |
| [ITS-routing / ROUTING](https://github.com/0x1F980/ITS-ROUTING) | Orchestration CLI |

---

## Quick CLI

```bash
echo -n "my-secret" | sss_chain generate --root - --link-count 10 --out chain.ssc
sss_chain validate --root secret.txt --in chain.ssc
```

See `man/sss_chain.1` and shell completions in `completions/` (bash, zsh, fish, PowerShell).

---

## Quick API

```rust
use sss_chain::{sss_chain_generate, sss_chain_validate_full, SssChainConfig, SssChainSpan};

let cfg = SssChainConfig::new(16, SssChainSpan::LinkCount(10));
let root = b"operator-root";
let chain = sss_chain_generate(root, &cfg).unwrap();
assert!(sss_chain_validate_full(root, &chain, &cfg));
```
