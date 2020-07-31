
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;

impl<'a> Input<'a> {

    pub fn parse_top_level(&mut self) -> Result<TopLevel, ParseError> {
        // TODO pub?
        let x = self.parse_fun_def()?;
        //Ok( TopLevel::FunDef { def: self.parse_fun_def()?, public: true } )
        Ok( TopLevel::FunDef { def: x, public: true } )
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

}
