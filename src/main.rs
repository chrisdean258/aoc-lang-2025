pub mod error;
pub mod lex;
pub mod location;
pub mod token;

fn main() {
    let lexer = lex::Lexer::new("test");
    for token in lexer {
        println!("{token:?}");
    }
}
