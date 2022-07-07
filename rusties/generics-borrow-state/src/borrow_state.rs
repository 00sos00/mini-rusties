#[derive(Debug)]
pub enum BorrowedAs {
    Ref,
    Mut,
}

pub trait BorrowState<U>: Sized {
    type Unborrowed;

    const BORROW_STATE: BorrowedAs;
}

impl<T, U: 'static> BorrowState<U> for &T {
    type Unborrowed = T;

    const BORROW_STATE: BorrowedAs = BorrowedAs::Ref;
}

impl<T, U: 'static> BorrowState<U> for &mut T {
    type Unborrowed = T;

    const BORROW_STATE: BorrowedAs = BorrowedAs::Mut;
}
