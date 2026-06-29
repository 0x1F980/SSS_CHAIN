# SSS_CHAIN — Mathematical Specification, Postulates & Proofs

## License: GNU GPLv3 Only
## Target: Mathematicians, Cryptographers & Independent Reviewers

---

## Purpose

This document specifies the **mathematical object** called **SSS-chaining** ($k=2$ threshold layout): a forward-only sequence of links over $\mathbb{F}_p$ whose security rests on two independent ideas:

1. **Layer 1 (ITS):** Publishing only a transition **sum** $T = s_{\mathrm{next}} + s_{\mathrm{cur}}$ does not determine the pair $(s_{\mathrm{cur}}, s_{\mathrm{next}})$ uniquely — even against unbounded computation.
2. **Layer 2 (root):** Validating a **full link chain** requires knowledge of a root secret $\rho$ whose normalized anchor binds the recurrence; without $\rho$, an adversary cannot authenticate the chain (except by search in the normalization class).

**Design goal:** Offer **information-theoretic backward ambiguity** on transition sums (as in Shamir $k=2$ geometry) combined with **root-gated forward validation** — **not** classical hash-chain one-wayness.

**Reviewer task:** Read the postulates and proofs below, then inspect the implementation labels in **Appendix A** to decide whether the code realises the stated mathematics (**confirm**) or deviates (**reject**).

> **Convention.** Throughout §2–§12, only symbols ($\rho$, $\alpha$, $\phi$, $T$, …) are mathematical notation. Rust identifiers appear **only** in *implementation correspondence* lines and Appendix A; they name code artefacts for audit and are **not** part of any definition, theorem, or proof.

---

## Postulates

These are the explicit assumptions on which the security analysis rests. Reject the design if you disagree with a postulate.

| ID | Postulate |
|----|-----------|
| **P0** | Arithmetic takes place in the finite field $\mathbb{F}_p$ for a fixed odd prime $p$ (primary: $p = 2^{31}-1$). |
| **P1** | **Adversary model:** Eve knows this specification, all published link bytes, all published transition sums, and the complete forward algorithm. She does **not** know $\rho$ unless she guesses it. Eve has **unbounded** computational power. |
| **P2** | **Layer 1 classification:** Security of transition backward ambiguity is **information-theoretic (ITS)** — it does not rely on computational hardness or hardness assumptions. |
| **P3** | **Layer 2 classification:** Security of root-gated validation is **operational** — it reduces to the entropy of $\rho$ after normalization (search space $\le 256^m$). It is **not** information-theoretic. |
| **P4** | **Zero epoch entropy:** The forward map uses no auxiliary randomness; given $(\alpha, s_0)$ the sequence $(s_i)$ is uniquely determined. |
| **P5** | **Shamir $k=2$ geometry:** Share values lie on a degree-1 polynomial $P(x) = S + ax$. Publishing one linear constraint in two share values leaves exactly one degree of freedom. |
| **P6** | **Lossy encoding:** Material bytes embed into $\mathbb{F}_p$ via $\phi$ and write-back via $\psi$; $\phi \circ \psi$ is **not** assumed injective. |
| **P7** | **Published anchor:** A fully public chain includes $L_0 = N(\rho)$; normalized anchor bytes are therefore visible on the wire. Layer 2 must not assume hiding of $N(\rho)$. |
| **P8** | **Non-claim:** SSS-chaining is **not** claimed to be a one-way function, collision-resistant digest, or IND-CCA encryption scheme. |

**ITS** = *information-theoretic secrecy*: advantage does not shrink toward zero via faster algorithms; ambiguity is exact.

---

## 0. Notation

| Symbol | Mathematical meaning |
|--------|----------------------|
| $p$ | Prime modulus; primary $p = 2^{31}-1$; alternate $p = 2^{61}-1$ |
| $\mathbb{F}_p$ | Field $\mathbb{Z}/p\mathbb{Z}$ as $\{0,\ldots,p-1\}$ |
| $L \ge 8$ | Link length (bytes) |
| $m = L - 4$ | Material byte count per link |
| $n \ge 1$ | Number of links |
| $\rho$ | Root secret; $|\rho|$ not published |
| $N(\rho)$ | Normalized anchor link |
| $\alpha$ | Anchor field element $\phi(\mathrm{mat}(N(\rho)))$ |
| $L_i$ | Link $i$; byte string in $\{0,\ldots,255\}^L$ |
| $\iota(L_i)$ | Index in bytes 0–3 (big-endian) |
| $\mathrm{mat}(L_i)$ | Material bytes $(L_i[4],\ldots,L_i[L-1])$ |
| $s_i$ | Material field $\phi(\mathrm{mat}(L_i))$ |
| $\phi$ | Embed $\{0,\ldots,255\}^m \to \mathbb{F}_p$ |
| $\psi$ | Lossy write-back $\mathbb{F}_p \to \{0,\ldots,255\}^m$ |
| $\tau_i$ | Epoch transition value $s_i + s_{i-1}$ |
| $T$ | Generic transition sum $s_{\mathrm{next}} + s_{\mathrm{cur}}$ |

---

## 1. Proof map

| § | Content | Status |
|---|---------|--------|
| 2 | Field & parameters | Setting (P0) |
| 3–4 | Links, $\phi$, $\psi$, $N(\rho)$ | Definitions |
| 5 | Forward recurrence | Proved (algebra) |
| 6 | Transition non-injectivity | **Theorem (ITS, P2, P5)** |
| 7 | Validation game | Propositions |
| 8–9 | Threat model & layers | P1, P2, P3 |
| 10 | vs hash chains | Comparative |
| 11 | Non-claims | P8 |
| 12 | Review checklist | — |
| **A** | Implementation correspondence | Code audit only |

---

## 2. Algebraic setting

Fix odd prime $p$. Primary instance:

$$p = 2^{31} - 1 = 2147483647.$$

Alternate parameter: $p = 2^{61} - 1$. All operations are in $\mathbb{F}_p$. Since $p$ is odd, $2^{-1}$ exists in $\mathbb{F}_p$.

*Implementation correspondence:* `MODULUS`, `FieldElement` — field carrier and reduction in `sss_chain_field.rs`.

---

## 3. Links and material embedding

### 3.1 Link structure

Link $L_i \in \{0,\ldots,255\}^L$ satisfies:

$$\iota(L_i) = i, \qquad \mathrm{mat}(L_i) \in \{0,\ldots,255\}^m.$$

Chain $(L_0,\ldots,L_{n-1})$: $\iota(L_{i+1}) = \iota(L_i) + 1$. Wire size $B = nL$; specifying $(L,B)$ with $B \equiv 0 \pmod L$ gives $n = B/L$.

*Implementation correspondence:* `write_link_index` / `read_link_index` — $\iota$; `SssChainSpan::LinkCount(n)` and `SssChainSpan::TotalMemoryBytes(B)` — chain length specification.

### 3.2 Embedding $\phi$

$$\phi(b_0,\ldots,b_{m-1}) = \sum_{j=0}^{m-1} b_j \cdot 257^{j} \pmod p, \qquad s_i = \phi(\mathrm{mat}(L_i)).$$

*Implementation correspondence:* `field_from_material_bytes` — $\phi$; `material_field` — $s_i = \phi(\mathrm{mat}(L_i))$.

### 3.3 Write-back $\psi$ (lossy)

Store $v \in \mathbb{F}_p$ as the rightmost $\min(m,8)$ bytes of the big-endian representation of $v$, left-padded to length $m$. Map $\psi: \mathbb{F}_p \to \{0,\ldots,255\}^m$.

**Lemma 3.1 (P6).** $\phi \circ \psi$ is generally **not** injective when $m$ is small.

**Proof.** $|\{0,\ldots,255\}^m| = 256^m \ll p = |\mathbb{F}_p|$ for typical $m$; surjectivity of any round-trip through a $256^m$-sized byte space onto $\mathbb{F}_p$ is impossible. $\square$

*Implementation correspondence:* `write_material_field` — $\psi$.

---

## 4. Root normalization

$\rho \in \{0,1\}^*$; $|\rho|$ not transmitted.

**Definition 4.1.** $N(\rho) \in \{0,\ldots,255\}^L$:

1. Zero-initialize; $\iota(N(\rho)) = 0$.
2. $c = \min(|\rho|, m)$: copy $\rho[0..c)$ into bytes $4..4+c$.
3. If $|\rho| > m$: truncate — bytes $4..L$ $\leftarrow$ $\rho[0..m)$.

**Definition 4.2.** Anchor field $\alpha = \phi(\mathrm{mat}(N(\rho)))$.

| Property | Statement |
|----------|-----------|
| Length hiding | Wire shows $N(\rho)$ only; $|\rho|$ not encoded |
| Entropy cap (P7) | Entropy beyond first $m$ bytes discarded |
| Many-to-one | $\rho_1 \neq \rho_2$ may give $N(\rho_1) = N(\rho_2)$ |

*Implementation correspondence:* `normalize_per_link_bytes`, `normalize_sss_chain_root` — $N(\rho)$; `root_anchor_field` — $\alpha$.

---

## 5. Forward SSS chain (P4)

Given $\alpha$ and $s_0 = \phi(\mathrm{mat}(L_0))$, $L_0 = N(\rho)$. For $i \ge 1$:

$$\tau_i = 2s_{i-1} + \alpha + i \pmod p, \qquad s_i = \tau_i - s_{i-1} = s_{i-1} + \alpha + i \pmod p.$$

Note $\tau_i = s_i + s_{i-1}$.

**Proposition 5.1 (closed form).**

$$s_i = s_0 + i\alpha + \frac{i(i+1)}{2} \pmod p.$$

**Proof.** Induction on $i$. Base $i=0$ trivial. Step: $s_i = s_{i-1} + \alpha + i = s_0 + (i-1)\alpha + \frac{(i-1)i}{2} + \alpha + i = s_0 + i\alpha + \frac{i(i+1)}{2}$. $\square$

**Proposition 5.2 (determinism, P4).** $(s_0,\ldots,s_{n-1})$ is uniquely fixed by $(\alpha, s_0)$.

*Implementation correspondence:* `sss_chain_epoch_step_forward(..., entropy = 0)` — one step $s_{i-1} \mapsto s_i$; `sss_chain_generate` — full chain; `expected_link_from_previous` — embed $s_i$ via $\psi$ into $L_i$.

---

## 6. Transition algebra (P2, P5)

$$T = s_{\mathrm{next}} + s_{\mathrm{cur}} \pmod p, \qquad s_{\mathrm{next}} = T - s_{\mathrm{cur}} \pmod p.$$

### Theorem 6.1 (Transition non-injectivity)

$$|\mathcal{P}(T)| = p \quad \text{where} \quad \mathcal{P}(T) = \{(s_{\mathrm{cur}}, s_{\mathrm{next}}) \in \mathbb{F}_p^2 : s_{\mathrm{cur}} + s_{\mathrm{next}} = T\}.$$

**Proof.** Bijection $s_{\mathrm{cur}} \mapsto (s_{\mathrm{cur}}, T - s_{\mathrm{cur}})$ with $\mathbb{F}_p$. $\square$

### Corollary 6.2 (backward ambiguity, P2)

An observer of $T$ alone learns **no** information about which pair in $\mathcal{P}(T)$ occurred. Holds against unbounded Eve (P1).

### Theorem 6.3 (invertibility)

$s_{\mathrm{next}} = T - s_{\mathrm{cur}}$ whenever $T = s_{\mathrm{next}} + s_{\mathrm{cur}}$. *Proof:* cancellation in $\mathbb{F}_p$. $\square$

### 6.1 Shamir $k=2$ (P5)

$P(x) = S + ax$. Two points determine $P$; one equation $y_{\mathrm{next}} + y_{\mathrm{cur}} = T$ leaves one degree of freedom — origin of Theorem 6.1.

Lagrange at $x_1=1$, $x_2=2$: $S = 2y_1 - y_2 \pmod p$.

*Implementation correspondence:* `sss_chain_transition` — publish $T$; `sss_chain_step_forward_from_transition` — recover $s_{\mathrm{next}}$; `lagrange_interpolate`, `verify_backward_share` — Lagrange path when two shares plus root at $0$ are available (`sss_chain_lagrange.rs`, `sss_chain_otm.rs`).

### 6.2 Worked example

$s_{\mathrm{cur}}=40$, $s_{\mathrm{next}}=60 \Rightarrow T=100$. Also $(45,55)$ gives $T=100$. Distinct histories; same published sum.

*Implementation correspondence:* test `backward_transition_underdetermination` in `sss_chain_link.rs` — numeric witness for Theorem 6.1.

---

## 7. Validation game (P3, P7)

**Public:** $(L,n)$ and chain $(L_0,\ldots,L_{n-1})$. **Secret:** $\rho$.

Validation **iff**:

1. $|L_i|=L$, $\iota(L_i)=i$.
2. $L_0 = N(\rho)$.
3. $s_i = s_{i-1} + \alpha + i$ for all $i \ge 1$, with $s_j = \phi(\mathrm{mat}(L_j))$, $\alpha = \phi(\mathrm{mat}(N(\rho)))$, and material consistent with $\psi(s_i)$.

**Proposition 7.1.** Validation $\Leftrightarrow$ consecutive indices and $s_i = s_0 + i\alpha + i(i+1)/2$ for all $i$.

**Proposition 7.2.** Without correct $\rho$ (step 2) or $\alpha$ (step 3), validation fails except by guessing $\rho$ in the normalization class.

**Observation 7.3 (P7).** Public $L_0$ exposes $\mathrm{mat}(N(\rho))$. Layer 2 $\le 256^m$ candidates; need $\approx m \log_2 256$ bits entropy in normalized material.

*Implementation correspondence:* `sss_chain_validate_full` — full game; `sss_chain_validate_root_link` — step 2; `sss_chain_validate_link` — step 3; `sss_chain_arrange_links` — reorder published links then validate.

---

## 8. Threat model (P1)

| Eve's goal | Blocked by | Mathematical basis |
|------------|------------|-------------------|
| Unique backward pair from $T$ | Layer 1 | Theorem 6.1 |
| Authenticate chain without $\rho$ | Layer 2 | §7 |
| Recover $|\rho|$ | — | Not encoded |
| Forge chain without $\rho$ | Layer 2 | Search in $\{\rho' : N(\rho') = L_0\}$ |

Layer 1 does not prevent guessing $\rho$. Layer 2 cost is **independent of $n$** for fixed $L$.

---

## 9. Security layers

| Layer | Type (postulate) | Object | Guarantee |
|-------|------------------|--------|-----------|
| **1 — SSS ITS** | P2 | $T = s_{\mathrm{next}} + s_{\mathrm{cur}}$ | $|\mathcal{P}(T)| = p$ |
| **2 — Root** | P3 | $N(\rho)$ | Search / entropy; not ITS |

Not a one-way function (P8). ITS ambiguity on sums + root-gated validation on full chains.

---

## 10. Comparison to hash chains

| | Hash chain $h_{i+1}=H(h_i)$ | SSS-chaining |
|---|------------------------------|--------------|
| Backward from digest | Computational preimage | ITS ambiguity from $T$ (Thm 6.1) |
| Unbounded CPU | Breaks finite domain | Layer 1 unchanged |
| Forward verify | One-way walk | Requires $\rho$ (Layer 2) |
| Assumption | $H$ hard | None for Thm 6.1 |

---

## 11. Explicit non-claims (P8)

Reject if you require:

1. Computational one-wayness without $\rho$.
2. Hiding $N(\rho)$ when $L_0$ is public (P7).
3. In-crate entropy stretching (external KDF is operational).
4. $n$-of-$k$ threshold recovery from links alone.
5. Side-channel resistance of verification (implementation).

---

## 12. Review checklist

| Step | Question | Confirm if |
|------|----------|------------|
| 1 | Accept P0–P8? | Postulates match your threat model |
| 2 | Accept Theorem 6.1? | ITS Layer 1 holds for transition-only wire |
| 3 | Accept Observation 7.3? | Layer 2 entropy cap acceptable when $L_0$ public |
| 4 | Code audit (Appendix A) | Each math map matches Rust implementation |
| 5 | Reject? | You need OWF / hidden anchor / ITS root |

---

## 13. Summary identities

$$\phi(b_0,\ldots,b_{m-1}) = \sum_{j=0}^{m-1} b_j \cdot 257^j \pmod p$$

$$\alpha = \phi(\mathrm{mat}(N(\rho))), \qquad s_i = s_{i-1} + \alpha + i = s_0 + i\alpha + \frac{i(i+1)}{2} \pmod p$$

$$T = s_{\mathrm{next}} + s_{\mathrm{cur}}, \qquad |\mathcal{P}(T)| = p$$

---

## Appendix A — Implementation correspondence (code audit)

**This appendix is not mathematics.** It maps each mathematical object to a Rust identifier so a reviewer can open the source and verify **confirm** (implementation matches spec) or **reject** (mismatch).

| Mathematical object | Rust label | Location |
|---------------------|------------|----------|
| $\mathbb{F}_p$, $p$ | `FieldElement`, `MODULUS` | `src/sss_chain_field.rs` |
| $\iota(L_i)$ | `write_link_index`, `read_link_index` | `src/sss_chain_link.rs` |
| $\phi$ | `field_from_material_bytes` | `src/sss_chain_link.rs` |
| $\psi$ | `write_material_field` | `src/sss_chain_link.rs` |
| $s_i$ | `material_field` | `src/sss_chain_link.rs` |
| $N(\rho)$ | `normalize_sss_chain_root`, `normalize_per_link_bytes` | `src/sss_chain_link.rs` |
| $\alpha$ | `root_anchor_field` | `src/sss_chain_link.rs` |
| Forward step $s_{i-1} \mapsto s_i$ | `sss_chain_epoch_step_forward` | `src/sss_chain_epoch.rs` |
| $T = s_{\mathrm{next}} + s_{\mathrm{cur}}$ | `sss_chain_transition` | `src/sss_chain_epoch.rs` |
| $s_{\mathrm{next}} = T - s_{\mathrm{cur}}$ | `sss_chain_step_forward_from_transition` | `src/sss_chain_epoch.rs` |
| Chain generation | `sss_chain_generate` | `src/sss_chain_link.rs` |
| Full validation game §7 | `sss_chain_validate_full` | `src/sss_chain_link.rs` |
| Lagrange interpolation | `lagrange_interpolate` | `src/sss_chain_lagrange.rs` |
| Shamir share verify | `verify_forward_share`, `verify_backward_share` | `src/sss_chain_otm.rs` |
| Chain length $(L,n)$ or $(L,B)$ | `SssChainConfig`, `SssChainSpan` | `src/sss_chain_link.rs` |
| Thm 6.1 numeric witness | test `backward_transition_underdetermination` | `src/sss_chain_link.rs` |
| Thm 6.3 machine-checked | `transition_invertible` | ITS-self_enclosed_timelock `Stl/TimeLock.lean` |

**Audit procedure:** For each row, expand the Rust definition and check it implements the equation in §2–§7. Any deviation is grounds to **reject** the implementation relative to this specification.

Symbol table (compact): [SSS_CHAIN_FORMULAS.md](SSS_CHAIN_FORMULAS.md). Layer narrative: [SSS_CHAIN_SECURITY_LAYERS.md](SSS_CHAIN_SECURITY_LAYERS.md).
