use nom::types::CompleteStr;
use std;
use std::io;
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;
use std::num::ParseIntError;

use crate::vm::VM;
use crate::assembler::program_parsers::{program};

/// Core structure for the REPL for the assembler
#[derive(Default)]
pub struct REPL {
    command_buffer: Vec<String>,
    // The vm the REPL will use to execute code
    vm: VM
}

impl REPL {
    pub fn run(&mut self) {
        println!("Welcome to Iridium! Let's be productive!");

        loop {
            let mut buffer = String::new();
            // Block until user types in a command
            let stdin = io::stdin();

            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            stdin.read_line(&mut buffer).expect("Unable to read line from user");
            let buffer = buffer.trim();

            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".quit" => {
                    println!("Farewell! Have a great day!");
                    std::process::exit(0);
                },
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                },
                ".program" => {
                    println!("Listing instructions currently in VM's program vector");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("End of program listing");
                },
                ".registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of Register Listing");
                },
                ".clear" => {
                    self.vm.program.clear();
                    println!("Clearing program vector");
                },
                ".load_file" => {
                    print!("Please enter the path of the file you wish to load: ");
                    io::stdout().flush().expect("Unable to flush stdout");
                    let mut tmp = String::new();
                    stdin.read_line(&mut tmp).expect("Unable to read line from user");
                    let tmp = tmp.trim();
                    let filename = Path::new(&tmp);
                    let mut f = File::open(Path::new(&filename)).expect("File not found");
                    let mut contents = String::new();
                    f.read_to_string(&mut contents).expect("There was an error reading from the file");
                    let program = match program(CompleteStr(&contents)) {
                        Ok((remainder, program)) => {
                            program
                        },
                        Err(e) => {
                            println!("Unable to parse input: {:?}", e);
                            continue;
                        }
                    };
                    self.vm.program.append(&mut program.to_bytes());
                }
                _ => {
                    let program = match program(buffer.into()) {
                        Ok((_, program)) => program,
                        Err(e) => {
                            println!("Unable to parse input: {}", e);
                            continue;
                        }
                    };
                    self.vm.program.append(&mut program.to_bytes());
                    self.vm.run_once();
                }
            };
        }
    }

    /// Acceps a hexidecimal string WITHOUT a leading `0x` and returns a Vec of u8
    /// Example for a load command: 00 01 03 E8
    #[allow(dead_code)]
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(' ').collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                },
                Err(e) => {
                    return Err(e)
                }
            }
        }
        Ok(results)
    }
}
