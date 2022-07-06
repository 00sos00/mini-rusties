use std::cell::UnsafeCell;

#[derive(Debug)]
enum BorrowedAs {
    Ref,
    Mut,
}

trait BorrowState: Sized {
    type Unborrowed;

    const BORROW_STATE: BorrowedAs;
}

impl<T> BorrowState for &T {
    type Unborrowed = T;

    const BORROW_STATE: BorrowedAs = BorrowedAs::Ref;
}

impl<T> BorrowState for &mut T {
    type Unborrowed = T;

    const BORROW_STATE: BorrowedAs = BorrowedAs::Mut;
}

const unsafe fn coerce<T, U>(from: T) -> U {
    use std::mem::ManuallyDrop;

    #[repr(C)]
    union Transmuter<T, U> {
        from: ManuallyDrop<T>,
        to: ManuallyDrop<U>,
    }

    ManuallyDrop::into_inner(
        Transmuter {
            from: ManuallyDrop::new(from),
        }
        .to,
    )
}

struct Holder<T> {
    value: UnsafeCell<T>,
}

impl<T: 'static> Holder<T> {
    fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    fn get<U: BorrowState + 'static>(&self) -> U {
        use std::any::{type_name, TypeId};

        assert!(
            TypeId::of::<U::Unborrowed>() == TypeId::of::<T>(),
            "Expected {} but received {}",
            type_name::<T>(),
            type_name::<U::Unborrowed>()
        );

        match U::BORROW_STATE {
            BorrowedAs::Ref => unsafe { coerce(&*self.value.get()) },
            BorrowedAs::Mut => unsafe { coerce(&mut *self.value.get()) },
        }
    }
}

fn main() {
    let holder = Holder::new(5u8);

    let holder_val1 = holder.get::<&u8>();
    let holder_val2 = holder.get::<&mut u8>();

    println!("{holder_val1} {holder_val2}");

    *holder_val2 += 1;

    println!("{}", holder.get::<&u8>());
}
