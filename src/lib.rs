#[macro_export]
macro_rules! error_defs {
    ($(error $type_name:ident {
        $($variant_name:ident 
            => $short:tt $(($long:tt $(, $long_arg:expr)*))*),*
    })*) => {
        $(
            pub enum $type_name {
                $($variant_name,)*
            }

            impl ::std::fmt::Debug for $type_name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match self {
                        $(&$type_name::$variant_name => {
                            try!(write!(f, concat!(stringify!($variant_name), " /* {} */"), self));
                        }),*
                    }
                    Ok(())
                }
            }

            impl ::std::fmt::Display for $type_name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match self {
                        $(&$type_name::$variant_name => {
                            try!(write!(f, $short));
                            $(try!(write!(f, concat!(". ", $long) $(, $long_arg)*));)*
                        }),*
                    }
                    Ok(())
                }
            }
        )*
    }
}

error_defs! {
    error Balls {
        Flim => "wow" ("really though, wow {}", 23),
        Flam => "wib"
    }
}

