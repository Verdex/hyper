
use crate::gen::lua_ast::*;

pub fn gen_code(ast : Vec<Statement>) -> String {
    let mut ret : Vec<String> = vec![]; 
    let mut tab = 0;

    for statement in ast {
        let (v,t) = gen_statement(statement, tab);
        ret.push(v);
        tab = t;
    }

    ret.into_iter().map(|v| format!("{}\n", v)).collect::<String>()
}

fn gen_statement(statement : Statement, mut tab : usize) -> (String, usize) {
    match statement {
        Statement::LocalVarDeclare(name) => (format!("{}local {}", " ".repeat(tab * 4), name), tab),
        Statement::Return(es) => {
            let exprs = es.into_iter().map(gen_expr).collect::<Vec<String>>();

            (format!("{}return {}", " ".repeat(tab * 4), exprs.join(", ")), tab)
        },
        Statement::Break => (format!("{}break", " ".repeat(tab * 4)), tab),
        Statement::If { test, statements } => {
            tab += 1; 
            let test_text = gen_expr(test);
            
            let mut text = vec![];
            for s in statements {
                let (st, tab) = gen_statement(s, tab);
                text.push(st);
            }

            let s = text.join("\n");

            tab -= 1;

            (format!( "{}if {} then\n{}{}end"
                    , " ".repeat(tab * 4)
                    , test_text
                    , s
                    , " ".repeat(tab * 4)
                    ), tab)
        },
        Statement::Elseif { test, statements } => {
            let test_text = gen_expr(test);
            
            let mut text = vec![];
            for s in statements {
                let (st, tab) = gen_statement(s, tab);
                text.push(st);
            }

            let s = text.join("\n");

            (format!( "{}elseif {} then\n{}"
                    , " ".repeat((tab - 1) * 4)
                    , test_text
                    , s
                    ), tab)
        },
        Statement::Else(statements) => {
            let mut text = vec![];
            for s in statements {
                let (st, tab) = gen_statement(s, tab);
                text.push(st);
            }

            let s = text.join("\n");

            (format!( "{}else\n{}"
                    , " ".repeat((tab - 1) * 4)
                    , s
                    ), tab)
        },
        _ => panic!("blah"),
    }
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

