#[derive(Debug)]
enum BorrowedAs {
    Ref,
    Mut,
}

trait BorrowState {
    const BORROW_STATE: BorrowedAs;
}

impl<T: ?Sized> BorrowState for &T {
    const BORROW_STATE: BorrowedAs = BorrowedAs::Ref;
}

impl<T: ?Sized> BorrowState for &mut T {
    const BORROW_STATE: BorrowedAs = BorrowedAs::Mut;
}

fn state<T: BorrowState>() -> BorrowedAs {
    T::BORROW_STATE
}

struct SomeRandomStruct;

fn main() {
    println!("{:?}", state::<&u8>());
    println!("{:?}", state::<&mut u8>());
    println!("{:?}", state::<&SomeRandomStruct>());
    println!("{:?}", state::<&mut SomeRandomStruct>());
}
