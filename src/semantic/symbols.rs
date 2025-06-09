use std::collections::HashMap;

use crate::ast::{ArgDecl, Class, Feature, VarDecl};
use crate::semantic::errors::SemanticError::*;
use crate::semantic::collector::ErrorCollector;
use crate::semantic::class_table::build_class_table;

fn is_builtin_class(name: &str) -> bool {
    matches!(name, "Object" | "IO" | "String" | "Int" | "Bool")
}

/// # Description
/// 
/// Given a slice of AST‐classes, build attribute/method symbol tables
/// and detect:
///  - DuplicateAttribute, DuplicateMethod
///  - MethodOverrideMismatch
pub fn check_class_features(classes: &[Class], ec: &mut ErrorCollector) {
    // First, build an empty class table
    let mut class_table = build_class_table(classes);

    // 1) Fill in attributes and methods for each class
    for c in classes {
        if is_builtin_class(&c.name) {
            continue;
        }
        let info = class_table.get_mut(&c.name).unwrap();

        // Keep local attribute & method lookups to detect duplicates
        let mut attrs_seen = HashMap::new();
        let mut methods_seen = HashMap::new();

        for feat in &c.feature_list {
            match feat {
                Feature::Attribute(vd) => {
                    let VarDecl { oid, tid, .. } = vd;
                    if attrs_seen.insert(oid.clone(), ()) .is_some() {
                        ec.add(DuplicateAttribute {
                            class: c.name.clone(),
                            attr: oid.clone(),
                        });
                    } else {
                        info.attributes.push((oid.as_str(), tid.as_str()));
                    }
                }
                Feature::Method(name, args, ret_type, _body_opt) => {
                    if methods_seen.insert(name.clone(), ()).is_some() {
                        ec.add(DuplicateMethod {
                            class: c.name.clone(),
                            method: name.clone(),
                        });
                    } else {
                        // Record (method_name, return_type, param_types)
                        let params: Vec<&str> = args.iter().map(|ArgDecl { id: _, tid }| tid.as_str()).collect();
                        info.methods.push((name.as_str(), ret_type.as_str(), params));
                    }
                }
            }
        }
    }

    // 2) Check overrides against parent signatures
    for c in classes {
        if is_builtin_class(&c.name) {
            continue;
        }
        if let Some(parent_name) = &c.inherits {
            // If parent not in table, that was flagged earlier; skip
            if let Some(parent_info) = class_table.get(parent_name) {
                let child_info = class_table.get(&c.name).unwrap();

                // Build a quick lookup for parent methods
                let parent_methods: HashMap<&str, (&str, &[&str])> = parent_info
                    .methods
                    .iter()
                    .map(|(mname, rtype, params)| (*mname, (*rtype, params.as_slice())))
                    .collect();

                for (mname, rtype, params) in &child_info.methods {
                    if let Some((exp_ret, exp_params)) = parent_methods.get(mname) {
                        // Compare signature: return type must match exactly and param list length/order
                        let found_param_types: Vec<&str> = params.clone();
                        let exp_param_types: Vec<&str> = exp_params.to_vec();

                        if *exp_ret != *rtype || exp_param_types != found_param_types {
                            ec.add(MethodOverrideMismatch {
                                class: c.name.clone(),
                                method: mname.to_string(),
                                parent: parent_name.clone(),
                                expected: {
                                    let mut e = Vec::new();
                                    e.extend(exp_param_types.iter().map(|&s| s.to_string()));
                                    e
                                },
                                found: {
                                    let mut f = Vec::new();
                                    f.extend(found_param_types.iter().map(|&s| s.to_string()));
                                    f
                                },
                            });
                        }
                    }
                }
            }
        }
    }
}
