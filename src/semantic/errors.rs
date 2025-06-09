use std::fmt;

#[derive(Debug)]
pub enum SemanticError {
    // Inheritance graph errors
    DuplicateClass { class: String },
    InheritanceCycle { cycle: Vec<String> },
    UndefinedParent { class: String, parent: String },
    InheritBasicType { class: String, parent: String },

    // Attribute/method errors
    DuplicateAttribute { class: String, attr: String },
    DuplicateMethod { class: String, method: String },
    MethodOverrideMismatch {
        class: String,
        method: String,
        parent: String,
        expected: Vec<String>,
        found: Vec<String>,
    },

    // Type errors in expressions
    UndefinedClass { type_name: String, line: usize },
    UndefinedVariable { name: String, line: usize },
    TypeMismatch {
        expected: String,
        found: String,
        line: usize,
    },
    ArgumentCountMismatch {
        method: String,
        expected: usize,
        found: usize,
        line: usize,
    },
    DispatchOnVoid { line: usize },
    CaseOnVoid { line: usize },
    NoBranchInCase { expr_type: String, line: usize },
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use SemanticError::*;
        match self {
            DuplicateClass { class } => write!(f, "Duplicate class '{}'", class),
            InheritanceCycle { cycle } => {
                write!(f, "Inheritance cycle detected: {}", cycle.join(" → "))
            }
            UndefinedParent { class, parent } => {
                write!(f, "Class '{}' inherits from undefined parent '{}'", class, parent)
            }
            InheritBasicType { class, parent } => {
                write!(f, "Class '{}' cannot inherit from basic type '{}'", class, parent)
            }
            DuplicateAttribute { class, attr } => {
                write!(f, "In class '{}', attribute '{}' is duplicated", class, attr)
            }
            DuplicateMethod { class, method } => {
                write!(f, "In class '{}', method '{}' is duplicated", class, method)
            }
            MethodOverrideMismatch { class, method, parent, expected, found } => write!(
                f,
                "Invalid override of method '{}' in '{}': parent '{}' signature = {:?}, found = {:?}",
                method, class, parent, expected, found
            ),
            UndefinedClass { type_name, line } => {
                write!(f, "[line {}] Type '{}' is not defined", line, type_name)
            }
            UndefinedVariable { name, line } => {
                write!(f, "[line {}] Variable '{}' is not declared", line, name)
            }
            TypeMismatch { expected, found, line } => {
                write!(
                    f,
                    "[line {}] Type mismatch: expected '{}', found '{}'",
                    line, expected, found
                )
            }
            ArgumentCountMismatch { method, expected, found, line } => write!(
                f,
                "[line {}] Method '{}' expects {} arguments, but {} were given",
                line, method, expected, found
            ),
            DispatchOnVoid { line } => {
                write!(f, "[line {}] Dispatch on void reference", line)
            }
            CaseOnVoid { line } => {
                write!(f, "[line {}] Case on void expression", line)
            }
            NoBranchInCase { expr_type, line } => write!(
                f,
                "[line {}] No 'case' branch for dynamic type '{}'",
                line, expr_type
            ),
        }
    }
}
