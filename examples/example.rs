#[macro_use(error_defs)]
extern crate error_defs;

use std::io;
use std::error::Error;

error_defs! {
    error Example<T, U: Error> {
        VariantA
            => "short description",
        VariantB
            => "another short description" ("long description, 23 == {}.", 23),
        VariantC { arg0: u32, arg1: T }
            => "c's short description" ("long description, arg0 == {}.", arg0),
        VariantD { io_err #[cause] : io::Error }
            => "d's short description" ("IO error!"),
        VariantE { arg #[cause]: U }
            => "e's short description" ("Variant E error."),
    }
}

fn main() {
    for e in &[
        Example::VariantA,
        Example::VariantB,
        Example::VariantC { arg0: 45, arg1: 'x' },
        Example::VariantD { io_err: io::Error::new(io::ErrorKind::Other, "hello.") },
        Example::VariantE {
            arg: Example::VariantD::<char, io::Error> {
                io_err: io::Error::new(io::ErrorKind::Other, "I'm an IO error.")
            }
        },
    ][..] {
        println!("error::Error::description => {}", e.description());
        println!("error: Error::cause       => {:?}", e.cause());
        println!("fmt::Display::fmt         => {}", e);
        println!("fmt::Debug::fmt           => {:?}", e);
        println!("fmt::Debug::fmt (pretty)  => {:#?}", e);
        println!("");
    }
}

