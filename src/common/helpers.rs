use std::{collections::HashMap, ops::Add};

use crate::common::constants::*;

pub fn is_valid_register(name: &str) -> bool {
    let regs = [
        "R0",
        "R1",
        "R2",
        "R3"
    ];

    return regs.contains(&name);
}

pub fn get_register_code(name: &str) -> i32 {
    let reg: HashMap<&str, i32> = HashMap::from([
        ("R0", R0),
        ("R1", R1),
        ("R2", R2),
        ("R3", R3)
    ]);

    return reg.get(name).expect("Not found!").clone();
}

pub fn get_instruction_code(name: &str) -> i32 {
    let instr: HashMap<&str, i32> = HashMap::from([
        ("MOVR", MOVR),
        ("ADD", ADD),
        ("MOVV", MOVV),
        ("PUSH", PUSH),
        ("POP", POP),
        ("JP", JP),
        ("JL", JL),
        ("CALL", CALL),
        ("FLAG", FLAG),
        ("RET", RET),
        ("PRINT", PRINT),
        ("PRINTS", PRINTS),
        ("SCAN", SCAN),
        ("CLS", CLS),
        ("HALT", HALT)
    ]);

    return instr.get(name).expect("Not found!").clone();
}

pub fn get_instruction_name(code: &i32) -> String {
    let instr: HashMap<i32, &str> = HashMap::from([
        (MOVR, "MOVR"),
        (ADD, "ADD"),
        (MOVV, "MOVV"),
        (PUSH, "PUSH"),
        (POP, "POP"),
        (JP, "JP"),
        (JL, "JL"),
        (CALL, "CALL"),
        (FLAG, "FLAG"),
        (RET, "RET"),
        (PRINT, "PRINT"),
        (PRINTS, "PRINTS"),
        (SCAN, "SCAN"),
        (CLS, "CLS"),
        (HALT, "HALT")
    ]);

    return instr.get(code).expect("Not found!").clone().to_string();
}

pub fn read_string(program: &Vec<i32>, mut pc: i32) -> (String, i32) {
    let mut total_string: String = String::new();

    if program[pc as usize] == STRING_STOPPER {
        pc = pc + 1;
        while program[pc as usize] != STRING_STOPPER {
            let char_code: u8 = program[pc as usize] as u8;
            let char_to_add: char = char_code as char;
            total_string = total_string.add(&char_to_add.to_string());
            pc = pc + 1;
        }
    }

    (total_string, pc)
}

pub fn flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
} 