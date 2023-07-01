use crate::{Dissectible, Structure, StructFields};

macro_rules! tuple_impl {
    ($dummy: ident: $($ty: ident)*) => {
        #[derive(Dissectible)]
        struct $dummy <$($ty,)*> ($($ty,)*);

        impl <$($ty,)*> Dissectible for ($($ty,)*) {
            fn field_info() -> crate::LayoutInfo {
                let mut info = <$dummy <$($ty,)*> as Dissectible>::field_info();
                info.name = "";
                if let Structure::Struct(StructFields::Tuple(t)) = info.structure {
                    info.structure = Structure::Tuple(t)
                } else { unreachable!() };
                info
            }
        }
    };
}

tuple_impl!(T:);
tuple_impl!(TA: A);
tuple_impl!(TB: A B);
tuple_impl!(TC: A B C);
tuple_impl!(TD: A B C D);
tuple_impl!(TE: A B C D  E);
tuple_impl!(TF: A B C D  E F);
tuple_impl!(TG: A B C D  E F G);
tuple_impl!(TH: A B C D  E F G H);
tuple_impl!(TI: A B C D  E F G H  I);
tuple_impl!(TJ: A B C D  E F G H  I J);
tuple_impl!(TK: A B C D  E F G H  I J K);
tuple_impl!(TL: A B C D  E F G H  I J K L);
tuple_impl!(TM: A B C D  E F G H  I J K L  M);
tuple_impl!(TN: A B C D  E F G H  I J K L  M N);
tuple_impl!(TO: A B C D  E F G H  I J K L  M N O);
tuple_impl!(TP: A B C D  E F G H  I J K L  M N O P);