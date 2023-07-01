#[cfg(feature="dissect_std")]
pub mod std;

#[macro_export]
macro_rules! impl_mirror_mock {
    (with <$($generics: ident),*>: $mock: ty => $real: ty) => {
        impl<$($generics,)*> Dissectible for $real {
            fn field_info() -> crate::LayoutInfo {
                let mut info = <$mock as Dissectible>::field_info();
                info.name = ::std::any::type_name::<$real>();
                info
            }
        }
    };
    (with <$($generics: ident),*>: $mock: ty => $real: ty where $($constraint: tt)*) => {
        impl<$($generics,)*> Dissectible for $real where $($constraint)* {
            fn field_info() -> crate::LayoutInfo {
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
