#![allow(dead_code)]

pub mod cell;
pub mod sync;

use dummy::*;

use std::marker::PhantomData;
use std::ptr::NonNull;
use std::rc::Rc;
use std::{ptr::Unique};
use std::alloc::Allocator;

use crate::{impl_mirror_mock, Dissectible};

#[derive(Dissectible)]
struct MockString { 
    vec: Vec<u8>,
}
impl_mirror_mock!(MockString => String);

#[derive(Dissectible)]
enum MockOption<T> { 
    None, 
    Some(T), 
}
impl_mirror_mock!(with <T>: MockOption<T> => Option<T>);

#[derive(Dissectible)]
enum MockResult<T, E> { 
    Ok(T), 
    Err(E), 
}
impl_mirror_mock!(with <T, E>: MockResult<T, E> => Result<T, E>);

#[derive(Dissectible)]
struct MockVec<T, A> where A: Allocator{
    buf: RawVec<T, A>,
    len: usize
}
impl_mirror_mock!(with <T, A>: MockVec<T, A> => Vec<T, A> where A: Allocator);

#[derive(Dissectible)]
struct MockBox<T, A>(Unique<T>, A) where T: ?Sized;
impl_mirror_mock!(with <T, A>: MockBox<T, A> => Box<T, A> where A: Allocator);

#[derive(Dissectible)]
struct MockRc<T> {
    ptr: NonNull<RcBox<T>>,
    phantom: PhantomData<RcBox<T>>,
}
impl_mirror_mock!(with <T>: MockRc<T> => Rc<T>);

pub mod dummy {
    use std::{ptr::Unique, alloc::Allocator, cell::Cell};

    use crate::Dissectible;

    #[derive(Dissectible)]
    /// This is a dummy copy of the internal type RawVec<T, A>, used as a field in Vec<T, A>
    pub struct RawVec<T, A> where A: Allocator {
        ptr: Unique<T>,
        cap: usize,
        alloc: A,
    }

    #[derive(Dissectible)]
    #[repr(C)]
    /// This is a dummy copy of the internal type RcBox<T>, used as a field in Rc<T>
    pub struct RcBox<T> {
        strong: Cell<usize>,
        weak: Cell<usize>,
        value: T,
    }
}