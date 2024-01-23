// loader for java code generator

fn create_class (name:String){
    let mut file = File::create(name).expect("Unable to create file");
    file.write_all(b"Hello, world!").expect("Unable to write data");
    
}

fn main(){
    let commands = vec!["struct_wti"];
    //get cli args
    let args: Vec<String> = std::env::args().collect();
    //check if args are empty
    if args.len() == 1 {
        println!("No args provided");
        return;
    }
    //command struct_impl
    let command = args[1].clone();

    println!("Hello, world!");
}