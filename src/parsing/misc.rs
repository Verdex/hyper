
use super::base_ast::*;
use super::proc_ast::*;
use super::parse_error::ParseError;
use super::input::Input;

impl<'a> Input<'a> {

    pub fn parse_use(&mut self) -> Result<Use, ParseError> {
        fn parse_star_or_sym(input : &mut Input) -> Result<Import, ParseError> {
            match input.parse_symbol() {
                Ok(sym) => return Ok(Import::Item(sym)),
                Err(_) => (),
            }
            match input.expect("*") {
                Ok(_) => return Ok(Import::Everything),
                Err(x) => Err(x),
            }
        }

        self.expect("use")?;

        let mut namespace = vec![];

        namespace.push(self.parse_symbol()?);

        loop {
            self.expect("::")?;
            match self.expect("{") {
                Ok(_) => break,
                Err(_) => (),
            }
            namespace.push(self.parse_symbol()?);
        }
        
        let imports = self.list(parse_star_or_sym)?;

        self.expect("}")?;

        self.expect(";")?;

        Ok( Use { imports, namespace } )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sym_proj( v : &PSym ) -> String {
        match v {
            PSym { value, .. } => value.to_string(),
        }
    }

    #[test]
    fn should_parse_empty_use() -> Result<(), ParseError> {
        let i = "use symb::{};".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_use()?;
        assert_eq!( u.imports.len(), 0 );
        assert_eq!( u.namespace.len(), 1);
        assert_eq!( sym_proj(&u.namespace[0]), "symb" );
        Ok(())
    }
    
    #[test]
    fn should_parse_use_with_everything() -> Result<(), ParseError> {
        let i = "use symb::{*};".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_use()?;
        assert_eq!( u.imports.len(), 1 );
        assert!( matches!( u.imports[0], Import::Everything ) );
        assert_eq!( u.namespace.len(), 1);
        assert_eq!( sym_proj(&u.namespace[0]), "symb" );
        Ok(())
    }

    #[test]
    fn should_parse_use_with_everythings() -> Result<(), ParseError> {
        let i = "use symb::{*, *};".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_use()?;
        assert_eq!( u.imports.len(), 2 );
        assert!( matches!( u.imports[0], Import::Everything ) );
        assert!( matches!( u.imports[1], Import::Everything ) );
        assert_eq!( u.namespace.len(), 1);
        assert_eq!( sym_proj(&u.namespace[0]), "symb" );
        Ok(())
    }

    #[test]
    fn should_parse_use_with_long_namespace() -> Result<(), ParseError> {
        let i = "use symb::other::some::{*, *};".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_use()?;
        assert_eq!( u.imports.len(), 2 );
        assert!( matches!( u.imports[0], Import::Everything ) );
        assert!( matches!( u.imports[1], Import::Everything ) );
        assert_eq!( u.namespace.len(), 3);
        assert_eq!( sym_proj(&u.namespace[0]), "symb" );
        assert_eq!( sym_proj(&u.namespace[1]), "other" );
        assert_eq!( sym_proj(&u.namespace[2]), "some" );
        Ok(())
    }

    #[test]
    fn should_parse_use_with_everything_and_item() -> Result<(), ParseError> {
        let i = "use symb::other::some::{*, item};".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_use()?;
        assert_eq!( u.imports.len(), 2 );
        assert!( matches!( u.imports[0], Import::Everything ) );
        assert!( matches!( u.imports[1], Import::Item(_) ) );
        match &u.imports[1] {
            Import::Item(item) if sym_proj(&item) == "item" => (),
            _ => assert!(false),
        }
        
        assert_eq!( u.namespace.len(), 3);
        assert_eq!( sym_proj(&u.namespace[0]), "symb" );
        assert_eq!( sym_proj(&u.namespace[1]), "other" );
        assert_eq!( sym_proj(&u.namespace[2]), "some" );
        Ok(())
    }

    #[test]
    fn should_parse_use_with_items() -> Result<(), ParseError> {
        let i = "use symb::other::some::{item1, item2};".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_use()?;
        assert_eq!( u.imports.len(), 2 );
        assert!( matches!( u.imports[0], Import::Item(_) ) );
        assert!( matches!( u.imports[1], Import::Item(_) ) );
        match &u.imports[0] {
            Import::Item(item) if sym_proj(&item) == "item1" => (),
            _ => assert!(false),
        }
        match &u.imports[1] {
            Import::Item(item) if sym_proj(&item) == "item2" => (),
            _ => assert!(false),
        }
        
        assert_eq!( u.namespace.len(), 3);
        assert_eq!( sym_proj(&u.namespace[0]), "symb" );
        assert_eq!( sym_proj(&u.namespace[1]), "other" );
        assert_eq!( sym_proj(&u.namespace[2]), "some" );
        Ok(())
    }
}

