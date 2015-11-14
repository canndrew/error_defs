#[macro_export]
macro_rules! error_defs {
    ($(error $type_name:ident {
        $($variant_name:ident $({$($memb_id:ident $(#[$memb_attr:ident])*: $memb_ty:ty),*})*
            => $short:tt $(($long:tt $(, $long_arg:expr)*))*,)*
    })*) => {
        $(
            pub enum $type_name {
                $($variant_name $({$($memb_id: $memb_ty),*})*,)*
            }

            impl ::std::fmt::Debug for $type_name {
                #[allow(unused_variables)]
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match self {
                        $(&$type_name::$variant_name $({$(ref $memb_id),*})* => {
                            try!(f.debug_struct(stringify!($variant_name))$($(.field(stringify!($memb_id), $memb_id))*)*.finish());
                            try!(write!(f, " /* {} */", self));
                        }),*
                    }
                    Ok(())
                }
            }

            impl ::std::fmt::Display for $type_name {
                #[allow(unused_variables)]
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match self {
                        $(&$type_name::$variant_name $({$(ref $memb_id),*})* => {
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
                        $(&$type_name::$variant_name $({$(ref $memb_id),*})* => concat!($short)),*
                    }
                }

                #[allow(unused_variables, unreachable_code)]
                fn cause(&self) -> Option<&::std::error::Error> {
                    match self {
                        $(&$type_name::$variant_name $({$(ref $memb_id),*})* => {
                            $($($(
                                let ret: &$memb_ty = ::std::convert::From::$memb_attr($memb_id);
                                return Some(ret);
                            )*)*)*
                            None
                        }),*
                    }
                }
            }

            $($($($(
                impl ::std::convert::From<$memb_ty> for $type_name {
                      fn $memb_attr(e: ::std::io::Error) -> $type_name {
                            $type_name::$variant_name {
                                $memb_id: e
                            }
                      }
                }
            )*)*)*)*
        )*
    }
}

