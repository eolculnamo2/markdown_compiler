mod lexer;
mod parser;

fn main() {
    let test_raw = "#foo bar bajdasd **i am bold** *I am italic* and ***I am bold and italic*** \n test **test** test \n ##Test test *test*".to_string();
    let tokens = lexer::start(test_raw);
    let ast = parser::start(tokens);
    // tokens.iter().for_each(|t| println!("{}", t.value ));
}

