use crate::Storage;

/// Binds a storage to the view's state.
pub trait Bind {
    fn bind(&self, storage: &Storage) {
        // Bind nothing by default
    }
}

impl Bind for ! {}

impl Bind for () {}

impl<T> Bind for (T,) where T: Bind {
    fn bind(&self, storage: &Storage) {
        self.0.bind(storage);
    }
}

impl<T, U> Bind for (T, U) where T: Bind, U: Bind {
    fn bind(&self, storage: &Storage) {
        self.0.bind(storage);
        self.1.bind(storage);
    }
}

impl<T, U, V> Bind for (T, U, V) where T: Bind, U: Bind, V: Bind {
    fn bind(&self, storage: &Storage) {
        self.0.bind(storage);
        self.1.bind(storage);
        self.2.bind(storage);
    }
}

impl<T, U, V, W> Bind for (T, U, V, W) where T: Bind, U: Bind, V: Bind, W: Bind {
    fn bind(&self, storage: &Storage) {
        self.0.bind(storage);
        self.1.bind(storage);
        self.2.bind(storage);
        self.3.bind(storage);
    }
}

impl<T, U, V, W, X> Bind for (T, U, V, W, X) where T: Bind, U: Bind, V: Bind, W: Bind, X: Bind {
    fn bind(&self, storage: &Storage) {
        self.0.bind(storage);
        self.1.bind(storage);
        self.2.bind(storage);
        self.3.bind(storage);
        self.4.bind(storage);
    }
}
