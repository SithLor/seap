use rusty_jsc::{JSContext, JSValue};
use rusty_jsc_macros::callback;
use base64::prelude::*;

#[callback]
fn greet(
    ctx: JSContext,
    function: JSObject,
    this: JSObject,
    args: &[JSValue],
) -> Result<JSValue, JSValue> {
    Ok(JSValue::string(&ctx, format!("Hello, {}", args[0].to_string(&ctx).unwrap())))
}
//https://github.com/wasmerio/rusty_jsc/blob/main/examples/hello.rs
fn main() {
    let mut context: JSContext = JSContext::default();
    let callback: JSValue = JSValue::callback(&context, Some(greet));
    let global: rusty_jsc::JSObject = context.get_global_object();
    global.set_property(&context, "greet", callback).unwrap();

    match context.evaluate_script("greet('Tom')", 1) {
        Ok(value) => {
            println!("{}", value.to_string(&context).unwrap());
        }
        Err(e) => {
            println!("Uncaught: {}", e.to_string(&context).unwrap())
        }
    }
}