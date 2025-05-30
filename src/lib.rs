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

        dbg!(context.get("123"));
        context.pop();
    }

    context.get("123")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = action();
        assert_eq!(result, "lol");
    }
}
