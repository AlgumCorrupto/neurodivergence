use std::io::{stdin, Read};

use console::Term;

pub fn interpret(raw: Vec<u8>) {
    let mut memory: Vec<u8> = vec![0; 30_000];
    let mut mem_pointer = 0;
    let mut program_pointer = 0;
    let mut loop_stack: Vec<usize> = Vec::new();
    let term = Term::stdout();
    let mut ignoring_idx: i32 = -2;

    let program_len = raw.len();

    while program_len != program_pointer {
        if raw[program_pointer] ==  b']' {
            if ignoring_idx == loop_stack.len() as i32 - 1 {
                ignoring_idx = -2;
            }
            
            if memory[mem_pointer] != 0 {
                program_pointer = loop_stack.last().unwrap().clone();
            }
            else {
                loop_stack.pop();
            }
        }
        else if raw[program_pointer] ==  b'[' {
            loop_stack.push(program_pointer);
            if (memory[mem_pointer] == 0) && (ignoring_idx == -2) {
                ignoring_idx = loop_stack.len() as i32 - 1;
            }
        };
        
        if ignoring_idx == -2 {
            match raw[program_pointer] {
                b'>' => {
                    if mem_pointer == 30_000 - 1 {
                        mem_pointer = 0
                    }
                    else {
                        mem_pointer+=1
                    }
                },
                b'<' => {
                    if mem_pointer == 0 {
                        mem_pointer = 30_000 - 1;
                    }
                    else {
                        mem_pointer-=1;
                    }
                },
                b'+' => memory[mem_pointer] = memory[mem_pointer].wrapping_add(1),
                b'-' => memory[mem_pointer] = memory[mem_pointer].wrapping_sub(1),
                b'.' => {
                    print!("{}", memory[mem_pointer] as char)
                },
                b',' => {                
                    memory[mem_pointer] = Term::read_char(&term).unwrap().to_string().as_bytes()[0];
                }
                _ => (),
            }
        }
        program_pointer+=1;
    }
    }
