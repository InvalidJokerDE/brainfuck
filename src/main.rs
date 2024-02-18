use std::io::{self, Read};

struct Interpreter {
    tape: Vec<u8>,
    ptr: usize,
    program: Vec<char>,
    pc: usize,
}

impl Interpreter {
    fn new(program: &str) -> Interpreter {
        Interpreter {
            tape: vec![0; 30000],
            ptr: 0,
            program: program
                .chars()
                .filter(|c| "+-<>.,[]".contains(*c)) // Only use valid bf symbols
                .collect(),
            pc: 0,
        }
    }

    fn run(&mut self) {
        while self.pc < self.program.len() {
            match self.program[self.pc] {
                '+' => self.tape[self.ptr] = self.tape[self.ptr].wrapping_add(1),
                '-' => self.tape[self.ptr] = self.tape[self.ptr].wrapping_sub(1),
                '>' => self.ptr += 1,
                '<' => self.ptr -= 1,
                '.' => print!("{}", self.tape[self.ptr] as char),
                ',' => {
                    let mut input = [0; 1];
                    io::stdin().read_exact(&mut input).unwrap();
                    self.tape[self.ptr] = input[0];
                }
                '[' => {
                    if self.tape[self.ptr] == 0 {
                        let mut nesting = 1;
                        while nesting > 0 {
                            self.pc += 1;
                            match self.program[self.pc] {
                                '[' => nesting += 1,
                                ']' => nesting -= 1,
                                _ => {}
                            }
                        }
                    }
                }
                ']' => {
                    if self.tape[self.ptr] != 0 {
                        let mut nesting = 1;
                        while nesting > 0 {
                            self.pc -= 1;
                            match self.program[self.pc] {
                                ']' => nesting += 1,
                                '[' => nesting -= 1,
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
            self.pc += 1;
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let mut file = std::fs::File::open(&args[1]).unwrap();
    let mut program = String::new();
    file.read_to_string(&mut program).unwrap();

    let mut bf = Interpreter::new(&program);

    bf.run();
}
