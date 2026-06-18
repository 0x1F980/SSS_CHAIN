// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980
//
// This file is part of SSS_CHAIN.
// SSS_CHAIN is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License only.

use alloc::vec;
use alloc::vec::Vec;

use crate::sss_chain_epoch::sss_chain_epoch_step_forward;
use crate::sss_chain_error::SssChainError;
use crate::sss_chain_field::FieldElement;
use subtle::ConstantTimeEq;

/// One link in an SSS chain (`link_byte_len` bytes).
pub type SssChainLink = Vec<u8>;

/// Minimum bytes per link (4-byte index + at least 4 bytes of material).
pub const SSS_CHAIN_MIN_LINK_BYTE_LEN: usize = 8;

/// Maximum supported link count.
pub const SSS_CHAIN_MAX_LINKS: u32 = 1_048_576;

const INDEX_LEN: usize = 4;

/// How the chain length is specified.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SssChainSpan {
    LinkCount(u32),
    TotalMemoryBytes(usize),
}

/// Configuration for SSS chain generation and validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SssChainConfig {
    pub link_byte_len: usize,
    pub span: SssChainSpan,
}

impl SssChainConfig {
    pub fn new(link_byte_len: usize, span: SssChainSpan) -> Self {
        Self {
            link_byte_len,
            span,
        }
    }
}

pub fn sss_chain_total_bytes(cfg: &SssChainConfig) -> Result<usize, SssChainError> {
    let count = resolve_link_count(cfg)?;
    count
        .checked_mul(cfg.link_byte_len as u32)
        .map(|v| v as usize)
        .ok_or(SssChainError::LinkCountTooLarge {
            max: SSS_CHAIN_MAX_LINKS,
            actual: count,
        })
}

/// Bytes of `root_pass` material used for the root anchor after truncate/pad (excludes index slot).
pub fn sss_chain_root_anchor_material_len(cfg: &SssChainConfig) -> Result<usize, SssChainError> {
    validate_cfg(cfg)?;
    Ok(cfg.link_byte_len.saturating_sub(INDEX_LEN))
}

pub fn normalize_sss_chain_root(
    root_pass: &[u8],
    cfg: &SssChainConfig,
) -> Result<SssChainLink, SssChainError> {
    validate_cfg(cfg)?;
    let mut link = normalize_per_link_bytes(root_pass, cfg.link_byte_len);
    write_link_index(&mut link, 0);
    Ok(link)
}

pub fn sss_chain_generate(
    root_pass: &[u8],
    cfg: &SssChainConfig,
) -> Result<Vec<SssChainLink>, SssChainError> {
    validate_cfg(cfg)?;

    let count = resolve_link_count(cfg)?;
    let anchor = root_anchor_field(root_pass, cfg.link_byte_len);

    let mut links = Vec::with_capacity(count as usize);
    links.push(normalize_sss_chain_root(root_pass, cfg)?);

    for i in 1..count {
        let prev = links[(i - 1) as usize].clone();
        links.push(expected_link_from_previous(cfg, &prev, i, anchor)?);
    }

    Ok(links)
}

pub fn sss_chain_validate_root_link(
    root_pass: &[u8],
    link: &SssChainLink,
    cfg: &SssChainConfig,
) -> bool {
    match normalize_sss_chain_root(root_pass, cfg) {
        Ok(expected) => links_ct_eq(&expected, link),
        Err(_) => false,
    }
}

pub fn sss_chain_validate_link(
    root_pass: &[u8],
    previous_link: &SssChainLink,
    link: &SssChainLink,
    cfg: &SssChainConfig,
) -> bool {
    if validate_cfg(cfg).is_err() {
        return false;
    }
    if !link_has_len(link, cfg.link_byte_len) || !link_has_len(previous_link, cfg.link_byte_len) {
        return false;
    }

    let prev_idx = read_link_index(previous_link);
    let idx = read_link_index(link);
    if idx != prev_idx.wrapping_add(1) {
        return false;
    }

    let anchor = root_anchor_field(root_pass, cfg.link_byte_len);

    match expected_link_from_previous(cfg, previous_link, idx, anchor) {
        Ok(expected) => links_ct_eq(&expected, link),
        Err(_) => false,
    }
}

pub fn sss_chain_validate_full(
    root_pass: &[u8],
    links: &[SssChainLink],
    cfg: &SssChainConfig,
) -> bool {
    let count = match resolve_link_count(cfg) {
        Ok(v) => v,
        Err(_) => return false,
    };
    if links.len() != count as usize {
        return false;
    }
    if !sss_chain_validate_root_link(root_pass, &links[0], cfg) {
        return false;
    }
    for i in 1..count as usize {
        if !sss_chain_validate_link(root_pass, &links[i - 1], &links[i], cfg) {
            return false;
        }
    }
    true
}

pub fn sss_chain_arrange_links(
    root_pass: &[u8],
    links: &[SssChainLink],
    cfg: &SssChainConfig,
) -> Result<Vec<SssChainLink>, SssChainError> {
    validate_cfg(cfg)?;
    let count = resolve_link_count(cfg)?;

    let mut filtered: Vec<SssChainLink> = links
        .iter()
        .filter(|l| link_has_len(l, cfg.link_byte_len))
        .cloned()
        .collect();

    if filtered.len() != count as usize {
        return Err(SssChainError::MissingLinkIndex { index: 0 });
    }

    filtered.sort_by_key(|l| read_link_index(l));

    for (expected_idx, link) in filtered.iter().enumerate() {
        let idx = read_link_index(link);
        if idx != expected_idx as u32 {
            if filtered.iter().filter(|l| read_link_index(l) == idx).count() > 1 {
                return Err(SssChainError::DuplicateLinkIndex { index: idx });
            }
            return Err(SssChainError::MissingLinkIndex {
                index: expected_idx as u32,
            });
        }
    }

    if !sss_chain_validate_root_link(root_pass, &filtered[0], cfg) {
        return Err(SssChainError::InvalidLink { index: 0 });
    }

    for i in 1..count as usize {
        if !sss_chain_validate_link(root_pass, &filtered[i - 1], &filtered[i], cfg) {
            return Err(SssChainError::InvalidLink { index: i as u32 });
        }
    }

    Ok(filtered)
}

pub fn sss_chain_depth_from_previous_link(
    root_pass: &[u8],
    previous_link: &SssChainLink,
    link: &SssChainLink,
    cfg: &SssChainConfig,
) -> Result<u32, SssChainError> {
    validate_cfg(cfg)?;

    let prev_idx = read_link_index(previous_link);
    let idx = read_link_index(link);
    let expected = prev_idx.wrapping_add(1);
    if idx != expected {
        return Err(SssChainError::DepthMismatch {
            expected,
            actual: idx,
        });
    }
    if !sss_chain_validate_link(root_pass, previous_link, link, cfg) {
        return Err(SssChainError::InvalidLink { index: idx });
    }
    Ok(idx)
}

fn validate_cfg(cfg: &SssChainConfig) -> Result<(), SssChainError> {
    if cfg.link_byte_len < SSS_CHAIN_MIN_LINK_BYTE_LEN {
        return Err(SssChainError::LinkByteLenTooSmall {
            min: SSS_CHAIN_MIN_LINK_BYTE_LEN,
            actual: cfg.link_byte_len,
        });
    }
    resolve_link_count(cfg)?;
    Ok(())
}

fn resolve_link_count(cfg: &SssChainConfig) -> Result<u32, SssChainError> {
    let count = match cfg.span {
        SssChainSpan::LinkCount(n) => n,
        SssChainSpan::TotalMemoryBytes(bytes) => {
            if bytes % cfg.link_byte_len != 0 {
                return Err(SssChainError::MemoryNotAligned {
                    total_bytes: bytes,
                    link_byte_len: cfg.link_byte_len,
                });
            }
            (bytes / cfg.link_byte_len) as u32
        }
    };
    if count == 0 {
        return Err(SssChainError::LinkCountZero);
    }
    if count > SSS_CHAIN_MAX_LINKS {
        return Err(SssChainError::LinkCountTooLarge {
            max: SSS_CHAIN_MAX_LINKS,
            actual: count,
        });
    }
    Ok(count)
}

fn normalize_per_link_bytes(root_pass: &[u8], link_byte_len: usize) -> SssChainLink {
    let mut link = vec![0u8; link_byte_len];
    let material_len = link_byte_len.saturating_sub(INDEX_LEN);
    let copy_len = root_pass.len().min(material_len);
    link[INDEX_LEN..INDEX_LEN + copy_len].copy_from_slice(&root_pass[..copy_len]);
    if root_pass.len() > material_len {
        link[INDEX_LEN..].copy_from_slice(&root_pass[..material_len]);
    }
    link
}

fn write_link_index(link: &mut [u8], index: u32) {
    link[..INDEX_LEN].copy_from_slice(&index.to_be_bytes());
}

fn read_link_index(link: &[u8]) -> u32 {
    let mut buf = [0u8; INDEX_LEN];
    buf.copy_from_slice(&link[..INDEX_LEN]);
    u32::from_be_bytes(buf)
}

fn link_has_len(link: &SssChainLink, expected: usize) -> bool {
    link.len() == expected
}

fn links_ct_eq(a: &SssChainLink, b: &SssChainLink) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut eq = 1u8;
    for (x, y) in a.iter().zip(b.iter()) {
        eq &= x.ct_eq(y).unwrap_u8();
    }
    eq == 1
}

fn field_from_material_bytes(bytes: &[u8]) -> FieldElement {
    let mut acc = FieldElement::zero();
    let mut weight = FieldElement::one();
    for chunk in bytes.chunks(4) {
        let mut word = [0u8; 4];
        word[..chunk.len()].copy_from_slice(chunk);
        acc = acc + FieldElement::new(u32::from_be_bytes(word)) * weight;
        weight = weight * FieldElement::new(257);
    }
    acc
}

fn write_material_field(out: &mut [u8], fe: FieldElement) {
    let value = fe.value();
    let bytes = value.to_be_bytes();
    let copy_len = out.len().min(bytes.len());
    out[..copy_len].copy_from_slice(&bytes[bytes.len() - copy_len..]);
}

fn material_field(link: &SssChainLink) -> FieldElement {
    field_from_material_bytes(&link[INDEX_LEN..])
}

fn root_anchor_field(root_pass: &[u8], link_byte_len: usize) -> FieldElement {
    let norm = normalize_per_link_bytes(root_pass, link_byte_len);
    field_from_material_bytes(&norm[INDEX_LEN..])
}

fn expected_link_from_previous(
    cfg: &SssChainConfig,
    previous_link: &SssChainLink,
    link_index: u32,
    root_anchor: FieldElement,
) -> Result<SssChainLink, SssChainError> {
    if !link_has_len(previous_link, cfg.link_byte_len) {
        return Err(SssChainError::InvalidLinkLength {
            expected: cfg.link_byte_len,
            actual: previous_link.len(),
        });
    }

    let s_cur = material_field(previous_link);
    let s_next = sss_chain_epoch_step_forward(
        s_cur,
        root_anchor,
        link_index,
        FieldElement::zero(),
    );

    let mut link = vec![0u8; cfg.link_byte_len];
    write_link_index(&mut link, link_index);
    write_material_field(&mut link[INDEX_LEN..], s_next);
    Ok(link)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(count: u32, link_byte_len: usize) -> SssChainConfig {
        SssChainConfig {
            link_byte_len,
            span: SssChainSpan::LinkCount(count),
        }
    }

    #[test]
    fn root_truncate_and_pad() {
        let cfg = cfg(1, 16);
        let long_root = [0xABu8; 32];
        let link = normalize_sss_chain_root(&long_root, &cfg).unwrap();
        assert_eq!(link.len(), 16);
        assert_eq!(read_link_index(&link), 0);
        assert_eq!(&link[4..], &[0xAB; 12]);

        let short_root = [0xCDu8; 4];
        let link = normalize_sss_chain_root(&short_root, &cfg).unwrap();
        assert_eq!(&link[4..8], &[0xCD; 4]);
        assert_eq!(&link[8..], &[0; 8]);
    }

    #[test]
    fn generate_and_validate() {
        let cfg = cfg(5, 16);
        let root = b"test-root-pass-1234";
        let chain = sss_chain_generate(root, &cfg).unwrap();
        assert_eq!(chain.len(), 5);
        assert!(sss_chain_validate_full(root, &chain, &cfg));
    }

    #[test]
    fn long_root_accepted_without_length_requirement() {
        let cfg = cfg(10, 16);
        let root = [0x42u8; 160];
        let chain = sss_chain_generate(&root, &cfg).unwrap();
        assert_eq!(chain.len(), 10);
        assert!(sss_chain_validate_full(&root, &chain, &cfg));
    }

    #[test]
    fn arrange_shuffled_links() {
        let cfg = cfg(4, 16);
        let root = b"arrange-me-please!!";
        let mut chain = sss_chain_generate(root, &cfg).unwrap();
        chain.reverse();
        let arranged = sss_chain_arrange_links(root, &chain, &cfg).unwrap();
        assert!(sss_chain_validate_full(root, &arranged, &cfg));
        for i in 0..4 {
            assert_eq!(read_link_index(&arranged[i as usize]), i);
        }
    }

    #[test]
    fn single_byte_flip_fails_validate() {
        let cfg = cfg(3, 16);
        let root = b"flip-test-root!!!!!";
        let mut chain = sss_chain_generate(root, &cfg).unwrap();
        chain[1][10] ^= 0x01;
        assert!(!sss_chain_validate_full(root, &chain, &cfg));
    }

    #[test]
    fn depth_from_previous_link() {
        let cfg = cfg(3, 16);
        let root = b"depth-test-root!!!!";
        let chain = sss_chain_generate(root, &cfg).unwrap();
        let depth = sss_chain_depth_from_previous_link(root, &chain[0], &chain[1], &cfg).unwrap();
        assert_eq!(depth, 1);
    }

    #[test]
    fn memory_span_resolves_link_count() {
        let cfg = SssChainConfig {
            link_byte_len: 16,
            span: SssChainSpan::TotalMemoryBytes(160),
        };
        assert_eq!(sss_chain_total_bytes(&cfg).unwrap(), 160);
        let root = b"memory-span-root!!";
        let chain = sss_chain_generate(root, &cfg).unwrap();
        assert_eq!(chain.len(), 10);
    }

    #[test]
    fn validate_fails_without_root() {
        let cfg = cfg(3, 16);
        let root = b"correct-root-pass!!";
        let chain = sss_chain_generate(root, &cfg).unwrap();
        assert!(!sss_chain_validate_full(b"", &chain, &cfg));
    }

    #[test]
    fn backward_transition_underdetermination() {
        use crate::sss_chain_epoch::sss_chain_transition;

        let next_a = FieldElement::new(60);
        let cur_a = FieldElement::new(40);
        let trans = sss_chain_transition(next_a, cur_a);

        let next_b = FieldElement::new(55);
        let cur_b = trans - next_b;
        assert_eq!(cur_b.value(), 45);
        assert_eq!(sss_chain_transition(next_b, cur_b).value(), trans.value());
        assert_ne!(next_a.value(), next_b.value());
        assert_ne!(cur_a.value(), cur_b.value());
    }

    #[test]
    fn wrong_root_fails_validate() {
        let cfg = cfg(3, 16);
        let root = b"correct-root-pass!!";
        let chain = sss_chain_generate(root, &cfg).unwrap();
        let wrong = b"wrong-root-pass!!!!";
        assert!(!sss_chain_validate_full(wrong, &chain, &cfg));
    }
}
