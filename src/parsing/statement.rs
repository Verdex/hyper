
use parse_input::{Input, ParseError};
use parse_type::{Type, parse_type};

use super::proc_ast::*;


pub fn parse_statement(input : &mut Input) -> Result<Statement, ParseError> {
    input.choice( &[ parse_let
                   , parse_if
                   , parse_elseif
                   , parse_else
                   , parse_set
                   , parse_return 
                   , parse_yield
                   , parse_expr_statement
                   , parse_foreach
                   , parse_while
                   , parse_break
                   ] )
}

fn parse_elseif(input : &mut Input) -> Result<Statement, ParseError> {
    input.expect("elseif")?;
    let test = parse_expr(input)?;
    input.expect("{")?;
    let statements = input.zero_or_more(parse_statement)?;
    input.expect("}")?;
    Ok(Statement::ElseIf { test, statements })
}

fn parse_else(input : &mut Input) -> Result<Statement, ParseError> {
    input.expect("else")?;
    input.expect("{")?;
    let statements = input.zero_or_more(parse_statement)?;
    input.expect("}")?;
    Ok(Statement::Else(statements))
}

fn parse_if(input : &mut Input) -> Result<Statement, ParseError> {
    input.expect("if")?;
    let test = parse_expr(input)?;
    input.expect("{")?;
    let statements = input.zero_or_more(parse_statement)?;
    input.expect("}")?;
    Ok(Statement::If { test, statements })
}

fn parse_foreach(input : &mut Input) -> Result<Statement, ParseError> {
    input.expect("foreach")?;
    let var = input.parse_symbol()?;
    input.expect("in")?;
    let items = parse_expr(input)?;
    input.expect("{")?;
    let statements = input.zero_or_more(parse_statement)?;
    input.expect("}")?;
    Ok(Statement::Foreach { var, items, statements })
}

fn parse_while(input : &mut Input) -> Result<Statement, ParseError> {
    input.expect("while")?;
    let test = parse_expr(input)?;
    input.expect("{")?;
    let statements = input.zero_or_more(parse_statement)?;
    input.expect("}")?;
    Ok(Statement::While { test, statements })
}

fn parse_break(input : &mut Input) -> Result<Statement, ParseError> {
    input.expect("break")?;
    input.expect(";")?;
    Ok(Statement::Break)
}

fn parse_set(input : &mut Input) -> Result<Statement, ParseError> {
    input.expect("set")?;
    let target = parse_expr(input)?;
    input.expect("=")?;
    let new_value = parse_expr(input)?;
    input.expect(";")?;
    Ok(Statement::Set { target, new_value })
}

fn parse_let(input : &mut Input) -> Result<Statement, ParseError> {
    input.expect("let")?;
    let name = input.parse_symbol()?;
    match input.expect(":") {
        Ok(_) => {
            let value_type = parse_type(input)?;
            input.expect("=")?;
            let expr = parse_expr(input)?;
            input.expect(";")?;
            Ok(Statement::Let { name, value_type, expr })
        },
        Err(_) => {
            input.expect("=")?;
            let expr = parse_expr(input)?;
            input.expect(";")?;
            Ok(Statement::Let { name, value_type: Type::Infer, expr })
        },
    }
}

fn parse_expr_statement(input : &mut Input) -> Result<Statement, ParseError> {
    let expr = parse_expr(input)?;
    input.expect(";")?;
    Ok(Statement::Expr(expr))
}

fn parse_yield(input : &mut Input) -> Result<Statement, ParseError> {
    input.expect("yield")?;
    let expr = input.maybe(parse_expr);
    input.expect(";")?;
    Ok(Statement::Yield(expr))
}

fn parse_return(input : &mut Input) -> Result<Statement, ParseError> {
    input.expect("return")?;
    let expr = input.maybe(parse_expr);
    input.expect(";")?;
    Ok(Statement::Return(expr))
}

fn parse_expr(input : &mut Input) -> Result<Expr, ParseError> {
                  
    let expr = input.choice( &[ |input| Ok(Expr::Number(input.parse_number()?))
                              , |input| Ok(Expr::PString(input.parse_string()?))
                              , parse_bool
                              , parse_lambda
                              , parse_result_cons
                              , parse_struct_cons
                              , parse_variable
                              , parse_list_cons
                              , parse_paren_expr
                              ] )?;

    parse_post_expr(input, expr)
}

fn parse_list_cons(input : &mut Input) -> Result<Expr, ParseError> {
    input.expect("[")?;
    let es = input.list(parse_expr)?;
    input.expect("]")?;
    Ok(Expr::ListCons(es))
}

fn parse_result_cons(input : &mut Input) -> Result<Expr, ParseError> {
    match input.expect("Ok") {
        Ok(_) => {
            input.expect("(")?;
            let e = parse_expr(input)?;
            input.expect(")")?;
            return Ok(Expr::ResultCons(ResultValue::Okay(Box::new(e))));
        },
        Err(_) => {},
    }

    input.expect("Err")?;
    input.expect("(")?;
    let e = parse_expr(input)?;
    input.expect(")")?;
    Ok(Expr::ResultCons(ResultValue::Error(Box::new(e))))
}

fn parse_struct_cons(input : &mut Input) -> Result<Expr, ParseError> {
    fn parse_struct_slot(input : &mut Input) -> Result<StructSlot, ParseError> {
        let name = input.parse_symbol()?;
        input.expect(":")?;
        let value = parse_expr(input)?;
        Ok(StructSlot { name, value })
    }
    input.expect("new")?;
    let name = input.maybe(|i| i.parse_symbol());
    input.expect("{")?;
    let slots = input.list(parse_struct_slot)?;
    input.expect("}")?;
    Ok(Expr::StructCons { name, slots })
}

fn parse_variable(input : &mut Input) -> Result<Expr, ParseError> {
    let namespace = input.zero_or_more(|i| {
        let v = i.parse_symbol()?;
        i.expect("::")?;
        Ok(v)
    })?;

    let name = input.parse_symbol()?;
    Ok(Expr::Variable { namespace, name })
}

fn parse_paren_expr(input : &mut Input) -> Result<Expr, ParseError> {
    input.expect("(")?;
    let expr = parse_expr(input)?;
    input.expect(")")?;
    Ok(expr)
}

// dash call, call, dot, try
fn parse_post_expr(input : &mut Input, e : Expr) -> Result<Expr, ParseError> {
    match input.expect("-") {
        Ok(_) => {
            let func = input.parse_symbol()?;
            return parse_post_expr(input, Expr::Dash { object: Box::new(e), func });
        },
        Err(_) => (),
    }

    match input.expect(".") {
        Ok(_) => {
            let slot = input.parse_symbol()?;
            return parse_post_expr(input, Expr::Dot { object: Box::new(e), slot });
        },
        Err(_) => (),
    }

    match input.expect("(") {
        Ok(_) => {
            let params = input.list(parse_expr)?;

            input.expect(")")?; 
        
            return parse_post_expr(input, Expr::Call { func: Box::new(e), params });
        },
        Err(_) => (),
    }

    match input.expect("?") {
        Ok(_) => return parse_post_expr(input, Expr::Try(Box::new(e))),
        Err(_) => (),
    }

    Ok(e)
}

fn parse_lambda(input : &mut Input) -> Result<Expr, ParseError> {
    fn parse_param(input : &mut Input) -> Result<FunParam, ParseError> {
        let name = input.parse_symbol()?; 
        match input.expect(":") {
            Ok(_) => { 
                let param_type = parse_type(input)?;
                Ok(FunParam { name, param_type })
            },
            Err(_) => {
                Ok(FunParam { name, param_type: Type::Infer })
            },
        }
    }
    input.expect("|")?;
    let params = input.list(parse_param)?;
    input.expect("|")?;
    match input.expect("->") {
        Ok(_) => {
            let return_type = parse_type(input)?;
            match input.expect("{") {
                Ok(_) => {
                    let definition = input.zero_or_more(parse_statement)?;
                    input.expect("}")?;
                    Ok(Expr::StatementLambda { params, return_type, definition })
                },
                Err(_) => {
                    let definition = Box::new(parse_expr(input)?);
                    Ok(Expr::ExprLambda { params, return_type, definition })
                },
            }
        },
        Err(_) => {
            match input.expect("{") {
                Ok(_) => {
                    let definition = input.zero_or_more(parse_statement)?;
                    input.expect("}")?;
                    Ok(Expr::StatementLambda { params, return_type: Type::Infer, definition })
                },
                Err(_) => {
                    let definition = Box::new(parse_expr(input)?);
                    Ok(Expr::ExprLambda { params, return_type: Type::Infer, definition })
                },
            }
        },
    }
}

fn parse_bool(input : &mut Input) -> Result<Expr, ParseError> {
    let rp = input.create_restore();
    let value = input.parse_symbol()?;
    if value.value == "true" {
        Ok(Expr::Bool(true))
    }
    else if value.value == "false" {
        Ok(Expr::Bool(false))
    }
    else {
        input.restore(rp);
        Err(ParseError::ErrorAt(value.start, "Expected boolean".to_string()))
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_expr_lambda() -> Result<(), ParseError> {
        let i = r#"|a, b, c| 0"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = parse_lambda(&mut input)?;
        assert!( matches!( u, Expr::ExprLambda { .. } ) );
        Ok(())
    }

    #[test]
    fn should_parse_statement_lambda() -> Result<(), ParseError> {
        let i = r#"|a, b, c| { return 0; }"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = parse_lambda(&mut input)?;
        assert!( matches!( u, Expr::StatementLambda { .. } ) );
        Ok(())
    }

    #[test]
    fn should_parse_statement_lambda_with_types() -> Result<(), ParseError> {
        let i = r#"|a : A, b : B, c| -> R { return 0; }"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = parse_lambda(&mut input)?;
        assert!( matches!( u, Expr::StatementLambda { .. } ) );
        Ok(())
    }

    #[test]
    fn should_parse_expr_lambda_with_types() -> Result<(), ParseError> {
        let i = r#"|a, b : B, c : C| -> R<T> 0"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = parse_lambda(&mut input)?;
        assert!( matches!( u, Expr::ExprLambda { .. } ) );
        Ok(())
    }

    #[test]
    fn should_parse_call() -> Result<(), ParseError> {
        let i = r#"x()"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = parse_expr(&mut input)?;
        assert!( matches!( u, Expr::Call { .. } ) );
        Ok(())
    }

    #[test]
    fn should_parse_call_call() -> Result<(), ParseError> {
        let i = r#"x()()"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = parse_expr(&mut input)?;
        let call = match u {
           Expr::Call { func, .. } => *func, 
           e => panic!("expected call but found {:?}", e),
        };

        assert!( matches!( call, Expr::Call { .. } ) );
        Ok(())
    }

    #[test]
    fn should_parse_call_call_with_param() -> Result<(), ParseError> {
        let i = r#"x(a)(b)"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = parse_expr(&mut input)?;
        let (call, mut params) = match u {
           Expr::Call { func, params } => (*func, params), 
           e => panic!("expected call but found {:?}", e),
        };

        let var = match params.pop().unwrap() {
            Expr::Variable { name, .. } => name.value,
            e => panic!("expected variable but found {:?}", e),
        };

        assert_eq!( var, "b" );

        let (call, mut params) = match call {
           Expr::Call { func, params } => (*func, params), 
           e => panic!("expected call but found {:?}", e),
        };

        let var = match params.pop().unwrap() {
            Expr::Variable { name, .. } => name.value,
            e => panic!("expected variable but found {:?}", e),
        };

        assert_eq!( var, "a" );

        assert!( matches!(call, Expr::Variable { .. }) );

        Ok(())
    }

    #[test]
    fn should_parse_complex_post_expr() -> Result<(), ParseError> {
        let i = r#"(|z| z.b(5))(a)(b)?-blarg(a, b.c)()"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let _ = parse_expr(&mut input)?;
        Ok(())
    }

    #[test]
    fn should_parse_namespaced_variable() -> Result<(), ParseError> {
        let i = r#"alpha::beta::name"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = parse_expr(&mut input)?;
        let (ns, n) = match u {
            Expr::Variable { namespace, name } => (namespace, name),
            e => panic!("Expected variable but found {:?}", e),
        };
        assert_eq!( n.value, "name" );
        assert_eq!( ns.len(), 2 );
        assert_eq!( ns[0].value, "alpha" );
        assert_eq!( ns[1].value, "beta" );
        Ok(())
    }

    #[test]
    fn should_parse_ok_result_cons() -> Result<(), ParseError> {
        let i = r#"Ok(blah::ikky)"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = parse_expr(&mut input)?;
        match u {
            Expr::ResultCons(ResultValue::Okay(_)) => {},
            e => panic!("Expected ResultCons(Okay) but found {:?}", e),
        }
        Ok(())
    }

    #[test]
    fn should_parse_err_result_cons() -> Result<(), ParseError> {
        let i = r#"Err(blah::ikky)"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = parse_expr(&mut input)?;
        match u {
            Expr::ResultCons(ResultValue::Error(_)) => {},
            e => panic!("Expected ResultCons(Error) but found {:?}", e),
        }
        Ok(())
    }

    #[test]
    fn should_parse_list_cons() -> Result<(), ParseError> {
        let i = r#"[1,2,3,4]"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = parse_expr(&mut input)?;
        match u {
            Expr::ListCons(_) => {},
            e => panic!("Expected list cons but found {:?}", e),
        }
        Ok(())
    }

    #[test]
    fn should_parse_nested_list_cons() -> Result<(), ParseError> {
        let i = r#"[ [1], [1, 2], [], [4, 5] ]"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = parse_expr(&mut input)?;
        match u {
            Expr::ListCons(_) => {},
            e => panic!("Expected list cons but found {:?}", e),
        }
        Ok(())
    }

    #[test]
    fn should_parse_anon_struct() -> Result<(), ParseError> {
        let i = r#"new { blah : 5 }"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = parse_expr(&mut input)?;
        match u {
            Expr::StructCons { name: None, .. } => {},
            e => panic!("Expected anon struct cons but found {:?}", e),
        }
        Ok(())
    }

    #[test]
    fn should_parse_anon_empty_struct() -> Result<(), ParseError> {
        let i = r#"new { }"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = parse_expr(&mut input)?;
        match u {
            Expr::StructCons { name: None, .. } => {},
            e => panic!("Expected anon struct cons but found {:?}", e),
        }
        Ok(())
    }

    #[test]
    fn should_parse_named_struct() -> Result<(), ParseError> {
        let i = r#"new Blah { a: 1, b: [], c: new {} }"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = parse_expr(&mut input)?;
        match u {
            Expr::StructCons { name: Some(_), .. } => {},
            e => panic!("Expected struct cons but found {:?}", e),
        }
        Ok(())
    }
}
