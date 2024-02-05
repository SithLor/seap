use rusty_jsc::{JSContext, JSString, JSValue};

pub fn js_args_to_utf8_vec(ctx: &JSContext, args: &[JSValue]) -> Vec<u8> {
    args.iter()
        .map(|arg| arg.to_string(ctx).unwrap().into_bytes())
        .flatten()
        .collect()
}

pub fn StringToJSValue(ctx: &JSContext,_string:String){
    JSString::from_utf8(_string);
}