use crate::compile::Instruction;
use std::io::{self, Read};

pub fn interpret(instructions: &[Instruction]) {
    let mut instruction_pointer = 0;
    let mut data_pointer = 0;
    let mut data = vec![0u8; 512];

    while instruction_pointer < instructions.len() {
        if data_pointer >= data.len() {
            data.extend_from_slice(&[0u8; 10]);
        }
        let instruction = instructions[instruction_pointer];
        match instruction {
            Instruction::Move(i) => data_pointer = ((data_pointer as i32) + i) as usize,
            Instruction::Add(i) => data[data_pointer] = ((data[data_pointer] as i32 + i) % 256) as u8,
            Instruction::Output => print!("{}", data[data_pointer] as char),
            Instruction::Input => io::stdin().read_exact(&mut data[data_pointer..data_pointer+1]).unwrap(),
            Instruction::JmpEq(loop_end) => {
                if data[data_pointer] == 0 {
                    instruction_pointer = loop_end - 1;
                }
            },
            Instruction::JmpNEq(loop_begin) => {
                if data[data_pointer] != 0 {
                    instruction_pointer = loop_begin - 1;
                }
            }
        }
        instruction_pointer += 1;
    }
}
