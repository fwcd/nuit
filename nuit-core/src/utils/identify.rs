use crate::Id;

pub trait Identify {
    fn id(&self) -> Id;
}

impl Identify for usize {
    fn id(&self) -> Id {
        Id::index(*self)
    }
}

impl Identify for String {
    fn id(&self) -> Id {
        Id::string(self.clone())
    }
}
