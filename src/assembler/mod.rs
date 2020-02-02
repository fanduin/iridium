use nom::types::CompleteStr;

pub mod opcode_parsers;
pub mod register_parsers;
pub mod operand_parsers;
pub mod instruction_parsers;
pub mod program_parsers;
pub mod label_parsers;
pub mod directive_parsers;
pub mod assembler_errors;
pub mod symbols;

use crate::instruction::Opcode;
use program_parsers::{program, Program};
use instruction_parsers::{AssemblerInstruction};
use assembler_errors::AssemblerError;
use symbols::{Symbol, SymbolTable, SymbolType};

pub const PIE_HEADER_PREFIX: [u8; 4] = [45, 50, 49, 45];
pub const PIE_HEADER_LENGTH: usize = 64;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op {code: Opcode},
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
    IrString { name: String }
}

#[derive(Debug, Default)]
pub struct Assembler {
    phase: AssemblerPhase,
    pub symbols: SymbolTable,
    pub ro: Vec<u8>,
    pub bytecode: Vec<u8>,
    ro_offset: u32,
    sections: Vec<AssemblerSection>,
    current_section: Option<AssemblerSection>,
    current_instruction: u32,
    errors: Vec<AssemblerError>
}

#[derive(Debug, PartialEq)]
pub enum AssemblerPhase {
    First,
    Second
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssemblerSection {
    Data { starting_instruction: Option<u32> },
    Code { starting_instruction: Option<u32> },
    Unknown
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            current_instruction: 0,
            ro_offset: 0,
            ro: vec![],
            bytecode: vec![],
            sections: vec![],
            errors: vec![],
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new(),
            current_section: None
        }
    }

    pub fn assemble(&mut self, raw: &str) -> Result<Vec<u8>, Vec<AssemblerError>> {
        match program(CompleteStr(raw)) {
            Ok((_remainder, program)) => {
                let mut assembled_program = self.write_pie_header();
                self.process_first_phase(&program);

                if !self.errors.is_empty() {
                    return Err(self.errors.clone());
                }

                if self.sections.len() != 2 {
                    println!("Did not find at least two sections.");
                    self.errors.push(AssemblerError::InsufficientSections);
                    return Err(self.errors.clone());
                }

                let mut body = self.process_second_phase(&program);

                assembled_program.append(&mut body);
                Ok(assembled_program)
            },
            Err(e) => {
                println!("There was an error assembling the code: {:?}", e);
                Err(vec![AssemblerError::ParseError{ error: e.to_string() }])
            }
        }
    }

    fn process_first_phase(&mut self, p: &Program) {
        for i in &p.instructions {
            if i.is_label() {
                if self.current_section.is_some() {
                    self.process_label_declaration(&i);
                } else {
                    self.errors.push(AssemblerError::NoSegmentDeclarationFound{ instruction: self.current_instruction });
                }
            }

            if i.is_directive() {
                self.process_directive(i);
            }
            
            self.current_instruction += 1;
        }
        self.phase = AssemblerPhase::Second;
    }

    fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
        self.current_instruction = 0;
        let mut program = vec![];

        for i in &p.instructions {
            if i.is_opcode() {
                let mut bytes = i.to_bytes(&self.symbols);
                program.append(&mut bytes);
            }
            if i.is_directive() {
                self.process_directive(i);
            }
            self.current_instruction += 1;
        }
        program
    }

    fn process_label_declaration(&mut self, i: &AssemblerInstruction) {
        let name = match i.get_label_name() {
            Some(name) => { name },
            None => {
                self.errors.push(AssemblerError::StringConstantDeclaredWithoutLabel{ instruction: self.current_instruction });
                return;
            }
        };

        if self.symbols.has_symbol(&name) {
            self.errors.push(AssemblerError::SymbolAlreadyDeclared);
            return;
        }

        let symbol = Symbol::new(name, SymbolType::Label);
        self.symbols.add_symbol(symbol);
    }

    fn process_directive(&mut self, i: &AssemblerInstruction) {
        let directive_name = match i.get_directive_name() {
            Some(name) => {
                name
            },
            None => {
                println!("Directive has an invalid name: {:?}", i);
                return;
            }
        };

        if i.has_operands() {
            match directive_name.as_ref() {
                "asciiz" => {
                    self.handle_asciiz(i);
                },
                _ => {
                    self.errors.push(AssemblerError::UnknownDirectiveFound{ directive: directive_name.clone() });
                    return;
                }
            }
        } else {
            self.process_section_header(&directive_name);
        }
    }

    fn process_section_header(&mut self, header_name: &str) {
        let new_section: AssemblerSection = header_name.into();
        if new_section == AssemblerSection::Unknown {
            println!("Found a section  header is unknown: {:?}", header_name);
            return;
        }

        self.sections.push(new_section.clone());
        self.current_section = Some(new_section);
    }

    fn handle_asciiz(&mut self, i: &AssemblerInstruction) {
        if self.phase != AssemblerPhase::First { return; };

        match i.get_string_constant() {
            Some(s) => {
                match i.get_label_name() {
                    Some(name) => { self.symbols.set_symbol_offset(&name, self.ro_offset); },
                    None => {
                        println!("Found a string constant with no associated label!");
                        return;
                    }
                };

                for byte in s.as_bytes() {
                    self.ro.push(*byte);
                    self.ro_offset += 1;
                }

                self.ro.push(0);
                self.ro_offset += 1;
            },
            None => {
                println!("String constant following an .asciiz was empty");
            }
        }
    }

    fn write_pie_header(&self) -> Vec<u8> {
        let mut header = vec![];
        for byte in &PIE_HEADER_PREFIX {
            header.push(byte.clone());
        }
        while header.len() < PIE_HEADER_LENGTH {
            header.push(0 as u8);
        }

        println!("Header length: {}", header.len());
        header
    }
}

impl Default for AssemblerPhase {
    fn default() -> AssemblerPhase {
        AssemblerPhase::First
    }
}

impl Default for AssemblerSection {
    fn default() -> Self {
        AssemblerSection::Unknown
    }
}

impl<'a> From<&'a str> for AssemblerSection {
    fn from(name: &str) -> AssemblerSection {
        match name {
            "data" => {
                AssemblerSection::Data { starting_instruction: None }
            }
            "code" => {
                AssemblerSection::Code { starting_instruction: None }
            }
            _ => {
                AssemblerSection::Unknown
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::VM;
    #[test]
    fn test_symbol_table() {
        let mut sym = SymbolTable::default();
        let new_symbol = Symbol::new_with_offset("test".to_string(), SymbolType::Label, 12);
        sym.add_symbol(new_symbol);
        assert_eq!(sym.symbols.len(), 1);
        let v = sym.symbol_value("test");
        assert_eq!(true, v.is_some());
        let v = v.unwrap();
        assert_eq!(v, 12);
        let v = sym.symbol_value("does_not_exist");
        assert_eq!(false, v.is_some());
    }

    #[test]
    fn test_assemble_program() {
        let mut asm = Assembler::new();
        let test_string = ".data\n.code\nload $0 #100\nload $1 #1\nload $2 #0\ntest: inc $0\nneq $0 $2\njmpe @test\nhlt";
        // let test_string = "test: inc $0\njmp test";
        println!("Attempting to assemble: {:?}", test_string);
        let program = asm.assemble(test_string).unwrap();
        println!("Assembled Program: {:?}", program);
        let mut vm = VM::default();
        assert_eq!(program.len(), 92);
        vm.add_bytes(program);
        assert_eq!(vm.program.len(), 92)
    }

    #[test]
    fn test_ro_data() {
        let mut asm = Assembler::new();
        let test_string = ".data\ntest: .asciiz 'This is a test'\n.code\n";
        let program = asm.assemble(test_string);
        assert_eq!(program.is_ok(), true);
    }

    #[test]
    fn test_bad_ro_data() {
        let mut asm = Assembler::new();
        let test_string = ".code\ntest: .asciiz 'This is a test'\n.wrong\n";
        let program = asm.assemble(test_string);
        assert_eq!(program.is_ok(), false);
    }

    #[test]
    fn test_first_phase_no_segment() {
        let mut asm = Assembler::new();
        let test_string = "hello: .asciiz 'Fail'";
        let result = program(CompleteStr(test_string));
        assert_eq!(result.is_ok(), true);
        let (_, p) = result.unwrap();
        asm.process_first_phase(&p);
        assert_eq!(asm.errors.len(), 1);
    }

    #[test]
    fn test_first_phase_inside_segment() {
        let mut asm = Assembler::new();
        let test_string = ".data\ntest: .asciiz 'Hello'";
        let result = program(CompleteStr(test_string));
        assert_eq!(result.is_ok(), true);
        let (_, p) = result.unwrap();
        asm.process_first_phase(&p);
        assert_eq!(asm.errors.len(), 0);
    }
}
