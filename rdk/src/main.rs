//wasm call
fn extern_add(a: i32, b: i32) -> i32 {
    use wasm3::Environment;
    use wasm3::Runtime ;
    use wasm3::Module;
    use wasm3::Function;
    let path = "../../wasm_examples/out/add.wasm";
    let func_name = "extern_add";

    let env: Environment = Environment::new().expect("Unable to create environment");
    let rt:Runtime = env
        .create_runtime(1024 * 60)
        .expect("Unable to create runtime");
    let module = Module::parse(&env, &include_bytes!(path)[..])
        .expect("Unable to parse module");
    let mut module: Module = rt.load_module(module).expect("Unable to load module");
    //edit stuff here
    let func: Function<(i64, i64), i64> = module
        .find_function::<(i64, i64), i64>("add")
        .expect("Unable to find function");
    func.call(a, b).unwrap()  
}

fn main(){
    let a = 1;
    let b = 2;
    let result = extern_add(a, b);
    println!("{} + {} = {}", a, b, result);
}