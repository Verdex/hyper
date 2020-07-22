
use crate::gen::lua_ast::*;

pub fn gen_code(ast : Vec<Statement>) -> String {
    let mut ret : Vec<String> = vec![]; 
    let mut tab = 0;

    for statement in ast {
        match statement {
            Statement::Return(e) => {
                let exprs = e.into_iter().map(gen_expr).collect::<Vec<String>>();

                ret.push(format!("{}return {}", " ".repeat(tab * 4), exprs.join(", ")));
            },
            _ => panic!("blah"),
        }
    }

    ret.into_iter().map(|v| format!("{}\n", v)).collect::<String>()
}

fn gen_expr(expr : Expr) -> String {
    "".to_string()
}

