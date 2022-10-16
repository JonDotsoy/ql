#[macro_use]
extern crate assert_matches;

#[cfg(test)]
mod tekenizer_tests {

    use insta::assert_debug_snapshot;
    use ql::QL;

    #[test]
    fn tekenizer() {
        let payload = r#"   SELECT  * FROM cars   "#;

        let tokens = QL::lexer(payload);

        assert_matches!(tokens, Ok(_));
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tekenizer_2() {
        let payload = r#"SELECT"#;

        let tokens = QL::lexer(payload);

        assert_matches!(tokens, Ok(_));
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tekenizer_lexer_string() {
        let payload = r#" "any world" try"#;

        let ref tokens = QL::lexer(payload);

        assert_matches!(tokens, Ok(_));
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tekenizer_lexer_numeric() {
        let payload = r#"9"#;

        let ref tokens = QL::lexer(payload);

        // assert_matches!(tokens, Ok(_));
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tekenizer_lexer_numeric_2() {
        let payload = r#"9.12"#;

        let ref tokens = QL::lexer(payload);

        // assert_matches!(tokens, Ok(_));
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tekenizer_lexer_numeric_3() {
        let payload = r#"9_123_123.122_123_943"#;

        let ref tokens = QL::lexer(payload);

        // assert_matches!(tokens, Ok(_));
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tekenizer_lexer_parenthesis_and_bracket() {
        let payload = r#"(var_1)[var_2]"#;

        let ref tokens = QL::lexer(payload);

        // assert_matches!(tokens, Ok(_));
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tekenizer_lexer_newline() {
        let payload = "abc\ndef";

        let ref tokens = QL::lexer(payload);

        // assert_matches!(tokens, Ok(_));
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tekenizer_lexer_newline_2() {
        let payload = "abc\r\ndef";

        let ref tokens = QL::lexer(payload);

        // assert_matches!(tokens, Ok(_));
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tekenizer_lexer_query_1() {
        let payload = "ctx.var_name:123 ";
        let ref tokens = QL::lexer(payload);
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_lexer_sample_1() {
        let payload = "search value";
        let ref tokens = QL::lexer(payload);
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_lexer_sample_2() {
        let payload = "search value condition:";
        let ref tokens = QL::lexer(payload);
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_lexer_sample_3() {
        let payload = r#"search value condition: "value""#;
        let ref tokens = QL::lexer(payload);
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_lexer_sample_4() {
        let payload = r#"
            search value
            condition: "value"
            context.condition > "value"
        "#;
        let ref tokens = QL::lexer(payload);
        assert_debug_snapshot!(tokens);
    }
}
