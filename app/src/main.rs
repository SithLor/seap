use rsjs_lib::excel::UsFiat;



fn main() {
    let mut file: UsFiat = UsFiat::new();
    file.add_five_dollars(20);
    println!("{}",file.get_value());
}