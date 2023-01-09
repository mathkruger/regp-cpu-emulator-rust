use std::collections::HashMap;

pub const MOVR: i32 = 10;
pub const MOVV: i32 = 11;
pub const ADD: i32 = 20;
pub const SUB: i32 = 21;
pub const PUSH: i32 = 30;
pub const POP: i32 = 31;
pub const JP: i32 = 40;
pub const JL: i32 = 41;
pub const CALL: i32 = 42;
pub const FLAG: i32 = 43;
pub const RET: i32 = 50;
pub const PRINT: i32 = 60;
pub const SCAN: i32 = 62;
pub const CLS: i32 = 70;
pub const HALT: i32 = 255;

pub const R0: i32 = 0;
pub const R1: i32 = 1;
pub const R2: i32 = 2;
pub const R3: i32 = 3;

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
        ("SCAN", SCAN),
        ("CLS", CLS),
        ("HALT", HALT)
    ]);

    return instr.get(name).expect("Not found!").clone();
}

// pub fn get_instruction_name(code: &i32) -> String {
//     let instr: HashMap<i32, &str> = HashMap::from([
//         (10, "MOVR"),
//         (20, "ADD"),
//         (11, "MOVV"),
//         (30, "PUSH"),
//         (31, "POP"),
//         (40, "JP"),
//         (41, "JL"),
//         (42, "CALL"),
//         (43, "FLAG"),
//         (50, "RET"),
//         (60, "PRINT"),
//         (62, "SCAN"),
//         (70, "CLS"),
//         (255, "HALT")
//     ]);

//     return instr.get(code).expect("Not found!").clone().to_string();
// }

pub fn flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
} 
