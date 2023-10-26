use crate::Id;

pub trait Identifiable {
    fn id(&self) -> Id;
}

impl Identifiable for usize {
    fn id(&self) -> Id {
        Id::index(*self)
    }
}

impl Identifiable for String {
    fn id(&self) -> Id {
        Id::string(self.clone())
    }
}
