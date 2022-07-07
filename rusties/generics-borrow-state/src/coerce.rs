pub fn coerce<T, U>(from: T) -> U {
    use std::mem::ManuallyDrop;

    #[repr(C)]
    union Transmuter<T, U> {
        from: ManuallyDrop<T>,
        to: ManuallyDrop<U>,
    }

    unsafe {
        ManuallyDrop::into_inner(
            Transmuter {
                from: ManuallyDrop::new(from),
            }
            .to,
        )
    }
}
