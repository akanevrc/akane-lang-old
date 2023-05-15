mod lexer;
mod parser;

fn main() {
    println!("{:?}", parser::parse(lexer::lex("f a b = a + b".to_owned()).unwrap()).unwrap());
}
