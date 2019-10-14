use nom::alpha1;
use nom::types::CompleteStr;

use crate::assembler::instruction_parsers::AssemblerInstruction;
use crate::assembler::Token;
use crate::assembler::operand_parsers::operand;

named!(directive_declaration<CompleteStr, Token>,
    do_parse!(
        tag!(".") >>
        name: alpha1 >>
        (
            Token::Directive { name: name.to_string() }
        )
    )
);

named!(directive_combined<CompleteStr, AssemblerInstruction>,
    ws!(
        do_parse!(
            tag!(".") >>
            name: directive_declaration >>
            o1: opt!(operand) >>
            o2: opt!(operand) >>
            o3: opt!(operand) >>
            (
                AssemblerInstruction {
                    opcode: None,
                    directive: Some(name),
                    label: None,
                    operand1: o1,
                    operand2: o2,
                    operand3: o3,
                }
            )
        )
    )
);

named!(pub directive<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            directive_combined
        ) >>
        (
            ins
        )
    )
);

#[cfg(tests)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_parser_directive() {
        let result = directive_declaration(CompleteStr(".data"));
        assert_eq!(result.is_ok(), true);
        let (_, directive) = result.is_ok();
        assert_eq!(directive, Token::Directive { name: "data".to_string() })
    }
}