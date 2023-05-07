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
    Div
}

enum ParseError {
    InvalidNumber(String),
    InvalidCharacter(char)
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
        }
    }
}

fn get_first_number(iter : &mut iter::Peekable<str::Chars>)
-> Result<Option<f64>, ParseError>
{
    let mut num = String::with_capacity(8);
   
    while let Some(c) = iter.next_if(|c| c.is_numeric() || *c == '.') {
        num.push(c);
    }

    if num.len() == 0 {
        Ok(None)
    }
    else if let Ok(value) = num.parse::<f64>() {
        Ok(Some(value))
    }
    else {
        Err(ParseError::InvalidNumber(num))
    }
}

fn get_token(iter : &mut iter::Peekable<str::Chars>)
-> Result<Option<Token>, ParseError>
{
    loop {
        if let None = iter.next_if(|c| c.is_whitespace()) {
            break;
        }
    }

    match get_first_number(iter) {
        Ok(option) => {
            if let Some(value) = option {
                return Ok(Some(Token::Number(value)));
            }
        }
        Err(s) => { return Err(s); }
    };

    let c = match iter.next() {
        Some(v) => v,
        None => { return Ok(None); }
    };

    let token = match c {
        '(' => Token::LeftBracket,
        ')' => Token::RightBracket,
        '+' => Token::Add,
        '-' => Token::Sub,
        '*' => Token::Mul,
        '/' => Token::Div,
        other => { return Err(ParseError::InvalidCharacter(other)); }
    };

    Ok(Some(token))
}

fn tokenize(s : String) -> Result<Vec<Token>, ParseError> {
    let mut tokens = Vec::<Token>::new();
    let mut iter = s.chars().peekable();
    loop {
        let token = get_token(&mut iter)?;
        match token {
            Some(t) => { tokens.push(t); }
            None => { break; }
        };
    }

    Ok(tokens)
}

enum EvaluateError {
    UnexpectedToken(Token),
    UnexpectedEnding
}

/*
grammar:
    factor = number
    factor = (expression)
*/
fn evaluate_factor<T>(tokens : &mut iter::Peekable<T>)
-> Result<f64, EvaluateError>
where T: Iterator<Item = Token>
{
    if let Some(token) = tokens.next() {
        match token {
            Token::Number(value) => Ok(value),
            Token::LeftBracket => {
                let value = evaluate_expression(tokens)?;
                if let Some(token) = tokens.next() {
                    match token {
                        Token::RightBracket => Ok(value),
                        _ => Err(EvaluateError::UnexpectedToken(token))
                    }
                }
                else {
                    Err(EvaluateError::UnexpectedEnding)
                }
            }
            other => Err(EvaluateError::UnexpectedToken(other))
        }
    }
    else {
        Err(EvaluateError::UnexpectedEnding)
    }
}

/*
grammar:
    term = factor * term
    term = factor / term
    term = factor
*/
fn evaluate_term<T>(tokens : &mut iter::Peekable<T>) -> Result<f64, EvaluateError>
where T: Iterator<Item = Token>
{
    let value = evaluate_factor(tokens)?;

    if let Some(token) = tokens.next_if(
        |t| match t {
            Token::Mul | Token::Div => true,
            _ => false
        }
    ) {
        match token {
            Token::Mul => Ok(value * evaluate_term(tokens)?),
            Token::Div => Ok(value / evaluate_term(tokens)?),
            other => Err(EvaluateError::UnexpectedToken(other))
        }
    }
    else {
        Ok(value)
    }
}

/*
grammar:
    expression = term + expression
    expression = term - expression
    expression = term
*/
fn evaluate_expression<T>(tokens : &mut iter::Peekable<T>)
-> Result<f64, EvaluateError>
where T: Iterator<Item = Token>
{
    let value = evaluate_term(tokens)?;

    if let Some(token) = tokens.next_if(
        |t| match t {
            Token::Add | Token::Sub => true,
            _ => false
        }
    ) {
        match token {
            Token::Add => Ok(value + evaluate_expression(tokens)?),
            Token::Sub => Ok(value - evaluate_expression(tokens)?),
            other => Err(EvaluateError::UnexpectedToken(other))
        }
    }
    else {
        Ok(value)
    }
}

fn evaluate(tokens : Vec<Token>) -> Result<f64, EvaluateError>
//where T: iter::Iterator<Item = Token>
{
    let mut tokens = tokens.into_iter().peekable();
    let val = evaluate_expression(&mut tokens)?;
    match tokens.next() {
        None => Ok(val),
        Some(t) => Err(EvaluateError::UnexpectedToken(t))
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

        let tokens = match tokenize(input) {
            Err(e) => {
                match e {
                    ParseError::InvalidCharacter(c) => {
                        println!("Invalid character: {}", c);
                    }
                    ParseError::InvalidNumber(s) => {
                        println!("Invalid number: {}", {s});
                    }
                }
                continue;
            }
            Ok(t) => t
        };
        
        match evaluate(tokens) {
            Ok(value) => println!("{}", value),
            Err(token) => {println!("Couldn't evaluate that")}
        }
    }
}
