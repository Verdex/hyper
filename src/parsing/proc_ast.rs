
use super::base_ast::*;

#[derive(Debug)]
pub enum Type {
    Unit,
    Simple(PSym),
    Index(PSym, Vec<Type>),
    Fun { input : Vec<Type>, output : Box<Type> },
    Tuple(Vec<Type>), // TODO remove
    Namespace(Vec<PSym>, Box<Type>),
    Infer,
    // TODO row type
}

#[derive(Debug)]
pub struct Use {
    pub namespace : Vec<PSym>,
    pub imports : Vec<Import>,
}

#[derive(Debug)]
pub enum Import {
    Everything,
    Item(PSym),
}

#[derive(Debug)]
pub enum Expr {
    Number(PSym),
    PString(PSym),  
    Bool(bool),
    Variable { namespace : Vec<PSym>, name : PSym },
    StatementLambda { params : Vec<FunParam>
                    , return_type : Type
                    , definition : Vec<Statement>
                    },
    ExprLambda { params : Vec<FunParam>
               , return_type : Type
               , definition : Box<Expr>
               },
    Call { func : Box<Expr>, params : Vec<Expr> },
    Try(Box<Expr>),
    Dot { object : Box<Expr>, slot : PSym },
    Dash { object : Box<Expr>, func : PSym },
    StructCons { name : Option<PSym>, slots : Vec<StructSlot> },
    ListCons(Vec<Expr>),
    ResultCons(ResultValue),
}

#[derive(Debug)]
pub enum Statement {
    Expr(Expr),
    Return(Option<Expr>),    
    Yield(Option<Expr>),
    Let { name : PSym, value_type : Type, expr : Expr },
    Set { target : Expr, new_value : Expr },
    Break,
    While { test : Expr, statements : Vec<Statement> },
    Foreach { var : PSym, items : Expr, statements : Vec<Statement> },
    If { test : Expr, statements : Vec<Statement> },
    ElseIf { test : Expr, statements : Vec<Statement> },
    Else(Vec<Statement>),
}

#[derive(Debug)]
pub struct Mod {
    pub fun_defs : Vec<FunDef>,
    pub struct_defs : Vec<StructDef>,
    pub enum_defs : Vec<EnumDef>,
    pub fun_exports : Vec<String>,
    pub struct_exports : Vec<String>,
    pub enum_exports : Vec<String>,
}

#[derive(Debug)]
pub enum TopLevel {
    FunDef { def : FunDef, public : bool },
    EnumDef { def : EnumDef, public : bool },
    StructDef { def : StructDef, public : bool },
    Import(Use),
}

#[derive(Debug)]
pub struct EnumDef {
    pub name : PSym,
    pub items : Vec<PSym>, 
}

#[derive(Debug)]
pub struct StructDef {
    pub name : PSym, 
    pub type_params : Vec<PSym>, 
    pub items : Vec<StructItem>,
}

#[derive(Debug)]
pub struct StructItem {
    pub name : PSym,
    pub item_type : Type,
}

#[derive(Debug)]
pub struct StructSlot {
    pub name : PSym,
    pub value : Expr,
}

#[derive(Debug)]
pub enum ResultValue {
    Okay(Box<Expr>),
    Error(Box<Expr>),
}

#[derive(Debug)]
pub struct FunDef {
    pub name : PSym, 
    pub type_params : Vec<PSym>, 
    pub params : Vec<FunParam>,
    pub return_type : Type,
    pub definition : Vec<Statement>,
}

#[derive(Debug)]
pub struct FunParam {
    pub name : PSym,
    pub param_type : Type,
}

