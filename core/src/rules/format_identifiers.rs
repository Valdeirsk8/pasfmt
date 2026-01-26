use std::collections::HashMap;

use crate::prelude::*;

pub struct FormatIdentifiers {
    /// HashMap: lowercase_identifier -> formatted_identifier
    identifiers: HashMap<String, String>,
}

impl FormatIdentifiers {
    pub fn new(identifiers: Vec<String>) -> Self {
        let map = identifiers
            .into_iter()
            .map(|id| (id.to_ascii_lowercase(), id))
            .collect();
        Self { identifiers: map }
    }
}

impl LogicalLineFileFormatter for FormatIdentifiers {
    fn format(&self, formatted_tokens: &mut FormattedTokens<'_>, _input: &[LogicalLine]) {
        if self.identifiers.is_empty() {
            return;
        }

        for (tok, _) in formatted_tokens.tokens_mut() {
            let Ok(tok) = tok else { continue };
            if matches!(tok.get_token_type(), TokenType::Identifier) {
                let content_lower = tok.get_content().to_ascii_lowercase();
                if let Some(formatted) = self.identifiers.get(&content_lower) {
                    if tok.get_content() != formatted {
                        tok.set_content(formatted.clone());
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn formatter(identifiers: Vec<&str>) -> Formatter {
        Formatter::builder()
            .lexer(DelphiLexer {})
            .parser(DelphiLogicalLineParser::default())
            .token_ignorer(FormattingToggler {})
            .file_formatter(FormatIdentifiers::new(
                identifiers.into_iter().map(String::from).collect(),
            ))
            .reconstructor(default_test_reconstructor())
            .build()
    }

    fn assert_format(identifiers: Vec<&str>, input: &str, expected: &str) {
        let fmt = formatter(identifiers);
        let output = fmt.format(input, FileOptions::new());
        assert_eq!(output, expected);
    }

    #[test]
    fn lowercase_self_to_pascal_case() {
        assert_format(vec!["Self"], "self.Value", "Self.Value");
    }

    #[test]
    fn uppercase_self_to_pascal_case() {
        assert_format(vec!["Self"], "SELF.Value", "Self.Value");
    }

    #[test]
    fn mixed_case_to_pascal_case() {
        assert_format(vec!["Self"], "SeLf.Value", "Self.Value");
    }

    #[test]
    fn result_formatting() {
        assert_format(vec!["Result"], "result := 1", "Result := 1");
    }

    #[test]
    fn multiple_identifiers() {
        assert_format(
            vec!["Self", "Result", "EmptyStr"],
            "self.Value := emptystr; result := self.Value",
            "Self.Value := EmptyStr; Result := Self.Value",
        );
    }

    #[test]
    fn already_correct_no_change() {
        assert_format(vec!["Self"], "Self.Value", "Self.Value");
    }

    #[test]
    fn unlisted_identifier_no_change() {
        assert_format(vec!["Self"], "other.Value", "other.Value");
    }

    #[test]
    fn empty_list_no_change() {
        assert_format(vec![], "self.Value", "self.Value");
    }

    #[test]
    fn keywords_not_affected() {
        assert_format(vec!["Begin", "End"], "begin end", "begin end");
    }

    #[test]
    fn ignored_tokens_not_affected() {
        assert_format(
            vec!["Self"],
            "{pasfmt off} self {pasfmt on} self",
            "{pasfmt off} self {pasfmt on} Self",
        );
    }
}
