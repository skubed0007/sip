use crate::ast::{Var, NT};

pub fn cgen(ast: &[NT]) -> String {
    let mut main_body = String::with_capacity(1024); // Pre-allocate memory for the main body
    let mut headers = String::with_capacity(512); // Pre-allocate memory for headers

    // Iterate over AST nodes
    for node in ast {
        match node {
            NT::Link(names) => {
                for name in names {
                    headers.push_str(&format!("#include <{}>\n", name));
                }
            }

            NT::FNCALL(name, args) => {
                main_body.push_str(&format!("    {}(", name)); // Indent inside main
                let mut first = true;

                // Process arguments
                for arg in args {
                    if !first {
                        main_body.push_str(", ");
                    }
                    first = false;
                    
                    match arg {
                        Var::F32(val, _) => main_body.push_str(&val.to_string()),
                        Var::I32(val, _) => main_body.push_str(&val.to_string()),
                        Var::Generic(name) => {
                            main_body.push_str(&format!("{}", name));
                        }
                        Var::List(items) => {
                            let mut first_in_list = true;
                            for item in items {
                                if !first && !first_in_list {
                                    main_body.push_str(", ");
                                }
                                first = false;
                                first_in_list = false;
                        
                                match item {
                                    Var::F32(val, _) => main_body.push_str(&val.to_string()),
                                    Var::I32(val, _) => main_body.push_str(&val.to_string()),
                                    Var::Generic(name) => main_body.push_str(&name.to_string()),
                                    _ => main_body.push_str("0"), // fallback
                                }
                            }
                        }
                        
                        _ => main_body.push_str("0"), // For unsupported types, use 0
                    }
                }

                main_body.push_str(");\n");
            }

            _ => {} // You can expand for more AST node types
        }
    }

    // Assemble the final C code
    let mut full_code = String::with_capacity(headers.len() + main_body.len() + 50);
    full_code.push_str(&headers.trim_end());
    full_code.push_str("\nint main() {\n");
    full_code.push_str(&main_body);
    full_code.push_str("}\n");

    full_code
}
