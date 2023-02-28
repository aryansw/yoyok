use std::collections::HashMap;

use super::{error::Error, value::Value};
use crate::ast::ast::*;

use anyhow::Error as AnyError;

#[derive(Debug)]
pub struct Env {
    vars: HashMap<String, (bool, Value)>,
    funcs: HashMap<String, Function>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
        }
    }

    pub fn from_funcs(funcs: Vec<Function>) -> Self {
        let mut env = Self::new();
        for func in funcs {
            env.funcs.insert(func.name.clone(), func);
        }
        env
    }

    // Creates a copy of the env for a function call
    pub fn call(&self) -> Self {
        Self {
            vars: HashMap::new(),
            funcs: self.funcs.clone(),
        }
    }

    pub fn update(&mut self, name: &String, value: Value) -> Result<Value, AnyError> {
        if let Some((mutable, val)) = self.vars.get(name) {
            if val.type_of() != value.type_of() {
                Err(Error::UnexpectedType(val.type_of(), value.type_of()))?
            } else if !mutable {
                Err(Error::ImmutableVariable(name.clone()))?
            } else {
                let update = self
                    .vars
                    .get_mut(name)
                    .ok_or(Error::UndefinedVariable(name.clone()))?;
                *update = (true, value.clone());
                Ok(value)
            }
        } else {
            Err(Error::UndefinedVariable(name.clone()))?
        }
    }

    pub fn get(&self, name: &String) -> Result<Value, AnyError> {
        if let Some((_, val)) = self.vars.get(name) {
            Ok(val.clone())
        } else if let Some(func) = self.funcs.get(name) {
            Ok(Value::Function(func.clone()))
        } else {
            Err(Error::UndefinedVariable(name.clone()))?
        }
    }

    pub fn insert(&mut self, name: &String, value: Value, mutable: bool) {
        self.vars.insert(name.clone(), (mutable, value));
    }

    pub fn find_func(&self, name: &String) -> Result<Function, AnyError> {
        if let Some(func) = self.funcs.get(name) {
            Ok(func.clone())
        } else {
            Err(Error::UndefinedFunction(name.clone()))?
        }
    }
}
