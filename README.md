# Nuit

[![Build](https://github.com/fwcd/nuit/actions/workflows/build.yml/badge.svg)](https://github.com/fwcd/nuit/actions/workflows/build.yml)

A declarative, cross-platform UI library for Rust that uses native controls.

| Crate | Description | Version | Docs |
| - | - | - | - |
| [nuit](./nuit) | Umbrella crate for the library | [![crates.io](https://img.shields.io/crates/v/nuit)](https://crates.io/crates/nuit) | [![docs.rs](https://img.shields.io/docsrs/nuit)](https://docs.rs/nuit) |
| [nuit-bridge-swiftui](./nuit-bridge-swiftui) | SwiftUI bindings (macOS, iOS) | [![crates.io](https://img.shields.io/crates/v/nuit-bridge-swiftui)](https://crates.io/crates/nuit-bridge-swiftui) | [![docs.rs](https://img.shields.io/docsrs/nuit-bridge-swiftui)](https://docs.rs/nuit-bridge-swiftui) |
| [nuit-core](./nuit-core) | Core structures and traits | [![crates.io](https://img.shields.io/crates/v/nuit-core)](https://crates.io/crates/nuit-core) | [![docs.rs](https://img.shields.io/docsrs/nuit-core)](https://docs.rs/nuit-core) |
| [nuit-derive](./nuit-derive) | Derive macros | [![crates.io](https://img.shields.io/crates/v/nuit-derive)](https://crates.io/crates/nuit-derive) | [![docs.rs](https://img.shields.io/docsrs/nuit-derive)](https://docs.rs/nuit-derive) |

Nuit's API takes inspiration from SwiftUI, Xilem, React and a number of other frameworks, while itself using SwiftUI under the hood on macOS.

> [!NOTE]
> Nuit currently requires a nightly Rust toolchain as it uses a number of cutting edge/unstable compiler features, including
>
> - [impl Trait in type aliases](https://github.com/rust-lang/rust/issues/63063)
> - [associated type defaults](https://github.com/rust-lang/rust/issues/29661)
> - [macro metavariable expressions](https://github.com/rust-lang/rust/issues/83527)
> - [`let` chains](https://github.com/rust-lang/rust/issues/53667)
> - [reentrant locks](https://github.com/rust-lang/rust/issues/121440)
>
> With `rustup` this can be configured conveniently on a per-directory basis `rustup override set nightly` or, as in this repository, automatically with a [`rust-toolchain.toml`](rust-toolchain.toml).

## Example

```rust
use nuit::{Text, VStack, View, Bind, Button, State};

#[derive(Bind)]
struct CounterView {
    count: State<i32>,
}

impl CounterView {
    fn new() -> Self {
        Self { count: State::new(0) }
    }
}

impl View for CounterView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let count = self.count.clone();
        VStack::new((
            Text::new(format!("Count: {}", count.get())),
            Button::new(Text::new("Increment"), move || {
                count.set(count.get() + 1);
            })
        ))
    }
}

fn main() {
    nuit::run_app(CounterView::new());
}
```

Running this example, e.g. with `cargo run --example counter`, launches:

<img src="screenshots/counter.png" width="335">

