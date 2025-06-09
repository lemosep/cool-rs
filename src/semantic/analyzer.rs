use std::collections::{HashMap, HashSet};
use crate::ast::Class;
use crate::semantic::errors::SemanticError::*;
use crate::semantic::collector::ErrorCollector;

/// 1) Verifies duplicate class names, undefined parents, forbidden basic‐type inheritance,
///    and genuine inheritance‐cycles (excluding the trivial Object→Object loop).
pub fn check_inheritance(classes: &[Class], ec: &mut ErrorCollector) {
    // 1.1) Detect duplicate class names
    let mut seen_names: HashSet<&str> = HashSet::new();
    for c in classes {
        if !seen_names.insert(&c.name) {
            ec.add(DuplicateClass { class: c.name.clone() });
        }
    }

    // 1.2) Build a parent_map for every class (treat “no parent” as inheriting Object).
    //      Except for "Object" itself, we always map c.name → parent_name.
    let mut parent_map: HashMap<&str, &str> = HashMap::new();
    for c in classes {
        let parent = c.inherits.as_deref().unwrap_or("Object");
        // Inheriting from Int, String, Bool, or SELF_TYPE is illegal
        if matches!(parent, "Int" | "String" | "Bool" | "SELF_TYPE") {
            ec.add(InheritBasicType {
                class: c.name.clone(),
                parent: parent.to_string(),
            });
        }
        if c.name != "Object" {
            parent_map.insert(&c.name, parent);
        }
    }
    // 1.3) Make sure Object → Object exists so that any chain landing on Object stops.
    if !parent_map.contains_key("Object") {
        parent_map.insert("Object", "Object");
    }

    // 1.4) Check for undefined parents (except “Object,” which we assume always exists)
    let defined: HashSet<&str> = classes.iter().map(|c| c.name.as_str()).collect();
    for (&cls, &parent) in parent_map.iter() {
        if parent != "Object" && !defined.contains(parent) {
            ec.add(UndefinedParent {
                class: cls.to_string(),
                parent: parent.to_string(),
            });
        }
    }

    // 1.5) Detect real inheritance cycles using Floyd’s algorithm.
    //      We ignore the trivial “Object→Object” loop.
    for c in classes {
        if c.name == "Object" {
            continue;
        }

        // Start both “tortoise” and “hare” at c.name
        let mut tortoise = c.name.as_str();
        let mut hare = c.name.as_str();

        loop {
            // Move tortoise one step
            if let Some(&next_t) = parent_map.get(tortoise) {
                tortoise = next_t;
            } else {
                // No further parent → no cycle along this chain
                break;
            }

            // Move hare two steps; if any step fails, stop
            let mut advance_ok = true;
            for _ in 0..2 {
                if let Some(&next_h) = parent_map.get(hare) {
                    hare = next_h;
                } else {
                    advance_ok = false;
                    break;
                }
            }
            if !advance_ok {
                // Hare could not advance twice → no cycle
                break;
            }

            // If both meet and neither is “Object,” that is a real cycle
            if tortoise == hare {
                if tortoise != "Object" {
                    // Reconstruct the cycle path
                    let mut cycle = Vec::new();
                    let mut start = tortoise;
                    loop {
                        cycle.push(start.to_string());
                        if let Some(&p) = parent_map.get(start) {
                            start = p;
                        } else {
                            break;
                        }
                        if start == tortoise {
                            break;
                        }
                    }
                    cycle.push(tortoise.to_string()); // close the loop
                    ec.add(InheritanceCycle { cycle });
                }
                break;
            }

            // If either pointer has reached “Object,” no real cycle
            if tortoise == "Object" || hare == "Object" {
                break;
            }
        }
    }
}
