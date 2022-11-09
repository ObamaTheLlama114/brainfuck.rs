use std::{
    fs,
    io::{self, Read, Write},
};

use clap::Parser;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to brainfuck file
    #[arg(short, long)]
    filepath: String,
}

fn main() {
    let args = Args::parse();
    let mut memory: [u8; 3000] = [0; 3000];
    let mut pointer: usize = 0;
    let program: String = fs::read_to_string(args.filepath).expect("File does not exist");
    let mut program_index: usize = 0;
    let mut loop_stack: Vec<usize> = vec![];

    'main: loop {
        if program_index >= program.len() {
            break 'main;
        }
        match program
            .chars()
            .collect::<Vec<char>>()
            .get(program_index)
            .expect("token index out of bounds")
        {
            '+' => *(memory.get_mut(pointer).expect("Pointer out of bounds")) += 1,
            '-' => *(memory.get_mut(pointer).expect("Pointer out of bounds")) -= 1,
            '<' => pointer -= 1,
            '>' => pointer += 1,
            '[' => loop_stack.push(program_index),
            ']' => {
                if *(memory.get(pointer).expect("Pointer out of bounds")) == 0 {
                    loop_stack.pop().expect("Did not close loop");
                } else {
                    program_index = *(loop_stack.last().expect("Did not close loop"));
                };
            }
            '.' => {
                print!(
                    "{}",
                    *(memory.get(pointer).expect("Pointer out of bounds")) as char
                );
                io::stdout().flush().expect("IO Error");
            }
            ',' => {
                enable_raw_mode().expect("IO Error");
                let mut buf = [0; 1];
                io::stdin().read(&mut buf).expect("IO Error");
                *(memory.get_mut(pointer).expect("Pointer out of bounds")) = buf[0];
                disable_raw_mode().expect("IO Error");
            }
            _ => {}
        }
        program_index += 1;
    }
    println!();
}
