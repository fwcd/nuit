use std::{rc::Rc, fmt};

use crate::{clone, Animation};

use super::Access;

/// A pair of a getter and a setter, encapsulating a reference to some value.
#[derive(Clone)]
pub struct Binding<T> {
    pub(crate) get: Rc<dyn Fn() -> T>,
    pub(crate) change: Rc<dyn Fn(T, Option<Animation>)>,
}

impl<T> Binding<T> where T: 'static {
    /// Creates a new binding with the given getter and setter.
    pub fn new(
        get: impl Fn() -> T + 'static,
        change: impl Fn(T, Option<Animation>) + 'static
    ) -> Self {
        Self { get: Rc::new(get), change: Rc::new(change) }
    }

    /// Creates a new value from the given access.
    pub fn from_access(access: impl Access<Value = T> + Clone + 'static) -> Self {
        Self::new(
            clone!(access => move || access.get()),
            move |value, animation| access.change(value, animation),
        )
    }

    /// Creates a new binding with the given getter and setter. This ignores
    /// any animation.
    pub fn with_get_set(
        get: impl Fn() -> T + 'static,
        set: impl Fn(T) + 'static
    ) -> Self {
        Self { get: Rc::new(get), change: Rc::new(move |v, _| set(v)) }
    }
}

impl<T> Access for Binding<T> where T: 'static {
    type Value = T;

    /// Fetches the referenced value.
    #[must_use]
    fn get(&self) -> T {
        (self.get)()
    }

    fn change(&self, value: impl Into<T>, animation: Option<Animation>) {
        (self.change)(value.into(), animation);
    }

    fn binding(&self) -> Binding<Self::Value> where Self: Clone + 'static {
        self.clone()
    }
}

impl<T> Binding<T> where T: Clone + 'static {
    /// Creates a binding to the given value with an empty setter.
    pub fn constant(value: T) -> Self {
        Self::new(
            move || value.clone(),
            |_, _| {},
        )
    }
}

impl<T> fmt::Debug for Binding<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Binding")
    }
}
