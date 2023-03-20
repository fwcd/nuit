use std::marker::PhantomData;

#[derive(Clone)]
pub struct Binding<T, G, S> {
    get: G,
    set: S,
    phantom: PhantomData<T>,
}

impl<T, G, S> Binding<T, G, S> where G: Fn() -> T + Clone, S: Fn(T) + Clone {
    pub fn new(get: G, set: S) -> Self {
        Self { get, set, phantom: PhantomData }
    }

    pub fn get(&self) -> T {
        (self.get)()
    }

    pub fn set(&self, value: T) {
        (self.set)(value)
    }
}
