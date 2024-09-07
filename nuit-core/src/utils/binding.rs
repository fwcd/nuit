use std::{rc::Rc, fmt};

/// A pair of a getter and a setter, encapsulating a reference to some value.
#[derive(Clone)]
pub struct Binding<T> {
    get: Rc<dyn Fn() -> T>,
    set: Rc<dyn Fn(T)>,
}

impl<T> Binding<T> where T: 'static {
    pub fn new(get: impl Fn() -> T + 'static, set: impl Fn(T) + 'static) -> Self {
        Self { get: Rc::new(get), set: Rc::new(set) }
    }

    #[must_use]
    pub fn get(&self) -> T {
        (self.get)()
    }

    pub fn set(&self, value: T) {
        (self.set)(value);
    }
}

impl<T> Binding<T> where T: Clone + 'static {
    pub fn constant(value: T) -> Self {
        Self::new(move || value.clone(), |_| {})
    }
}

impl<T> fmt::Debug for Binding<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Binding")
    }
}
