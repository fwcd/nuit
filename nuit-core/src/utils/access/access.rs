use crate::{clone, Animation};

use super::Binding;

/// An trait for types offering read/write access to an underlying value.
pub trait Access {
    /// The type of the underlying value.
    type Value;

    /// Fetches the underlying value.
    fn get(&self) -> Self::Value;

    /// Updates the underlying value with the given animation.
    fn change(&self, value: impl Into<Self::Value>, animation: Option<Animation>);

    /// Sets the referenced value.
    fn set(&self, value: impl Into<Self::Value>) {
        self.change(value, None);
    }

    /// Sets the referenced value with the given animation.
    fn set_with(&self, animation: Animation, value: impl Into<Self::Value>) {
        self.change(value, Some(animation));
    }

    /// Obtains a [`Binding`] to the underlying value.
    fn binding(&self) -> Binding<Self::Value> where Self: Clone + 'static {
        Binding::from_access(self.clone())
    }

    /// Binds to a "sub-value" using the given projection. This is conceptually
    /// analogous to the `map` function on iterators, but slightly less generic
    /// due to the requirement to operate on mutable references.
    fn project<U>(&self, projection: impl Fn(&mut Self::Value) -> &mut U + Clone + 'static) -> Binding<U>
    where
        Self: Clone + 'static,
        U: Clone + 'static
    {
        let binding = self.binding();
        let get = binding.get;
        let change = binding.change;
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
