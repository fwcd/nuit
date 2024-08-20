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
impl_tuple_bind!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_tuple_bind!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_tuple_bind!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_tuple_bind!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_tuple_bind!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
impl_tuple_bind!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
impl_tuple_bind!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
impl_tuple_bind!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
impl_tuple_bind!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
impl_tuple_bind!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
impl_tuple_bind!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
impl_tuple_bind!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
impl_tuple_bind!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
impl_tuple_bind!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
