#[macro_use(error_defs)]
extern crate error_defs;

error_defs! {
    error Example {
        VariantA
            => "short description",
        VariantB
            => "another short description" ("long description, 23 == {}", 23),
        VariantC { arg: u32 }
            => "another short description" ("long description, 23 == {}", 23),
    }
}

fn main() {
    use std::error::Error;

    let e = Example::VariantC { arg: 23 };
    println!("fmt::Debug::fmt           => {:?}", e);
    println!("fmt::Display::fmt         => {}", e);
    println!("error::Error::description => {}", e.description());
}

