use std::io;
use crate::common::instructions as Instructions;

pub fn start(program: &Vec<i32>) {
    let mut regs: [i32; 4] = [0, 0, 0, 0];
    let mut stack: Vec<i32> = Vec::new();
    let mut pc: i32 = 0;
    let mut halted: bool = false;

    while !halted {
        (regs, stack, halted, pc) = run_instruction(&program, regs, stack, halted, pc)
    }
}

fn run_instruction(
    program: &Vec<i32>,
    mut regs: [i32; 4],
    mut stack: Vec<i32>,
    mut halted: bool,
    pc: i32,
) -> ([i32; 4], Vec<i32>, bool, i32) {
    if halted {
        return (regs, stack, halted, pc);
    }

    let mut current_pc: i32 = pc.clone();
    let instr = program
        .get(current_pc as usize)
        .expect("pointer counter out of range!");

    match instr {
        &Instructions::MOVR => {
            current_pc += 1;

            let register_dest = program[current_pc as usize];
            current_pc += 1;

            let register_src = program[current_pc as usize];
            current_pc += 1;

            regs[register_dest as usize] = regs[register_src as usize];
        }

        &Instructions::MOVV => {
            current_pc += 1;

            let register_dest = program[current_pc as usize];
            current_pc += 1;

            let value = program[current_pc as usize];
            current_pc += 1;

            regs[register_dest as usize] = value;
        }

        &Instructions::ADD => {
            current_pc += 1;

            let register_dest = program[current_pc as usize];
            current_pc += 1;

            let register_src = program[current_pc as usize];
            current_pc += 1;

            regs[register_dest as usize] += regs[register_src as usize];
        }

        &Instructions::SUB => {
            current_pc += 1;

            let register_dest = program[current_pc as usize];
            current_pc += 1;

            let register_src = program[current_pc as usize];
            current_pc += 1;

            regs[register_dest as usize] -= regs[register_src as usize];
        }

        &Instructions::PUSH => {
            current_pc += 1;

            let register_src = program[current_pc as usize];
            current_pc += 1;

            stack.push(program[register_src as usize]);
        }

        &Instructions::POP => {
            current_pc += 1;

            let register_dest = program[current_pc as usize];
            current_pc += 1;

            regs[register_dest as usize] = stack.pop().expect("You need to push before pop!");
        }

        &Instructions::JP => {
            current_pc += 1;

            let address = program[current_pc as usize];
            current_pc = address;
        }

        &Instructions::JL => {
            current_pc += 1;

            let r1 = program[current_pc as usize];
            current_pc += 1;

            let r2 = program[current_pc as usize];
            current_pc += 1;

            let address = program[current_pc as usize];
            current_pc += 1;

            if regs[r1 as usize] < regs[r2 as usize] {
                current_pc = address;
            }
        }

        &Instructions::CALL => {
            current_pc += 1;

            let address = program[current_pc as usize];
            current_pc += 1;

            stack.push(current_pc);
            current_pc = address;
        }

        &Instructions::FLAG => {
            current_pc += 1;
        }

        &Instructions::RET => {
            current_pc += 1;

            let address = program[current_pc as usize];
            current_pc = address;
        }

        &Instructions::PRINT => {
            current_pc += 1;

            let register = program[current_pc as usize];
            current_pc += 1;

            println!("{}", regs[register as usize]);
        }

        // &Instructions::PRINTS => {
        //     current_pc += 1;

        //     let mut text: String = String::new();

        //     if program[current_pc as usize] == STRING_STOPPER {
        //         current_pc += 1;
        //         loop {
        //             if program[current_pc as usize] == STRING_STOPPER {
        //                 break;
        //             }

        //             text += program[current_pc as usize].;
        //             current_pc += 1;
        //         }
        //     }

        //     println!("{}", regs[register as usize]);
        // }
        &Instructions::SCAN => {
            current_pc += 1;

            let register = program[current_pc as usize];
            current_pc += 1;

            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            let input: i32 = input.trim().parse().expect("The input must be a number!");

            regs[register as usize] = input;
        }

        &Instructions::CLS => {
            current_pc += 1;

            assert!(std::process::Command::new("cls")
                .status()
                .or_else(|_| std::process::Command::new("clear").status())
                .unwrap()
                .success());
        }

        &Instructions::HALT => {
            current_pc += 1;

            halted = true;
        }

        _ => println!("Instruction not found!"),
    }

    (regs, stack, halted, current_pc)
}
