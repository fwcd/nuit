use crate::Id;

pub trait Identifiable {
    fn id(&self) -> Id;
}
