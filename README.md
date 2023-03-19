# NUI

A declarative, cross-platform UI library for Rust that uses native controls.

NUI's API takes inspiration from SwiftUI, Xilem, React and a number of other frameworks, while itself using SwiftUI under the hood on macOS.

> Note that NUI currently requires using a nightly Rust toolchain as it uses some unstable compiler features, e.g. [impl Trait in type aliases](https://github.com/rust-lang/rust/issues/63063), [associated type defaults](https://github.com/rust-lang/rust/issues/29661) and the [never type `!`](https://github.com/rust-lang/rust/issues/35121).
