extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloWorld)]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_hello_world(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}


fn impl_hello_world(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let structname = name.to_string();
    let _body = &ast.body;
    let mut fieldlines : Vec<String> = vec![];
    let n = match ast.body {
        syn::Body::Struct(ref data) => {
            for field in data.fields() {
                //field.ty; //type of the field
                //field.ident; //name (tuple struct fields have none here)
                //field.vis; //visibility
                //field.attrs; //attributes
                println!("Fieldtype = {:?} and Name = {:?}", field.ty, field.ident);
                let fieldname : String = format!("{}", field.ident.clone().unwrap().to_string());
                match field.ty {
                    syn::Ty::Array(ref _b, ref _c) => {
                        unimplemented!()
                    },
                    syn::Ty::Ptr(ref _p) => {
                        unimplemented!()
                    },
                    syn::Ty::Path(ref _qselfopt, ref path) => {
                        fieldlines.push(format!("{} : {} ;", fieldname, path.segments.last().unwrap().ident));
                    },
                    _ => unimplemented!(),
                }
                //fieldlines.push(format!("{}: {};", field.ident.unwrap().to_string(), field.ty.get_type_id().to_string()));
            }
            data.fields().len()
        },
        syn::Body::Enum(_) => panic!("#[derive(HelloWorld)] can only be used with structs"),
    };
    let mut s = "".to_string();
    for fieldline in fieldlines {
        s = s + &fieldline + "\n";
    }
    let complete_string: String = format!("export interface {} {{ \n {} \n }}", structname, s);
    quote! {
        impl HelloWorld for #name {
            fn hello_world() {
                println!("Hello, World! My name is {} and i have {} fields", stringify!(#name), #n);
            }

            fn as_typescript_interface_definition() -> String {
                format!("{}\n", stringify!(#complete_string))
            }

            fn as_typescript_type() -> String {
                format!("{}\n", stringify!(#name))
            }
        }
    }
}