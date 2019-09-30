use nom::types::CompleteStr;
use nom::digit;

use assembler::Token;

named!(register<CompleteStr, Token>, <1>
    ws!( <2>
        do_parse!( <3>
            tag!("$") >> <4>
            reg_num: digit >> <5>
            ( <6>
                Token::Register{ <7>
                    reg_num: reg_num.parse::<u8>().unwrap() <8>
                } <9>
            ) <10>
        )
    )
);



mod tests {
    #[test]
    fn test_parse_register() {
        let result = register(CompleteStr("$0"));
        assert_eq!(result.is_ok(), true);
        let result = register(CompleteStr("0"));
        assert_eq!(result.is_ok(), false);
        let result = register(CompleteStr("$a"));
        assert_eq!(result.is_ok(), false);
    }
}