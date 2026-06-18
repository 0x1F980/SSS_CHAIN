// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980
//
// This file is part of SSS_CHAIN.
// SSS_CHAIN is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License only.

//! CLI integration: generate/validate/arrange with stdin/stdout.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_sss_chain"))
}

struct TempDir(PathBuf);

impl TempDir {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "sss_chain_cli_{name}_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&path).expect("temp dir");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn cli_generate_validate_file() {
    let dir = TempDir::new("gen");
    let root = dir.path().join("root.txt");
    let chain = dir.path().join("chain.ssc");
    fs::write(&root, b"integration-test-root").unwrap();

    assert!(cli()
        .args([
            "generate",
            "--root",
            root.to_str().unwrap(),
            "--link-count",
            "5",
            "--link-byte-len",
            "16",
            "--out",
            chain.to_str().unwrap(),
        ])
        .status()
        .unwrap()
        .success());

    let verify = cli()
        .args([
            "validate",
            "--root",
            root.to_str().unwrap(),
            "--in",
            chain.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(verify.status.success(), "{:?}", verify.stderr);
    assert!(String::from_utf8_lossy(&verify.stdout).contains("VALID"));
}

#[test]
fn cli_generate_stdin_stdout() {
    let dir = TempDir::new("pipe");
    let root = dir.path().join("root.txt");
    fs::write(&root, b"pipe-root-secret!!").unwrap();

    let mut child = cli()
        .args([
            "generate",
            "--root",
            root.to_str().unwrap(),
            "--link-count",
            "3",
            "--out",
            "-",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let ssc = std::io::read_to_string(child.stdout.take().unwrap()).unwrap();
    assert!(child.wait().unwrap().success());
    assert!(ssc.contains("# sss-chain v1"));

    let mut validate = cli()
        .args(["validate", "--root", root.to_str().unwrap(), "--in", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    validate.stdin.as_mut().unwrap().write_all(ssc.as_bytes()).unwrap();
    let out = validate.wait_with_output().unwrap();
    assert!(out.status.success());
}

#[test]
fn cli_invalid_chain_exits_one() {
    let dir = TempDir::new("bad");
    let root = dir.path().join("root.txt");
    let chain = dir.path().join("chain.ssc");
    fs::write(&root, b"bad-root-test!!!!!!").unwrap();
    assert!(cli()
        .args([
            "generate",
            "--root",
            root.to_str().unwrap(),
            "--link-count",
            "2",
            "--out",
            chain.to_str().unwrap(),
        ])
        .status()
        .unwrap()
        .success());

    let mut content = fs::read_to_string(&chain).unwrap();
    // Flip one hex nibble in the first link payload
    if let Some(idx) = content.find("hex: ") {
        let line_start = idx + 5;
        if content.as_bytes().get(line_start) == Some(&b'a') {
            content.replace_range(line_start..line_start + 1, "b");
        } else {
            content.replace_range(line_start..line_start + 1, "a");
        }
    }
    fs::write(&chain, content).unwrap();

    let out = cli()
        .args([
            "validate",
            "--root",
            root.to_str().unwrap(),
            "--in",
            chain.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!out.status.success());
}

#[test]
fn cli_demo_exits_zero() {
    assert!(cli()
        .args(["demo", "--quiet", "--out", "-"])
        .status()
        .unwrap()
        .success());
}
