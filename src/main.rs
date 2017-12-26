#[macro_use]
extern crate hello_world_derive;

use std::collections::HashSet;
use std::collections::HashMap;

trait HelloWorld {
    fn hello_world();
    fn as_typescript_interface_definition() -> String;
    fn as_typescript_type() -> String;
}

#[derive(HelloWorld)]
struct FrenchToast {
    pub i : u32,
    pub v : Vec<u8>,
    pub hashmap: HashMap<String, u16>,
    pub hashset: HashSet<u32>,
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
    println!("Typescript output for Waffles: \n{}", Waffles::as_typescript_interface_definition() );
    println!("Typescript output for FrenchToast: \n{}", FrenchToast::as_typescript_interface_definition() );
    println!("Typescript type for Waffles: \n{}", Waffles::as_typescript_type() );
}