
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;

impl<'a> Input<'a> {

    // TODO parse tuple in expr

    pub fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        self.choice( &[ |input| input.parse_let() 
                      , |input| input.parse_match()
                      , |input| input.parse_if()
                      , |input| input.parse_elseif()
                      , |input| input.parse_else()
                      , |input| input.parse_set()
                      , |input| input.parse_return() 
                      , |input| input.parse_yield()
                      , |input| input.parse_expr_statement()
                      , |input| input.parse_foreach()
                      , |input| input.parse_while()
                      , |input| input.parse_break()
                      , |input| input.parse_continue()
                      ] )
    }

    fn parse_match(&mut self) -> Result<Statement, ParseError> {
        self.expect("match")?;
        let target = self.parse_expr()?;
        self.expect("{")?;
        let cases = self.zero_or_more(|input| input.parse_case())?;
        self.expect("}")?;
        Ok(Statement::Match { target, cases })
    }

    fn parse_case(&mut self) -> Result<MatchCase, ParseError> {
        let pattern = self.parse_pattern()?;

        let test = self.maybe(|input| {
            input.expect("if")?;
            input.parse_expr()
        });

        self.expect("=>")?;
        match self.expect("{") {
            Ok(_) => {
                let statements = self.zero_or_more(|input| input.parse_statement())?; 
                self.expect("}")?;
                Ok(MatchCase { pattern, test, statements })
            },
            Err(_) => {
                let statements = vec![self.parse_statement()?];
                Ok(MatchCase { pattern, test, statements })
            },
        }
    }

    fn parse_pattern(&mut self) -> Result<MatchPattern, ParseError> {
        let sym = self.parse_symbol()?;

        if sym.value == "_" {
            return Ok(MatchPattern::Wildcard);
        }
        
        match self.expect("(") {
            Ok(_) => {
                let items = self.list(|input| input.parse_pattern())?;
                self.expect(")")?;
                Ok(MatchPattern::Cons(sym, items))
            }
            Err(_) => Ok(MatchPattern::Cons(sym, vec![])), 
        }
    }

    fn parse_elseif(&mut self) -> Result<Statement, ParseError> {
        self.expect("elseif")?;
        let test = self.parse_expr()?;
        self.expect("{")?;
        let statements = self.zero_or_more(|input| input.parse_statement())?;
        self.expect("}")?;
        Ok(Statement::ElseIf { test, statements })
    }

    fn parse_else(&mut self) -> Result<Statement, ParseError> {
        self.expect("else")?;
        self.expect("{")?;
        let statements = self.zero_or_more(|input| input.parse_statement())?;
        self.expect("}")?;
        Ok(Statement::Else(statements))
    }

    fn parse_if(&mut self) -> Result<Statement, ParseError> {
        self.expect("if")?;
        let test = self.parse_expr()?;
        self.expect("{")?;
        let statements = self.zero_or_more(|input| input.parse_statement())?;
        self.expect("}")?;
        Ok(Statement::If { test, statements })
    }

    fn parse_foreach(&mut self) -> Result<Statement, ParseError> {
        self.expect("foreach")?;
        let var = self.parse_symbol()?;
        self.expect("in")?;
        let items = self.parse_expr()?;
        self.expect("{")?;
        let statements = self.zero_or_more(|input| input.parse_statement())?;
        self.expect("}")?;
        Ok(Statement::Foreach { var, items, statements })
    }

    fn parse_while(&mut self) -> Result<Statement, ParseError> {
        self.expect("while")?;
        let test = self.parse_expr()?;
        self.expect("{")?;
        let statements = self.zero_or_more(|input| input.parse_statement())?;
        self.expect("}")?;
        Ok(Statement::While { test, statements })
    }

    fn parse_continue(&mut self) -> Result<Statement, ParseError> {
        self.expect("continue")?;
        self.expect(";")?;
        Ok(Statement::Continue)
    }

    fn parse_break(&mut self) -> Result<Statement, ParseError> {
        self.expect("break")?;
        self.expect(";")?;
        Ok(Statement::Break)
    }

    fn parse_set(&mut self) -> Result<Statement, ParseError> {
        self.expect("set")?;
        let target = self.parse_expr()?;
        self.expect("=")?;
        let new_value = self.parse_expr()?;
        self.expect(";")?;
        Ok(Statement::Set { target, new_value })
    }

    fn parse_let(&mut self) -> Result<Statement, ParseError> {
        self.expect("let")?;
        let name = self.parse_symbol()?;
        match self.expect(":") {
            Ok(_) => {
                let value_type = self.parse_type()?;
                self.expect("=")?;
                let expr = self.parse_expr()?;
                self.expect(";")?;
                Ok(Statement::Let { name, value_type, expr })
            },
            Err(_) => {
                self.expect("=")?;
                let expr = self.parse_expr()?;
                self.expect(";")?;
                Ok(Statement::Let { name, value_type: Type::Infer, expr })
            },
        }
    }

    fn parse_expr_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.parse_expr()?;
        self.expect(";")?;
        Ok(Statement::Expr(expr))
    }

    fn parse_yield(&mut self) -> Result<Statement, ParseError> {
        self.expect("yield")?;
        let expr = self.maybe( |input| input.parse_expr() );
        self.expect(";")?;
        Ok(Statement::Yield(expr))
    }

    fn parse_return(&mut self) -> Result<Statement, ParseError> {
        self.expect("return")?;
        let expr = self.maybe( |input| input.parse_expr() );
        self.expect(";")?;
        Ok(Statement::Return(expr))
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
                      
        let expr = self.choice( &[ |input| Ok(Expr::Number(input.parse_number()?))
                                 , |input| Ok(Expr::PString(input.parse_string()?))
                                 , |input| input.parse_bool()
                                 , |input| input.parse_lambda()
                                 , |input| input.parse_variable() 
                                 , |input| input.parse_paren_expr()
                                 ] )?;

        self.parse_post_expr(expr)
    }

    fn parse_variable(&mut self) -> Result<Expr, ParseError> {
        let namespace = self.zero_or_more(|input| {
            let v = input.parse_symbol()?;
            input.expect("::")?;
            Ok(v)
        })?;

        let name = self.parse_symbol()?;
        Ok(Expr::Variable { namespace, name })
    }

    fn parse_paren_expr(&mut self) -> Result<Expr, ParseError> {
        self.expect("(")?;
        let expr = self.parse_expr()?;
        self.expect(")")?;
        Ok(expr)
    }

    // dash call, call, dot, try
    fn parse_post_expr(&mut self, e : Expr) -> Result<Expr, ParseError> {
        match self.expect("-") {
            Ok(_) => {
                let func = self.parse_symbol()?;
                return self.parse_post_expr(Expr::Dash { object: Box::new(e), func });
            },
            Err(_) => (),
        }

        match self.expect(".") {
            Ok(_) => {
                let slot = self.parse_symbol()?;
                return self.parse_post_expr(Expr::Dot { object: Box::new(e), slot });
            },
            Err(_) => (),
        }

        match self.expect("(") {
            Ok(_) => {
                let params = self.list(|input| input.parse_expr())?;

                self.expect(")")?; 
            
                return self.parse_post_expr(Expr::Call { func: Box::new(e), params });
            },
            Err(_) => (),
        }

        match self.expect("?") {
            Ok(_) => return self.parse_post_expr(Expr::Try(Box::new(e))),
            Err(_) => (),
        }

        Ok(e)
    }

    fn parse_lambda(&mut self) -> Result<Expr, ParseError> {
        fn parse_param(input : &mut Input) -> Result<FunParam, ParseError> {
            let name = input.parse_symbol()?; 
            match input.expect(":") {
                Ok(_) => { 
                    let param_type = input.parse_type()?;
                    Ok(FunParam { name, param_type })
                },
                Err(_) => {
                    Ok(FunParam { name, param_type: Type::Infer })
                },
            }
        }
        self.expect("|")?;
        let params = self.list(parse_param)?;
        self.expect("|")?;
        match self.expect("->") {
            Ok(_) => {
                let return_type = self.parse_type()?;
                match self.expect("{") {
                    Ok(_) => {
                        let definition = self.zero_or_more(|input| input.parse_statement())?;
                        self.expect("}")?;
                        Ok(Expr::StatementLambda { params, return_type, definition })
                    },
                    Err(_) => {
                        let definition = Box::new(self.parse_expr()?);
                        Ok(Expr::ExprLambda { params, return_type, definition })
                    },
                }
            },
            Err(_) => {
                match self.expect("{") {
                    Ok(_) => {
                        let definition = self.zero_or_more(|input| input.parse_statement())?;
                        self.expect("}")?;
                        Ok(Expr::StatementLambda { params, return_type: Type::Infer, definition })
                    },
                    Err(_) => {
                        let definition = Box::new(self.parse_expr()?);
                        Ok(Expr::ExprLambda { params, return_type: Type::Infer, definition })
                    },
                }
            },
        }
    }

    fn parse_bool(&mut self) -> Result<Expr, ParseError> {
        let rp = self.create_restore();
        let value = self.parse_symbol()?;
        if value.value == "true" {
            Ok(Expr::Bool(true))
        }
        else if value.value == "false" {
            Ok(Expr::Bool(false))
        }
        else {
            self.restore(rp);
            Err(ParseError::ErrorAt(value.start, "Expected boolean".to_string()))
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_expr_lambda() -> Result<(), ParseError> {
        let i = r#"|a, b, c| 0"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_lambda()?;
        assert!( matches!( u, Expr::ExprLambda { .. } ) );
        Ok(())
    }

    #[test]
    fn should_parse_statement_lambda() -> Result<(), ParseError> {
        let i = r#"|a, b, c| { return 0; }"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_lambda()?;
        assert!( matches!( u, Expr::StatementLambda { .. } ) );
        Ok(())
    }

    #[test]
    fn should_parse_statement_lambda_with_types() -> Result<(), ParseError> {
        let i = r#"|a : A, b : B, c| -> R { return 0; }"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_lambda()?;
        assert!( matches!( u, Expr::StatementLambda { .. } ) );
        Ok(())
    }

    #[test]
    fn should_parse_expr_lambda_with_types() -> Result<(), ParseError> {
        let i = r#"|a, b : B, c : C| -> R<T> 0"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_lambda()?;
        assert!( matches!( u, Expr::ExprLambda { .. } ) );
        Ok(())
    }

    #[test]
    fn should_parse_call() -> Result<(), ParseError> {
        let i = r#"x()"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_expr()?;
        assert!( matches!( u, Expr::Call { .. } ) );
        Ok(())
    }

    #[test]
    fn should_parse_call_call() -> Result<(), ParseError> {
        let i = r#"x()()"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_expr()?;
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
        let u = input.parse_expr()?;
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
        let _ = input.parse_expr()?;
        Ok(())
    }

    #[test]
    fn should_parse_empty_match() -> Result<(), ParseError> {
        let i = r#"
match a {

}"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let m = input.parse_match()?;
        assert!( matches!( m, Statement::Match{..} ) );
        Ok(())
    }

    #[test]
    fn should_parse_match() -> Result<(), ParseError> {
        let i = r#"
match a.b()-c() {
    A(b) if true => {
        blah();
        ikky();
    }
    A(B(c,d,e), G, E(_)) => a();
    _ if false => return 8;
}"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let m = input.parse_match()?;
        assert!( matches!( m, Statement::Match{..} ) );
        Ok(())
    }

    #[test]
    fn should_parse_namespaced_variable() -> Result<(), ParseError> {
        let i = r#"alpha::beta::name"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_expr()?;
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
}
