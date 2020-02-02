use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub enum AssemblerError {
    NoSegmentDeclarationFound{ instruction: u32 },
    StringConstantDeclaredWithoutLabel{ instruction: u32 },
    SymbolAlreadyDeclared,
    UnknownDirectiveFound{ directive: String },
    NonOpcodeInOpcodeField,
    InsufficientSections,
    ParseError{ error: String }
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            AssemblerError::NoSegmentDeclarationFound{ instruction } => {
                f.write_str(&format!("No segment declaration (e.g., .code, .data) prior to finding an opcode. Instruction # was {}", instruction))
            }
            AssemblerError::StringConstantDeclaredWithoutLabel{ instruction } => {
                f.write_str(&format!("Found a string constant without a corresponding label. Instruction # was {}", instruction))
            },
            AssemblerError::SymbolAlreadyDeclared => {
                f.write_str(&format!("This symbol was previously declared"))
            },
            AssemblerError::UnknownDirectiveFound{ directive } => {
                f.write_str(&format!("Invalid or unknown directive. Directive name was: {}", directive))
            },
            AssemblerError::NonOpcodeInOpcodeField => {
                f.write_str(&format!("A non-opcode was found in an opcode field"))
            },
            AssemblerError::InsufficientSections => {
                f.write_str(&format!("Less than two sections/segments were found in the code"))
            },
            AssemblerError::ParseError{ error } => {
                f.write_str(&format!("There was an error parsing the code: {}", error))
            }
        }
    }
}

impl Error for AssemblerError {
    fn description(&self) -> &str {
        match self {
            AssemblerError::NoSegmentDeclarationFound{ instruction: u32 } => {
                "No segment declaration (e.g., .code, .data) prior to finding an opcode. Instruction # was {}"
            },
            AssemblerError::StringConstantDeclaredWithoutLabel{ instruction: u32 } => {
                "Found a string constant without a corresponding label. Instruction # was {}"
            },
            AssemblerError::SymbolAlreadyDeclared => {
                "This symbol was previously declared"
            },
            AssemblerError::UnknownDirectiveFound{ directive: u32 } => {
                "Invalid or unknown directive. Directive name was: {}"
            },
            AssemblerError::NonOpcodeInOpcodeField => {
                "A non-opcode was found in an opcode field"
            },
            AssemblerError::InsufficientSections => {
                "Less than two sections/segments were found in the code"
            },
            AssemblerError::ParseError{ error: String } => {
                "There was an error parsing the code: {}"
            }
        }
    }
}
