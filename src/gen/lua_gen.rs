
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
        Statement::If { test, statements, followed } => {
            tab += 1; 
            let test_text = gen_expr(test);
            
            let mut text = vec![];
            for s in statements {
                let (st, tab) = gen_statement(s, tab);
                text.push(st);
            }

            let s = text.join("\n");

            tab -= 1;

            if followed {
                (format!( "{}if {} then\n{}"
                        , " ".repeat(tab * 4)
                        , test_text
                        , s
                        ), tab)
            }
            else {
                (format!( "{}if {} then\n{}\n{}end"
                        , " ".repeat(tab * 4)
                        , test_text
                        , s
                        , " ".repeat(tab * 4)
                        ), tab)
            }
        },
        Statement::Elseif { test, statements, followed } => {
            tab += 1;
            let test_text = gen_expr(test);
            
            let mut text = vec![];
            for s in statements {
                let (st, tab) = gen_statement(s, tab);
                text.push(st);
            }

            let s = text.join("\n");

            tab -= 1;

            if followed {
                (format!( "{}elseif {} then\n{}"
                        , " ".repeat(tab * 4)
                        , test_text
                        , s
                        ), tab)
            }
            else {
                (format!( "{}elseif {} then\n{}\n{}end"
                        , " ".repeat(tab * 4)
                        , test_text
                        , s
                        , " ".repeat(tab * 4)
                        ), tab)
            }
        },
        Statement::Else(statements) => {
            tab += 1;

            let mut text = vec![];
            for s in statements {
                let (st, tab) = gen_statement(s, tab);
                text.push(st);
            }

            let s = text.join("\n");

            tab -= 1;

            (format!( "{}else\n{}\n{}end"
                    , " ".repeat(tab * 4)
                    , s
                    , " ".repeat(tab * 4)
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
        Expr::Nil => "nil".to_string(),
        _ => panic!("expr"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_handle_if() {
        let ast = vec! [ Statement::If { test: Expr::Number("0".to_string())
                                       , followed: false
                                       , statements: vec! [ Statement::Break 
                                                          , Statement::Break
                                                          ]
                                       }
                       ];
                                         
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
        let ast = vec! [ Statement::If { test: Expr::Number("0".to_string())
                                       , followed: true 
                                       , statements: vec! [ Statement::Break 
                                                          , Statement::Break
                                                          ]
                                       }
                       , Statement::Elseif { test: Expr::Number("1".to_string())
                                           , followed: false
                                           , statements: vec! [ Statement::Break, Statement::Break ]
                                           }
                       ];
                                         
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
        let ast = vec! [ Statement::If { test: Expr::Number("0".to_string())
                                       , followed: true 
                                       , statements: vec! [ Statement::Break 
                                                          , Statement::Break
                                                          ]
                                       }
                       , Statement::Elseif { test: Expr::Number("1".to_string())
                                           , followed: true 
                                           , statements: vec! [ Statement::Break, Statement::Break ]
                                           }
                       , Statement::Elseif { test: Expr::Number("2".to_string())
                                           , followed: false 
                                           , statements: vec! [ Statement::Break, Statement::Break ]
                                           }
                       ];
                                         
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
        let ast = vec! [ Statement::If { test: Expr::Number("0".to_string())
                                       , followed: true 
                                       , statements: vec! [ Statement::Break 
                                                          , Statement::Break
                                                          ]
                                       }
                       , Statement::Elseif { test: Expr::Number("1".to_string())
                                           , followed: true 
                                           , statements: vec! [ Statement::Break, Statement::Break ]
                                           }
                       , Statement::Elseif { test: Expr::Number("2".to_string())
                                           , followed: true 
                                           , statements: vec! [ Statement::Break, Statement::Break ]
                                           }
                       , Statement::Else ( 
                                            vec! [ Statement::Break, Statement::Break ]
                                         )
                       ];
                                         
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
        let ast = vec! [ Statement::If { test: Expr::Number("0".to_string())
                                       , followed: true 
                                       , statements: vec! [ Statement::Break 
                                                          , Statement::Break
                                                          ]
                                       }
                       , Statement::Else ( 
                                            vec! [ Statement::Break, Statement::Break ]
                                         )
                       ];
                                         
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
        let ast = vec! [ Statement::If { test: Expr::Number("0".to_string())
                                       , followed: false 
                                       , statements: vec! [ Statement::If { test: Expr::Number("1".to_string())
                                                                          , followed: false 
                                                                          , statements: vec! [ Statement::Break 
                                                                                             , Statement::Break
                                                                                             ]
                                                                          }
                                                          ]
                                       }
                       ];
                                         
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
