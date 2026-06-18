// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980
//
// This file is part of SSS_CHAIN.
// SSS_CHAIN is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License only.

use crate::sss_chain_field::FieldElement;

/// Computes the transition value `trans = next + current` (timelock / SSS epoch binding).
#[inline]
pub fn sss_chain_transition(next: FieldElement, current: FieldElement) -> FieldElement {
    next + current
}

/// Derives the next share from a transition: `next = trans - current`.
#[inline]
pub fn sss_chain_step_forward_from_transition(
    current: FieldElement,
    transition: FieldElement,
) -> FieldElement {
    transition - current
}

/// One forward step in an SSS chain link sequence (k=2 layout, deterministic entropy).
///
/// `trans = current + current + root_anchor + index + entropy`
/// `next = trans - current`
#[inline]
pub fn sss_chain_epoch_step_forward(
    current: FieldElement,
    root_anchor: FieldElement,
    link_index: u32,
    entropy: FieldElement,
) -> FieldElement {
    let idx = FieldElement::new(link_index);
    let transition = current + current + root_anchor + idx + entropy;
    sss_chain_step_forward_from_transition(current, transition)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transition_roundtrip() {
        let cur = FieldElement::new(7);
        let nxt = FieldElement::new(42);
        let trans = sss_chain_transition(nxt, cur);
        assert_eq!(sss_chain_step_forward_from_transition(cur, trans).value(), nxt.value());
    }

    #[test]
    fn epoch_step_is_deterministic() {
        let cur = FieldElement::new(3);
        let anchor = FieldElement::new(99);
        let a = sss_chain_epoch_step_forward(cur, anchor, 1, FieldElement::zero());
        let b = sss_chain_epoch_step_forward(cur, anchor, 1, FieldElement::zero());
        assert_eq!(a.value(), b.value());
    }
}
