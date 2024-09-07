#![allow(clippy::doc_markdown)]

#[cfg(target_vendor = "apple")]
use nuit_core::CRoot;

#[cfg(target_vendor = "apple")]
extern "C" {
    /// Runs the given app root using SwiftUI.
    #[link_name = "nuit_bridge_swiftui_run_app"]
    pub fn run_app(root: *const CRoot);
}
