use nom::types::CompleteStr;

use crate::assembler::Token;
use crate::assembler::Opcode;

named!(opcode_load<CompleteStr, Token>,
    do_parse!(
        tag!("load") >> (Token::Op{code: Opcode::LOAD})
    )
);

mod tests {
    use super::*;

    #[test]
    fn test_opcode_load() {
        // First tests the opcode is detected and parsed correctly
        let result = opcode_load(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op{code: Opcode::LOAD});
        assert_eq!(rest, CompleteStr(""));

        // Tests that an individual opcode isnt recognized
        let result = opcode_load(CompleteStr("aold"));
        assert_eq!(result.is_ok(), false);
    }
}
