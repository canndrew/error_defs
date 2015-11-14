#[macro_export]
macro_rules! error_defs {
    ($(error $type_name:ident {
        $($variant_name:ident $({$memb_id:ident: $memb_ty:ty})*
            => $short:tt $(($long:tt $(, $long_arg:expr)*))*,)*
    })*) => {
        $(
            pub enum $type_name {
                $($variant_name $({$memb_id: $memb_ty})*,)*
            }

            impl ::std::fmt::Debug for $type_name {
                #[allow(unused_variables)]
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match self {
                        $(&$type_name::$variant_name $({$memb_id})* => {
                            try!(write!(f, concat!(stringify!($variant_name), " /* {} */"), self));
                        }),*
                    }
                    Ok(())
                }
            }

            impl ::std::fmt::Display for $type_name {
                #[allow(unused_variables)]
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match self {
                        $(&$type_name::$variant_name $({$memb_id})* => {
                            try!(write!(f, $short));
                            $(try!(write!(f, concat!(". ", $long) $(, $long_arg)*));)*
                        }),*
                    }
                    Ok(())
                }
            }

            impl ::std::error::Error for $type_name {
                #[allow(unused_variables)]
                fn description(&self) -> &str {
                    match self {
                        $(&$type_name::$variant_name $({$memb_id})* => concat!($short)),*
                    }
                }

                fn cause(&self) -> Option<&::std::error::Error> {
                    None
                }
            }
        )*
    }
}

