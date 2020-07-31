
#[derive(Debug)]
pub struct PSym {
    pub start : usize,
    pub end : usize,
    pub value : String,
}

#[derive(Debug)]
pub enum Type {
    Unit,
    Simple(PSym),
    Index(PSym, Vec<Type>),
    Fun { input : Vec<Type>, output : Box<Type> },
    Tuple(Vec<Type>),
    Namespace(Vec<PSym>, Box<Type>),
    Infer,
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
}

#[derive(Debug)]
pub enum Statement {
    Expr(Expr),
    Return(Option<Expr>),    
    Yield(Option<Expr>),
    Let { name : PSym, value_type : Type, expr : Expr },
    Set { target : Expr, new_value : Expr },
    Break,
    Continue,
    While { test : Expr, statements : Vec<Statement> },
    Foreach { var : PSym, items : Expr, statements : Vec<Statement> },
    If { test : Expr, statements : Vec<Statement> },
    ElseIf { test : Expr, statements : Vec<Statement> },
    Else(Vec<Statement>),
    Match { target : Expr, cases : Vec<MatchCase> },
}

#[derive(Debug)]
pub struct MatchCase {
    pub pattern : MatchPattern,
    pub test : Option<Expr>,
    pub statements : Vec<Statement>,
}

#[derive(Debug)]
pub enum MatchPattern {
    Wildcard,
    Cons(PSym, Vec<MatchPattern>),
}

#[derive(Debug)]
pub struct Mod {
    pub fun_defs : Vec<FunDef>,
    pub fun_exports : Vec<String>,
}

#[derive(Debug)]
pub enum TopLevel {
    FunDef { def : FunDef, public : bool }
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

