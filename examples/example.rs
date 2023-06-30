use mem_scalpel::{Dissectible, print_info};

#[derive(Dissectible)]
#[repr(C)]
struct Foo<T> where T: Sized {
    o: Option<T>,
    b: bool,
    f: f32,
    x: [u8;4],
    u: u64,
    s: &'static str
}

#[derive(Dissectible)]
struct Bar(bool, bool, bool, i32);

#[derive(Dissectible)]
struct Biz(());

#[derive(Dissectible)]
struct Baz();

#[derive(Dissectible)]
struct Buz;

fn main() {
    print_info::<Foo<&'static str>>();
    println!("\n");
    print_info::<Bar>();
    println!("\n");
    print_info::<Biz>();
    println!("\n");
    print_info::<Baz>();
    println!("\n");
    print_info::<Buz>();
}