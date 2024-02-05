use core::panic;

use rusty_jsc::JSContext;
use rusty_jsc::JSObject;
use rusty_jsc::JSString;
use rusty_jsc_macros::callback;

#[callback]
pub fn alert(
    ctx: JSContext,
    function: JSObject,
    this: JSObject,
    args: &[JSValue],
){
    
}