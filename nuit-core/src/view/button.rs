use crate::{View, Primitive, Bind, Context, Id, Event};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Button<T, F> {
    label: T,
    action: Option<F>,
}

impl<T, F> Button<T, F> {
    pub fn new(label: T, action: F) -> Self {
        Self {
            label,
            action: Some(action),
        }
    }
}

impl<T, F> Bind for Button<T, F> where T: Bind, F: Fn() + 'static {
    fn bind(&mut self, context: &Context) {
        let storage = context.storage();
        if let Some(action) = self.action.take() {
            storage.insert_event_handler(context.id_path().clone(), move |event| {
                if let Event::Click {} = event {
                    action();
                }
            });
        }
    }
}

impl<T, F> View for Button<T, F> where T: View, F: Fn() + 'static {
    fn render(&mut self, context: &Context) -> Id<Primitive> {
        self.bind(context);
        context.identify(Primitive::Button { label: Box::new(self.label.render(&context.child(0))) })
    }
}
