use rusty_jsc::JSContext;
use rusty_jsc::JSValue;
use rusty_jsc::JSObject;
use rusty_jsc_macros::callback;

mod base64;

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
    
    let base64_atob_callback: JSValue = JSValue::callback(&context, Some(base64::atob));
    let base64_btoa_callback:JSValue = JSValue::callback(&context, Some(base64::btoa));
    
    let global: JSObject = context.get_global_object();


    global.set_property(&context, "_atob", base64_atob_callback).unwrap();
    global.set_property(&context, "_btoa", base64_btoa_callback).unwrap();


    match context.evaluate_script("_atob('Tom')", 1) {
        Ok(value) => {
            println!("{}", value.to_string(&context).unwrap());
        }
        Err(e) => {
            println!("Uncaught: {}", e.to_string(&context).unwrap())
        }
    }
}