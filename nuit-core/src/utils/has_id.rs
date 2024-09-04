use crate::Id;

/// An identifiable value.
pub trait HasId {
    fn id(&self) -> Id;
}

impl HasId for usize {
    fn id(&self) -> Id {
        Id::index(*self)
    }
}

impl HasId for String {
    fn id(&self) -> Id {
        Id::string(self.clone())
    }
}

impl<'a> HasId for &'a str {
    fn id(&self) -> Id {
        Id::string(self.to_string())
    }
}
