pub trait Identifiable {
    type Id: Copy;

    fn id(&self) -> Self::Id;
}
