// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980
//
// This file is part of SSS_CHAIN.
// SSS_CHAIN is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License only.

#![no_std]

//! Minimal `#![no_std]` crate for SSS-chaining link generation, validation, and arrangement.
//!
//! Used by ITS-OTM, ITS-timelock, and related ITS crates.

extern crate alloc;

pub mod sss_chain_error;
pub mod sss_chain_epoch;
pub mod sss_chain_field;
pub mod sss_chain_lagrange;
pub mod sss_chain_link;
pub mod sss_chain_otm;
pub mod sss_chain_poly;

pub use sss_chain_error::SssChainError;
pub use sss_chain_epoch::{
    sss_chain_epoch_step_forward, sss_chain_step_forward_from_transition, sss_chain_transition,
};
pub use sss_chain_field::{FieldElement, FieldStorage, MODULUS};
pub use sss_chain_lagrange::lagrange_interpolate;
pub use sss_chain_link::{
    normalize_sss_chain_root, sss_chain_arrange_links, sss_chain_depth_from_previous_link,
    sss_chain_generate, sss_chain_root_anchor_material_len, sss_chain_total_bytes,
    sss_chain_validate_full, sss_chain_validate_link, sss_chain_validate_root_link, SssChainConfig,
    SssChainLink, SssChainSpan, SSS_CHAIN_MAX_LINKS, SSS_CHAIN_MIN_LINK_BYTE_LEN,
};
pub use sss_chain_otm::{
    combine_sss_chains, derive_forward_secret, verify_backward_share, verify_forward_share,
};
pub use sss_chain_poly::Polynomial;
