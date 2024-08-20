// https://haim.dev/posts/2020-09-10-linking-swift-code-into-rust-app/

use serde::Deserialize;
use std::{env, str};
use std::process::Command;

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

fn out_dir() -> String {
    env::var("OUT_DIR").unwrap()
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
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    match os.as_str() {
        "macos" => "macosx".to_owned(),
        "ios" => "ios".to_owned(),
        _ => os,
    }
}

fn target_os_version() -> String {
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    match os.as_str() {
        "macos" => "14".to_owned(),
        "ios" => "17.0".to_owned(),
        _ => os,
    }
}

fn target_sdk() -> String {
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    match os.as_str() {
        "macos" => "macosx".to_owned(),
        "ios" => "iphoneos".to_owned(),
        _ => os,
    }
}

fn target_vendor() -> String {
    env::var("CARGO_CFG_TARGET_VENDOR").unwrap()
}

fn target() -> String {
    format!("{}-{}-{}", target_arch(), target_vendor(), target_os())
}

fn target_with_version() -> String {
    format!("{}{}", target(), target_os_version())
}

fn find_swift_runtime_libs() {
    let target_with_version = target_with_version();

    let raw_target_info = Command::new("swift")
        .args(&["-target", &target_with_version, "-print-target-info"])
        .output()
        .unwrap()
        .stdout;

    let target_info: SwiftTargetInfo = serde_json::from_slice(&raw_target_info)
        .expect("Could not parse Swift target info");

    for path in target_info.paths.runtime_library_paths {
        println!("cargo:rustc-link-search=native={}", path);
    }
}

fn build_nuit_bridge_swiftui() {
    let out_dir = out_dir();
    let profile = profile();

    let target = target();

    let sdk = target_sdk();
    let sdk_path = str::from_utf8(
        &Command::new("xcrun")
            .args(&["--sdk", &sdk, "--show-sdk-path"])
            .output()
            .unwrap()
            .stdout
    ).unwrap().trim().to_owned();

    let build_succeeded = Command::new("xcrun")
        .args(&[
            // We need for make sure that we always use the macOS version of the
            // Swift Package Manager, even when cross-compiling for iOS.
            "--sdk", "macosx",
            "swift",
            "build",
            "--sdk", &sdk_path,
            "--triple", &target,
            "--build-path", &out_dir,
            "-c", &profile,
        ])
        .status()
        .unwrap()
        .success();

    if !build_succeeded {
        panic!("Swift build failed");
    }

    let root = manifest_dir();

    println!("cargo:rustc-link-search=native={}/{}/{}", out_dir, target, profile);
    println!("cargo:rustc-link-lib=static=nuit-bridge-swiftui");
    println!("cargo:rerun-if-changed={}/Sources/**/*.swift", root);
}

fn main() {
    if target_vendor() == "apple" {
        find_swift_runtime_libs();
        build_nuit_bridge_swiftui();
    }
}
