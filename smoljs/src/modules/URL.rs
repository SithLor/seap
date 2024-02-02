use rusty_jsc::JSObject;
use rusty_jsc::JSContext;
use rusty_jsc::JSValue;
use rusty_jsc::JSString;
use rusty_jsc_macros::callback;
use url::Url;
use url::ParseError;

use super::helper::js_args_to_utf8_vec


struct UrlParts {
    scheme: String,
    username: String,
    password: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    path: String,
    query: Option<String>,
    fragment: Option<String>,
}
fn paser_url(url:&str)-> UrlParts{
    let parsed_url: Url;
    let e_1: Result<Url, ParseError> = url::Url::parse(url);       
    let data: Url = match e_1{
        Ok(url) => rust_url,
        Err(e) => {
            panic!("You Some How Messuped");
        }
    };
    let parts: UrlParts = UrlParts {
        scheme: parsed_url.scheme().to_string(),
        username: parsed_url.username().to_string(),
        password: parsed_url.password().map(|s| s.to_string()),
        host: parsed_url.host_str().map(|s| s.to_string()),
        port: parsed_url.port(),
        path: parsed_url.path().to_string(),
        query: parsed_url.query().map(|s| s.to_string()),
        fragment: parsed_url.fragment().map(|s| s.to_string()),
    };

    return parts;
}

fn URL(
    ctx: JSContext,
    function: JSObject,
    this: JSObject,
    args: &[JSValue],
) -> Result<JSObject,JSObject> {

    let object: JSObject = JSObject::new(&ctx);

     
    
        
    let e: UrlParts = paser_url(String::from(js_args_to_utf8_vec(&ctx,args)[0].to_string()).as_str())
    let _e = [
        JSValue::string(&ctx, e.fragment)
    ] 

}