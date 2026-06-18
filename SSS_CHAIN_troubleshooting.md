# SSS_CHAIN — Troubleshooting (SSS_CHAIN_troubleshooting.md)

## License: GNU GPLv3 Only

---

## CLI / `.ssc`

### `parse: missing link_byte_len`

**Cause:** Truncated or hand-edited `.ssc` header.

**Fix:** Regenerate with `sss_chain generate` or restore header lines before `---` blocks.

### `parse: expected N links, found M`

**Cause:** `link_count` header does not match `---` blocks.

**Fix:** Count blocks; fix header or regenerate.

### `INVALID` on validate with correct-looking file

**Cause:** Wrong `--root`, flipped byte, or modulus mismatch (M31 vs M61 build).

**Fix:** Confirm root bytes; rebuild consumer and CLI with same `m61` feature flag.

### `arrange: DuplicateLinkIndex`

**Cause:** Two links share the same index field.

**Fix:** Deduplicate source; ensure one link per index.

### `arrange: MissingLinkIndex`

**Cause:** Gap in indices (e.g. 0,2 without 1) or wrong link count.

**Fix:** Recover missing link from backup or regenerate chain.

### `MemoryNotAligned`

**Cause:** `--total-bytes` not divisible by `--link-byte-len`.

**Fix:** Adjust bytes or link length.

---

## Library

### `LinkByteLenTooSmall`

Minimum `link_byte_len` is 8 (4-byte index + 4 material).

### `LinkCountZero` / `LinkCountTooLarge`

Span must be 1 .. 1_048_576 links.

---

## Integration

### OTM/timelock field mismatch

Both must use same `sss_chain` revision and `m61` feature. OTM re-exports `field_arith` from `sss_chain`.

---

## Docker

### musl build fails on non-x86_64

Dockerfile targets `x86_64-unknown-linux-musl`. For other arches, adjust `rustup target add` and COPY path.
