use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    String(String),
    Int(i64),
    Nil,
}

pub struct Context {
    scopes: VecDeque<HashMap<String, RefCell<Value>>>,
}

impl Into<Value> for &str {
    fn into(self) -> Value {
        Value::String(self.to_string())
    }
}

impl Into<Value> for String {
    fn into(self) -> Value {
        Value::String(self)
    }
}

impl Into<Value> for i64 {
    fn into(self) -> Value {
        Value::Int(self)
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Nil
    }
}

impl PartialEq<&str> for Value {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Value::String(s) => s == other,
            _ => false,
        }
    }
}

impl PartialEq<String> for Value {
    fn eq(&self, other: &String) -> bool {
        match self {
            Value::String(s) => s == other,
            _ => false,
        }
    }
}

impl Context {
    pub fn new() -> Context {
        Context {
            scopes: VecDeque::new(),
        }
    }

    pub fn set<T: AsRef<str>, U: Into<Value>>(&mut self, key: T, value: U) {
        let scope = self
            .scopes
            .iter_mut()
            .find(|scope| scope.contains_key(key.as_ref()));

        match scope {
            Some(s) => {
                let a = s.get(key.as_ref()).expect("already checky");
                let mut b = a.borrow_mut();
                *b = value.into();
            }
            None => {
                self.scopes[0].insert(key.as_ref().to_string(), RefCell::new(value.into()));
            }
        };
    }

    pub fn get(&self, key: &str) -> Value {
        let scope = self.scopes.iter().find(|scope| scope.contains_key(key));

        match scope {
            Some(s) => s.get(key).unwrap().clone().borrow().clone(),
            None => Value::Nil,
        }
    }

    pub fn push(&mut self) {
        self.scopes.push_front(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.scopes.pop_front();
    }
}
