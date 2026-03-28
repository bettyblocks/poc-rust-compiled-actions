#[cfg(feature = "javascript")]
pub mod js_context;

use crate::Value;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

pub type Input = HashMap<String, Value>;
pub type UintMapping = HashMap<&'static str, u32>;
pub type Scope = HashMap<u32, Value>;

fn map_uint_vars(mut vars: Input, uint_mapping: UintMapping) -> Scope {
    let mut new_scope = HashMap::new();
    for (name, value) in vars.drain() {
        if let Some(key) = uint_mapping.get(AsRef::<str>::as_ref(&name)) {
            new_scope.insert(*key, value);
        }
    }
    new_scope
}

#[derive(Debug, Clone)]
pub struct Context {
    scopes: VecDeque<HashMap<u32, Value>>,
    result: Value,
}

impl Context {
    pub fn new() -> Self {
        let mut ctx = Self {
            scopes: VecDeque::new(),
            result: Value::Nil,
        };

        ctx.push(HashMap::new());

        ctx
    }

    pub fn new_refcell() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new()))
    }

    fn push(&mut self, map: HashMap<u32, Value>) {
        self.scopes.push_front(map);
    }

    fn pop(&mut self) {
        self.scopes.pop_front();
    }
}

#[allow(async_fn_in_trait)]
pub trait ContextMethods {
    async fn scope(&self, input: Input, uint_mapping: UintMapping, closure: impl AsyncFn());

    fn set<U: Into<Value>>(&self, key: u32, value: U);

    fn get(&self, key: u32) -> Value;

    fn set_result(&self, value: Value);

    fn result(&self) -> Value;
}

impl ContextMethods for Rc<RefCell<Context>> {
    async fn scope(&self, input: Input, uint_mapping: UintMapping, closure: impl AsyncFn()) {
        self.borrow_mut().push(map_uint_vars(input, uint_mapping));
        closure().await;
        self.borrow_mut().pop();
    }

    fn set<U: Into<Value>>(&self, key: u32, value: U) {
        let mut mut_self = self.borrow_mut();
        let scope = mut_self
            .scopes
            .iter_mut()
            .find(|scope| scope.contains_key(&key));

        match scope {
            Some(s) => {
                let a = s.get_mut(&key).expect("already checked if in this scope");
                *a = value.into();
            }
            None => {
                mut_self.scopes[0].insert(key, value.into());
            }
        };
    }

    fn get(&self, key: u32) -> Value {
        let self_borrow = self.borrow();
        let scope = self_borrow
            .scopes
            .iter()
            .find(|scope| scope.contains_key(&key));

        match scope {
            Some(s) => s
                .get(&key)
                .expect("already checked if in this scope")
                .clone(),
            None => Value::Nil,
        }
    }

    fn set_result(&self, value: Value) {
        self.borrow_mut().result = value
    }

    fn result(&self) -> Value {
        self.borrow().result.clone()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
