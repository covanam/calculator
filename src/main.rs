use std::str;
use std::fmt;
use std::io;
use std::io::Write;
use std::iter;

enum Token {
    Number(f64),
    LeftBracket,
    RightBracket,
    Add,
    Sub,
    Mul,
    Div,
    Invalid(char)
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(v) => write!(f, "{}", v),
            Token::LeftBracket => write!(f, "("),
            Token::RightBracket => write!(f, ")"),
            Token::Add => write!(f, "+"),
            Token::Sub => write!(f, "-"),
            Token::Mul => write!(f, "*"),
            Token::Div => write!(f, "/"),
            Token::Invalid(c) => write!(f, "Invalid({})", c),
        }
    }
}

fn get_first_number(iter : &mut iter::Peekable<str::Chars>) -> Option<f64> {
    let mut num = String::with_capacity(8);
   
    while let Some(c) = iter.next_if(|c| c.is_numeric() || *c == '.') {
        num.push(c);
    }

    if let Ok(value) = num.parse::<f64>() {
        Some(value)
    }
    else {
        None
    }
}

fn get_token(iter : &mut iter::Peekable<str::Chars>) -> Option<Token> {
    loop {
        if let None = iter.next_if(|c| c.is_whitespace()) {
            break;
        }
    }

    if let Some(value) = get_first_number(iter) {
        return Some(Token::Number(value));
    }

    let c = match iter.next() {
        Some(v) => v,
        None => { return None; }
    };

    let token = match c {
        '(' => Token::LeftBracket,
        ')' => Token::RightBracket,
        '+' => Token::Add,
        '-' => Token::Sub,
        '*' => Token::Mul,
        '/' => Token::Div,
        other => Token::Invalid(other)
    };

    Some(token)
}

fn tokenize(s : String) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    let mut iter = s.chars().peekable();
    loop {
        let token = get_token(&mut iter);
        match token {
            Some(t) => { tokens.push(t); }
            None => { break; }
        };
    }

    tokens
}

/*
grammar:
    factor = number
    factor = (expression)
    factor = + factor
    factor = - factor
*/
fn evaluate_factor<T>(tokens : &mut iter::Peekable<T>) -> Option<f64>
where T: Iterator<Item = Token>
{
    if let Some(token) = tokens.next() {
        match token {
            Token::Number(value) => Some(value),
            Token::LeftBracket => {
                let value = evaluate_expression(tokens)?;
                if let Some(token) = tokens.next() {
                    match token {
                        Token::RightBracket => Some(value),
                        _ => None
                    }
                }
                else {
                    None
                }
            }
            Token::Add => evaluate_factor(tokens),
            Token::Sub => {
                match evaluate_factor(tokens) {
                    Some(value) => Some(-value),
                    None => None
                }
            }
            other => None
        }
    }
    else {
        None
    }
}

/*
grammar:
    term = factor
    term' = * factor term'
          | / factor term
          | nothing
*/
fn evaluate_term<T>(tokens : &mut iter::Peekable<T>) -> Option<f64>
where T: Iterator<Item = Token>
{
    let mut value = evaluate_factor(tokens)?;

    loop {
        if let Some(token) = tokens.peek() {
            match token {
                Token::Mul => {
                    tokens.next()?;
                    value *= evaluate_factor(tokens)?;
                }
                Token::Div => {
                    tokens.next()?;
                    value /= evaluate_factor(tokens)?;
                }
                _ => { return Some(value); }
            }
        }
        else {
            return Some(value);
        }
    }
}

/*
grammar:
    expression = term expression'
    expression' = + term expression'
                | - term expression'
                | nothing
*/
fn evaluate_expression<T>(tokens : &mut iter::Peekable<T>) -> Option<f64>
where T: Iterator<Item = Token>
{
    let mut value = evaluate_term(tokens)?;

    loop {
        if let Some(token) = tokens.peek() {
            match token {
                Token::Add => {
                    tokens.next()?;
                    value += evaluate_term(tokens)?;
                }
                Token::Sub => {
                    tokens.next()?;
                    value -= evaluate_term(tokens)?;
                }
                _ => { return Some(value); }
            }
        }
        else {
            return Some(value);
        }
    }
}

fn evaluate<T>(tokens : T) -> Option<f64> where T: iter::Iterator<Item = Token> {
    let mut tokens = tokens.into_iter().peekable();
    let val = evaluate_expression(&mut tokens)?;
    match tokens.next() {
        None => Some(val),
        Some(_) => None
    }
}

fn main() {
    loop {
        let mut input = String::new();

        print!(">> ");
        if let Err(e) = io::stdout().flush() {
            panic!("Flush error: {}", e);
        }

        io::stdin().read_line(&mut input).expect("Something wrong");

        let tokens = tokenize(input).into_iter();

        match evaluate(tokens) {
            Some(value) => println!("{}", value),
            None => println!("Syntax error")
        }
    }
}
