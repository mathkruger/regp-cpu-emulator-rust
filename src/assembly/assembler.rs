use regex::Regex;

use crate::common::constants::*;

pub fn assemble(code: String) -> String {
    let tokens: Vec<Vec<String>> = get_tokens(&code);
    let bytecodes: String = get_bytecode(&tokens);

    bytecodes
}

fn get_tokens(code: &str) -> Vec<Vec<String>> {
    let mut lines: Vec<Vec<String>> = Vec::new();

    for line in code.lines() {
        let mut line_vector: Vec<String> = Vec::new();
        let separator = Regex::new(r"([\s,])").expect("Invalid regex");
        let splits: Vec<_> = separator.split(line).into_iter().collect();

        for token in splits {
            line_vector.push(token.to_string());
        }

        lines.push(line_vector);
    }

    lines
}

fn get_bytecode(tokens: &Vec<Vec<String>>) -> String {
    let mut bytes: Vec<i32> = Vec::new();
    
    for line in tokens {
        let mut index: i32 = 0;

        for item in line {
            let token = item.trim().to_uppercase();

            if index == 0 {
                if !token.starts_with(".") && !token.starts_with(":") {
                    bytes.push(get_instruction_code(&token));
                }
            } else if token.starts_with(".") && token.contains(":") {
                bytes.push(FLAG);
            } else {
                if is_valid_register(&token) {
                    bytes.push(get_register_code(&token));
                } else {
                    if token.starts_with(".") && !token.contains(":") {
                        let tokens_flatten = flatten(tokens.clone());
                        let token_to_find = token + ":";
                        let address = tokens_flatten.into_iter().find(|x| token_to_find == x.to_uppercase()).expect("Flag reference not found!");

                        bytes.push(address.parse().expect("An address should be a number!"));

                    } else if !token.contains(":") {
                        bytes.push(token.parse().expect("A register reference/value should be a number!"));
                    }
                }
            }

            index += 1;
        }
    }

    let mut results: Vec<String> = Vec::new();

    for item in bytes.into_iter().map(|x| x.to_string()) {
        results.push(item);
    };

    return results.join(",");
}