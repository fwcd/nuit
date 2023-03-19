/// An empty/uninhabited type. Once https://github.com/rust-lang/rust/issues/35121
/// is merged, we can replace this by !.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Never {}
