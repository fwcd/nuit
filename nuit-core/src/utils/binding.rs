use std::{rc::Rc, fmt};

use crate::clone;

use super::Animation;

// TODO: Extract a trait from get/set/set_with and implement it for `State` too. This would let us implement `project` generically.
// TODO: Move this to a top-level `binding` folder/module?
// TODO: Add tests

/// A pair of a getter and a setter, encapsulating a reference to some value.
#[derive(Clone)]
pub struct Binding<T> {
    get: Rc<dyn Fn() -> T>,
    change: Rc<dyn Fn(T, Option<Animation>)>,
}

impl<T> Binding<T> where T: 'static {
    /// Creates a new binding with the given getter and setter.
    pub fn new(
        get: impl Fn() -> T + 'static,
        change: impl Fn(T, Option<Animation>) + 'static
    ) -> Self {
        Self { get: Rc::new(get), change: Rc::new(change) }
    }

    /// Creates a new binding with the given getter and setter. This ignores
    /// any animation.
    pub fn with_get_set(
        get: impl Fn() -> T + 'static,
        set: impl Fn(T) + 'static
    ) -> Self {
        Self { get: Rc::new(get), change: Rc::new(move |v, _| set(v)) }
    }

    /// Fetches the referenced value.
    #[must_use]
    pub fn get(&self) -> T {
        (self.get)()
    }

    /// Sets the referenced value.
    pub fn set(&self, value: T) {
        (self.change)(value, None);
    }

    /// Sets the referenced value with the given animation.
    pub fn set_with(&self, animation: Animation, value: T) {
        (self.change)(value, Some(animation));
    }

    /// Binds to a "sub-value" using the given projection. This is conceptually
    /// analogous to the `map` function on iterators, but slightly less generic
    /// due to the requirement to operate on mutable references.
    pub fn project<U>(&self, projection: impl Fn(&mut T) -> &mut U + Clone + 'static) -> Binding<U> where U: Clone + 'static {
        let get = self.get.clone();
        let change = self.change.clone();
        Binding::new(
            clone!(get, projection => move || projection(&mut get()).clone()),
            move |new, animation| {
                let mut value = get();
                *projection(&mut value) = new;
                change(value, animation);
            }
        )
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
