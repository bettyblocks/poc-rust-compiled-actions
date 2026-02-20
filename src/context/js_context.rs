use std::{cell::RefCell, collections::HashMap, rc::Rc};

use rquickjs::{Atom, FromJs, JsLifetime, class::Trace, methods};

use crate::{Context, Value, context::{ContextMethods, Scope}};

type JsMap<'js> = HashMap<Atom<'js>, rquickjs::Value<'js>>;

fn js_to_rs_scope<'js>(ctx: &rquickjs::Ctx<'js>, mut js_scope: JsMap<'js>) -> rquickjs::Result<Scope> {
    let mut new_map = HashMap::new();
                    for entry in js_scope.drain() {
                        new_map.insert(entry.0.to_string()?.parse().map_err(|_| rquickjs::Error::FromJs { from: "string", to: "u32", message: Some(String::from("Could not parse scope key")) })?, Value::from_js(ctx, entry.1)?);
                    }
                    Ok::<HashMap<u32, Value>, rquickjs::Error>(new_map)
}

/// vars: String -> Value mapping
/// uint_mapping: &'static str -> key mapping
fn js_map_uint_vars<'js>(ctx: &rquickjs::Ctx<'js>, mut vars: JsMap<'js>, mut uint_mapping: JsMap<'js>) -> rquickjs::Result<Scope> {
    let mut new_scope = HashMap::new();
    for (name, value) in vars.drain() {
        if let Some(key) = uint_mapping.remove(&name) {
            new_scope.insert(u32::from_js(ctx, key)?, Value::from_js(ctx, value)?);
        }
    }
    Ok(new_scope)
}

#[rquickjs::class]
#[derive(JsLifetime, Clone, Debug)]
pub struct JsContext {
    inner: Rc<RefCell<Context>>
}

impl<'js> Trace<'js> for JsContext {
    fn trace<'a>(&self, _tracer: rquickjs::class::Tracer<'a, 'js>) {
        
    }
}

impl JsContext {
    pub fn new(inner: Rc<RefCell<Context>>) -> Self {
        Self {inner}
    }

    pub fn log_inner(&self) {
        println!("{:?}", self.inner.borrow());
    }
}

#[methods]
impl JsContext {
    pub fn get(&self, key: u32) -> Value {
        self.inner.get(key)
    }

    pub fn set(&self, key: u32, value: Value) {
        self.inner.set(key, value);
    }

    pub fn push<'js> (&self, ctx: rquickjs::Ctx<'js>, map: JsMap<'js>) -> rquickjs::Result<()> {
        self.inner.borrow_mut().push(js_to_rs_scope(&ctx, map)?);
        Ok(())
    }

    pub fn pop(&self) {
        self.inner.borrow_mut().pop();
    }

    pub async fn scope<'js>(
        &self,
        ctx: rquickjs::Ctx<'js>,
        input: JsMap<'js>,
        uint_mapping: JsMap<'js>,
        closure: rquickjs::Function<'js>,
    ) -> rquickjs::Result<()> {
        self.inner.borrow_mut().push(js_map_uint_vars(&ctx, input, uint_mapping)?);
        closure.call::<(), rquickjs::Value<'js>>(())?.into_promise().ok_or(rquickjs::Error::FromJs { from: "Scope callback return value", to: "Promise", message: None })?.into_future::<()>().await?;
        self.pop();
        Ok(())
    }

    pub fn set_result(&self, value: Value) {
        self.inner.set_result(value)
    }

    pub fn result(&self) -> Value {
        self.inner.result()
    }
}