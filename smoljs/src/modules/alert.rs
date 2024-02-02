use core::panic;

use rusty_jsc::JSContext;
use rusty_jsc::JSObject;
use rusty_jsc::JSString;
use rusty_jsc_macros::callback;

#[callback]
fn crash(){
}