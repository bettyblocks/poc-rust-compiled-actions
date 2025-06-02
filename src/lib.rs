mod context;

use context::Value;

use crate::context::Context;

pub fn action() -> Value {
    let mut context = Context::new();

    context.set("123", "yes1");

    if context.get("123") == "yes1" {
        context.push();

        // To run an action step, we might apply a general function such as:
        // context.step("action_step_uuid")
        //
        // Or a concrete function like
        // run_action_step_uuid();

        let oke = context.get("123");
        context.set("123", "yes2");
        context.set("124", oke);
        context.set("124", 123);

        context.pop();
    }

    if context.get("124") == "yes1" {
        context.get("123")
    } else {
        context.get("123")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let expected = "yes2".to_string();

        // emulate a generated function by wit-bindgen
        let nice_function = |p: String| expected == p;

        let result = action();
        assert!(nice_function(result.try_into()?));

        Ok(())
    }
}
