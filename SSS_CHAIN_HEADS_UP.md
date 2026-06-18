# SSS_CHAIN — HEADS_UP (SSS_CHAIN_HEADS_UP.md)

## License: GNU GPLv3 Only
## Target: High-threat operators

---

## 1. Root material

- **TRNG** or **KDF output** from ITS-KeyManagement — not bare short passphrases in high-threat models.
- Root length on the wire is **not** published; Eve still brute-forces the **normalized anchor space** (~ `256^(link_byte_len-4)` material bytes).
- If root is guessed, full chain validates — treat root compromise as total break.

---

## 2. Layer 1 vs Layer 2

Layer 1 (ITS backward ambiguity) protects **history denial** against infinite CPU.

Layer 2 (root) is **your** responsibility. This crate does not enforce entropy or hide weak passphrases.

---

## 3. Coercion

Published links alone do not prove which root was used. Coercion scenarios may involve revealing root — outside crate scope. See timelock `deny()` for analogous deniability patterns.

---

## 4. Side channels

CLI and library aim for constant-time field ops (`subtle`). Terminal timing on validate loops is not hardened — run offline for high-threat verification.

---

## 5. Physical exfil

`.ssc` files on disk contain **public** chain material (not root). Protect root files separately; zeroize after use where possible.
