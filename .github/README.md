# struct_scalpel
[![Github](https://img.shields.io/badge/github-DragonFighter603/struct_scalpel?style=flat-square&labelColor=555555&logo=github)](https://github.com/DragonFighter603/struct_scalpel)
[![crates.io](https://img.shields.io/crates/v/struct_scalpel?style=flat-square&color=fc8d62&logo=rust)](https://crates.io/crates/struct_scalpel)
[![License](https://img.shields.io/github/license/DragonFIghter603/struct_scalpel?style=flat-square)](https://github.com/DragonFighter603/struct_scalpel/blob/main/LICENSE)
[![Stars](https://img.shields.io/github/stars/DragonFighter603/struct_scalpel?style=flat-square)](https://github.com/DragonFighter603/struct_scalpel/stargazers)
![Lines of code](https://raster.shields.io/tokei/lines/github/DragonFighter603/struct_scalpel?style=flat-square)
<!-- 
[![docs.rs](https://img.shields.io/badge/docs.rs-struct_scalpel-66c2a5?style=flat-square&labelColor=555555&logo=docs.rs)](https://docs.rs/struct_scalpel)
-->
---
Memory layout analysis tool for rust structs, enums, unions, tuples, and references, prints to console using ansi escape codes.

Simply derive `Dissectible` and call `print_dissection_info` to get a well formatted overview.

```rs
use struct_scalpel::{Dissectible, print_dissection_info};

#[derive(Dissectible)]
struct Person {
    name: String,
    age: u32,
    is_student: bool,
    height: f64,
    favorite_numbers: Vec<u8>,
}

fn main() {
    print_dissection_info::<Person>();
}
```

![](img/person.png)

With `dissect_std` you can even disect some std items, with only a few type path deviations for private types.
- String
- Option<T>
- Result<T, E>
- Box<T, A: Allocator>
- Rc<T>
- cell
    - UnsafeCell<T>
    - Cell<T>
    - RefCell<T>
    - OnceCell<T>
- sync
    - OnceLock<T>

Disclaimer: This may not work in all cases, such as where some fields are unsized or dependant on a sys (os) implementation

This also works with generics and different layout strategies

![](img/struct_foo.png)
![](img/c_struct_foo.png)

![](img/2enum.png)

![](img/union.png)