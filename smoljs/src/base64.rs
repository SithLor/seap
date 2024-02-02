use std::string;
use std::vec;

use base64::Engine;
use rusty_jsc::JSString;
use rusty_jsc_macros::callback;
use rusty_jsc::JSContext;
use rusty_jsc::JSObject;
use rusty_jsc::JSValue;
use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use base64::DecodeError;
use core::result;
use std::string::FromUtf8Error;

#[callback]
pub fn atob(
    ctx: JSContext,
    function: JSObject,
    this: JSObject,
    args: &[JSValue],
) -> Result<JSValue,JSValue>{
    Ok(
        JSValue::string(&ctx,
            STANDARD.encode(args[0].to_string(&ctx).unwrap().to_string().as_str())
        )
    )
}
#[callback]
pub fn btoa(
    ctx: JSContext,
    function: JSObject,
    this: JSObject,
    args: &[JSValue],
) -> Result<JSValue,JSValue>{
    let data: &str = args[0].to_string(&ctx).unwrap().to_string().as_str();
    let decode = 
    Ok(
        JSValue::string(&ctx,
            
        )
    )
}