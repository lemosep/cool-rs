use std::boxed::Box;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub classes: Vec<Class>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub inherits: Option<String>,
    pub name: String,
    pub feature_list: Vec<Feature>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Feature {
    Attribute(VarDecl),
    Method(String, Box<Vec<ArgDecl>>, String, Box<Option<Expr>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    pub oid: String,
    pub tid: String,
    pub expr: Box<Option<Expr>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArgDecl {
    pub id: String,
    pub tid: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Identifier(String),
    Bool(bool),
    Int(i32),
    Str(String),
    New(String),
    Block(Box<Vec<Expr>>),
    Case(Box<Expr>, Box<Vec<CaseBranch>>),
    Paren(Box<Expr>),
    Let(Box<Vec<()>>, Box<Expr>),
    Comparison {
        lhs: Box<Expr>,
        op: ComparisonOperator,
        rhs: Box<Expr>,
    },
    Math {
        lhs: Box<Expr>,
        op: MathOperator,
        rhs: Box<Expr>,
    },
    UnaryOperation {
        op: UnaryOperator,
        s: Box<Expr>,
    },
    Assignment(String, Box<Expr>),
    Conditional {
        test: Box<Expr>,
        then: Box<Expr>,
        orelse: Box<Expr>,
    },
    While {
        test: Box<Expr>,
        exec: Box<Expr>,
    },
    Isvoid(Box<Expr>),
    Dispatch {
        target: Box<Option<Expr>>,
        targettype: Option<String>,
        id: String,
        exprs: Box<Vec<Expr>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct CaseBranch {
    pub id: String,
    pub tid: String,
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOperator {
    Lt,
    Le,
    Equal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MathOperator {
    Add,
    Subtract,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Neg,
    Not,
}

impl Program {
    pub fn new(classes: Vec<Class>) -> Self {
        Program { classes }
    }
}

impl Class {
    pub fn new(name: String, inherits: Option<String>, feature_list: Vec<Feature>) -> Self {
        Class {
            name,
            inherits,
            feature_list,
        }
    }
}

impl Feature {
    pub fn new_attribute(decl: VarDecl) -> Self {
        Feature::Attribute(decl)
    }

    pub fn new_method(
        name: String,
        args: Vec<ArgDecl>,
        return_type: String,
        body: Option<Expr>,
    ) -> Self {
        Feature::Method(name, Box::new(args), return_type, Box::new(body))
    }
}

impl VarDecl {
    pub fn new(oid: String, tid: String, expr: Option<Expr>) -> Self {
        VarDecl {
            oid,
            tid,
            expr: Box::new(expr),
        }
    }
}

impl ArgDecl {
    pub fn new(id: String, tid: String) -> Self {
        ArgDecl { id, tid }
    }
}