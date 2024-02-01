use std::task::Context;





fn main(){
    //bootstarp 
    let mut context: rusty_jsc::JSContext = rusty_jsc::JSContext::default();
    let value = context.evaluate_script("'hello, world'", 1);
    if let Ok(value) = value {
        println!("{}", value.to_js_string(&context).unwrap());
    } else {
        let ex = value.unwrap_err().to_js_string(&context).unwrap();
        println!("Uncaught: {}", ex);
    }
}