use libbpf_cargo::SkeletonBuilder;
use std::{env, fs::File, path::PathBuf, process::Command};

const SRC_TC: &str = "src/bpf/tc.bpf.c";

fn main() {
    println!("cargo:rerun-if-changed={}", SRC_TC);

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let vmlinux_path = PathBuf::from_iter([&manifest_dir, "src/bpf/vmlinux.h"]);
    let vmlinux = File::create(vmlinux_path).unwrap();

    Command::new("bpftool")
        .args([
            "btf",
            "dump",
            "file",
            "/sys/kernel/btf/vmlinux",
            "format",
            "c",
        ])
        .stdout(vmlinux)
        .status()
        .unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let tc_skel = PathBuf::from_iter([&out_dir, "tc.skel.rs"]);

    SkeletonBuilder::new()
        .source(SRC_TC)
        .build_and_generate(tc_skel)
        .unwrap();
}
