#[macro_use]
extern crate hello_world_derive;

trait HelloWorld {
    fn hello_world();
}

#[derive(HelloWorld)]
struct FrenchToast {
    pub i : u32,
}

#[derive(HelloWorld)]
struct Waffles {
    pub t : i64,
    pub x : bool,
}

fn main() {
    FrenchToast::hello_world();
    Waffles::hello_world();
}