use crate::{Dissectible, LayoutInfo};

#[cfg(feature="dissect_std")]
pub mod std;

pub mod tuples;

#[macro_export]
macro_rules! impl_mirror_mock {
    (with <$($generics: ident),*>: $mock: ty => $real: ty) => {
        impl<$($generics,)*> Dissectible for $real {
            fn field_info() -> $crate::LayoutInfo {
                let mut info = <$mock as Dissectible>::field_info();
                info.name = ::std::any::type_name::<$real>();
                info
            }
        }
    };
    (with <$($generics: ident),*>: $mock: ty => $real: ty where $($constraint: tt)*) => {
        impl<$($generics,)*> Dissectible for $real where $($constraint)* {
            fn field_info() -> $crate::LayoutInfo {
                let mut info = <$mock as Dissectible>::field_info();
                info.name = ::std::any::type_name::<$real>();
                info
            }
        }
    };
    ($mock: ty => $real: ty) => {
        impl Dissectible for $real {
            fn field_info() -> crate::LayoutInfo {
                let mut info = <$mock as Dissectible>::field_info();
                info.name = ::std::any::type_name::<$real>();
                info
            }
        }
    };
}

impl<T> Dissectible for &T where T: ?Sized {
    fn field_info() -> crate::LayoutInfo {
        LayoutInfo {
            attrs: vec![],
            name: ::std::any::type_name::<T>(),
            size: ::std::mem::size_of::<&T>(),
            align: ::std::mem::align_of::<&T>(),
            structure: crate::Structure::Reference(false),
        }
    }
}

impl<T> Dissectible for &mut T where T: ?Sized {
    fn field_info() -> crate::LayoutInfo {
        LayoutInfo {
            attrs: vec![],
            name: ::std::any::type_name::<T>(),
            size: ::std::mem::size_of::<&T>(),
            align: ::std::mem::align_of::<&T>(),
            structure: crate::Structure::Reference(true),
        }
    }
}