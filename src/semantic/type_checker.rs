use std::collections::HashMap;

use crate::ast::{Class, Expr, TypedExpr, VarDecl, CaseBranch, Feature, ArgDecl};
use crate::semantic::errors::SemanticError::*;
use crate::semantic::collector::ErrorCollector;
use crate::semantic::class_table::{build_class_table, ClassInfo};

/// A simple environment mapping variable names → their declared type
type TypeEnv<'a> = HashMap<String, String>;

/// Return true if this class is one of the built-in COOL types.
fn is_builtin_class(name: &str) -> bool {
    matches!(name, "Object" | "IO" | "String" | "Int" | "Bool")
}

/// Walk the inheritance chain to see if `sub` is a subtype of `sup`.
fn is_subtype(
    sub: &str,
    sup: &str,
    class_table: &HashMap<String, ClassInfo<'_>>,
) -> bool {
    // Fast check for exact match
    if sub == sup {
        return true;
    }

    // Walk upward from `sub` toward `Object`
    let mut current = sub;
    while let Some(info) = class_table.get(current) {
        if &info.parent == sup {
            return true;
        }
        if info.parent == current {
            // reached the root ("Object" → "Object"), stop
            break;
        }
        current = &info.parent;
    }

    false
}

/// Top-level: for every user-defined class (skip built-ins), check attribute initializers and method bodies.
pub fn check_expressions(classes: &[Class], ec: &mut ErrorCollector) {
    // Build class info so we can lookup attribute/method return types
    let class_table = build_class_table(classes);

    for c in classes {
        // Skip built-in classes entirely
        if is_builtin_class(&c.name) {
            continue;
        }

        // Start environment with “self : ClassName”
        let mut env = TypeEnv::new();
        env.insert("self".into(), c.name.clone());

        // 1) Check each attribute’s initializer
        for feat in &c.feature_list {
            if let Feature::Attribute(VarDecl { oid, tid, expr }) = feat {
                if let Some(init_expr) = expr.as_ref() {
                    let found = infer_expr_type(init_expr, &c.name, &env, &class_table, ec);
                    // Replace strict equality with subtype check:
                    if !is_subtype(&found, tid, &class_table) {
                        ec.add(TypeMismatch {
                            expected: tid.clone(),
                            found,
                            line: init_expr.line,
                        });
                    }
                }
                env.insert(oid.clone(), tid.clone());
            }
        }

        // 2) Check each method body
        for feat in &c.feature_list {
            if let Feature::Method(_name, args, ret_type, body) = feat {
                let mut method_env = env.clone();
                for ArgDecl { id, tid } in args.iter() {
                    method_env.insert(id.clone(), tid.clone());
                }

                let found = infer_expr_type(body, &c.name, &method_env, &class_table, ec);
                if !is_subtype(&found, ret_type, &class_table) {
                    ec.add(TypeMismatch {
                        expected: ret_type.clone(),
                        found,
                        line: body.line,
                    });
                }
            }
        }
    }
}

/// Infer the type of a `TypedExpr`. Errors are reported via `ec`.
fn infer_expr_type(
    expr: &TypedExpr,
    current_class: &str,
    env: &TypeEnv<'_>,
    class_table: &HashMap<String, ClassInfo<'_>>,
    ec: &mut ErrorCollector,
) -> String {
    match &expr.expr {
        Expr::Identifier(name) => {
            if let Some(ty) = env.get(name) {
                ty.clone()
            } else {
                ec.add(UndefinedVariable {
                    name: name.clone(),
                    line: expr.line,
                });
                "Object".into()
            }
        }
        Expr::Int(_) => "Int".into(),
        Expr::Bool(_) => "Bool".into(),
        Expr::Str(_) => "String".into(),
        Expr::New(type_name) => {
            if !class_table.contains_key(type_name) {
                ec.add(UndefinedClass {
                    type_name: type_name.clone(),
                    line: expr.line,
                });
                "Object".into()
            } else {
                type_name.clone()
            }
        }
        Expr::Assignment(var_name, rhs) => {
            let rhs_ty = infer_expr_type(rhs, current_class, env, class_table, ec);
            if let Some(var_ty) = env.get(var_name) {
                if !is_subtype(&rhs_ty, var_ty, class_table) {
                    ec.add(TypeMismatch {
                        expected: var_ty.clone(),
                        found: rhs_ty.clone(),
                        line: expr.line,
                    });
                }
                var_ty.clone()
            } else {
                ec.add(UndefinedVariable {
                    name: var_name.clone(),
                    line: expr.line,
                });
                rhs_ty
            }
        }
        Expr::Math { lhs, op: _, rhs } => {
            let lt = infer_expr_type(lhs, current_class, env, class_table, ec);
            let rt = infer_expr_type(rhs, current_class, env, class_table, ec);
            if lt != "Int" {
                ec.add(TypeMismatch {
                    expected: "Int".into(),
                    found: lt.clone(),
                    line: lhs.line,
                });
            }
            if rt != "Int" {
                ec.add(TypeMismatch {
                    expected: "Int".into(),
                    found: rt.clone(),
                    line: rhs.line,
                });
            }
            "Int".into()
        }
        Expr::Comparison { lhs, op: _, rhs } => {
            let lt = infer_expr_type(lhs, current_class, env, class_table, ec);
            let rt = infer_expr_type(rhs, current_class, env, class_table, ec);
            if lt != rt {
                ec.add(TypeMismatch {
                    expected: lt.clone(),
                    found: rt.clone(),
                    line: expr.line,
                });
            }
            "Bool".into()
        }
        Expr::UnaryOperation { op, s } => {
            let st = infer_expr_type(s, current_class, env, class_table, ec);
            match op {
                crate::ast::UnaryOperator::Neg => {
                    if st != "Int" {
                        ec.add(TypeMismatch {
                            expected: "Int".into(),
                            found: st.clone(),
                            line: s.line,
                        });
                    }
                    "Int".into()
                }
                crate::ast::UnaryOperator::Not => {
                    if st != "Bool" {
                        ec.add(TypeMismatch {
                            expected: "Bool".into(),
                            found: st.clone(),
                            line: s.line,
                        });
                    }
                    "Bool".into()
                }
            }
        }
        Expr::Dispatch { target, targettype, id, exprs } => {
            // Infer each argument
            let mut arg_types = Vec::new();
            for arg in exprs.iter() {
                arg_types.push(infer_expr_type(arg, current_class, env, class_table, ec));
            }

            // Determine which class to look up `id` in (static or dynamic)
            let mut lookup_class: &str = if let Some(tc) = targettype {
                if !class_table.contains_key(tc) {
                    ec.add(UndefinedClass {
                        type_name: tc.clone(),
                        line: expr.line,
                    });
                    "Object"
                } else {
                    tc
                }
            } else {
                if let Some(t) = target.as_ref() {
                    &infer_expr_type(t, current_class, env, class_table, ec)
                } else {
                    current_class
                }
            };

            // Walk up the inheritance chain until we find the method or hit "Object"
            let mut found_sig: Option<(&str, &Vec<&str>)> = None;
            let mut return_ty: Option<&str> = None;
            let mut expected_count = 0;

            while let Some(ci) = class_table.get(lookup_class) {
                for (mname, rtype, params) in &ci.methods {
                    if mname == id {
                        expected_count = params.len();
                        if params.len() == arg_types.len() {
                            found_sig = Some((rtype, params));
                            return_ty = Some(rtype);
                        }
                        break;
                    }
                }
                if found_sig.is_some() {
                    break;
                }
                if lookup_class == &ci.parent {
                    // reached root ("Object" → "Object")
                    break;
                }
                lookup_class = &ci.parent;
            }

            if let Some((rtype, param_list)) = found_sig {
                for (idx, actual) in arg_types.iter().enumerate() {
                    let expected_ty = param_list[idx];
                    if !is_subtype(actual, expected_ty, class_table) {
                        ec.add(TypeMismatch {
                            expected: expected_ty.to_string(),
                            found: actual.clone(),
                            line: expr.line,
                        });
                    }
                }
                return_ty.unwrap().to_string()
            } else {
                ec.add(ArgumentCountMismatch {
                    method: id.clone(),
                    expected: expected_count,
                    found: arg_types.len(),
                    line: expr.line,
                });
                "Object".into()
            }
        }
        Expr::Conditional { test, then, orelse } => {
            let t1 = infer_expr_type(test, current_class, env, class_table, ec);
            if t1 != "Bool" {
                ec.add(TypeMismatch {
                    expected: "Bool".into(),
                    found: t1.clone(),
                    line: test.line,
                });
            }
            let t2 = infer_expr_type(then, current_class, env, class_table, ec);
            let t3 = infer_expr_type(orelse, current_class, env, class_table, ec);
            if !is_subtype(&t3, &t2, class_table) && !is_subtype(&t2, &t3, class_table) {
                // If branches do not share a common subtype relationship, report mismatch
                ec.add(TypeMismatch {
                    expected: t2.clone(),
                    found: t3.clone(),
                    line: expr.line,
                });
            }
            // The result is the least common ancestor, but for now pick one:
            if is_subtype(&t2, &t3, class_table) {
                t3
            } else if is_subtype(&t3, &t2, class_table) {
                t2
            } else {
                "Object".into()
            }
        }
        Expr::While { test, exec } => {
            let t1 = infer_expr_type(test, current_class, env, class_table, ec);
            if t1 != "Bool" {
                ec.add(TypeMismatch {
                    expected: "Bool".into(),
                    found: t1.clone(),
                    line: test.line,
                });
            }
            let _ = infer_expr_type(exec, current_class, env, class_table, ec);
            "Object".into()
        }
        Expr::Isvoid(inner) => {
            let _ = infer_expr_type(inner, current_class, env, class_table, ec);
            "Bool".into()
        }
        Expr::Block(exprs) => {
            let mut last = "Object".into();
            for e in exprs.iter() {
                last = infer_expr_type(e, current_class, env, class_table, ec);
            }
            last
        }
        Expr::Let(bindings, body) => {
            let mut new_env = env.clone();
            for (id, typeid, init_opt) in bindings.iter() {
                if let Some(init_expr) = init_opt {
                    let found =
                        infer_expr_type(init_expr, current_class, &new_env, class_table, ec);
                    if !is_subtype(&found, typeid, class_table) {
                        ec.add(TypeMismatch {
                            expected: typeid.clone(),
                            found: found.clone(),
                            line: init_expr.line,
                        });
                    }
                }
                new_env.insert(id.clone(), typeid.clone());
            }
            infer_expr_type(body, current_class, &new_env, class_table, ec)
        }
        Expr::Case(expr, branches) => {
            let t_expr = infer_expr_type(expr, current_class, env, class_table, ec);
            if t_expr == "Object" {
                ec.add(CaseOnVoid { line: expr.line });
            }
            let mut result_type = "Object".to_string();
            for CaseBranch { id, tid, expr: br_expr } in branches.iter() {
                if !class_table.contains_key(tid) {
                    ec.add(UndefinedClass {
                        type_name: tid.clone(),
                        line: br_expr.line,
                    });
                }
                let mut branch_env = env.clone();
                branch_env.insert(id.clone(), tid.clone());
                let t_branch =
                    infer_expr_type(br_expr, current_class, &branch_env, class_table, ec);

                // Compute “join” of result_type and t_branch
                if is_subtype(&t_branch, &result_type, class_table) {
                    // t_branch ≤ result_type ⇒ keep result_type
                } else if is_subtype(&result_type, &t_branch, class_table) {
                    result_type = t_branch;
                } else {
                    // No direct subtype relationship ⇒ fallback to Object
                    result_type = "Object".to_string();
                }
            }
            result_type
        }
        Expr::Paren(inner) => infer_expr_type(inner, current_class, env, class_table, ec),
    }
}
