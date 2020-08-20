
use parse_input::{Input, ParseError};
use parse_type::{Type, parse_type};
use super::statement::parse_statement;
use super::proc_ast::*;


pub fn parse_top_level(input : &mut Input) -> Result<TopLevel, ParseError> {
    fn t<T>( v : Option<T> ) -> bool {
        match v {
            Some(_) => true,
            None => false,
        }
    }

    match parse_use(input) {
        Ok(v) => return Ok(TopLevel::Import(v)),
        Err(_) => { },
    }

    let public = input.maybe(|i| i.expect("pub"));
    
    match parse_fun_def(input) {
        Ok(def) => return Ok(TopLevel::FunDef{ def, public: t(public) }),
        Err(_) => { },
    }

    match parse_struct_def(input) {
        Ok(def) => return Ok(TopLevel::StructDef { def, public: t(public) }),
        Err(_) => { },
    }

    match parse_enum_def(input) {
        Ok(def) => Ok(TopLevel::EnumDef { def, public: t(public) }),
        Err(e) => Err(e),
    }
}

fn parse_use(input : &mut Input) -> Result<Use, ParseError> {
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

    input.expect("use")?;

    let mut namespace = vec![];

    namespace.push(input.parse_symbol()?);

    loop {
        input.expect("::")?;
        match input.expect("{") {
            Ok(_) => break,
            Err(_) => (),
        }
        namespace.push(input.parse_symbol()?);
    }
    
    let imports = input.list(parse_star_or_sym)?;

    input.expect("}")?;

    input.expect(";")?;

    Ok( Use { imports, namespace } )
}

fn parse_enum_def(input : &mut Input) -> Result<EnumDef, ParseError> {
    input.expect("enum")?;
    let name = input.parse_symbol()?;
    input.expect("{")?;
    let items = input.list(|i| i.parse_symbol())?;
    input.expect("}")?;
    Ok(EnumDef { name, items })
}

fn parse_struct_def(input : &mut Input) -> Result<StructDef, ParseError> {
    fn to_vec<T>( o : Option<Vec<T>> ) -> Vec<T> {
        match o {
            Some(v) => v,
            None => vec![],
        }
    }

    input.expect("struct")?;
    let name = input.parse_symbol()?;
    let type_params = to_vec(input.maybe(|i| {
        i.expect("<")?;
        let params = i.list(|ii| ii.parse_symbol())?;
        i.expect(">")?;
        Ok(params)
    }));
    
    input.expect("{")?; 
    
    let items = input.list(|i| {
        let name = i.parse_symbol()?;
        i.expect(":")?;
        let item_type = parse_type(i)?;
        Ok( StructItem { name, item_type } )
    })?;

    input.expect("}")?; 
    Ok( StructDef { name, type_params, items } ) 
}

fn parse_fun_def(input : &mut Input) -> Result<FunDef, ParseError> {
    fn parse_param(input : &mut Input) -> Result<FunParam, ParseError> {
        let name = input.parse_symbol()?; 
        input.expect(":")?;
        let param_type = parse_type(input)?;
        Ok(FunParam { name, param_type })
    }

    input.expect("fun")?;
    
    let name = input.parse_symbol()?;

    match input.maybe(|i| i.expect("<")) {
        Some(_) => {
            let type_params = input.list(|i| i.parse_symbol())?;
            input.expect(">")?;
            input.expect("(")?;
            let params = input.list(parse_param)?;
            input.expect(")")?;
            match input.expect("->") {
                Ok(_) => {
                    let return_type = parse_type(input)?;
                    input.expect("{")?;
                    let definition = input.zero_or_more(parse_statement)?;
                    input.expect("}")?;
                    Ok( FunDef { name, type_params, params, return_type, definition } )
                },
                Err(_) => { 
                    input.expect("{")?;
                    let definition = input.zero_or_more(parse_statement)?;
                    input.expect("}")?;
                    Ok( FunDef { name, type_params, params, return_type: Type::Unit, definition } )
                }, 
            }

        },
        None => {
            input.expect("(")?;
            let params = input.list(parse_param)?;
            input.expect(")")?;
            match input.expect("->") {
                Ok(_) => {
                    let return_type = parse_type(input)?;
                    input.expect("{")?;
                    let definition = input.zero_or_more(parse_statement)?;
                    input.expect("}")?;
                    Ok( FunDef { name, type_params: vec![], params, return_type, definition } )
                },
                Err(_) => { 
                    input.expect("{")?;
                    let definition = input.zero_or_more(parse_statement)?;
                    input.expect("}")?;
                    Ok( FunDef { name, type_params: vec![], params, return_type: Type::Unit, definition } )
                }, 
            }
        },
    }
}


#[cfg(test)]
mod test {
    use super::*;

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
