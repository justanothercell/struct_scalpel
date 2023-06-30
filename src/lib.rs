use std::sync::Once;

pub use mem_scalpel_proc_macro::Dissectible;

pub struct FieldInfo {
    pub type_name: &'static str,
    pub size: usize,
    pub align: usize,
    pub offset: usize
}

impl FieldInfo {
    pub fn from_val_and_base<T>(base: usize, v: &T) -> Self {
        Self {
            type_name: std::any::type_name::<T>(), 
            size: std::mem::size_of::<T>(), 
            align: std::mem::align_of::<T>(), 
            offset: v as *const _ as usize - base
        }
    }
}

pub enum LayoutInfo {
    NamedStruct {
        attrs: Vec<&'static str>,
        name: &'static str,
        size: usize,
        align: usize,
        fields: Vec<(&'static str, FieldInfo)>
    },
    TupleStruct {
        attrs: Vec<&'static str>,
        name: &'static str,
        size: usize,    
        align: usize,
        fields: Vec<FieldInfo>
    },
    UnitStruct {
        attrs: Vec<&'static str>,
        name: &'static str,
    }
}

pub trait Dissectible {
    fn field_info() -> LayoutInfo;
}

static ANSI_INIT: Once = Once::new();

pub fn print_info<T: Dissectible>() {
    ANSI_INIT.call_once(|| {
        enable_ansi_support::enable_ansi_support().expect("ansi not supported!");
    });

    match T::field_info() {
        LayoutInfo::NamedStruct { attrs, name, size, align, fields } => {
            for attr in attrs {
                println!("{attr}")
            }
            println!("\x1b[38;5;245m/* size={size:2}, align={align:2} */\x1b[0m");
            print!("struct \x1b[1;31m{name}\x1b[0m {{");
            if fields.len() == 0 { println!(" }}"); return }
            println!();
            let n: u8 = 255 / fields.len() as u8;
            for (f, (fname, field)) in fields.iter().enumerate() {
                let (r, g, b) = hsv_to_rgb(f as u8 * n, 200, 255);
                println!("    {}: \x1b[48;2;{r};{g};{b}m{}\x1b[0m,\t\x1b[38;5;245m/* offset={:2}, size={:2}, align={:2} */\x1b[0m", fname, field.type_name, field.offset, field.size, field.align)
            }
            println!("}}");
            if size == 0 { return }
            println!();
            'outer: for i in 0..size {
                for (f, (_fname, field)) in fields.iter().enumerate() {
                    if i >= field.offset && i < field.offset + field.size {
                        let (r, g, b) = hsv_to_rgb(f as u8 * n, 200, 255);
                        print!("\x1b[48;2;{r};{g};{b}m.");
                        continue 'outer;
                    }
                }
                print!("\x1b[48;2;100;100;100m.");
            }
            print!("\x1b[0m");
            println!()
        },
        LayoutInfo::TupleStruct { attrs, name, size, align, fields } => {
            for attr in attrs {
                println!("{attr}")
            }
            println!("\x1b[38;5;245m/* size={size:2}, align={align:2} */\x1b[0m");
            print!("struct \x1b[1;31m{name}\x1b[0m (");
            if fields.len() == 0 { println!(");"); return }
            println!();
            let n: u8 = 255 / fields.len() as u8;
            for (f,  field) in fields.iter().enumerate() {
                let (r, g, b) = hsv_to_rgb(f as u8 * n, 200, 255);
                println!("    \x1b[48;2;{r};{g};{b}m{}\x1b[0m,\t\x1b[38;5;245m/* offset={:2}, size={:2}, align={:2} */\x1b[0m", field.type_name, field.offset, field.size, field.align)
            }
            println!(");");
            if size == 0 { return }
            println!();
            'outer: for i in 0..size {
                for (f,  field) in fields.iter().enumerate() {
                    if i >= field.offset && i < field.offset + field.size {
                        let (r, g, b) = hsv_to_rgb(f as u8 * n, 200, 255);
                        print!("\x1b[48;2;{r};{g};{b}m.");
                        continue 'outer;
                    }
                }
                print!("\x1b[48;2;100;100;100m.");
            }
            print!("\x1b[0m");
            println!()
        },
        LayoutInfo::UnitStruct { attrs, name } => {
            for attr in attrs {
                println!("{attr}")
            }
            println!("\x1b[38;5;245m/* size= 0, align= 1 */\x1b[0m");
            println!("struct \x1b[1;31m{name}\x1b[0m;");
        },
    }
}

fn hsv_to_rgb(h: u8, s: u8, v: u8) -> (u8, u8, u8) {
    let h = h as u32;
    let s = s as u32;
    let v = v as u32;

    if s == 0 {
        return (v as u8, v as u8, v as u8);
    }
    
    let region = h / 43;
    let remainder = (h - (region * 43)) * 6; 
    
    let p = (v * (255 - s)) >> 8;
    let q = (v * (255 - ((s * remainder) >> 8))) >> 8;
    let t = (v * (255 - ((s * (255 - remainder)) >> 8))) >> 8;
    
    match region {
        0 => (v as u8, t as u8, p as u8),
        1 => (q as u8, v as u8, p as u8),
        2 => (p as u8, v as u8, t as u8),
        3 => (p as u8, q as u8, v as u8),
        4 => (t as u8, p as u8, v as u8),
        _ => (v as u8, p as u8, q as u8),
    }  
}