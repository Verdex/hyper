
use crate::gen::lua_ast::*;

pub fn gen_code(ast : Vec<Statement>) -> String {
    let mut ret : Vec<String> = vec![]; 
    let mut tab = 0;

    for statement in ast {
        let v = gen_statement(statement, tab);
        ret.push(v);
    }

    ret.into_iter().map(|v| format!("{}\n", v)).collect::<String>()
}

fn gen_statement(statement : Statement, tab : usize) -> String {
    match statement {
        Statement::LocalVarDeclare(name) => format!("{}local {}", " ".repeat(tab * 4), name),
        Statement::Return(es) => {
            let exprs = es.into_iter().map(gen_expr).collect::<Vec<String>>();

            format!("{}return {}", " ".repeat(tab * 4), exprs.join(", "))
        },
        Statement::Break => format!("{}break", " ".repeat(tab * 4)),
        Statement::If { mut if_statements, else_statement } => {
            let first = if_statements.remove(0);

            let first_text = gen_if(first, tab);

            let elseifs = if_statements
                .into_iter()
                .map(|s| gen_if(s, tab))
                .map(|s| format!("{}elseif {}\n", " ".repeat(tab * 4), s))
                .collect::<String>();

            if else_statement.len() == 0 {
                format!("{}if {}\n{}{}end",
                    " ".repeat(tab * 4),
                    first_text,
                    elseifs,
                    " ".repeat(tab * 4))
            }
            else {
                let else_statements = else_statement
                    .into_iter()
                    .map(|s| gen_statement(s, tab + 1))
                    .map(|s| format!("{}\n", s))
                    .collect::<String>();

                format!("{}if {}\n{}{}else\n{}{}end",
                    " ".repeat(tab * 4),
                    first_text,
                    elseifs,
                    " ".repeat(tab * 4),
                    else_statements,
                    " ".repeat(tab * 4))
            }
        },
        _ => panic!("blah"),
    }
}

fn gen_if(if_statement : If, tab : usize) -> String {
    let test = gen_expr(if_statement.test); 
    let mut statements = vec![];
    for s in if_statement.statements {
        let st = gen_statement(s, tab + 1);
        statements.push(st);
    }

    let statements_text = statements.join("\n");

    format!( "{} then\n{}", test, statements_text )
}

fn gen_expr(expr : Expr) -> String {
    match expr {
        Expr::Var(s) => s,
        Expr::Bool(true) => "true".to_string(),
        Expr::Bool(false) => "false".to_string(),
        Expr::String(s) => s,
        Expr::Number(s) => s,
        Expr::Nil => "nil".to_string(),
        _ => panic!("expr"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_handle_if() {
        let if_statements = vec! [
                                    If { test: Expr::Number("0".to_string())
                                       , statements: vec! [ Statement::Break 
                                                          , Statement::Break
                                                          ]
                                       }
                                 ];

        let ast = vec! [ Statement::If { if_statements, else_statement: vec![] } ];
                                         
        let output = gen_code( ast );

        assert_eq!( output, 
r#"if 0 then
    break
    break
end
"#);
    }

    #[test]
    fn should_handle_if_with_elseif() {
        let if_statements = vec! [
            If { test: Expr::Number("0".to_string())
               , statements: vec! [ Statement::Break 
                                  , Statement::Break
                                  ]
               },
            If { test: Expr::Number("1".to_string())
               , statements: vec! [ Statement::Break, Statement::Break ]
               }
        ];


        let ast = vec! [ Statement::If { if_statements, else_statement: vec![] } ];
                                         
        let output = gen_code( ast );

        assert_eq!( output, 
r#"if 0 then
    break
    break
elseif 1 then
    break
    break
end
"#);
    }

    #[test]
    fn should_handle_if_with_elseif_with_elseif() {

        let if_statements = vec! [
            If { test: Expr::Number("0".to_string())
               , statements: vec! [ Statement::Break 
                                  , Statement::Break
                                  ]
               },
            If { test: Expr::Number("1".to_string())
               , statements: vec! [ Statement::Break, Statement::Break ]
               },
            If { test: Expr::Number("2".to_string())
               , statements: vec! [ Statement::Break, Statement::Break ]
               }
        ];

        let ast = vec! [ Statement::If { if_statements, else_statement: vec![] } ];
                                         
        let output = gen_code( ast );

        assert_eq!( output, 
r#"if 0 then
    break
    break
elseif 1 then
    break
    break
elseif 2 then
    break
    break
end
"#);
    }

    #[test]
    fn should_handle_if_with_elseif_with_elseif_with_else() {

        let if_statements = vec! [
            If { test: Expr::Number("0".to_string())
               , statements: vec! [ Statement::Break 
                                  , Statement::Break
                                  ]
               },
            If { test: Expr::Number("1".to_string())
               , statements: vec! [ Statement::Break, Statement::Break ]
               },
            If { test: Expr::Number("2".to_string())
               , statements: vec! [ Statement::Break, Statement::Break ]
               }
        ];

        let ast = vec! [ Statement::If { if_statements, else_statement: vec![ Statement::Break, Statement::Break ] } ];

        let output = gen_code( ast );

        assert_eq!( output, 
r#"if 0 then
    break
    break
elseif 1 then
    break
    break
elseif 2 then
    break
    break
else
    break
    break
end
"#);
    }

    #[test]
    fn should_handle_if_with_else() {

        let if_statements = vec! [
            If { test: Expr::Number("0".to_string())
               , statements: vec! [ Statement::Break 
                                  , Statement::Break
                                  ]
               },
        ];

        let ast = vec! [ Statement::If { if_statements, else_statement: vec![ Statement::Break, Statement::Break ] } ];
                                         
        let output = gen_code( ast );

        assert_eq!( output, 
r#"if 0 then
    break
    break
else
    break
    break
end
"#);
    }

    #[test]
    fn should_handle_if_with_nested_if() {

        let if_statements = vec! [
            If { test: Expr::Number("0".to_string())
               , statements: vec![ Statement::If { 
                                       if_statements: vec![ If { test: Expr::Number("1".to_string()),
                                                                 statements: vec![ Statement::Break, 
                                                                                   Statement::Break
                                                                                 ] 
                                                               }
                                                          ],
                                       else_statement: vec![]
                                                 }
                                 ]
               }
        ];

        let ast = vec! [ Statement::If { if_statements, else_statement: vec![] } ];

        let output = gen_code( ast );

        assert_eq!( output, 
r#"if 0 then
    if 1 then
        break
        break
    end
end
"#);
    }
}
