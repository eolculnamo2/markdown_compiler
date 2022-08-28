#[derive(Clone)]
enum TokenType {
    WhiteSpace,
    NewLine,
    H1,
    H2,
    H3,
    Bold,
    Italic,
    Character, // part of a string and only exists in lexer
    Text, // String of consecutive Characters 
}

#[derive(Clone)]
struct Token {
    token_type: TokenType,
    value: String,
    start: u32, // might be able to get rid of these but don't want to yet
    end: u32,
}

fn characters_to_texts(char_tokens: Vec<Token>) -> Vec<Token> {
    let mut text_tokens: Vec<Token> = Vec::new();
    
    let mut current_text: Vec<Token> = Vec::new();
    for i in 0..char_tokens.len() {
        let ch = char_tokens.get(i).unwrap().clone();
        let should_push = current_text.is_empty() || ch.end - 1 == current_text.last().unwrap().end; 
        if should_push {
            current_text.push(ch);
        } else {

            // let combined_chars: String = current_text.into_iter().reduce(|agg, cur| format!("{}{}", agg.value, cur.value)).unwrap();
            let combined_chars: String = current_text.clone().into_iter()
                .map(|x| x.value)
                .reduce(|a, b| {
                    format!("{}{}",a,b)
                })
                .unwrap()
                .to_string();
            text_tokens.append(&mut vec![Token {
                token_type: TokenType::Text,
                value: combined_chars,
                start: current_text.clone()
                    .into_iter()
                    .next()
                    .unwrap()
                    .start,
                end: ch.clone().end,
            }]);
            current_text = vec![ch];
        }
    }
    println!("Found {} unique texts", text_tokens.len());
    text_tokens
}

fn lexer(raw: String) -> Vec<Token> {
    let chars: Vec::<&str> = raw.split("").collect();
    let mut tokens: Vec<Token> = Vec::new();
    let mut char_tokens: Vec<Token> = Vec::new();
    let mut skip_next = false;
    let mut bold_stack: Vec<u32> = Vec::new();
    let mut italic_stack: Vec<u32> = Vec::new();
    for i in 0..chars.len() {
        if skip_next {
            skip_next = false;
            continue;
        }

        let c = chars.get(i).unwrap();
        // this casting is probably bad
        let index: u32 = i as u32;
        // check for whitespace
        if c == &" " {
            tokens.push(Token { token_type: TokenType::WhiteSpace, value: " ".to_string(), start: index, end: index + 1});
            continue;
        }
        if c == &"\n" {
            tokens.push(Token { token_type: TokenType::NewLine, value: "\n".to_string(), start: index, end: index + 1 });
        }
        // must handle lookahead
        if c == &"#" && i == 0 {
            let second_space = chars.get(i + 1).unwrap_or(&"");
            let third_space = chars.get(i + 2).unwrap_or(&"");
            match (second_space, third_space) {
                (&"#", &"#") => tokens.push(Token { token_type: TokenType::H3, value: "h3".to_string(), start: index, end: index }),
                (&"#", _) => tokens.push(Token { token_type: TokenType::H2, value: "h2".to_string(), start: index, end: index }),
                _ => tokens.push(Token { token_type: TokenType::H1, value: "h1".to_string(), start: index, end: index })
            }
            continue;
        }
        // handle Bold and italic with lookahead
        if c == &"*" {
            // must make sure we don't count the same string twice.
            // ***foo*** is corner case where is both bold and italic
            let space_ahead = chars.get(i + 1);
            match space_ahead {
                // BOLD CASE
                Some(s) if s == &"*" => {
                    // possibly start of bold token? 
                    if !bold_stack.is_empty() {
                        tokens.push(Token { token_type: TokenType::Bold, value: "**".to_string(), start: index, end: index });     
                        let popped_index = bold_stack.pop().unwrap();
                        tokens.push(Token { token_type: TokenType::Bold, value: "**".to_string(), start: popped_index, end: popped_index }); 
                    }
                    else {
                        bold_stack.push(index);
                    }
                    // shouldnt need to check the next character since its also ** which we're
                    // accounting for here.
                    skip_next = true;
                },
                // ITALIC CASE
                _ => {
                    // possibly start of italic token
                    if !italic_stack.is_empty() {
                        tokens.push(Token { token_type: TokenType::Italic, value: "*".to_string(), start: index, end: index });     
                        let popped_index = italic_stack.pop().unwrap();
                        tokens.push(Token { token_type: TokenType::Italic, value: "*".to_string(), start: popped_index, end: popped_index }); 
                    }
                    else {
                        italic_stack.push(index);
                    }
                } 
            }
            continue;
        }
        
        // rest are treated as characters
        char_tokens.push(Token { token_type: TokenType::Character, value: c.to_string(), start: index, end: index });
    }
    
    // handle remaining stacks.. if no matches found previously, assumed to be characters.
    bold_stack.iter().for_each(|current| {
        println!("Bold Left over {} ", current);
        char_tokens.push(Token { token_type: TokenType::Character, value: "**".to_string(), start: current.clone(), end: current.clone() })
    });

    italic_stack.iter().for_each(|current| {
        println!("Italic Leftover {} ", current);
        char_tokens.push(Token { token_type: TokenType::Character, value: "*".to_string(), start: current.clone(), end: current.clone() })
    });
    let text_tokens = characters_to_texts(char_tokens);
    [tokens.as_slice(), text_tokens.as_slice()].concat()
}

fn parser(tokens: Vec<Token>) {
    // form words from characters between symbols
    // structure this in ast hierarcy
}


fn main() {
    let test_raw = "#foo bar bajdasd **i am bold** *I am italic* and ***I am bold and italic***".to_string();
    let tokens = lexer(test_raw);
    // tokens.iter().for_each(|t| println!("{}", t.value ));
}

