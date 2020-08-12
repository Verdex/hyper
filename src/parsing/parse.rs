
use super::proc_ast::*;
use super::parse_error::ParseError;
use super::input::Input;


pub fn parse_proc(s : &str) -> Result<Mod, ParseError> {
    let ci = s.char_indices().collect::<Vec<(usize, char)>>();
    let mut input = Input::new(&ci);
    // TODO if everything fails then the zero or more is going to return no errors, which means we won't be able to show the 
    // user any sort of errors
    let top_level_items = input.zero_or_more(|i| i.parse_top_level() )?; 
    let mut fun_defs = vec![];
    let mut fun_exports = vec![];

    let mut struct_defs = vec![];
    let mut struct_exports = vec![];

    let mut enum_defs = vec![];
    let mut enum_exports = vec![];
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
            _ => panic!("missing case"),
        }
    }
    Ok( Mod { fun_defs
            , fun_exports 
            , struct_defs
            , struct_exports
            , enum_defs
            , enum_exports
            } )
    // TODO make sure we make sure we've consumed the entire input
}


#[cfg(test)]
mod test {
    use super::*;

}
