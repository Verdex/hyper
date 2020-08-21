
use parse_input::{Input, ParseError};

use super::ast::*;
use super::top_level::parse_top_level;


pub fn parse_proc(s : &str) -> Result<Mod, ParseError> {
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

}
