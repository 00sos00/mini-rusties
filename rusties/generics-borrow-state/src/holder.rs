use crate::{
    borrow_state::{BorrowState, BorrowedAs},
    coerce::coerce,
};
use std::cell::UnsafeCell;

#[derive(Debug)]
pub enum HolderError<'a> {
    MismatchedTypes { expected: &'a str, found: &'a str },
}

pub struct Holder<T> {
    value: UnsafeCell<T>,
}

impl<T: 'static> Holder<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn get<U: BorrowState<T> + 'static>(&self) -> Result<U, HolderError> {
        use std::any::{type_name, TypeId};

        let same_type = TypeId::of::<U::Unborrowed>() == TypeId::of::<T>();

        same_type
            .then_some(match U::BORROW_STATE {
                BorrowedAs::Ref => coerce(unsafe { &*self.value.get() }),
                BorrowedAs::Mut => coerce(unsafe { &mut *self.value.get() }),
            })
            .ok_or(HolderError::MismatchedTypes {
                expected: type_name::<T>(),
                found: type_name::<U::Unborrowed>(),
            })
    }
}
