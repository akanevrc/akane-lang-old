mod lexer;

fn main() {
    println!("{:?}", lexer::lex("f a b = a + b".to_owned()).unwrap());
}
