
use super::base_ast::*;
use super::proc_ast::*;
use super::parse_error::ParseError;
use super::input::Input;

impl<'a> Input<'a> {

    pub fn parse_top_level(&mut self) -> Result<TopLevel, ParseError> {
        // TODO pub?
        let x = self.parse_fun_def()?;
        //Ok( TopLevel::FunDef { def: self.parse_fun_def()?, public: true } )
        Ok( TopLevel::FunDef { def: x, public: true } )
    }

    fn parse_enum_def(&mut self) -> Result<EnumDef, ParseError> {
        self.expect("enum")?;
        let name = self.parse_symbol()?;
        self.expect("{")?;
        let items = self.list(|input| input.parse_symbol())?;
        self.expect("}")?;
        Ok(EnumDef { name, items })
    }

    fn parse_struct_def(&mut self) -> Result<StructDef, ParseError> {
        fn to_vec<T>( o : Option<Vec<T>> ) -> Vec<T> {
            match o {
                Some(v) => v,
                None => vec![],
            }
        }

        self.expect("struct")?;
        let name = self.parse_symbol()?;
        let type_params = to_vec(self.maybe(|input| {
            input.expect("<")?;
            let params = input.list(|i| i.parse_symbol())?;
            input.expect(">")?;
            Ok(params)
        }));
        
        self.expect("{")?; 
        
        let items = self.list(|input| {
            let name = input.parse_symbol()?;
            input.expect(":")?;
            let item_type = input.parse_type()?;
            Ok( StructItem { name, item_type } )
        })?;

        self.expect("}")?; 
        Ok( StructDef { name, type_params, items } ) 
    }

    fn parse_fun_def(&mut self) -> Result<FunDef, ParseError> {
        fn parse_param(input : &mut Input) -> Result<FunParam, ParseError> {
            let name = input.parse_symbol()?; 
            input.expect(":")?;
            let param_type = input.parse_type()?;
            Ok(FunParam { name, param_type })
        }

        self.expect("fun")?;
        
        let name = self.parse_symbol()?;

        match self.maybe(|input| input.expect("<")) {
            Some(_) => {
                let type_params = self.list(|input| input.parse_symbol())?;
                self.expect(">")?;
                self.expect("(")?;
                let params = self.list(parse_param)?;
                self.expect(")")?;
                match self.expect("->") {
                    Ok(_) => {
                        let return_type = self.parse_type()?;
                        self.expect("{")?;
                        let definition = self.zero_or_more(|input| input.parse_statement())?;
                        self.expect("}")?;
                        Ok( FunDef { name, type_params, params, return_type, definition } )
                    },
                    Err(_) => { 
                        self.expect("{")?;
                        let definition = self.zero_or_more(|input| input.parse_statement())?;
                        self.expect("}")?;
                        Ok( FunDef { name, type_params, params, return_type: Type::Unit, definition } )
                    }, 
                }

            },
            None => {
                self.expect("(")?;
                let params = self.list(parse_param)?;
                self.expect(")")?;
                match self.expect("->") {
                    Ok(_) => {
                        let return_type = self.parse_type()?;
                        self.expect("{")?;
                        let definition = self.zero_or_more(|input| input.parse_statement())?;
                        self.expect("}")?;
                        Ok( FunDef { name, type_params: vec![], params, return_type, definition } )
                    },
                    Err(_) => { 
                        self.expect("{")?;
                        let definition = self.zero_or_more(|input| input.parse_statement())?;
                        self.expect("}")?;
                        Ok( FunDef { name, type_params: vec![], params, return_type: Type::Unit, definition } )
                    }, 
                }
            },
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_fun_def() -> Result<(), ParseError> {
        let i = r#"
fun blah() {
    return 0;
}"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_fun_def()?;
        
        assert_eq!( u.name.value, "blah" );
        assert_eq!( u.type_params.len(), 0 );
        assert_eq!( u.params.len(), 0 );
        assert_eq!( u.definition.len(), 1 );
        assert!( matches!( u.return_type, Type::Unit ) );

        Ok(())
    }
    
    #[test]
    fn should_parse_fun_def_extras() -> Result<(), ParseError> {
        let i = r#"
fun blah<a,b,c>( a : b, c : d ) -> number {
    return 0;
}"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_fun_def()?;
        
        assert_eq!( u.name.value, "blah" );
        assert_eq!( u.type_params.len(), 3 );
        assert_eq!( u.params.len(), 2 );
        assert_eq!( u.definition.len(), 1 );
        
        let return_name = match u.return_type {
            Type::Simple(n) => n,
            e => panic!( "expected Type::Simple but found {:?}", e ),
        };

        assert_eq!( return_name.value, "number" );

        Ok(())
    }

    #[test]
    fn should_parse_enum() -> Result<(), ParseError> {
        let i = r#"
enum blah {
    One,
    Two, 
    Three
}"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_enum_def()?;
        
        assert_eq!( u.name.value, "blah" );
        assert_eq!( u.items.len(), 3 );
        assert_eq!( u.items[0].value, "One" ); 
        assert_eq!( u.items[1].value, "Two" ); 
        assert_eq!( u.items[2].value, "Three" ); 

        Ok(())
    }

    #[test]
    fn should_parse_struct() -> Result<(), ParseError> {
        let i = r#"
struct blah {
    a : b,
    c : d
}"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_struct_def()?;
        
        assert_eq!( u.name.value, "blah" );
        assert_eq!( u.type_params.len(), 0 );
        assert_eq!( u.items.len(), 2 );

        Ok(())
    }

    #[test]
    fn should_parse_struct_with_type_params() -> Result<(), ParseError> {
        let i = r#"
struct blah<a,b,c> {
    a : b,
    c : d
}"#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_struct_def()?;
        
        assert_eq!( u.name.value, "blah" );
        assert_eq!( u.items.len(), 2 );
        assert_eq!( u.type_params.len(), 3 );
        assert_eq!( u.type_params[0].value, "a" );
        assert_eq!( u.type_params[1].value, "b" );
        assert_eq!( u.type_params[2].value, "c" );

        Ok(())
    }
}
