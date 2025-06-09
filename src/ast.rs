use std::boxed::Box;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub classes: Vec<Class>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: String,
    pub inherits: Option<String>,
    pub feature_list: Vec<Feature>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Feature {
    Attribute(VarDecl),
    Method(String, Vec<ArgDecl>, String, TypedExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    pub oid: String,
    pub tid: String,
    pub expr: Option<TypedExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArgDecl {
    pub id: String,
    pub tid: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CaseBranch {
    pub id: String,
    pub tid: String,
    pub expr: TypedExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Identifier(String),
    Bool(bool),
    Int(i32),
    Str(String),
    New(String),
    Block(Vec<TypedExpr>),
    Case(Box<TypedExpr>, Vec<CaseBranch>),
    Paren(Box<TypedExpr>),
    Let(Vec<(String, String, Option<TypedExpr>)>, Box<TypedExpr>),
    Comparison {
        lhs: Box<TypedExpr>,
        op: ComparisonOperator,
        rhs: Box<TypedExpr>,
    },
    Math {
        lhs: Box<TypedExpr>,
        op: MathOperator,
        rhs: Box<TypedExpr>,
    },
    UnaryOperation {
        op: UnaryOperator,
        s: Box<TypedExpr>,
    },
    Assignment(String, Box<TypedExpr>),
    Conditional {
        test: Box<TypedExpr>,
        then: Box<TypedExpr>,
        orelse: Box<TypedExpr>,
    },
    While {
        test: Box<TypedExpr>,
        exec: Box<TypedExpr>,
    },
    Isvoid(Box<TypedExpr>),
    Dispatch {
        target: Option<Box<TypedExpr>>,
        targettype: Option<String>,
        id: String,
        exprs: Vec<TypedExpr>,
    },
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

#[derive(Debug, Clone, PartialEq)]
pub struct TypedExpr {
    pub expr: Expr,
    pub static_type: Option<String>,
    pub line: usize,
}

impl TypedExpr {
    pub fn new(expr: Expr, line: usize) -> Self {
        TypedExpr {
            expr,
            static_type: None,
            line,
        }
    }
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
    pub fn new_attribute(oid: String, tid: String, init: Option<TypedExpr>) -> Self {
        Feature::Attribute(VarDecl { oid, tid, expr: init })
    }

    pub fn new_method(
        name: String,
        args: Vec<ArgDecl>,
        return_type: String,
        body: TypedExpr,
    ) -> Self {
        Feature::Method(name, args, return_type, body)
    }
}

impl VarDecl {
    pub fn new(oid: String, tid: String, expr: Option<TypedExpr>) -> Self {
        VarDecl { oid, tid, expr }
    }
}

impl ArgDecl {
    pub fn new(id: String, tid: String) -> Self {
        ArgDecl { id, tid }
    }
}

impl CaseBranch {
    pub fn new(id: String, tid: String, expr: TypedExpr) -> Self {
        CaseBranch { id, tid, expr }
    }
}
