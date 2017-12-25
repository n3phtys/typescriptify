#[macro_use]
extern crate hello_world_derive;

trait HelloWorld {
    fn hello_world();
    fn to_typescript_interface() -> String;
}

#[derive(HelloWorld)]
struct FrenchToast {
    pub i : u32,
}

#[derive(HelloWorld)]
struct Waffles {
    pub t : i64,
    pub x : bool,
    pub subtoast : FrenchToast,
}

fn main() {
    FrenchToast::hello_world();
    Waffles::hello_world();
    println!("Typescript output for Waffles: \n{}", Waffles::to_typescript_interface() );
}