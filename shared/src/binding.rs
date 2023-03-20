use std::rc::Rc;

#[derive(Clone)]
pub struct Binding<T> {
    get: Rc<dyn Fn() -> T>,
    set: Rc<dyn Fn(T)>,
}

impl<T> Binding<T> {
    pub fn new(get: impl Fn() -> T + 'static, set: impl Fn(T) + 'static) -> Self {
        Self { get: Rc::new(get), set: Rc::new(set) }
    }

    pub fn get(&self) -> T {
        (self.get)()
    }

    pub fn set(&self, value: T) {
        (self.set)(value)
    }
}
