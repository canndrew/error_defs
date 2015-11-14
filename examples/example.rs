#[macro_use(error_defs)]
extern crate error_defs;

error_defs! {
    error Example {
        VariantA
            => "short description",
        VariantB
            => "another short description" ("long description, 23 == {}", 23)
    }
}

fn main() {
    let e = Example::VariantB;
    println!("e (debug) == {:?}", e);
    println!("e (display) == {:?}", e);
}

