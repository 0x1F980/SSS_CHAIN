// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980
//
// This file is part of SSS_CHAIN.
// SSS_CHAIN is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License only.

use core::fmt;

/// Errors returned by the SSS chain link API.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SssChainError {
    LinkByteLenTooSmall { min: usize, actual: usize },
    LinkCountZero,
    LinkCountTooLarge { max: u32, actual: u32 },
    MemoryNotAligned { total_bytes: usize, link_byte_len: usize },
    DuplicateLinkIndex { index: u32 },
    MissingLinkIndex { index: u32 },
    InvalidLink { index: u32 },
    InvalidLinkLength { expected: usize, actual: usize },
    DepthMismatch { expected: u32, actual: u32 },
}

impl fmt::Display for SssChainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LinkByteLenTooSmall { min, actual } => {
                write!(f, "link_byte_len {actual} < minimum {min}")
            }
            Self::LinkCountZero => write!(f, "link count must be > 0"),
            Self::LinkCountTooLarge { max, actual } => {
                write!(f, "link count {actual} exceeds maximum {max}")
            }
            Self::MemoryNotAligned {
                total_bytes,
                link_byte_len,
            } => write!(
                f,
                "total memory {total_bytes} is not aligned to link_byte_len {link_byte_len}"
            ),
            Self::DuplicateLinkIndex { index } => {
                write!(f, "duplicate link index {index}")
            }
            Self::MissingLinkIndex { index } => write!(f, "missing link index {index}"),
            Self::InvalidLink { index } => write!(f, "invalid link at index {index}"),
            Self::InvalidLinkLength { expected, actual } => write!(
                f,
                "link length {actual} != expected {expected}"
            ),
            Self::DepthMismatch { expected, actual } => write!(
                f,
                "link depth {actual} != expected {expected} from previous link"
            ),
        }
    }
}
