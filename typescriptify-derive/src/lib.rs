extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(TypeScriptify)]
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
    let mut complete_string: String = "".to_string();
    let _n = match ast.body {
        syn::Body::Struct(ref data) => {
            let mut fieldlines: Vec<String> = vec![];
            for field in data.fields() {
                //field.ty; //type of the field
                //field.ident; //name (tuple struct fields have none here)
                //field.vis; //visibility
                //field.attrs; //attributes
                println!("Fieldtype = {:?} and Name = {:?}", field.ty, field.ident);
                let fieldname: String = format!("{}", field.ident.clone().unwrap().to_string());
                match field.ty {
                    syn::Ty::Array(ref _b, ref _c) => {
                        unimplemented!()
                    }
                    syn::Ty::Ptr(ref _p) => {
                        unimplemented!()
                    }
                    syn::Ty::Path(ref _qselfopt, ref path) => {
                        let intype = format!("{}", path.segments.last().unwrap().ident);
                        let generic_params_unformated = &path.segments.last().clone().unwrap().parameters;
                        let mut generics_parameters: Vec<String> = Vec::new();
                        match generic_params_unformated {
                            &syn::PathParameters::AngleBracketed(ref angle_bracketed_parameter_data) => {
                                for ty in &angle_bracketed_parameter_data.types {
                                    match ty {
                                        &syn::Ty::Path(ref _qotherself, ref qotherpath) => {
                                            generics_parameters.push(format!("{}", qotherpath.segments.last().unwrap().ident));
                                        }
                                        _ => unimplemented!(),
                                    }
                                }
                            }
                            _ => unimplemented!(),
                        };
                        //treat option special, as types in typescript are already nullable
                        let mtyp: String = if intype.eq("Option") {
                            (match generics_parameters.first().unwrap().as_ref() {
                                "i64" => "number",
                                "u32" => "number",
                                "u16" => "number",
                                "u8" => "number",
                                "bool" => "boolean",
                                "String" => "string",
                                "f32" => "number",
                                "f64" => "number",
                                "HashMap" => "Map",
                                "Vec" => "Array",
                                "HashSet" => "Array",
                                a @ _ => a,
                            }).to_string()
                        } else {
                            let mut generic_term_in_angle_brackets: String = if generics_parameters.is_empty() { "".to_string() } else { "<".to_string() };
                            for gen in &generics_parameters {
                                if generic_term_in_angle_brackets.len() > 1 {
                                    generic_term_in_angle_brackets = generic_term_in_angle_brackets + ", ";
                                }
                                generic_term_in_angle_brackets = generic_term_in_angle_brackets + match gen.as_ref() {
                                    "i64" => "number",
                                    "u32" => "number",
                                    "u16" => "number",
                                    "u8" => "number",
                                    "bool" => "boolean",
                                    "String" => "string",
                                    "f32" => "number",
                                    "f64" => "number",
                                    "HashMap" => "Map",
                                    "Vec" => "Array",
                                    "HashSet" => "Array",
                                    a @ _ => a,
                                };
                            }
                            if !generics_parameters.is_empty() {
                                generic_term_in_angle_brackets = generic_term_in_angle_brackets + ">";
                            }
                            (match intype.as_ref() {
                                "i64" => "number".to_string(),
                                "u32" => "number".to_string(),
                                "u16" => "number".to_string(),
                                "u8" => "number".to_string(),
                                "bool" => "boolean".to_string(),
                                "String" => "string".to_string(),
                                "f32" => "number".to_string(),
                                "f64" => "number".to_string(),
                                "HashMap" => "Map".to_string(),
                                "Vec" => "Array".to_string(),
                                "HashSet" => "Array".to_string(),
                                a @ _ => a.to_string(),
                            } + &generic_term_in_angle_brackets)
                        };
                        fieldlines.push(format!("{}: {};", fieldname, mtyp));
                    }
                    _ => unimplemented!(),
                }
            }


            let mut s = "".to_string();
            for fieldline in fieldlines {
                s = s + "    " + &fieldline + "\n";
            }
            complete_string = format!("export interface {} {{\n{}}}", structname, s);
            data.fields().len()
        }
        syn::Body::Enum(ref variant_vec) => {
            let k = variant_vec.len();
            let enum_name = format!("{}", name);
            let mut variants: Vec<String> = Vec::new();
            for variant in variant_vec {
                let mut fieldlines: Vec<String> = vec![];
                let variant_name = format!("{}", variant.ident);
                variants.push(variant_name.to_string());
                let data = &variant.data;

                //add each variant as a field with the variant name as field name, and also its type

                for field in data.fields() {
                    //field.ty; //type of the field
                    //field.ident; //name (tuple struct fields have none here)
                    //field.vis; //visibility
                    //field.attrs; //attributes
                    println!("Fieldtype = {:?} and Name = {:?}", field.ty, field.ident);
                    let fieldname: String = format!("{}", field.ident.clone().unwrap().to_string());
                    match field.ty {
                        syn::Ty::Array(ref _b, ref _c) => {
                            unimplemented!()
                        }
                        syn::Ty::Ptr(ref _p) => {
                            unimplemented!()
                        }
                        syn::Ty::Path(ref _qselfopt, ref path) => {
                            let intype = format!("{}", path.segments.last().unwrap().ident);
                            let generic_params_unformated = &path.segments.last().clone().unwrap().parameters;
                            let mut generics_parameters: Vec<String> = Vec::new();
                            match generic_params_unformated {
                                &syn::PathParameters::AngleBracketed(ref angle_bracketed_parameter_data) => {
                                    for ty in &angle_bracketed_parameter_data.types {
                                        match ty {
                                            &syn::Ty::Path(ref _qotherself, ref qotherpath) => {
                                                generics_parameters.push(format!("{}", qotherpath.segments.last().unwrap().ident));
                                            }
                                            _ => unimplemented!(),
                                        }
                                    }
                                }
                                _ => unimplemented!(),
                            };
                            //treat option special, as types in typescript are already nullable
                            let mtyp: String = if intype.eq("Option") {
                                (match generics_parameters.first().unwrap().as_ref() {
                                    "i8" => "number",
                                    "i16" => "number",
                                    "i32" => "number",
                                    "i64" => "number",
                                    "u8" => "number",
                                    "u16" => "number",
                                    "u32" => "number",
                                    "u64" => "number",
                                    "bool" => "boolean",
                                    "String" => "string",
                                    "f32" => "number",
                                    "f64" => "number",
                                    "HashMap" => "Map",
                                    "Vec" => "Array",
                                    "HashSet" => "Array",
                                    a @ _ => a,
                                }).to_string()
                            } else {
                                let mut generic_term_in_angle_brackets: String = if generics_parameters.is_empty() { "".to_string() } else { "<".to_string() };
                                for gen in &generics_parameters {
                                    if generic_term_in_angle_brackets.len() > 1 {
                                        generic_term_in_angle_brackets = generic_term_in_angle_brackets + ", ";
                                    }
                                    generic_term_in_angle_brackets = generic_term_in_angle_brackets + match gen.as_ref() {
                                        "i8" => "number",
                                        "i16" => "number",
                                        "i32" => "number",
                                        "i64" => "number",
                                        "u8" => "number",
                                        "u16" => "number",
                                        "u32" => "number",
                                        "u64" => "number",
                                        "bool" => "boolean",
                                        "String" => "string",
                                        "f32" => "number",
                                        "f64" => "number",
                                        "HashMap" => "Map",
                                        "Vec" => "Array",
                                        "HashSet" => "Array",
                                        a @ _ => a,
                                    };
                                }
                                if !generics_parameters.is_empty() {
                                    generic_term_in_angle_brackets = generic_term_in_angle_brackets + ">";
                                }
                                (match intype.as_ref() {
                                    "i8" => "number".to_string(),
                                    "i16" => "number".to_string(),
                                    "i32" => "number".to_string(),
                                    "i64" => "number".to_string(),
                                    "u8" => "number".to_string(),
                                    "u16" => "number".to_string(),
                                    "u32" => "number".to_string(),
                                    "u64" => "number".to_string(),
                                    "bool" => "boolean".to_string(),
                                    "String" => "string".to_string(),
                                    "f32" => "number".to_string(),
                                    "f64" => "number".to_string(),
                                    "HashMap" => "Map".to_string(),
                                    "Vec" => "Array".to_string(),
                                    "HashSet" => "Array".to_string(),
                                    a @ _ => a.to_string(),
                                } + &generic_term_in_angle_brackets)
                            };
                            fieldlines.push(format!("{}: {};", fieldname, mtyp));
                        }
                        _ => unimplemented!(),
                    }
                }


                let mut s = "".to_string();
                for fieldline in &fieldlines {
                    s = s + "    " + fieldline.as_ref() + "\n";
                }

                complete_string = complete_string + &format!("export interface {} {{\n{}}}\n\n", variant_name, s);
            }

            //Add final enum interface:
            let mut s = "".to_string();
            for v in variants {
                s = s + "    " + v.as_ref() + ": " + v.as_ref() + ";\n";
            }
            complete_string = complete_string + &format!("export interface {} {{\n{}}}\n\n", enum_name, s);

            k
        }
    };
    quote! {
        impl TypeScriptifyTrait for #name {
            fn type_script_ify() -> String {
                format!("{}\n", #complete_string)
            }
        }
    }
}