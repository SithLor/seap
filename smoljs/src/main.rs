use rusty_jsc::JSContext;
use rusty_jsc::JSValue;
use rusty_jsc::JSObject;
use rusty_jsc_macros::callback;

mod base64;
mod URL;

//https://github.com/wasmerio/rusty_jsc/blob/main/examples/hello.rs
fn main() {
    let mut context: JSContext = JSContext::default();
    let global: JSObject = context.get_global_object();

    let base64_atob_callback: JSValue = JSValue::callback(&context, Some(base64::atob));
    //let base64_btoa_callback:JSValue = JSValue::callback(&context, Some(base64::btoa));
    global.set_property(&context, "_atob", base64_atob_callback).unwrap();
    //global.set_property(&context, "_btoa", base64_btoa_callback).unwrap();

    let code:&'static str = include_str!("./test.js");
    match context.evaluate_script(code, 1) {
        Ok(value) => {
            println!("{}", value.to_string(&context).unwrap());
        }
        Err(e) => {
            println!("Uncaught: {}", e.to_string(&context).unwrap())
        }
    }
}