pub fn assemble(code: String) -> String {
    let tokens: Vec<i32> = get_tokens(&code);
    let bytecodes: String = get_bytecode(&tokens);

    println!("NOT IMPLEMENTED");

    bytecodes
}

fn get_tokens(_: &str) -> Vec<i32> {
    let lines: Vec<i32> = Vec::new();

    lines
}

fn get_bytecode(_: &Vec<i32>) -> String {
    let result: String = String::new();

    result
}