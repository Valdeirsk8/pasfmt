use crate::prelude::*;
use crate::defaults::parser::DelphiLogicalLineParser;
use crate::traits::LogicalLineParser;

fn parse_with_all_wrap(input: &str) -> (Vec<LogicalLine>, Vec<Token<'_>>) {
    let lexer = DelphiLexer {};
    let tokens = lexer.lex(input);
    let parser = DelphiLogicalLineParser { 
        wrap_single_statement_if: true,
        wrap_single_statement_for: true,
        wrap_single_statement_while: true,
    };
    parser.parse(tokens)
}

fn reconstruct(lines: &[LogicalLine], tokens: &[Token<'_>]) -> String {
    let mut out = String::new();
    for (i, line) in lines.iter().enumerate() {
        let indent = "  ".repeat(line.get_level() as usize);
        let line_content: String = line.get_tokens().iter()
            .map(|&t_idx| tokens[t_idx].get_content())
            .collect::<Vec<_>>()
            .join(" ");
        out.push_str(&format!("L{}({}): {}{}\n", i, line.get_level(), indent, line_content));
    }
    out
}

#[test]
fn test_nested_if_in_for() {
    let input = "for i := 0 to 10 do if true then DoSomething;";
    let (lines, tokens) = parse_with_all_wrap(input);
    let result = reconstruct(&lines, &tokens);
    println!("{}", result);

    // Analyze levels
    // Expected (if fixed):
    // L0(0): for ... do
    // L1(0? or 1?): begin  <-- should ideally be 0 if aligned, or 1 if indented. 
    //                      User says currently it is biased to right.
    // L2(1): if ... then
    // L3(1): begin         <-- THIS is the complaint. Currently probably 2.
    // L4(2): DoSomething
    // L5(1): end
    // L6(0/1): end
    
    // We will verify the levels directly in the string
}

#[test]
fn test_nested_if_in_while() {
    let input = "while true do if true then DoSomething;";
    let (lines, tokens) = parse_with_all_wrap(input);
    let result = reconstruct(&lines, &tokens);
    println!("{}", result);
}
