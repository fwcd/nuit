// https://haim.dev/posts/2020-09-10-linking-swift-code-into-rust-app/

use serde::Deserialize;
use std::env;
use std::process::Command;

const MACOS_TARGET_VERSION: &str = "13";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SwiftPaths {
    runtime_library_paths: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SwiftTargetInfo {
    paths: SwiftPaths,
}

fn profile() -> String {
    env::var("PROFILE").unwrap()
}

fn manifest_dir() -> String {
    env::var("CARGO_MANIFEST_DIR").unwrap()
}

fn target_arch() -> String {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    match arch.as_str() {
        "aarch64" => "arm64".to_owned(),
        _ => arch,
    }
}

fn target_os() -> String {
    env::var("CARGO_CFG_TARGET_OS").unwrap()
}

fn find_swift_runtime_libs() {
    let arch = target_arch();
    let target = format!("{}-apple-macosx{}", arch, MACOS_TARGET_VERSION);

    let raw_target_info = Command::new("swift")
        .args(&["-target", &target, "-print-target-info"])
        .output()
        .unwrap()
        .stdout;

    let target_info: SwiftTargetInfo = serde_json::from_slice(&raw_target_info)
        .expect("Could not parse Swift target info");

    for path in target_info.paths.runtime_library_paths {
        println!("cargo:rustc-link-search=native={}", path);
    }
}

fn build_nuit_swiftui_bridge() {
    let profile = profile();

    let build_succeeded = Command::new("swift")
        .args(&["build", "-c", &profile])
        .status()
        .unwrap()
        .success();

    if !build_succeeded {
        panic!("Swift build failed");
    }

    let root = manifest_dir();
    let arch = target_arch();

    println!("cargo:rustc-link-search=native={}/.build/{}-apple-macosx/{}", root, arch, profile);
    println!("cargo:rustc-link-lib=static=nuit-swiftui-bridge");
    println!("cargo:rerun-if-changed={}/Sources/**/*.swift", root);
}

fn main() {
    if target_os() == "macos" {
        find_swift_runtime_libs();
        build_nuit_swiftui_bridge();
    }
}
