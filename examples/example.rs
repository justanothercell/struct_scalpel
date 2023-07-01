#![feature(allocator_api)]
#![allow(dead_code)]

use std::{alloc::Global, rc::Rc, sync::Arc, fmt::Display};

use struct_scalpel::{Dissectible, print_dissection_info, impls::std::dummy::{RawVec, RcBox}};

#[derive(Dissectible)]
struct Foo<T> where T: Sized {
    o: Option<T>,
    b: bool,
    f: f32,
    x: [u8;4],
    u: u64,
    s: &'static str
}

#[derive(Dissectible)]
enum Bar {
    A,
    B(usize),
    C(bool, i16),
    D {
        x: bool,
        y: u32
    },
    E(u32, u32, i32),
}

#[derive(Dissectible)]
enum Maybe<T> {
    Is(T),
    Isnt
}

#[derive(Dissectible)]
#[repr(C)]
union X {
    a: bool,
    b: f32,
    c: (u16, u8),
    d: u64
}

#[derive(Dissectible)]

struct I(bool);

#[derive(Dissectible)]

struct G<A, B, C>(A, B, C);

fn main() {
    print_dissection_info::<I>();
    print_dissection_info::<G<u32, u16, u128>>();
    print_dissection_info::<()>();
    print_dissection_info::<(Option<u32>,)>();
    print_dissection_info::<(((u32,),),)>();
    print_dissection_info::<(u16, u8)>();
    print_dissection_info::<(u8, u8, u8, u8)>();
    print_dissection_info::<(&str, bool)>();
}