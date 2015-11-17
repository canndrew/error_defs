#[macro_use(error_defs)]
extern crate error_defs;

use std::io;
use std::error::Error;

error_defs! {
    error Example<T> {
        VariantA
            => "short description",
        VariantB
            => "another short description" ("long description, 23 == {}", 23),
        VariantC { arg0: u32, arg1: T }
            => "another short description" ("long description, 23 == {}", 23),
        VariantD { io_err #[cause] : io::Error }
            => "another short description" ("long description, 23 == {}", 23),
    }
}

fn main() {
    for e in &[
        Example::VariantC { arg0: 45, arg1: 'x' },
        Example::VariantD { io_err: io::Error::new(io::ErrorKind::Other, "hello") },
    ][..] {
        println!("fmt::Debug::fmt           => {:?}", e);
        println!("fmt::Display::fmt         => {}", e);
        println!("error::Error::description => {}", e.description());
        println!("error: Error::cause       => {:?}", e.cause());
        println!("");
    }
}

