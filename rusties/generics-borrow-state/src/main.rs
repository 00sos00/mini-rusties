use std::any::TypeId;

#[derive(Debug)]
enum BorrowedAs {
    Ref,
    Mut,
}

trait BorrowState {
    type OriginalT;

    const BORROW_STATE: BorrowedAs;
}

impl<T: Sized + 'static> BorrowState for &T {
    type OriginalT = T;

    const BORROW_STATE: BorrowedAs = BorrowedAs::Ref;
}

impl<T: Sized + 'static> BorrowState for &mut T {
    type OriginalT = T;

    const BORROW_STATE: BorrowedAs = BorrowedAs::Mut;
}

fn state<BorrowedT: BorrowState + 'static>() -> (BorrowedAs, TypeId) {
    (BorrowedT::BORROW_STATE, TypeId::of::<BorrowedT::OriginalT>())
}

struct SomeRandomStruct;

fn main() {
    println!("{:?}", state::<&u8>());
    println!("{:?}", state::<&mut u8>());
    println!("{:?}", state::<&SomeRandomStruct>());
    println!("{:?}", state::<&mut SomeRandomStruct>());
}
