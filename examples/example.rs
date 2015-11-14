#[macro_use(error_defs)]
extern crate error_defs;

use std::io;

error_defs! {
    error Example {
        VariantA
            => "short description",
        VariantB
            => "another short description" ("long description, 23 == {}", 23),
        VariantC { arg0: u32, arg1: u32 }
            => "another short description" ("long description, 23 == {}", 23),
        VariantD { io_err #[from]: io::Error }
            => "another short description" ("long description, 23 == {}", 23),
    }
}

fn main() {
    use std::error::Error;

    for e in &[
        Example::VariantC { arg0: 45, arg1: 23 },
        Example::VariantD { io_err: io::Error::new(io::ErrorKind::Other, "hello") },
    ][..] {
        println!("fmt::Debug::fmt           => {:?}", e);
        println!("fmt::Display::fmt         => {}", e);
        println!("error::Error::description => {}", e.description());
        println!("error: Error::cause       => {:?}", e.cause());
        println!("");
    }
}

