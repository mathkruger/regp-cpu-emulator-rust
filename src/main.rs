use std::env;

use regp_cpu_emulator_rust as regp;

fn main() {
    let args: Vec<String> = env::args().collect();

    let op_type = &args[1];
    let file_path = &args[2];

    regp::run(op_type, file_path);
}