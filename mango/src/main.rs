use mango_lib::called_from;
fn main(){
    println!("Hello, world!");
    println!("called from: {}", called_from!());
}