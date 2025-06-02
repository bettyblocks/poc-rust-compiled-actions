use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    String(String),
    Int(i64),
    Nil,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValueType {
    String,
    Int,
    Nil
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::String => f.write_str("string"),
            ValueType::Int => f.write_str("integer"),
            ValueType::Nil => f.write_str("nil"),
        }
    }
}

impl Value {
    pub fn as_type(&self) -> ValueType {
        match self {
            Value::String(_) => ValueType::String,
            Value::Int(_) => ValueType::Int,
            Value::Nil => ValueType::Nil,
        }
    }
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

impl TryInto<String> for Value {
    type Error = String;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Value::String(x) => Ok(x),
            _ => Err(format!("Invalid type expected string found {}", self.as_type())),
        }
    }
}

impl TryInto<i64> for Value {
    type Error = String;

    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Value::Int(x) => Ok(x),
            _ => Err(format!("Invalid type expected integer found {}", self.as_type())),
        }
    }
}

impl TryInto<()> for Value {
    type Error = String;

    fn try_into(self) -> Result<(), Self::Error> {
        match self {
            Value::Nil => Ok(()),
            _ => Err(format!("Invalid type expected nil found {}", self.as_type())),
        }
    }
}

pub struct Context {
    scopes: VecDeque<HashMap<String, Value>>,
}

impl Context {
    pub fn new() -> Context {
        let mut ctx = Context {
            scopes: VecDeque::new(),
        };

        ctx.push();

        ctx
    }

    pub fn set<T: AsRef<str>, U: Into<Value>>(&mut self, key: T, value: U) {
        let scope = self
            .scopes
            .iter_mut()
            .find(|scope| scope.contains_key(key.as_ref()));

        match scope {
            Some(s) => {
                let a = s.get_mut(key.as_ref()).expect("already checked if in this scope");
                *a = value.into();
            }
            None => {
                self.scopes[0].insert(key.as_ref().to_string(), value.into());
            }
        };
    }

    pub fn get(&self, key: &str) -> Value {
        let scope = self.scopes.iter().find(|scope| scope.contains_key(key));

        match scope {
            Some(s) => s.get(key).expect("already checked if in this scope").clone(),
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
