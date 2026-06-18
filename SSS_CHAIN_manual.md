# SSS_CHAIN — Manual (SSS_CHAIN_manual.md)

## License: GNU GPLv3 Only
## Target: Integrators, operators

> **Scope:** [SSS_CHAIN_SECURITY_LAYERS.md](SSS_CHAIN_SECURITY_LAYERS.md)

---

## 1. Build & test

```bash
git clone git@github.com:0x1F980/SSS_CHAIN.git
cd SSS_CHAIN
cargo test
cargo build --release --bin sss_chain
nix-shell --run "cargo build --release --bin sss_chain"
docker build -t sss-chain:local .
```

M61 field:

```bash
cargo test --features m61
```

---

## 2. Rust library API

```rust
use sss_chain::{
    sss_chain_generate, sss_chain_validate_full, SssChainConfig, SssChainSpan,
};

let cfg = SssChainConfig::new(16, SssChainSpan::LinkCount(10));
let root = b"operator-secret";
let chain = sss_chain_generate(root, &cfg).unwrap();
assert!(sss_chain_validate_full(root, &chain, &cfg));
```

### Core types

| Type / fn | Role |
|-----------|------|
| `SssChainLink` | `Vec<u8>` one link |
| `SssChainConfig` | `link_byte_len`, `span` |
| `SssChainSpan::LinkCount` / `TotalMemoryBytes` | Chain length |
| `normalize_sss_chain_root` | Root → link 0 |
| `sss_chain_generate` | Full chain |
| `sss_chain_validate_*` | Root, link, full chain |
| `sss_chain_arrange_links` | Sort shuffled links |
| `sss_chain_depth_from_previous_link` | Depth check |

### Modules

| Module | Contents |
|--------|----------|
| `sss_chain_field` | M31/M61 field |
| `sss_chain_epoch` | Transition step |
| `sss_chain_otm` | OTM combine/verify shares |
| `sss_chain_link` | Link API |

---

## 3. CLI (`sss_chain`)

### Commands

```bash
sss_chain generate --root PATH --link-count N [--link-byte-len 16] [--out PATH]
sss_chain validate  --root PATH --in PATH [--quiet]
sss_chain arrange   --root PATH --in PATH [--out PATH]
sss_chain depth     --root PATH --in PATH --index N
sss_chain demo [--out PATH]
```

`PATH` = `-` for stdin/stdout.

### `.ssc` wire format

```text
# sss-chain v1
link_byte_len: 16
link_count: 5
---
index: 0
hex: 00000000...
```

### Exit codes

| Code | Meaning |
|------|---------|
| 0 | Success / VALID |
| 1 | Invalid / error |
| 2 | Usage |

---

## 4. Man page & completion

```bash
sudo cp man/sss_chain.1 /usr/local/share/man/man1/ && sudo mandb
man sss_chain
source completions/sss_chain.bash
```

---

## 5. Docker

Static musl binary in `scratch`:

```bash
docker build -t sss-chain:local .
docker run --rm sss-chain:local help
```

---

## 6. Nix

```bash
nix-shell
cargo test && cargo build --release --bin sss_chain
```
