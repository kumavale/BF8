use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};

use std::fmt;


#[derive(Clone, Copy, Debug)]
enum Instruction {
    Pincrement,  // 000 (>)
    Pdecrement,  // 001 (<)
    Increment,   // 010 (+)
    Decrement,   // 011 (-)
    Put,         // 100 (.)
    Get,         // 101 (,)
    Begin,       // 110 ([)
    End,         // 111 (])
    None,        // other
}

#[derive(Debug)]
struct Code {
    source: Vec<u8>,
    idx: usize,
}

impl Code {
    fn next(&mut self) -> Option<Instruction> {
        let mut oct = 0;
        let mut i = 0;
        while i < 3 {
            if self.idx < self.source.len() {
                let bin = self.source[self.idx] as i8 - 48;
                self.idx += 1;
                if bin != 0 && bin != 1 {
                    continue;
                }
                oct |= bin << 2-i;
                i += 1;
            } else {
                return None;
            }
        }
        let instruction = match oct {
            0 => Instruction::Pincrement,
            1 => Instruction::Pdecrement,
            2 => Instruction::Increment,
            3 => Instruction::Decrement,
            4 => Instruction::Put,
            5 => Instruction::Get,
            6 => Instruction::Begin,
            7 => Instruction::End,
            _ => Instruction::None,
        };
        Some(instruction)
    }
}

struct Tokens {
    instructions: Vec<Instruction>,
    idx: usize,        // index
    ptr: usize,        // pointer
    mem: [u8; 30000],  // memory
}

impl fmt::Debug for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.mem[..].fmt(f)
    }
}

impl Tokens {
    fn new() -> Self {
        let instructions: Vec<Instruction> = Vec::new();
        Tokens {
            instructions,
            idx: 0,
            ptr: 0,
            mem: [0; 30000],
        }
    }

    fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    fn exec(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Pincrement => self.ptr += 1,
            Instruction::Pdecrement => self.ptr -= 1,
            Instruction::Increment => self.mem[self.ptr] += 1,
            Instruction::Decrement => self.mem[self.ptr] -= 1,
            Instruction::Put => {
                let output = self.mem[self.ptr] as char;
                print!("{}", output);
                std::io::stdout().flush().unwrap();
            },
            Instruction::Get => {
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).unwrap();
                self.mem[self.ptr] = buf.as_bytes()[0];
            },
            Instruction::Begin => {
                if self.mem[self.ptr] == 0 {
                    let mut bracket_cnt = 1;
                    while bracket_cnt > 0 {
                        self.idx += 1;
                        match self.instructions[self.idx] {
                            Instruction::Begin => bracket_cnt += 1,
                            Instruction::End   => bracket_cnt -= 1,
                            _ => (),
                        }
                    }
                }
            },
            Instruction::End => {
                if self.mem[self.ptr] != 0 {
                    let mut bracket_cnt = 1;
                    while bracket_cnt > 0 {
                        self.idx -= 1;
                        match self.instructions[self.idx] {
                            Instruction::Begin => bracket_cnt -= 1,
                            Instruction::End   => bracket_cnt += 1,
                            _ => (),
                        }
                    }
                }
            },
            Instruction::None => (),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid argument");
    }

    let mut reader = BufReader::new(File::open(&args[1])
        .expect("Failed file open"));
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();

    let mut code = Code { source: buf, idx: 0 };
    let mut tokens = Tokens::new();

    while let Some(instruction) = code.next() {
        tokens.push(instruction);
    }

    while tokens.idx < tokens.instructions.len() {
        let instruction = tokens.instructions[tokens.idx];
        tokens.exec(instruction);
        tokens.idx += 1;
    }

    //print!("{:?}", code);
    //print!("{:?}", tokens);

}
