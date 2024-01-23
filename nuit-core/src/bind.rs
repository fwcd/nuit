use crate::Context;

/// Binds a storage to the view's state.
pub trait Bind {
    fn bind(&mut self, _context: &Context) {
        // Bind nothing by default
    }
}

macro_rules! impl_tuple_bind {
    ($($tvs:ident),*) => {
        impl<$($tvs),*> Bind for ($($tvs,)*) where $($tvs: Bind),* {}
    };
}

impl Bind for ! {}

impl_tuple_bind!();
impl_tuple_bind!(T1);
impl_tuple_bind!(T1, T2);
impl_tuple_bind!(T1, T2, T3);
impl_tuple_bind!(T1, T2, T3, T4);
impl_tuple_bind!(T1, T2, T3, T4, T5);
impl_tuple_bind!(T1, T2, T3, T4, T5, T6);
impl_tuple_bind!(T1, T2, T3, T4, T5, T6, T7);
impl_tuple_bind!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_tuple_bind!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_tuple_bind!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
