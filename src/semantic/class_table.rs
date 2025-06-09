// src/semantic/class_table.rs

use std::collections::HashMap;
use crate::ast::{Class, Feature, ArgDecl, VarDecl};

/// Entrada para a tabela de lookup de classes
#[derive(Debug)]
pub struct ClassInfo<'a> {
    pub ast: &'a Class,
    pub parent: String,
    /// Lista de (nome_atributo, tipo_atributo)
    pub attributes: Vec<(&'a str, &'a str)>,
    /// Lista de (nome_metodo, tipo_retorno, tipos_parametros)
    pub methods: Vec<(&'a str, &'a str, Vec<&'a str>)>,
}

pub fn build_class_table<'a>(classes: &'a [Class]) -> HashMap<String, ClassInfo<'a>> {
    let mut table = HashMap::new();

    // 1) Inserir todas as classes do usuário (ou builtins já injetadas), com vetores vazios
    for c in classes {
        let parent = c.inherits.clone().unwrap_or_else(|| "Object".into());
        let info = ClassInfo {
            ast: c,
            parent,
            attributes: Vec::new(),
            methods: Vec::new(),
        };
        table.insert(c.name.clone(), info);
    }

    // 2) Garantir que “Object” exista (caso o usuário não tenha definido)
    if !table.contains_key("Object") {
        let boxed = Box::new(Class {
            name: "Object".to_string(),
            inherits: None,
            feature_list: Vec::new(),
        });
        let dummy_obj: &'static Class = Box::leak(boxed);

        let info = ClassInfo {
            ast: dummy_obj,
            parent: "Object".into(),
            attributes: Vec::new(),
            methods: Vec::new(),
        };
        table.insert("Object".into(), info);
    }

    // 3) Agora que todas as entradas existem, varremos de novo para preencher attributes e methods
    for c in classes {
        if let Some(info) = table.get_mut(&c.name) {
            for feat in &c.feature_list {
                match feat {
                    Feature::Attribute(VarDecl { oid, tid, .. }) => {
                        // Atributo: (nome, tipo)
                        info.attributes.push((oid.as_str(), tid.as_str()));
                    }
                    Feature::Method(name, args, ret_type, _body) => {
                        // Método: (nome, retorno, [tipos dos parâmetros])
                        let param_types: Vec<&str> = args
                            .iter()
                            .map(|ArgDecl { tid, .. }| tid.as_str())
                            .collect();
                        info.methods.push((name.as_str(), ret_type.as_str(), param_types));
                    }
                }
            }
        }
    }

    table
}
