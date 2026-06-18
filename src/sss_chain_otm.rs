// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980
//
// This file is part of SSS_CHAIN.
// SSS_CHAIN is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License only.

use crate::sss_chain_field::FieldElement;
use crate::sss_chain_lagrange::lagrange_interpolate;
use crate::sss_chain_poly::Polynomial;
use subtle::{Choice, ConstantTimeEq};

#[inline]
pub fn combine_sss_chains(sss_forward: FieldElement, sss_backward: FieldElement) -> FieldElement {
    sss_forward + sss_backward
}

#[inline]
pub fn derive_forward_secret(
    prev_backward_point: (FieldElement, FieldElement),
    prev_message: FieldElement,
) -> FieldElement {
    prev_backward_point.1 + prev_message
}

pub fn verify_forward_share<const K: usize>(
    poly_forward: &Polynomial<K>,
    message: FieldElement,
    forward_point: (FieldElement, FieldElement),
) -> Choice {
    let x_matches = forward_point.0.ct_eq(&message);
    let expected_y = poly_forward.evaluate(message);
    let y_matches = forward_point.1.ct_eq(&expected_y);
    x_matches & y_matches
}

pub fn verify_backward_share<const K: usize>(
    master_root: FieldElement,
    prev_points: &[(FieldElement, FieldElement)],
    new_point: (FieldElement, FieldElement),
) -> Choice {
    let mut points = [(FieldElement::zero(), FieldElement::zero()); K];
    points[0] = (FieldElement::zero(), master_root);
    for (i, pt) in points.iter_mut().enumerate().take(K).skip(1) {
        let idx = i - 1;
        if idx < prev_points.len() {
            *pt = prev_points[idx];
        }
    }

    let expected_y = lagrange_interpolate(&points, new_point.0);
    new_point.1.ct_eq(&expected_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_and_derive() {
        let fwd = FieldElement::new(12);
        let bwd = FieldElement::new(7);
        assert_eq!(combine_sss_chains(fwd, bwd).value(), 19);
        let prev = (FieldElement::new(1), FieldElement::new(5));
        assert_eq!(derive_forward_secret(prev, FieldElement::new(3)).value(), 8);
    }
}
