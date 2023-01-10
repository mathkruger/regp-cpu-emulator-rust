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
        if line.starts_with(";") || line.len() == 0 {
            continue;
        }

        let mut line_vector: Vec<String> = Vec::new();
        let regex_string: &str = "([\"\"].+?[\"\"]|[^ ]+)";

        let regex = Regex::new(regex_string).expect("Invalid regex");

        for capture in regex.captures_iter(line) {
            line_vector.push(capture.get(0)
                .expect("The tokenization failed.")
                .as_str().to_string());
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
                if !token.starts_with(".") && !token.ends_with(":") {
                    bytes.push(get_instruction_code(&token));
                } else if token.starts_with(".") && token.ends_with(":") {
                    bytes.push(FLAG);
                }
            } else {
                if is_valid_register(&token) {
                    bytes.push(get_register_code(&token));
                } else {
                    if token.contains("\"") {
                        for char_to_add in token.chars() {
                            let charcode: u8 = char_to_add as u8;
                            bytes.push(charcode as i32);
                        }
                    } else if token.starts_with(".") && !token.ends_with(":") {
                        let mut list_to_find: Vec<String> = Vec::new();

                        for item in flatten(tokens.clone()) {
                            if item.contains("\"") {
                                let char_array: Vec<String> = item.chars()
                                    .map(|x| x.to_string())
                                    .collect();
                                
                                list_to_find = [list_to_find, char_array].concat();
                            } else {
                                list_to_find.push(item);
                            }
                        }

                        let token_to_find = token + ":";
                        let address = list_to_find
                            .into_iter()
                            .position(|x| token_to_find == x.to_uppercase())
                            .expect("Flag reference not found!");

                        bytes.push(address as i32);
                    } else if !token.ends_with(":") {
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