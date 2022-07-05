use std::cell::UnsafeCell;
use std::thread::spawn;
use std::sync::Arc;

struct ComponentContainer<T> {
    component: UnsafeCell<T>
}

impl<T: Sized> ComponentContainer<T> {
    fn new(component: T) -> Self {
        Self { component: UnsafeCell::new(component) }
    }

    fn to_ref(&self) -> &T {
        unsafe { &*self.component.get() }
    }
    
    #[allow(clippy::mut_from_ref)]
    #[allow(clippy::wrong_self_convention)]
    fn to_mut(&self) -> &mut T {
        unsafe { &mut *self.component.get() }
    }
}

unsafe impl<T: Send> Send for ComponentContainer<T> {}
unsafe impl<T: Send> Sync for ComponentContainer<T> {}

#[derive(Debug)]
struct Foo(u8);

fn main() {
    let container = Arc::new(ComponentContainer::new(Foo(0)));
    let container_clone = Arc::clone(&container);

    let h1 = spawn(move || {
        let c_ref = container.to_ref();
        let mut c_mut = container.to_mut();

        c_mut.0 = 1;
        println!("{c_ref:?}");
    });
    let h2 = spawn(move || {
        let c_ref = container_clone.to_ref();
        let mut c_mut = container_clone.to_mut();

        c_mut.0 = 2;
        println!("{c_ref:?}");
    });
    h1.join().unwrap();
    h2.join().unwrap();
}
