use crate::Id;

/// An identifiable value.
pub trait HasId {
    fn id(&self) -> Id;
}

macro_rules! impl_integer_has_id {
    ($($tys:ty),*) => {
        $(impl HasId for $tys {
            fn id(&self) -> Id {
                Id::from(*self)
            }
        })*
    };
}

impl_integer_has_id!(
    u8, u16, u32, u64, usize,
    i8, i16, i32, i64, isize
);

impl HasId for String {
    fn id(&self) -> Id {
        Id::string(self)
    }
}

impl<'a> HasId for &'a str {
    fn id(&self) -> Id {
        Id::string(*self)
    }
}
