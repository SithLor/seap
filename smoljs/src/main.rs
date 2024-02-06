use modules::URL;
use rusty_jsc::JSContext;
use rusty_jsc::JSValue;
use rusty_jsc::JSObject;
use rusty_jsc_macros::callback;

mod modules;

/// .
///
/// # Panics
///
/// Panics if .
//https://github.com/wasmerio/rusty_jsc/blob/main/examples/hello.rs
fn main() {
    let mut context: JSContext = JSContext::default();
    let global: JSObject = context.get_global_object();

    let base64_atob_callback:JSValue=JSValue::callback(&context, Some(modules::base64::atob));
    let url_calback=JSValue::callback(&context, Some(modules::URL::URL));
    //let base64_btoa_callback:JSValue = JSValue::callback(&context, Some(base64::btoa));
    global.set_property(&context, "_atob", base64_atob_callback).unwrap();
    global.set_property(&context, "URL", url_calback).unwrap();
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