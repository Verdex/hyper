
use parse_input::{Input, ParseError};

use super::ast::*;
use super::top_level::parse_top_level;


pub fn parse(s : &str) -> Result<Mod, ParseError> {
    let ci = s.char_indices().collect::<Vec<(usize, char)>>();
    let mut input = Input::new(&ci);

    let top_level_items = input.zero_or_more(parse_top_level)?; 
    let mut fun_defs = vec![];
    let mut fun_exports = vec![];

    let mut struct_defs = vec![];
    let mut struct_exports = vec![];

    let mut enum_defs = vec![];
    let mut enum_exports = vec![];

    let mut uses = vec![];

    for item in top_level_items.into_iter() {
        match item {
            TopLevel::FunDef { def, public: true } => {
                fun_exports.push(def.name.value.clone());
                fun_defs.push(def);
            },
            TopLevel::FunDef { def, public: false } => {
                fun_defs.push(def);
            },
            TopLevel::StructDef { def, public: true } => {
                struct_exports.push(def.name.value.clone());
                struct_defs.push(def);
            },
            TopLevel::StructDef { def, public: false } => {
                struct_defs.push(def);
            },
            TopLevel::EnumDef { def, public: true } => {
                enum_exports.push(def.name.value.clone());
                enum_defs.push(def);
            },
            TopLevel::EnumDef { def, public: false } => {
                enum_defs.push(def);
            },
            TopLevel::Import(u) => {
                uses.push(u);
            },
        }
    }

    input.expect_end()?;

    Ok( Mod { fun_defs
            , fun_exports 
            , struct_defs
            , struct_exports
            , enum_defs
            , enum_exports
            , uses
            } )
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn should_parse_program() -> Result<(), ParseError> {
        let input = r#"
use import::{*};
use import::other::{*};
use import::other::{*};
use import::{First, Second, Third};

pub fun fun_name<Type, Type>( first : First, second : Second ) -> path::Type<path::Type> {
    
    return t;
}

pub enum Type { 
    Case1,
    Case2
}

pub struct Name<Type, Type> {
    first : First<Type>,
    second : fun(A, B, C) -> Res
}
"#;

        let module = parse(input)?;

        assert_eq!( module.uses.len(), 4 );
        assert_eq!( module.fun_defs.len(), 1 );
        assert_eq!( module.fun_exports.len(), 1 );
        assert_eq!( module.enum_defs.len(), 1 );
        assert_eq!( module.enum_exports.len(), 1 );
        assert_eq!( module.struct_defs.len(), 1 );
        assert_eq!( module.struct_exports.len(), 1 );

        Ok(())
    }
}
