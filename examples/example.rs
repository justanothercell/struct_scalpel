use mem_scalpel::Dissect;

#[derive(Dissect)]
struct Foo {
    o: Option<u32>,
    b: bool,
    f: f32,
    x: [u8;4],
    u: u64,
    s: &'static str
}

fn main() {

}