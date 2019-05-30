use yawr::runtime::Runtime;
use yawr::types::Value;

fn main() {
    let input = include_bytes!("../../examples/is_one.wasm");

    let _ = env_logger::try_init().unwrap();

    let mut runtime = Runtime::from_bytes(input);

    let args = vec![Value::I32(5)];

    let res = runtime.invoke("is_one", &args).unwrap();

    println!("Function: {:?}, Args: {:?}", "add", args);
    println!("Result: {:?}", res);
}
