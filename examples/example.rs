#![allow(dead_code)]

use struct_scalpel::{Dissectible, print_dissection_info};

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


fn main() {
    print_dissection_info::<X>();
}