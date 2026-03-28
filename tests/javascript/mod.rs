use std::collections::HashMap;

use rquickjs::{AsyncContext, AsyncRuntime, async_with};

use rust_actions::{
    Context, ContextMethods, Value,
    context::{Input, js_context::JsContext},
};

thread_local! {
pub static JAVASCRIPT_RUNTIME: std::rc::Rc<AsyncRuntime> = std::rc::Rc::new(AsyncRuntime::new().expect("Not enough memory for a javascript runtime"));
}

async fn run_javascript_steps(_configurations: (), _input: Input, _user_id: ()) -> Value {
    let ctx = Context::new_refcell();

    ctx.scope(
        _input,
        HashMap::from([("array", 3), ("hallo", 24110)]),
        async || {
            javascript_step(
                ctx.clone(),
                String::from(
                    "
                    await $ctx.scope({a: 123}, {a: 100}, async () => {
                        $ctx.set(24110, \"Jallo!\")
                        $result = await $ctx.get(24110)
                    })
                ",
                ),
            )
            .await;
        },
    )
    .await;

    ctx.result()
}

async fn javascript_step(ctx: std::rc::Rc<std::cell::RefCell<Context>>, code: String) {
    let javascript_context = AsyncContext::full(&JAVASCRIPT_RUNTIME.with(|v| v.clone()))
        .await
        .expect("Not enough memory to create a javascript context");

    async_with!(javascript_context => |javascript_context| {
        let globals = javascript_context.globals();

        let jsctx = JsContext::new(ctx);
        globals.set("$ctx", jsctx.clone()).expect("Could not set javascript context");

        let jsctx_clone = jsctx.clone();
        let getter = move || jsctx_clone.result();
        let setter = move |value: Value| jsctx.set_result(value);
        globals.prop("$result", rquickjs::object::Accessor::new_get(getter).set(setter)).expect("Could not set result getters and setters");

        javascript_context.eval_promise(code).expect("Could not execute javascript component").into_future::<()>().await.expect("Could not await top level promise");
    })
    .await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn javascript_steps_test() {
        assert_eq!(
            run_javascript_steps(
                (),
                HashMap::from([(String::from("hallo"), Value::String(String::from("init")))]),
                (),
            )
            .await,
            "Jallo!"
        );
    }
}
