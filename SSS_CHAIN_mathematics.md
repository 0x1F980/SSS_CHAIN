# SSS_CHAIN — mathematics

## Threat model: unbounded-compute oracle (Eve)

**Assumption:** Eve knows everything **except** `root_pass`:

- The full algorithm (`sss_chain_epoch_step_forward`, k=2 layout)
- All **published** `SssChainLink` bytes
- Unbounded computation time

She **cannot** uniquely reverse prior links by algebra alone (SSS underdetermination — Layer 1). Her attack on the secret is **guessing** `root_pass` and running forward validation (Layer 2).

## Two security layers

| Layer | Protects against | Mechanism |
|-------|------------------|-----------|
| **1 — SSS-chaining ITS** | Unique backward reconstruction / proof of history without key | k=2 underdetermination; **not** computational hardness |
| **2 — Root secrecy** | Brute-force on `root_pass` | Operator-chosen root entropy; **not** tied to chain length |

**Important:** Layer 1 alone does not stop root guessing. Layer 2 depends on root strength (TRNG, KDF, passphrase). This crate does **not** require root length to equal total chain bytes.

## Comparison to hash one-wayness

| | Hash chain | SSS-chaining |
|---|------------|--------------|
| Backward without secret | Computationally hard (preimage) | **Not unique** — many consistent histories (ITS) |
| vs infinite CPU on Layer 1 | Eventually breaks at finite domain | Underdetermination **unchanged** |
| Forward verify | One preimage per digest | Requires `root_pass` to validate full chain |

SSS-chaining is **not** a classical one-way function; it is **ITS backward ambiguity** plus **root-gated forward validation**.

## Root normalization (PerLinkAnchor)

- `len(root) > material_len` → truncate into link material region
- `len(root) < material_len` → zero-pad right in `[4..link_byte_len)`
- `link_0` = normalized material + index `0` at `[0..4)`
- **Original passphrase length is not published** — Eve sees only normalized anchor bytes in `link_0`, not how long the operator's secret was

If Eve guesses the correct root, `sss_chain_validate_full` succeeds — **game over**, as with any keyed system.

## Link format

```
SssChainLink = link_byte_len bytes  (minimum 8)
  [0..4)   index u32 big-endian
  [4..)    material
```

## Forward chain (k=2)

1. `link_0` from normalized root
2. For i = 1 .. n-1: `sss_chain_epoch_step_forward` with entropy = 0

```
trans = current + current + root_anchor + index
next  = trans - current = current + root_anchor + index
```

## Span resolution

```rust
pub enum SssChainSpan {
    LinkCount(u32),
    TotalMemoryBytes(usize),
}
```

`TotalMemoryBytes % link_byte_len != 0` → `MemoryNotAligned`.

## k=2 backward underdetermination

An adversary without `root_pass` faces a **consistent family** of backward interpretations. Uniquely proving which chain occurred requires root material only the holder possesses.
