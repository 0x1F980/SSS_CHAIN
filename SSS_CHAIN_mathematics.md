# Forward Link Chain — Mathematical Specification, Postulates & Proofs

## License: GNU GPLv3 Only
## Target: Mathematicians, Cryptographers & Independent Reviewers

*(Implementation repository name: SSS_CHAIN. This document uses standard language; Rust identifiers appear only in Appendix A.)*

---

## Purpose

This document specifies a **forward link chain** over $\mathbb{F}_p$: a deterministic sequence of $n$ links $L_0,\ldots,L_{n-1}$ generated from a root secret $\rho$. Security has two **independent** parts:

1. **Transition ambiguity (information-theoretic):** If an adversary sees only a published **sum** $T = s_{\mathrm{prev}} + s_{\mathrm{next}}$, the pair $(s_{\mathrm{prev}}, s_{\mathrm{next}})$ is not unique — even with unbounded computation (Theorem 6.1). This is the same geometry as Shamir threshold $k=2$ with one linear constraint.
2. **Root authentication (search-based):** Validating a **full published chain** requires the root $\rho$ (or guessing it in the normalization class). This is **not** information-theoretic; entropy is at most $m \log_2 256$ bits when the anchor link $L_0$ is public.

**Not claimed:** one-way function, collision-resistant hash, or encryption.

**Reviewer task:** Read **§0.1** (worked example), then postulates and proofs, then **Appendix A** to confirm or reject the implementation.

> **Convention.** §2–§12 use mathematical symbols only. Code names are in Appendix A.

---

## 0. Notation

| Symbol | Meaning |
|--------|---------|
| $p$ | Prime modulus ($2^{31}-1$ default; optional $2^{61}-1$) |
| $\mathbb{F}_p$ | Finite field $\mathbb{Z}/p\mathbb{Z}$ |
| $L$ | Link size in bytes ($L \ge 8$) |
| $m$ | Material bytes per link ($m = L - 4$) |
| $n$ | Number of links |
| $\rho$ | Root secret (length not published) |
| $N(\rho)$ | Normalized anchor link (first link template) |
| $\alpha$ | Anchor scalar $\phi(\mathrm{mat}(N(\rho)))$ |
| $L_i$ | Link $i$ (byte string of length $L$) |
| $\iota(L_i)$ | Index field of $L_i$ (must equal $i$) |
| $s_i$ | Material scalar $\phi(\mathrm{mat}(L_i))$ |
| $\phi$, $\psi$ | Embed bytes $\to \mathbb{F}_p$; lossy write-back $\mathbb{F}_p \to$ bytes |
| $T$ | Transition sum $s_{\mathrm{prev}} + s_{\mathrm{next}}$ |

---

## 0.1 Worked example (read this first)

Parameters: $p = 2{,}147{,}483{,}647$, $n = 4$ links, anchor scalar $\alpha = 5$, initial material $s_0 = 10$. (Link encoding $\phi,\psi$ is omitted here; the recurrence lives in $\mathbb{F}_p$.)

### Forward generation

Recurrence for $i \ge 1$:

$$s_i = s_{i-1} + \alpha + i \pmod p.$$

| $i$ | Calculation | $s_i$ |
|-----|-------------|-------|
| $0$ | (anchor) | $10$ |
| $1$ | $10 + 5 + 1$ | $16$ |
| $2$ | $16 + 5 + 2$ | $23$ |
| $3$ | $23 + 5 + 3$ | $31$ |

**Closed form check:** $s_3 = s_0 + 3\alpha + \frac{3\cdot 4}{2} = 10 + 15 + 6 = 31$.

### Published transition sums

$$T_i = s_i + s_{i-1} \pmod p.$$

| Step | $s_{i-1}$ | $s_i$ | $T_i$ |
|------|-----------|-------|-------|
| $1$ | $10$ | $16$ | $26$ |
| $2$ | $16$ | $23$ | $39$ |
| $3$ | $23$ | $31$ | $54$ |

### Transition ambiguity (Theorem 6.1)

Suppose Eve sees **only** $T_1 = 26$ (not the full links). Any $s_{\mathrm{prev}} \in \mathbb{F}_p$ determines $s_{\mathrm{next}} = T_1 - s_{\mathrm{prev}}$. Two distinct examples:

| $s_{\mathrm{prev}}$ | $s_{\mathrm{next}}$ | Sum |
|---------------------|---------------------|-----|
| $10$ | $16$ | $26$ |
| $15$ | $11$ | $26$ |

There are exactly $p$ consistent pairs — no amount of computation identifies which occurred.

### Full chain vs transition-only

If Eve sees **all** link material $(s_0,s_1,s_2,s_3) = (10,16,23,31)$, she can read the scalars directly. **Authentication** still requires $\rho$: she must pass validation $L_0 = N(\rho)$ and the recurrence with $\alpha = \phi(\mathrm{mat}(N(\rho)))$. Without $\rho$, she cannot prove the chain was produced under a known root (except by brute force over normalized anchors, Postulate P7).

### Validation sketch

Given secret $\rho$ with $N(\rho)$ encoding anchor $s_0=10$ and $\alpha=5$, check $s_i = s_{i-1} + \alpha + i$ for $i=1,2,3$ — all hold above.

*Implementation correspondence:* recurrence and transition ambiguity witness — see Appendix A.

---

## Postulates

| ID | Postulate |
|----|-----------|
| **P0** | Arithmetic in $\mathbb{F}_p$, $p = 2^{31}-1$ (odd prime). |
| **P1** | Adversary Eve knows the specification, all published link bytes, all published transition sums, and the forward algorithm. She does not know $\rho$ unless she guesses. Unbounded computation. |
| **P2** | Transition ambiguity is **information-theoretic** (Theorem 6.1). |
| **P3** | Root validation is **operational** — search over normalized anchors, not information-theoretic. |
| **P4** | Forward map is deterministic: no auxiliary randomness given $(\alpha, s_0)$. |
| **P5** | Threshold-$2$ geometry: one equation $y_1 + y_2 = T$ in two share values leaves one degree of freedom. |
| **P6** | Byte embedding $\phi \circ \psi$ is not assumed injective. |
| **P7** | A public chain includes $L_0 = N(\rho)$; normalized anchor bytes are visible. |
| **P8** | Not a one-way function, digest, or encryption scheme. |

---

## 1. Proof map

| § | Content |
|---|---------|
| **0.1** | Worked example |
| 2–4 | Field, links, normalization |
| 5 | Forward recurrence |
| 6 | Transition theorems |
| 7 | Validation game |
| 8–12 | Threat model, hash comparison, checklist |
| **A** | Implementation audit |

---

## 2. Field (P0)

$$p = 2^{31} - 1 = 2147483647.$$

All equations below are in $\mathbb{F}_p$.

---

## 3. Links and embedding

### 3.1 Link format

Link $L_i \in \{0,\ldots,255\}^L$: bytes $0..3$ encode $\iota(L_i)=i$; bytes $4..L-1$ are material.

Valid chain: $\iota(L_{i+1}) = \iota(L_i) + 1$.

### 3.2 Embedding

$$\phi(b_0,\ldots,b_{m-1}) = \sum_{j=0}^{m-1} b_j \cdot 257^j \pmod p, \qquad s_i = \phi(\mathrm{mat}(L_i)).$$

### 3.3 Write-back $\psi$

Store field elements by truncating to $\min(m,8)$ big-endian bytes (lossy). **Lemma 3.1 (P6):** $\phi \circ \psi$ is generally not injective.

---

## 4. Root normalization

**Definition 4.1.** $N(\rho)$: zero-pad or truncate $\rho$ into material bytes of $L_0$, set $\iota(L_0)=0$.

**Definition 4.2.** $\alpha = \phi(\mathrm{mat}(N(\rho)))$.

| Property | Statement |
|----------|-----------|
| Length hiding | $|\rho|$ not on wire |
| Entropy cap | Only first $m$ bytes of $\rho$ affect $\alpha$ |
| Many-to-one | Distinct $\rho$ may share $N(\rho)$ |

---

## 5. Forward recurrence (P4)

Given $\alpha$ and $s_0$ from $L_0 = N(\rho)$, for $i \ge 1$:

$$s_i = s_{i-1} + \alpha + i \pmod p.$$

Equivalently $T_i = s_i + s_{i-1} = 2s_{i-1} + \alpha + i$.

**Proposition 5.1.**

$$s_i = s_0 + i\alpha + \frac{i(i+1)}{2} \pmod p.$$

**Proof.** By induction on $i$. $\square$

---

## 6. Transition algebra (P2, P5)

$$T = s_{\mathrm{prev}} + s_{\mathrm{next}}, \qquad s_{\mathrm{next}} = T - s_{\mathrm{prev}}.$$

### Theorem 6.1

For fixed $T$, the set $\{(s_{\mathrm{prev}}, s_{\mathrm{next}}) : s_{\mathrm{prev}} + s_{\mathrm{next}} = T\}$ has cardinality $p$.

**Proof.** Bijection $s_{\mathrm{prev}} \mapsto (s_{\mathrm{prev}}, T - s_{\mathrm{prev}})$. $\square$

### Corollary 6.2

An observer of $T$ alone learns nothing about which pair occurred (P2).

### Theorem 6.3

$s_{\mathrm{next}} = T - s_{\mathrm{prev}}$ whenever $T = s_{\mathrm{prev}} + s_{\mathrm{next}}$. $\square$

### Shamir connection (P5)

For line $P(x)=S+ax$, publishing $y_a + y_b = T$ is one linear constraint in two unknowns. With shares at $x=1,2$: $S = 2y_1 - y_2$.

---

## 7. Validation game (P3, P7)

**Public:** $(L,n)$ and $(L_0,\ldots,L_{n-1})$. **Secret:** $\rho$.

Valid **iff**:

1. $\iota(L_i)=i$ and $|L_i|=L$.
2. $L_0 = N(\rho)$.
3. $s_i = s_{i-1} + \alpha + i$ for all $i \ge 1$, with $\alpha = \phi(\mathrm{mat}(N(\rho)))$ and material consistent with $\psi(s_i)$.

**Proposition 7.1.** Equivalent to $s_i = s_0 + i\alpha + i(i+1)/2$ for all $i$.

**Proposition 7.2.** Without $\rho$ matching $L_0$, validation fails except by guessing the normalization class.

**Observation 7.3 (P7).** Public $L_0$ caps root search at $\le 256^m$ candidates; need $\approx m \log_2 256$ bits of entropy in normalized material.

---

## 8. Threat model (P1)

| Goal | Mechanism |
|------|-----------|
| Unique pair from $T$ only | Blocked by Theorem 6.1 |
| Authenticate chain without $\rho$ | Blocked by §7 (search) |
| Recover $|\rho|$ | Not encoded |
| Forge valid chain | Search over $\{\rho' : N(\rho')=L_0\}$ |

Root search cost does not grow with chain length $n$ for fixed $L$.

---

## 9. Two security properties

| Property | Type | Object |
|----------|------|--------|
| Transition ambiguity | Information-theoretic | Sum $T = s_{\mathrm{prev}} + s_{\mathrm{next}}$ |
| Root authentication | Search / entropy | Normalized anchor $N(\rho)$ |

---

## 10. Comparison to hash chains

| | Hash chain $h_{i+1}=H(h_i)$ | Forward link chain |
|---|---------------------------|-------------------|
| Backward from digest | Computational preimage | Ambiguity from $T$ alone |
| Unbounded CPU | Breaks finite domain | Transition ambiguity unchanged |
| Forward verify | Iterated hash | Requires $\rho$ |
| Assumption | $H$ hard | None for Theorem 6.1 |

---

## 11. Non-claims (P8)

Not provided: one-wayness without $\rho$; hiding $N(\rho)$ when $L_0$ is public; threshold secret recovery from links alone; side-channel-free verification.

---

## 12. Review checklist

1. Accept P0–P8?
2. Accept Theorem 6.1?
3. Accept Observation 7.3 for your deployment?
4. Verify Appendix A against code?
5. Reject if you need a hash one-way function or hidden anchor?

---

## 13. Summary identities

$$\phi(b_0,\ldots,b_{m-1}) = \sum_{j=0}^{m-1} b_j \cdot 257^j \pmod p$$

$$s_i = s_{i-1} + \alpha + i = s_0 + i\alpha + \frac{i(i+1)}{2} \pmod p$$

$$T = s_{\mathrm{prev}} + s_{\mathrm{next}}, \qquad |\{(s_{\mathrm{prev}},s_{\mathrm{next}}): s_{\mathrm{prev}}+s_{\mathrm{next}}=T\}| = p$$

---

## 14. Worked example (reference)

See **§0.1**: $\alpha=5$, $s = (10,16,23,31)$, transitions $(26,39,54)$, ambiguous pairs for $T=26$ include $(10,16)$ and $(15,11)$.

---

## Appendix A — Implementation correspondence

**Not part of the mathematical definition.**

| Math | Rust name | File |
|------|-----------|------|
| $\mathbb{F}_p$ | `FieldElement`, `MODULUS` | `sss_chain_field.rs` |
| $\iota(L_i)$ | `write_link_index`, `read_link_index` | `sss_chain_link.rs` |
| $\phi$, $\psi$ | `field_from_material_bytes`, `write_material_field` | `sss_chain_link.rs` |
| $s_i$ | `material_field` | `sss_chain_link.rs` |
| $N(\rho)$ | `normalize_sss_chain_root` | `sss_chain_link.rs` |
| $\alpha$ | `root_anchor_field` | `sss_chain_link.rs` |
| Forward step | `sss_chain_epoch_step_forward` | `sss_chain_epoch.rs` |
| $T$ publish / recover | `sss_chain_transition`, `sss_chain_step_forward_from_transition` | `sss_chain_epoch.rs` |
| Generate / validate | `sss_chain_generate`, `sss_chain_validate_full` | `sss_chain_link.rs` |
| Lagrange | `lagrange_interpolate` | `sss_chain_lagrange.rs` |
| Thm 6.1 witness | test `backward_transition_underdetermination` | `sss_chain_link.rs` |
| Thm 6.3 (machine-checked) | `transition_invertible` | timelock `Stl/TimeLock.lean` |

Compact symbols: [SSS_CHAIN_FORMULAS.md](SSS_CHAIN_FORMULAS.md). Narrative layers: [SSS_CHAIN_SECURITY_LAYERS.md](SSS_CHAIN_SECURITY_LAYERS.md).
