#[cfg(test)]
mod ast_tests {
    use insta::assert_debug_snapshot;
    use ql::{ast::AST, Tokenizer};

    #[test]
    fn ast_parse_identifier_1() {
        let payload = "$.a";
        let tokens = Tokenizer::lexer(payload).unwrap();
        assert_debug_snapshot!(AST::parse(tokens));
    }

    #[test]
    fn ast_parse_identifier_2() {
        let payload = "$.a.b";
        let tokens = Tokenizer::lexer(payload).unwrap();
        assert_debug_snapshot!(AST::parse(tokens));
    }

    #[test]
    fn ast_parse_identifier_3() {
        let payload = r#"$.a["as\"d"]"#;
        let tokens = Tokenizer::lexer(payload).unwrap();
        assert_debug_snapshot!(AST::parse(tokens));
    }
}
