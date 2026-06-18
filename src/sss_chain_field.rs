// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980
//
// This file is part of SSS_CHAIN.
// SSS_CHAIN is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License only.

use core::ops::{Add, Mul, Neg, Sub};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};
use zeroize::Zeroize;

#[cfg(not(feature = "m61"))]
pub type FieldStorage = u32;

#[cfg(feature = "m61")]
pub type FieldStorage = u64;

/// The prime modulus for our finite field Z_p (p = 2^31 - 1 or 2^61 - 1).
#[cfg(not(feature = "m61"))]
pub const MODULUS: FieldStorage = 2147483647;

#[cfg(feature = "m61")]
pub const MODULUS: FieldStorage = 2305843009213693951;

/// A transparent wrapper representing an element of the finite field Z_p.
#[derive(Clone, Copy, Debug, Default, Zeroize)]
pub struct FieldElement(pub FieldStorage);

impl FieldElement {
    #[inline]
    pub fn new(val: u32) -> Self {
        #[cfg(not(feature = "m61"))]
        {
            let sum = (val & 0x7FFFFFFF) + (val >> 31);
            let sub = sum.wrapping_sub(MODULUS);
            let is_negative = (sub >> 31) & 1;
            let mask = 0u32.wrapping_sub(is_negative);
            let r = (sub & !mask) | (sum & mask);
            FieldElement(r)
        }
        #[cfg(feature = "m61")]
        {
            FieldElement(val as u64)
        }
    }

    #[inline]
    pub fn from_u32(val: u32) -> Self {
        Self::new(val)
    }

    #[inline]
    pub fn from_u64(val: u64) -> Self {
        #[cfg(not(feature = "m61"))]
        {
            let mut sum = (val & 0x7FFFFFFF) + (val >> 31);
            sum = (sum & 0x7FFFFFFF) + (sum >> 31);
            let mut r = sum as u32;
            for _ in 0..2 {
                let sub = r.wrapping_sub(MODULUS);
                let is_negative = (sub >> 31) & 1;
                let mask = 0u32.wrapping_sub(is_negative);
                r = (sub & !mask) | (r & mask);
            }
            FieldElement(r)
        }
        #[cfg(feature = "m61")]
        {
            let sum = (val & 0x1FFFFFFFFFFFFFFF) + (val >> 61);
            let sub = sum.wrapping_sub(MODULUS);
            let is_negative = (sub >> 63) & 1;
            let mask = 0u64.wrapping_sub(is_negative);
            let r = (sub & !mask) | (sum & mask);
            FieldElement(r)
        }
    }

    #[inline]
    pub fn from_u128(val: u128) -> Self {
        #[cfg(not(feature = "m61"))]
        {
            Self::from_u64(val as u64)
        }
        #[cfg(feature = "m61")]
        {
            let mut sum = (val & 0x1FFFFFFFFFFFFFFF) + (val >> 61);
            sum = (sum & 0x1FFFFFFFFFFFFFFF) + (sum >> 61);
            let mut r = sum as u64;
            for _ in 0..2 {
                let sub = r.wrapping_sub(MODULUS);
                let is_negative = (sub >> 63) & 1;
                let mask = 0u64.wrapping_sub(is_negative);
                r = (sub & !mask) | (r & mask);
            }
            FieldElement(r)
        }
    }

    #[inline]
    pub fn zero() -> Self {
        FieldElement(0)
    }

    #[inline]
    pub fn one() -> Self {
        FieldElement(1)
    }

    #[inline]
    pub fn invert(&self) -> Self {
        let x = *self;
        let mut res = FieldElement::one();
        let mut base = x;

        #[cfg(not(feature = "m61"))]
        let mut exp = 2147483645u32;
        #[cfg(not(feature = "m61"))]
        let bits = 31;

        #[cfg(feature = "m61")]
        let mut exp = 2305843009213693949u64;
        #[cfg(feature = "m61")]
        let bits = 61;

        for _ in 0..bits {
            let bit = Choice::from((exp & 1) as u8);
            let multiplied = res * base;
            res = FieldElement::conditional_select(&res, &multiplied, bit);
            base = base * base;
            exp >>= 1;
        }

        let is_zero = x.0.ct_eq(&0);
        FieldElement::conditional_select(&res, &FieldElement::zero(), is_zero)
    }

    #[inline]
    pub fn value(&self) -> FieldStorage {
        self.0
    }
}

impl ConstantTimeEq for FieldElement {
    #[inline]
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl ConditionallySelectable for FieldElement {
    #[inline]
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        let val = FieldStorage::conditional_select(&a.0, &b.0, choice);
        FieldElement(val)
    }
}

impl Add for FieldElement {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        #[cfg(not(feature = "m61"))]
        {
            let sum = (self.0 as u64) + (other.0 as u64);
            let sub = sum.wrapping_sub(MODULUS as u64);
            let is_negative = (sub >> 63) & 1;
            let mask = 0u64.wrapping_sub(is_negative);
            let r = (sub & !mask) | (sum & mask);
            FieldElement(r as u32)
        }
        #[cfg(feature = "m61")]
        {
            let sum = (self.0 as u128) + (other.0 as u128);
            let sub = sum.wrapping_sub(MODULUS as u128);
            let is_negative = (sub >> 127) & 1;
            let mask = 0u128.wrapping_sub(is_negative);
            let r = (sub & !mask) | (sum & mask);
            FieldElement(r as u64)
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        #[cfg(not(feature = "m61"))]
        {
            let diff = (self.0 as i64) - (other.0 as i64);
            let is_negative = ((diff >> 63) & 1) as u64;
            let mask = 0u64.wrapping_sub(is_negative);
            let r = (diff as u64).wrapping_add((MODULUS as u64) & mask);
            FieldElement(r as u32)
        }
        #[cfg(feature = "m61")]
        {
            let diff = (self.0 as i128) - (other.0 as i128);
            let is_negative = ((diff >> 127) & 1) as u128;
            let mask = 0u128.wrapping_sub(is_negative);
            let r = (diff as u128).wrapping_add((MODULUS as u128) & mask);
            FieldElement(r as u64)
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        #[cfg(not(feature = "m61"))]
        {
            let prod = (self.0 as u64) * (other.0 as u64);
            FieldElement::from_u64(prod)
        }
        #[cfg(feature = "m61")]
        {
            let prod = (self.0 as u128) * (other.0 as u128);
            FieldElement::from_u128(prod)
        }
    }
}

impl Neg for FieldElement {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        let neg_val = MODULUS - self.0;
        let is_zero = self.0.ct_eq(&0);
        FieldElement::conditional_select(&FieldElement(neg_val), &FieldElement(0), is_zero)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_reduction() {
        assert_eq!(FieldElement::new(0).value(), 0);
        #[cfg(not(feature = "m61"))]
        {
            assert_eq!(FieldElement::new(2147483646).value(), 2147483646);
            assert_eq!(FieldElement::new(2147483647).value(), 0);
            assert_eq!(FieldElement::new(2147483648).value(), 1);
        }
    }

    #[test]
    fn test_inversion() {
        assert_eq!(FieldElement::zero().invert().value(), 0);
        let a = FieldElement::new(42);
        assert_eq!((a * a.invert()).value(), 1);
    }
}
