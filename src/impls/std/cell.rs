use std::cell::{UnsafeCell, Cell, OnceCell, RefCell};

use crate::{Dissectible, impl_mirror_mock};

#[derive(Dissectible)]
struct MockUnsafeCell<T> {
    value: T,
}
impl_mirror_mock!(with <T>: MockUnsafeCell<T> => UnsafeCell<T>);

#[derive(Dissectible)]
struct MockCell<T> {
    value: UnsafeCell<T>,
}
impl_mirror_mock!(with <T>: MockCell<T> => Cell<T>);

#[derive(Dissectible)]
struct MockRefCell<T> {
    borrow: Cell<isize>,
    borrowed_at: Cell<Option<&'static std::panic::Location<'static>>>,
    value: UnsafeCell<T>,
}
impl_mirror_mock!(with <T>: MockRefCell<T> => RefCell<T>);

#[derive(Dissectible)]
struct MockOnceCell<T> {
    inner: UnsafeCell<Option<T>>,
}
impl_mirror_mock!(with <T>: MockOnceCell<T> => OnceCell<T>);