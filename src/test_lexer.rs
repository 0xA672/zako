mod lexer;

use lexer::Lexer;

fn main() {
    let input = include_str!("../build.zig.zon");
    
    println!("=== Parsing build.zig.zon ===\n");
    println!("Input:\n{}\n", input);
    
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        match lexer.next_token() {
            Ok(token) => {
                if token == lexer::Token::Eof {
                    tokens.push(token);
                    break;
                }
                tokens.push(token);
            }
            Err(e) => {
                eprintln!("Lexer error: {}", e);
                std::process::exit(1);
            }
        }
    }
    
    println!("Total tokens: {}\n", tokens.len());
    for (i, token) in tokens.iter().enumerate() {
        println!("Token {}: {:?}", i, token);
    }
    
    println!("\n=== Success! ===");
}
