use rusty_jsc::JSObject;
use rusty_jsc::JSContext;
use rusty_jsc::JSValue;
use rusty_jsc::JSString;
use rusty_jsc_macros::callback;
use url::Url;
use url::ParseError;

use super::helper::js_args_to_utf8_vec;



fn paser_url(url:&str)-> Url{
    let parsed_url: Url;
    let e_1: Result<Url, ParseError> = url::Url::parse(url);       
    let data: Url = match e_1{
        Ok(url) => parsed_url,
        Err(e) => {
            panic!("You Some How Messuped");
        }
    };
    return parsed_url;
}


struct _URL {
    herf:JSValue,
    pathname:JSValue,
    hostname:JSValue,

}
pub fn URL(
    ctx: JSContext,
    function: JSObject,
    this: JSObject,
    args: &[JSValue],
) -> Result<JSObject,JSObject> {

    let object: JSObject = JSObject::new(&ctx);

     
    

    let e: Url= paser_url(String::from(js_args_to_utf8_vec(&ctx,args)[0].to_string()).as_str());
    let JS_Object: JSObject = JSObject::new(&ctx);
    
    //get the value from e.domain that has Option<&str> type
    let domain: String= e.domain().unwrap_or("localhost").to_string();

    JS_Object.set_property(
        &ctx, 
        "domain", 
        JSValue::string(&ctx, domain)
    );

    return Ok(JS_Object);
}