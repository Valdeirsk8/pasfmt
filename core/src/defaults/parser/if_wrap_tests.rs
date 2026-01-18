use crate::prelude::*;
use crate::defaults::parser::DelphiLogicalLineParser;
use crate::traits::LogicalLineParser;

fn parse_with_if_wrap(input: &str) -> (Vec<LogicalLine>, Vec<Token<'_>>) {
    let lexer = DelphiLexer {};
    let tokens = lexer.lex(input);
    let parser = DelphiLogicalLineParser { wrap_single_statement_if: true };
    parser.parse(tokens)
}

fn reconstruct(lines: Vec<LogicalLine>, tokens: Vec<Token<'_>>) -> String {
    for (i, line) in lines.iter().enumerate() {
        println!("Line {}: Parent={:?}, Level={}, Tokens={:?}", i, line.get_parent(), line.get_level(), line.get_tokens());
    }
    let mut out = String::new();
    for token in tokens {
        out.push_str(token.get_content());
        out.push(' ');
    }
    out.trim().to_string()
}

#[test]
fn test_simple_if_wrap() {
    let input = "if A then B;";
    let (lines, tokens) = parse_with_if_wrap(input);
    let result = reconstruct(lines, tokens);
    assert!(result.contains("begin B ; end ;"), "Result was: {}", result);
}

#[test]
fn test_if_else_wrap() {
    let input = "if A then B else C;";
    let (lines, tokens) = parse_with_if_wrap(input);
    let result = reconstruct(lines, tokens);
    assert!(result.contains("begin B end else begin C ; end ;"), "Result was: {}", result);
}

#[test]
fn test_nested_if_wrap() {
    let input = "if A then if B then C;";
    let (lines, tokens) = parse_with_if_wrap(input);
    let result = reconstruct(lines, tokens);
    assert!(result.contains("begin if B then begin C ; end ; end ;"), "Result was: {}", result);
}

#[test]
fn test_already_wrapped_if() {
    let input = "if A then begin B; end;";
    let (lines, tokens) = parse_with_if_wrap(input);
    let result = reconstruct(lines, tokens);
    let occurrences = result.matches("begin").count();
    assert_eq!(occurrences, 1, "Should only have one begin. Result was: {}", result);
}

#[test]
fn test_if_else_if_wrap() {
    let input = "if A then B else if C then D else E;";
    let (lines, tokens) = parse_with_if_wrap(input);
    let result = reconstruct(lines, tokens);
    assert!(result.contains("else if C then begin D end else begin E ; end ;"), "Result was: {}", result);
}

#[test]
fn test_if_with_comments() {
    let input = "if A then {comment} B else // line comment\n C;";
    let (lines, tokens) = parse_with_if_wrap(input);
    let result = reconstruct(lines, tokens);
    assert!(result.contains("else // line comment begin C ; end ;"), "Result was: {}", result);
}
