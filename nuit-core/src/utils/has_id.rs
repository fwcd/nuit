use crate::Id;

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
