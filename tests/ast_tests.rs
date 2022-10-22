#[cfg(test)]
mod ast_tests {
    use insta::assert_debug_snapshot;
    use ql::{ast::AST, Tokenizer};

    #[test]
    fn ast_parse_identifier_1() {
        let payload = "$.a.b";

        let tokens = Tokenizer::lexer(payload).unwrap();

        assert_debug_snapshot!(AST::parse(tokens));
    }
} 
