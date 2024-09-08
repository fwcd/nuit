#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{clone, prelude::*, Button, HStack, If, Text, VStack};

#[derive(Bind)]
struct FizzBuzzView {
    count: State<i32>,
}

impl FizzBuzzView {
    fn new() -> Self {
        Self { count: State::new(1) }
    }
}

impl View for FizzBuzzView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let count = self.count.clone();
        let fizz = count.get() % 3 == 0;
        let buzz = count.get() % 5 == 0;
        VStack::from((
            Button::with_text("Increment", clone!(count => move || {
                count.set(count.get() + 1);
            })),
            If::new_or_else(fizz || buzz, || {
                HStack::from((
                    If::new(fizz, || Text::new("Fizz")),
                    If::new(buzz, || Text::new("Buzz")),
                ))
            }, || {
                Text::new(format!("{}", self.count.get()))
            }),
        ))
    }
}

fn main() {
    nuit::run_app(FizzBuzzView::new());
}
