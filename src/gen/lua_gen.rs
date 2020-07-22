
use crate::gen::lua_ast::*;

pub fn gen_code(ast : Vec<Statement>) -> String {
    let mut ret : Vec<String> = vec![]; 
    let mut tab = 0;

    for statement in ast {
        match statement {
            Statement::Return(es) => {
                let exprs = es.into_iter().map(gen_expr).collect::<Vec<String>>();

                ret.push(format!("{}return {}", " ".repeat(tab * 4), exprs.join(", ")));
            },
            Statement::Break => ret.push(format!("{}break", " ".repeat(tab * 4))),
            _ => panic!("blah"),
        }
    }

    ret.into_iter().map(|v| format!("{}\n", v)).collect::<String>()
}

fn gen_expr(expr : Expr) -> String {
    match expr {
        Expr::Var(s) => s,
        Expr::Bool(true) => "true".to_string(),
        Expr::Bool(false) => "false".to_string(),
        Expr::String(s) => s,
        Expr::Number(s) => s,
        _ => panic!("expr"),
    }
}

