
use rusty_jsc::{JSContext, JSObject, JSValue, JSString};
use rusty_jsc_macros::callback;

#[callback]
fn greet(
    ctx: JSContext,
    function: JSObject,
    this: JSObject,
    args: &[JSValue],
) -> Result<JSValue, JSValue> {
    // Parse the argument as a function and call it with an argument
    let callback_function = args[0].to_object(&ctx).unwrap().call(&ctx, None, &[JSValue::string(&ctx, "Tom")]).unwrap();
    Ok(callback_function)
}

fn main() {
    let mut context = JSContext::default();
    let global: JSObject = context.get_global_object();
    
    let callback: JSValue = JSValue::callback(&context, Some(greet));
    // add the methods to the global object
    global.set_property(&context, "greet", callback).unwrap();

    match context.evaluate_script("greet((name) => 'Hello, ' + name)", 1) {
        Ok(value) => {
            println!("{}", value.to_string(&context).unwrap());
        }
        Err(e) => {
            println!("Uncaught: {}", e.to_string(&context).unwrap())
        }
    }
}