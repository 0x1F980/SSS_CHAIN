# SSS_CHAIN — Use-Cases (SSS_CHAIN_usecase.md)

## License: GNU GPLv3 Only

---

## 1. Pipe-generated session chain (operator)

```bash
trng_bytes | sss_chain generate --root - --link-count 100 --link-byte-len 32 --out session.ssc
sss_chain validate --root /secure/root.bin --in session.ssc
```

Publish individual links out-of-order; receiver:

```bash
sss_chain arrange --root /secure/root.bin --in shuffled.ssc --out ordered.ssc
```

---

## 2. ITS-OTM (library consumer)

OTM uses `sss_chain` for field, polynomial, Lagrange, and `combine_sss_chains` — not the link `.ssc` format. See [ITS-OTM_public_attestation](https://github.com/0x1F980/ITS-OTM_public_attestation).

```toml
sss_chain = { git = "...", branch = "main" }
```

---

## 3. ITS-timelock (epoch transitions)

Timelock calls `sss_chain_transition` and `sss_chain_step_forward_from_transition` in `time_lock.rs` — shared algebra with link chaining.

---

## 4. Fork guide

1. Depend on `sss_chain` path or git; enable `m61` if needed.
2. Do **not** duplicate `field_arith` — re-export from `sss_chain`.
3. KDF/root policy stays in your crate; call `sss_chain_generate` with opaque root bytes.

---

## 5. QR / sneakernet

Export `.ssc` as text; split `---` blocks across QR chunks. Reassemble file before `arrange`.
