use crate::prelude::*;
use crate::defaults::parser::DelphiLogicalLineParser;
use crate::traits::LogicalLineParser;

fn parse_with_loop_wrap(input: &str) -> (Vec<LogicalLine>, Vec<Token<'_>>) {
    let lexer = DelphiLexer {};
    let tokens = lexer.lex(input);
    let parser = DelphiLogicalLineParser { 
        wrap_single_statement_if: false,
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
fn test_simple_for_wrap_structure() {
    let input = "for i := 1 to 10 do writeln(i);";
    let (lines, tokens) = parse_with_loop_wrap(input);
    let result = reconstruct(&lines, &tokens);
    println!("{}", result);
    
    // Check for begin/end and indentation
    // Expected structure (approximate levels):
    // L0: for ... do
    // L1:   begin
    // L2:     writeln ( i ) ;
    // L1:   end ;
    
    // Note: The specific level numbers might vary starting at 0 or 1, but relative levels matter.
    assert!(result.contains("begin\n"), "Missing begin on its own line/block");
    assert!(result.contains("end ;\n"), "Missing end on its own line/block");
    
    // Basic verification of wrapping
    let output_str_only_tokens = tokens.iter().map(|t| t.get_content()).collect::<Vec<_>>().join(" ");
    assert!(output_str_only_tokens.contains("do begin writeln ( i ) ; end ;"));
}

#[test]
fn test_simple_while_wrap_structure() {
    let input = "while True do writeln('hit');";
    let (lines, tokens) = parse_with_loop_wrap(input);
    let result = reconstruct(&lines, &tokens);
    println!("{}", result);

    assert!(result.contains("begin\n"));
    assert!(result.contains("end ;"));
}

#[test]
fn test_nested_loop_wrap_structure() {
    // for ... do 
    //   for ... do 
    //     writeln
    
    // Should become:
    // for ... do
    //   begin
    //     for ... do
    //       begin
    //         writeln
    //       end
    //   end
    
    let input = "for i := 1 to 10 do for j := 1 to 10 do writeln(i, j);";
    let (lines, tokens) = parse_with_loop_wrap(input);
    let result = reconstruct(&lines, &tokens);
    println!("{}", result);

    let occurrences_begin = result.matches("begin").count();
    assert_eq!(occurrences_begin, 2);
}

#[test]
fn test_loop_already_wrapped_no_double_wrap() {
    let input = "for i := 1 to 10 do begin writeln(i); end;";
    let (lines, tokens) = parse_with_loop_wrap(input);
    let result = reconstruct(&lines, &tokens);
    println!("{}", result);

    let occurrences_begin = result.matches("begin").count();
    assert_eq!(occurrences_begin, 1);
}

#[test]
fn test_loop_multiline_input() {
    let input = r#"
    for i := 0 to 10 do
      DoSomething;
    "#;
    let (lines, tokens) = parse_with_loop_wrap(input);
    let result = reconstruct(&lines, &tokens);
    println!("{}", result);
    
    assert!(result.contains("begin"));
    assert!(result.contains("end"));
}
