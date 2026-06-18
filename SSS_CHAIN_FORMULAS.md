# SSS_CHAIN canonical formulas

| Symbol | Definition | Rust |
|--------|------------|------|
| $p$ | $2^{31}-1$ (M31) | `MODULUS` |
| `link[i][0..4)` | Index $i$ u32 BE | `write_link_index` |
| `anchor` | material(`normalize(root)`) | `root_anchor_field` |
| $s_i$ | material field of link $i$ | `material_field` |
| $s_{i+1}$ | $s_i + anchor + i$ | `sss_chain_epoch_step_forward(..., entropy=0)` |
| `trans` | $next + cur$ | `sss_chain_transition` |
| `next` | $trans - cur$ | `sss_chain_step_forward_from_transition` |

Link count from memory: `link_count = total_bytes / link_byte_len`.

CLI wire: `.ssc` text — see [SSS_CHAIN_manual.md](SSS_CHAIN_manual.md).

Timelock cross-ref: [ITS-self_enclosed_timelock_FORMULAS.md](https://github.com/0x1F464/ITS-self_enclosed_timelock/blob/master/ITS-self_enclosed_timelock_FORMULAS.md) — replace inline `transitions` with `sss_chain_transition`.
