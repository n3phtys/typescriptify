#[macro_use]
extern crate typescriptify_derive;


pub trait TypeScriptifyTrait {
    fn type_script_ify() -> String;
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::collections::HashMap;
    use TypeScriptifyTrait;

    #[derive(TypeScriptify)]
    struct FrenchToast {
        pub i: u32,
        pub v: Vec<u8>,
        pub hashmap: HashMap<String, u16>,
        pub hashset: HashSet<u32>,
        pub optional: Option<bool>,
    }

    #[derive(TypeScriptify)]
    struct Waffles {
        pub t: i64,
        pub x: bool,
        pub subtoast: FrenchToast,
    }

    #[derive(TypeScriptify)]
    pub enum Sweet {
        Caroline {
            x: i64,
            b: bool,
            hashmap: HashMap<String, u16>,
            hashset: HashSet<u32>,
        },
        Sugar {
            i: u32,
            x: u64,
            optional: Option<bool>,
            v: Vec<u8>,
        },
    }


    #[derive(TypeScriptify)]
    pub enum Enum {
        Created,
        Finalized,
        ExportedAtLeastOnce,
    }


    #[test]
    fn test_works() {


        let r = format!("Typescript output for Enum: \n{}", Enum::type_script_ify());
        let x = format!("Typescript output for Waffles: \n{}", Waffles::type_script_ify());
        let y = format!("Typescript output for FrenchToast: \n{}", FrenchToast::type_script_ify());
        let z = format!("Typescript output for Sweet: \n{}", Sweet::type_script_ify());

        println!("Typescript outputs:\n{}\n{}\n{}\n{}\n", r, x, y, z);

        assert_eq!(x.contains("subtoast: FrenchToast"), true);
        assert_eq!(y.contains("hashmap: Map<string, number>"), true);
        assert_eq!(z.contains("export interface Caroline"), true);
    }
}