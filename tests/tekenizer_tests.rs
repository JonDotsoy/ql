#[macro_use]
extern crate assert_matches;

#[cfg(test)]
mod tekenizer_tests {

    use insta::assert_debug_snapshot;
    use ql::Tokenizer;

    #[test]
    fn tekenizer() {
        let payload = r#"   SELECT  * FROM cars   "#;

        let tokens = Tokenizer::lexer(payload);

        assert_matches!(tokens, Ok(_));
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tekenizer_2() {
        let payload = r#"SELECT"#;

        let tokens = Tokenizer::lexer(payload);

        assert_matches!(tokens, Ok(_));
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tekenizer_lexer_string() {
        let payload = r#" "any world" try"#;

        let ref tokens = Tokenizer::lexer(payload);

        assert_matches!(tokens, Ok(_));
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tekenizer_lexer_numeric() {
        let payload = r#"9"#;

        let ref tokens = Tokenizer::lexer(payload);

        // assert_matches!(tokens, Ok(_));
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tekenizer_lexer_numeric_2() {
        let payload = r#"9.12"#;

        let ref tokens = Tokenizer::lexer(payload);

        // assert_matches!(tokens, Ok(_));
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tekenizer_lexer_numeric_3() {
        let payload = r#"9_123_123.122_123_943"#;

        let ref tokens = Tokenizer::lexer(payload);

        // assert_matches!(tokens, Ok(_));
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tekenizer_lexer_parenthesis_and_bracket() {
        let payload = r#"(var_1)[var_2]"#;

        let ref tokens = Tokenizer::lexer(payload);

        // assert_matches!(tokens, Ok(_));
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tekenizer_lexer_newline() {
        let payload = "abc\ndef";

        let ref tokens = Tokenizer::lexer(payload);

        // assert_matches!(tokens, Ok(_));
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tekenizer_lexer_newline_2() {
        let payload = "abc\r\ndef";

        let ref tokens = Tokenizer::lexer(payload);

        // assert_matches!(tokens, Ok(_));
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tekenizer_lexer_query_1() {
        let payload = "ctx.var_name:123 ";
        let ref tokens = Tokenizer::lexer(payload);
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_lexer_sample_1() {
        let payload = "search value";
        let ref tokens = Tokenizer::lexer(payload);
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_lexer_sample_2() {
        let payload = "search value condition:";
        let ref tokens = Tokenizer::lexer(payload);
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_lexer_sample_3() {
        let payload = r#"search value condition: "value""#;
        let ref tokens = Tokenizer::lexer(payload);
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_lexer_sample_4() {
        let payload = r#"
            search value
            condition: "value"
            context.condition > "value"
        "#;
        let ref tokens = Tokenizer::lexer(payload);
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_lexer_string_with_scape_char() {
        let payload = r#""i'm string \"""#;
        let ref tokens = Tokenizer::lexer(payload);
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_lexer_template_1() {
        let payload = "`abc${3}def`";
        let ref tokens = Tokenizer::lexer(payload);
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_lexer_string_template_1() {
        let payload = "`a${`b${\"c\"}d`}e`";
        let ref tokens = Tokenizer::lexer(payload);
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_lexer_string_template_2() {
        let payload = "`a$\\`b$\"c\"}d\\`}e`";
        let ref tokens = Tokenizer::lexer(payload);
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_lexer_string_template_3() {
        let payload = r#"`first string ${keyword} second string ${12_20 + `${"hola"}`}`"#;
        let ref tokens = Tokenizer::lexer(payload);
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_lexer_string_template_4() {
        let payload = r#"`a${b{c}d}e`"#;
        let ref tokens = Tokenizer::lexer(payload);
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_lexer_selector_with_variables() {
        let payload = r#"
            selector1.[$variable1].name
        "#;
        let tokens = Tokenizer::lexer(payload);
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_lexer_selector_with_variables_2() {
        todo!();
        // let payload = r#"
        //     $.user = $VAR && (
        //         $.tag[$TAG_NAME] = true ||
        //         $.tag[`${TAG_NAME}_alt`] = true
        //     )
        // "#;
        // let tokens = Tokenizer::lexer(payload);
        // assert_debug_snapshot!(tokens);
    }

    #[test]
    fn a() {
        trait A<'a> {
            fn get_a(&'a self, from: usize, at: usize) -> &str;
        }

        impl<'a> A<'a> for &'a str {
            fn get_a(&'a self, from: usize, at: usize) -> &str {
                let len = self.len();
                if at < from {
                    return "";
                }
                let real_from = if from < len { from } else { len };
                let real_at = if at < len { at } else { len };

                unsafe { self.get_unchecked(real_from..real_at) }
            }
        }

        impl<'a> A<'a> for String {
            fn get_a(&'a self, from: usize, at: usize) -> &str {
                let len = self.len();
                if at < from {
                    return "";
                }
                let real_from = if from < len { from } else { len };
                let real_at = if at < len { at } else { len };

                unsafe { self.get_unchecked(real_from..real_at) }
            }
        }

        let chars = "123456".get_a(0, 83);
        assert_debug_snapshot!(chars)
    }
}
