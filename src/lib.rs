use std::fs;

mod emulator;
mod common;
mod assembly;

pub fn run(op_type: &str, file_path: &str) {
    match op_type {
        "run" => {
            run_program(file_path);
        }

        "assemble" => {
            assemble_code(file_path);
        }

        "disassemble" => {
            disassemble_code(file_path);
        }

        _ => println!("operation nnot supported")
    }
}

fn run_program(file_path: &str) {
    let program: Vec<i32> = {
        let contents = fs::read_to_string(file_path).expect("File not found!");
        let mut results : Vec<i32> = Vec::new();

        for s in contents.split(",") {
            let instruction = s.trim().parse().expect("Just numbers are allowed as instructions!");
            results.push(instruction);
        }

        results
    };

    emulator::cpu::start(&program);
}

fn assemble_code(file_path: &str) {
    let code: String = fs::read_to_string(file_path).expect("File not found!");
    let result = &assembly::assembler::assemble(code);
    println!("{}", result);
}

fn disassemble_code(_: &str) {
    assembly::disassembler::disassemble();
}

