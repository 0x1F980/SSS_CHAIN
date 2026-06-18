// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980
//
// This file is part of SSS_CHAIN.
// SSS_CHAIN is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License only.

mod io_util {
    use std::fs;
    use std::io::{self, Read, Write};

    pub fn read_bytes(path: &str) -> io::Result<Vec<u8>> {
        if path == "-" {
            let mut buf = Vec::new();
            io::stdin().read_to_end(&mut buf)?;
            Ok(buf)
        } else {
            fs::read(path)
        }
    }

    pub fn read_text(path: &str) -> io::Result<String> {
        if path == "-" {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            Ok(buf)
        } else {
            fs::read_to_string(path)
        }
    }

    pub fn write_text(path: &str, text: &str) -> io::Result<()> {
        if path == "-" {
            print!("{text}");
            io::stdout().flush()
        } else {
            fs::write(path, text)
        }
    }
}

mod ssc {
    use sss_chain::{SssChainConfig, SssChainLink, SssChainSpan};

    pub const FORMAT_TAG: &str = "# sss-chain v1";

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SscFile {
        pub link_byte_len: usize,
        pub link_count: u32,
        pub links: Vec<SssChainLink>,
    }

    impl SscFile {
        pub fn from_chain(cfg: &SssChainConfig, links: Vec<SssChainLink>) -> Result<Self, String> {
            let link_count = match cfg.span {
                SssChainSpan::LinkCount(n) => n,
                SssChainSpan::TotalMemoryBytes(bytes) => {
                    if bytes % cfg.link_byte_len != 0 {
                        return Err("total bytes not aligned to link_byte_len".into());
                    }
                    (bytes / cfg.link_byte_len) as u32
                }
            };
            if links.len() != link_count as usize {
                return Err(format!(
                    "link count mismatch: expected {link_count}, got {}",
                    links.len()
                ));
            }
            Ok(SscFile {
                link_byte_len: cfg.link_byte_len,
                link_count,
                links,
            })
        }

        pub fn to_config(&self) -> SssChainConfig {
            SssChainConfig {
                link_byte_len: self.link_byte_len,
                span: SssChainSpan::LinkCount(self.link_count),
            }
        }

        pub fn serialize(&self) -> String {
            let mut out = String::new();
            out.push_str(FORMAT_TAG);
            out.push('\n');
            out.push_str(&format!("link_byte_len: {}\n", self.link_byte_len));
            out.push_str(&format!("link_count: {}\n", self.link_count));
            for link in &self.links {
                out.push_str("---\n");
                let idx = u32::from_be_bytes(link[0..4].try_into().unwrap());
                out.push_str(&format!("index: {idx}\n"));
                out.push_str(&format!("hex: {}\n", hex_encode(link)));
            }
            out
        }

        pub fn deserialize(text: &str) -> Result<Self, String> {
            let mut link_byte_len = None;
            let mut link_count = None;
            let mut links = Vec::new();
            let mut current_hex: Option<String> = None;

            for line in text.lines() {
                let line = line.trim();
                if line.is_empty() || line == FORMAT_TAG {
                    continue;
                }
                if line == "---" {
                    if let Some(hex) = current_hex.take() {
                        let link = hex_decode_link(hex.trim(), link_byte_len)?;
                        links.push(link);
                    }
                    continue;
                }
                if let Some(rest) = line.strip_prefix("link_byte_len:") {
                    link_byte_len = Some(parse_usize(rest)?);
                    continue;
                }
                if let Some(rest) = line.strip_prefix("link_count:") {
                    link_count = Some(parse_u32(rest)?);
                    continue;
                }
                if let Some(rest) = line.strip_prefix("hex:") {
                    current_hex = Some(rest.trim().to_string());
                }
            }
            if let Some(hex) = current_hex {
                let link = hex_decode_link(hex.trim(), link_byte_len)?;
                links.push(link);
            }

            let link_byte_len = link_byte_len.ok_or("missing link_byte_len")?;
            let link_count = link_count.ok_or("missing link_count")?;
            if links.len() != link_count as usize {
                return Err(format!(
                    "expected {link_count} links, found {}",
                    links.len()
                ));
            }
            Ok(SscFile {
                link_byte_len,
                link_count,
                links,
            })
        }
    }

    fn parse_usize(s: &str) -> Result<usize, String> {
        s.trim()
            .parse()
            .map_err(|_| format!("invalid number: {s}"))
    }

    fn parse_u32(s: &str) -> Result<u32, String> {
        s.trim()
            .parse()
            .map_err(|_| format!("invalid number: {s}"))
    }

    fn hex_encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    fn hex_decode_link(hex: &str, expected_len: Option<usize>) -> Result<SssChainLink, String> {
        let hex = hex.replace(' ', "");
        if !hex.len().is_multiple_of(2) {
            return Err("hex length must be even".into());
        }
        let mut link = Vec::with_capacity(hex.len() / 2);
        for chunk in hex.as_bytes().chunks(2) {
            let s = std::str::from_utf8(chunk).map_err(|_| "invalid hex utf8")?;
            link.push(u8::from_str_radix(s, 16).map_err(|_| format!("invalid hex: {s}"))?);
        }
        if let Some(len) = expected_len {
            if link.len() != len {
                return Err(format!("link length {} != expected {len}", link.len()));
            }
        }
        Ok(link)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use sss_chain::{sss_chain_generate, SssChainConfig, SssChainSpan};

        #[test]
        fn ssc_roundtrip() {
            let cfg = SssChainConfig::new(16, SssChainSpan::LinkCount(3));
            let root = b"roundtrip-root!!!";
            let chain = sss_chain_generate(root, &cfg).unwrap();
            let file = SscFile::from_chain(&cfg, chain).unwrap();
            let text = file.serialize();
            let parsed = SscFile::deserialize(&text).unwrap();
            assert_eq!(file, parsed);
        }
    }
}

use std::env;
use std::process;

use io_util::{read_bytes, read_text, write_text};
use ssc::SscFile;
use sss_chain::{
    sss_chain_arrange_links, sss_chain_depth_from_previous_link, sss_chain_generate,
    sss_chain_validate_full, SssChainConfig, SssChainSpan,
};

struct Args {
    positional: Vec<String>,
    flags: std::collections::HashMap<String, String>,
    quiet: bool,
}

fn parse_args(raw: &[String]) -> Args {
    let mut flags = std::collections::HashMap::new();
    let mut positional = Vec::new();
    let mut quiet = false;
    let mut i = 1;
    while i < raw.len() {
        let a = &raw[i];
        if a == "--quiet" {
            quiet = true;
            i += 1;
        } else if a.starts_with("--") && i + 1 < raw.len() {
            flags.insert(a.clone(), raw[i + 1].clone());
            i += 2;
        } else {
            positional.push(a.clone());
            i += 1;
        }
    }
    Args {
        positional,
        flags,
        quiet,
    }
}

fn flag(args: &Args, name: &str) -> Option<String> {
    args.flags.get(name).cloned()
}

fn require_flag(args: &Args, name: &str) -> String {
    flag(args, name).unwrap_or_else(|| {
        eprintln!("missing {name}");
        process::exit(2);
    })
}

fn usage() -> ! {
    eprintln!(
        "sss_chain v0.1 — SSS-chaining link CLI (k=2)

USAGE:
  sss_chain generate --root PATH [--link-byte-len N] (--link-count N | --total-bytes N) [--out PATH]
  sss_chain validate --root PATH --in PATH [--quiet]
  sss_chain arrange  --root PATH --in PATH [--out PATH] [--quiet]
  sss_chain depth    --root PATH --in PATH --index N [--quiet]
  sss_chain demo     [--out PATH]
  sss_chain help

PATH may be \"-\" for stdin (read) or stdout (write).
Default --out is \"-\" (stdout). Default --root for generate is \"-\" (stdin).

EXIT CODES:
  0 success / VALID
  1 INVALID / operation failed
  2 usage error

EXAMPLES:
  echo -n \"my-secret\" | sss_chain generate --root - --link-count 10 --out chain.ssc
  sss_chain validate --root secret.txt --in chain.ssc
  cat chain.ssc | sss_chain arrange --root secret.txt --in - --out sorted.ssc"
    );
    process::exit(2);
}

fn config_from_flags(args: &Args) -> SssChainConfig {
    let link_byte_len = flag(args, "--link-byte-len")
        .and_then(|s| s.parse().ok())
        .unwrap_or(16);
    let span = if let Some(n) = flag(args, "--link-count") {
        SssChainSpan::LinkCount(n.parse().unwrap_or_else(|_| {
            eprintln!("invalid --link-count");
            process::exit(2);
        }))
    } else if let Some(n) = flag(args, "--total-bytes") {
        SssChainSpan::TotalMemoryBytes(n.parse().unwrap_or_else(|_| {
            eprintln!("invalid --total-bytes");
            process::exit(2);
        }))
    } else {
        eprintln!("require --link-count or --total-bytes");
        process::exit(2);
    };
    SssChainConfig::new(link_byte_len, span)
}

fn cmd_generate(args: &Args) {
    let root_path = flag(args, "--root").unwrap_or_else(|| "-".to_string());
    let out_path = flag(args, "--out").unwrap_or_else(|| "-".to_string());
    let cfg = config_from_flags(args);
    let root = read_bytes(&root_path).unwrap_or_else(|e| {
        eprintln!("read root: {e}");
        process::exit(1);
    });
    let links = sss_chain_generate(&root, &cfg).unwrap_or_else(|e| {
        eprintln!("generate: {e}");
        process::exit(1);
    });
    let file = SscFile::from_chain(&cfg, links).unwrap_or_else(|e| {
        eprintln!("serialize: {e}");
        process::exit(1);
    });
    write_text(&out_path, &file.serialize()).unwrap_or_else(|e| {
        eprintln!("write: {e}");
        process::exit(1);
    });
    if !args.quiet {
        eprintln!(
            "generated {} links (link_byte_len {}) -> {}",
            file.link_count, file.link_byte_len, out_path
        );
    }
}

fn cmd_validate(args: &Args) {
    let root_path = require_flag(args, "--root");
    let in_path = require_flag(args, "--in");
    let root = read_bytes(&root_path).unwrap_or_else(|e| {
        eprintln!("read root: {e}");
        process::exit(1);
    });
    let text = read_text(&in_path).unwrap_or_else(|e| {
        eprintln!("read chain: {e}");
        process::exit(1);
    });
    let file = SscFile::deserialize(&text).unwrap_or_else(|e| {
        eprintln!("parse: {e}");
        process::exit(1);
    });
    let cfg = file.to_config();
    let ok = sss_chain_validate_full(&root, &file.links, &cfg);
    if ok {
        if !args.quiet {
            println!("VALID links={}", file.link_count);
        }
        process::exit(0);
    }
    eprintln!("INVALID");
    process::exit(1);
}

fn cmd_arrange(args: &Args) {
    let root_path = require_flag(args, "--root");
    let in_path = require_flag(args, "--in");
    let out_path = flag(args, "--out").unwrap_or_else(|| "-".to_string());
    let root = read_bytes(&root_path).unwrap_or_else(|e| {
        eprintln!("read root: {e}");
        process::exit(1);
    });
    let text = read_text(&in_path).unwrap_or_else(|e| {
        eprintln!("read chain: {e}");
        process::exit(1);
    });
    let file = SscFile::deserialize(&text).unwrap_or_else(|e| {
        eprintln!("parse: {e}");
        process::exit(1);
    });
    let cfg = file.to_config();
    let arranged = sss_chain_arrange_links(&root, &file.links, &cfg).unwrap_or_else(|e| {
        eprintln!("arrange: {e}");
        process::exit(1);
    });
    let out_file = SscFile::from_chain(&cfg, arranged).unwrap_or_else(|e| {
        eprintln!("serialize: {e}");
        process::exit(1);
    });
    write_text(&out_path, &out_file.serialize()).unwrap_or_else(|e| {
        eprintln!("write: {e}");
        process::exit(1);
    });
    if !args.quiet {
        eprintln!("arranged {} links -> {}", out_file.link_count, out_path);
    }
}

fn cmd_depth(args: &Args) {
    let root_path = require_flag(args, "--root");
    let in_path = require_flag(args, "--in");
    let index: u32 = require_flag(args, "--index").parse().unwrap_or_else(|_| {
        eprintln!("invalid --index");
        process::exit(2);
    });
    if index == 0 {
        eprintln!("--index must be >= 1");
        process::exit(2);
    }
    let root = read_bytes(&root_path).unwrap_or_else(|e| {
        eprintln!("read root: {e}");
        process::exit(1);
    });
    let text = read_text(&in_path).unwrap_or_else(|e| {
        eprintln!("read chain: {e}");
        process::exit(1);
    });
    let file = SscFile::deserialize(&text).unwrap_or_else(|e| {
        eprintln!("parse: {e}");
        process::exit(1);
    });
    let cfg = file.to_config();
    let i = index as usize;
    if i >= file.links.len() {
        eprintln!("index {index} out of range");
        process::exit(1);
    }
    let depth = sss_chain_depth_from_previous_link(
        &root,
        &file.links[i - 1],
        &file.links[i],
        &cfg,
    )
    .unwrap_or_else(|e| {
        eprintln!("depth: {e}");
        process::exit(1);
    });
    if !args.quiet {
        println!("{depth}");
    }
}

fn cmd_demo(args: &Args) {
    let out_path = flag(args, "--out").unwrap_or_else(|| "-".to_string());
    let cfg = SssChainConfig::new(16, SssChainSpan::LinkCount(4));
    let root = b"sss-chain-demo-root";
    let links = sss_chain_generate(root, &cfg).expect("demo generate");
    let file = SscFile::from_chain(&cfg, links).expect("demo file");
    write_text(&out_path, &file.serialize()).expect("demo write");
    if !args.quiet {
        eprintln!("demo chain written -> {out_path}");
    }
}

fn main() {
    let raw: Vec<String> = env::args().collect();
    if raw.len() < 2 {
        usage();
    }
    let args = parse_args(&raw);
    let cmd = args.positional.first().map(String::as_str).unwrap_or("");
    match cmd {
        "generate" => cmd_generate(&args),
        "validate" => cmd_validate(&args),
        "arrange" => cmd_arrange(&args),
        "depth" => cmd_depth(&args),
        "demo" => cmd_demo(&args),
        "help" | "-h" | "--help" => usage(),
        _ => usage(),
    }
}
