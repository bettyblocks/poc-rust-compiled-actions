use crate::Value;
use std::collections::{HashMap, VecDeque};

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
                let a = s
                    .get_mut(key.as_ref())
                    .expect("already checked if in this scope");
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
            Some(s) => s
                .get(key)
                .expect("already checked if in this scope")
                .clone(),
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
