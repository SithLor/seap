use rusty_jsc::JSObject;
use rusty_jsc::JSContext;
use rusty_jsc::JSValue;
use rusty_jsc_macros::callback;


fn URL(
    ctx: JSContext,
    function: JSObject,
    this: JSObject,
    args: &[JSValue],
) -> Result<JSValue,JSObject> {
    
}