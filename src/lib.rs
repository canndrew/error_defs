#[macro_export]
macro_rules! error_defs {
    ($(error $type_name:ident$(<$($generic:ident$(: $bound:ident$( + $extra_bound_base:ident$(::$extra_bound_more:ident)*)*)*),*>)* {
        $($variant_name:ident $({$($memb_id:ident $(#[$memb_attr:ident])*: $memb_ty:ty),*})*
            => $short:tt $(($long:tt $(, $long_arg:expr)*))*,)*
    })*) => {
        $(
            pub enum $type_name$(<$($generic$(: $bound$( + $extra_bound_base$(::$extra_bound_more)*)*)*),*>)* {
                $(
                    $variant_name $({$($memb_id: $memb_ty),*})*
                ,)*
            }

            impl$(<$($generic: ::std::any::Any + ::std::fmt::Debug$( + $bound$( + $extra_bound_base$(::$extra_bound_more)*)*)*),*>)* ::std::fmt::Debug for $type_name$(<$($generic),*>)* {
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

            impl$(<$($generic: ::std::any::Any + ::std::fmt::Debug$( + $bound$( + $extra_bound_base$(::$extra_bound_more)*)*)*),*>)* ::std::fmt::Display for $type_name$(<$($generic),*>)* {
                #[allow(unused_variables)]
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match self {
                        $(&$type_name::$variant_name $({$(ref $memb_id),*})* => {
                            use ::std::error::Error;
                            try!(write!(f, $short));
                            $(try!(write!(f, concat!(". ", $long) $(, $long_arg)*));)*
                            if let Some(e) = self.cause() {
                                try!(write!(f, " {}", e));
                            }
                        }),*
                    }
                    Ok(())
                }
            }

            impl$(<$($generic: ::std::any::Any + ::std::fmt::Debug$( + $bound$( + $extra_bound_base$(::$extra_bound_more)*)*)*),*>)* ::std::error::Error for $type_name$(<$($generic),*>)* {
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
                                let $memb_attr = {
                                    let ret: &$memb_ty = $memb_id;
                                    return Some(ret)
                                };
                            )*)*)*
                            None
                        }),*
                    }
                }
            }
        )*
    }
}

/*
pub trait Error: ::std::fmt::Display + ::std::fmt::Debug {
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&Error>;
}
*/


