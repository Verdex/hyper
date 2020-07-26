
pub enum Expr {
    Nil,
    Number(String),
    String(String),
    Bool(bool),
    Var(String),
    TableCons(Vec<InlineTableAssign>),
    TableAccess { expr : Box<Expr>, slot : String },
    ListCons(Vec<Expr>),
    ListAccess { expr : Box<Expr>, index : Box<Expr> },
    FunCall { fun : Box<Expr>, params : Vec<Expr> },
    Lambda { params : Vec<String>, statements : Vec<Statement> },
    Paren(Box<Expr>),
    CallSystemFun { fun : String, params : Vec<Expr> },
    CallBinFun { fun : String, a : Box<Expr>, b : Box<Expr> },
    CallUniFun { fun : String, a : Box<Expr> },
}

pub struct InlineTableAssign {
    pub key : String,
    pub value : Expr,
}

pub enum Statement {
    LocalVarDeclare(String),
    Break,
    Return(Vec<Expr>),
    If { if_statements : Vec<If>, else_statement : Vec<Statement> },
    AssignVar { vars : Vec<String>, exprs : Vec<Expr> },
    AssignListAccess { target : Expr, index : Expr, new_value : Expr },
    AssignTableAccess { target : Expr, slot : String, new_value : Expr },
    While { test : Expr, statements : Vec<Statement> },
    Repeat { test : Expr, statements : Vec<Statement> },
    For { vars : Vec<String>, iterator : Expr, statements : Vec<Statement> },
    ForI { var : String, start : Expr, end : Expr, increment : Option<Expr>, statements : Vec<Statement> },
    FunCall { fun : Expr, params : Vec<Expr> },
    CallSystemFun { fun : String, params : Vec<Expr> },
}


pub struct If { 
    pub test : Expr, 
    pub statements : Vec<Statement>, 
}


