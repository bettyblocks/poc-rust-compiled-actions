#[cfg(feature = "javascript")]
mod javascript;

use rust_actions::{Context, ContextMethods, Value, context::Input};
use std::collections::HashMap;

pub async fn nested_scope(_configurations: (), _input: Input, _user_id: ()) -> Value {
    let ctx = Context::new_refcell();

    ctx.scope(_input, HashMap::from([("hallo", 24110)]), async || {
        ctx.scope(
            HashMap::from([(
                String::from("hallo"),
                Value::String(String::from("Inner scope")),
            )]),
            HashMap::from([("hallo", 24110)]),
            async || {
                ctx.set(24110, Value::String(String::from("Very inner scope")));
            },
        )
        .await;

        {
            let input = ctx.get(24110).try_into().unwrap();
            ctx.set(24110, hallo(input).await);
        }

        let result = ctx.get(24110);
        ctx.set_result(result);
    })
    .await;

    ctx.result()
}

pub async fn multiple_action_steps(_configurations: (), _input: Input, _user_id: ()) -> Value {
    let ctx = Context::new_refcell();

    ctx.scope(_input, HashMap::from([("hallo", 24110)]), async || {
        {
            let input = ctx.get(24110).try_into().unwrap();
            ctx.set(24110, hallo(input).await);
        }

        {
            let input = ctx.get(24110).try_into().unwrap();
            ctx.set(24110, hallo(input).await);
        }

        let result = ctx.get(24110);
        ctx.set_result(result);
    })
    .await;

    ctx.result()
}

pub async fn conditional(_configurations: (), input: Input, _user_id: ()) -> Value {
    let ctx = Context::new_refcell();

    ctx.scope(input, HashMap::from([("hallo", 24110)]), async || {
        if ctx.get(24110) == "Go to path 1!" {
            let _label = "Path 1";
            ctx.scope(HashMap::from([]), HashMap::from([]), async || {
                {
                    let value = ctx.get(24110);
                    ctx.set(
                        24110,
                        multiple_action_steps(
                            _configurations,
                            HashMap::from([(String::from("hallo"), value)]),
                            _user_id,
                        )
                        .await,
                    )
                }

                let result = ctx.get(24110);
                ctx.set_result(result);
            })
            .await;
        } else {
            let _label = "Else";
            ctx.scope(HashMap::from([]), HashMap::from([]), async || {
                let result = Value::String(String::from("Else hit!"));
                ctx.set_result(result);
            })
            .await;
        }
    })
    .await;

    ctx.result()
}

pub async fn loop_steps(_configurations: (), _input: Input, _user_id: ()) -> Value {
    let ctx = Context::new_refcell();

    ctx.scope(
        _input,
        HashMap::from([("array", 3), ("hallo", 24110)]),
        async || {
            let arr = ctx.get(3);
            for (index, iterator) in TryInto::<Vec<Value>>::try_into(arr)
                .unwrap()
                .into_iter()
                .enumerate()
            {
                ctx.scope(
                    HashMap::from([
                        (String::from("iterator"), iterator),
                        (String::from("index"), Value::Int(index as i64)),
                    ]),
                    HashMap::from([("index", 1), ("iterator", 2)]),
                    async || {
                        let input = (
                            ctx.get(24110).try_into().unwrap(),
                            ctx.get(2).try_into().unwrap(),
                            ctx.get(1).try_into().unwrap(),
                        );

                        ctx.set(24110, looped_hallo(input).await);
                    },
                )
                .await;
            }

            let result = ctx.get(24110);
            ctx.set_result(result);
        },
    )
    .await;

    ctx.result()
}

async fn hallo(input: String) -> String {
    input + ", hallo!"
}

async fn looped_hallo(input: (String, String, i64)) -> String {
    input.0 + &input.1 + &input.2.to_string() + ", hallo!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn nested_scope_test() {
        assert_eq!(
            nested_scope(
                (),
                HashMap::from([(
                    String::from("hallo"),
                    Value::String(String::from("hallo_value")),
                )]),
                (),
            )
            .await,
            "hallo_value, hallo!"
        );
    }

    #[tokio::test]
    async fn conditional_path_1_test() {
        assert_eq!(
            conditional(
                (),
                HashMap::from([(
                    String::from("hallo"),
                    Value::String(String::from("Go to path 1!")),
                )]),
                (),
            )
            .await,
            "Go to path 1!, hallo!, hallo!"
        );
    }

    #[tokio::test]
    async fn conditional_else_test() {
        assert_eq!(
            conditional(
                (),
                HashMap::from([(
                    String::from("hallo"),
                    Value::String(String::from("Don't go to path 1!")),
                )]),
                (),
            )
            .await,
            "Else hit!"
        );
    }

    #[tokio::test]
    async fn multiple_action_steps_test() {
        assert_eq!(
            multiple_action_steps(
                (),
                HashMap::from([(
                    String::from("hallo"),
                    Value::String(String::from("hallo_value")),
                )]),
                (),
            )
            .await,
            "hallo_value, hallo!, hallo!"
        );
    }

    #[tokio::test]
    async fn loop_steps_test() {
        assert_eq!(
            loop_steps(
                (),
                HashMap::from([
                    (
                        String::from("array"),
                        Value::Vec(vec![
                            Value::String(String::from("First")),
                            Value::String(String::from("Second")),
                            Value::String(String::from("Third"))
                        ]),
                    ),
                    (String::from("hallo"), Value::String(String::from("")))
                ]),
                (),
            )
            .await,
            "First0, hallo!Second1, hallo!Third2, hallo!"
        );
    }
}
