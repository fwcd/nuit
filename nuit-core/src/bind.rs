use crate::Context;

/// Binds a storage to the view's state.
pub trait Bind {
    fn bind(&mut self, _context: &Context) {
        // Bind nothing by default
    }
}

impl Bind for ! {}

impl Bind for () {}

impl<T> Bind for (T,) where T: Bind {}

impl<T, U> Bind for (T, U) where T: Bind, U: Bind {}

impl<T, U, V> Bind for (T, U, V) where T: Bind, U: Bind, V: Bind {}

impl<T, U, V, W> Bind for (T, U, V, W) where T: Bind, U: Bind, V: Bind, W: Bind {}

impl<T, U, V, W, X> Bind for (T, U, V, W, X) where T: Bind, U: Bind, V: Bind, W: Bind, X: Bind {}
