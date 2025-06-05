use crate::ast::{NT, Var, CLINKT};

pub fn cgen(ast: &[NT]) -> String {
    let mut main_body = String::with_capacity(1024);
    let mut headers = String::with_capacity(512);

    for node in ast {
        match node {
            NT::Link(names) => {
                for (name, kind) in names {
                    match kind {
                        CLINKT::LIB => headers.push_str(&format!("#include <{}>\n", name)),
                        CLINKT::PATH => headers.push_str(&format!("#include \"{}\"\n", name)),
                    }
                }
            }

            NT::FNCALL(name, args) => {
                main_body.push_str(&format!("    {}(", name));
                let mut first = true;

                for arg in args {
                    if !first {
                        main_body.push_str(", ");
                    }
                    first = false;

                    match arg {
                        Var::F32(val, _) => main_body.push_str(&val.to_string()),
                        Var::I32(val, _) => main_body.push_str(&val.to_string()),
                        Var::Generic(name) => main_body.push_str(name),
                        Var::List(items) => {
                            let mut first_in_list = true;
                            for item in items {
                                if !first_in_list {
                                    main_body.push_str(", ");
                                }
                                first_in_list = false;

                                match item {
                                    Var::F32(val, _) => main_body.push_str(&val.to_string()),
                                    Var::I32(val, _) => main_body.push_str(&val.to_string()),
                                    Var::Generic(name) => main_body.push_str(name.as_ref()),
                                    _ => main_body.push('0'),
                                }
                            }
                        }
                        _ => main_body.push('0'),
                    }
                }

                main_body.push_str(");\n");
            }

            _ => {}
        }
    }

    let mut full_code = String::with_capacity(headers.len() + main_body.len() + 64);
    full_code.push_str(headers.trim_end());
    full_code.push_str("\n\nint main() {\n");
    full_code.push_str(&main_body);
    full_code.push_str("    return 0;\n}\n");

    full_code
}
