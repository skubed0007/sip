use crate::ast::{NT, Var};
use std::fmt::Write;

pub fn cgen(ast: &[NT]) -> String {
    let mut code = String::with_capacity(2048);

    for node in ast {
        if let NT::Link(names) = node {
            for name in names {
                let _ = writeln!(code, "#include <{}>", name);
            }
        }
    }

    code.push_str("\nint main() {\n");

    for node in ast {
        if let NT::FNCALL(name, args) = node {
            let _ = write!(code, "    {}(", name);
            let mut is_first = true;

            for arg in args {
                if !is_first {
                    code.push_str(", ");
                }

                match arg {
                    Var::F32(val, _) => {
                        let _ = write!(code, "{}", val);
                    }
                    Var::I32(val, _) => {
                        let _ = write!(code, "{}", val);
                    }
                    Var::Generic(name) => {
                        code.push_str(name);
                    }
                    Var::List(items) => {
                        let mut list_first = true;
                        for item in items {
                            if !is_first || !list_first {
                                code.push_str(", ");
                            }
                            list_first = false;

                            match item {
                                Var::F32(val, _) => {
                                    let _ = write!(code, "{}", val);
                                }
                                Var::I32(val, _) => {
                                    let _ = write!(code, "{}", val);
                                }
                                Var::Generic(name) => {
                                    code.push_str(name);
                                }
                                _ => code.push('0'),
                            }
                        }
                    }
                    _ => code.push('0'),
                }

                is_first = false;
            }

            code.push_str(");\n");
        }
    }

    code.push_str("}\n");
    code
}
