#[allow(dead_code)]
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum LiteralKind {
    Int { value: i64 },     // 123
    Float { value: f64 }, // 123.456
    Char { value: char }, // 'a'
    Bool { value: bool }, // true or false
    String { value: String } // "hello world"
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Plus, // +
    Minus, // -
    Multiply, // *
    Divide, // /
    Carat, // ^
    LessThan, // <
    GreaterThan, // >
    And, // &
    Eq, // =
    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }
    Colon, // :
    Identifier(String), // keywords are also considered identifiers
    Literal(LiteralKind),
    Semi, // ;
    Arrow, // ->
}

#[derive(Debug, PartialEq)]
pub struct Location {
    pub line: usize
}

/// Consume characters from a Peekable<Chars> iterator while a condition Fn(char) -> bool is true
/// and return the consumed characters as a String
pub fn consume_while<F>(chars: &mut Peekable<Chars>, condition: F) -> String
where
    F: Fn(char) -> bool,
{
    let mut result = String::new();
    while let Some(&c) = chars.peek() {
        if condition(c) {
            result.push(c);
            chars.next();
        } else {
            break;
        }
    }
    result
}

pub fn tokenize_number(chars: &mut Peekable<Chars>) -> Token {
    let number = consume_while(chars, |c| c.is_ascii_digit() || c == '.');
    if number.contains('.') {
        Token::Literal(LiteralKind::Float { value: number.parse().unwrap() })
    } else {
        Token::Literal(LiteralKind::Int { value: number.parse().unwrap() })
    }
}

pub fn tokenize_string(chars: &mut Peekable<Chars>) -> Token {
    chars.next();
    let string = consume_while(chars, |c| c != '"');
    if chars.peek() != Some(&'"') {
        panic!("Unterminated string");
    }
    chars.next();
    Token::Literal(LiteralKind::String { value: string })
}

pub fn tokenize_char(chars: &mut Peekable<Chars>) -> Token {
    chars.next();
    let character = consume_while(chars, |c| c != '\'');
    if character.len() != 1 {
        panic!("Invalid character literal: {}", character);
    }
    chars.next();
    Token::Literal(LiteralKind::Char { value: character.chars().next().unwrap() })
}

pub fn tokenize_identifier(chars: &mut Peekable<Chars>) -> Token {
    let condition = |c: char| c.is_alphabetic() || c == '_' || c.is_ascii_digit() || c == '\'';
    let identifier = consume_while(chars, condition);
    match identifier.as_str() {
        "true" => Token::Literal(LiteralKind::Bool { value: true }),
        "false" => Token::Literal(LiteralKind::Bool { value: false }),
        _ => Token::Identifier(identifier),
    }
}

pub fn tokenize_minus(chars: &mut Peekable<Chars>) -> Token {
    chars.next();
    if let Some(&'>') = chars.peek() {
        chars.next();
        Token::Arrow
    } else {
        Token::Minus
    }
}

/// Tokenize the input string and return a vector of tokens with their locations
pub fn tokenize<T: AsRef<str>>(input: T) -> Result<Vec<(Token, Location)>, (String, Location)> {
    let mut tokens: Vec<(Token, Location)> = Vec::new();
    let mut chars = input.as_ref().chars().peekable();
    let mut line = 0;
    while let Some(&c) = chars.peek() {
        let token = match c {
            '0'..='9' => tokenize_number(&mut chars),
            '"' => tokenize_string(&mut chars),
            '\'' => tokenize_char(&mut chars),
            'a'..='z' | 'A'..='Z' => tokenize_identifier(&mut chars),
            '+' => { chars.next(); Token::Plus },
            '-' => tokenize_minus(&mut chars),
            '*' => { chars.next(); Token::Multiply },
            '/' => { chars.next(); Token::Divide },
            '^' => { chars.next(); Token::Carat },
            '(' => { chars.next(); Token::LParen },
            ')' => { chars.next(); Token::RParen },
            '{' => { chars.next(); Token::LBrace },
            '}' => { chars.next(); Token::RBrace },
            ':' => { chars.next(); Token::Colon },
            ';' => { chars.next(); Token::Semi },
            '<' => { chars.next(); Token::LessThan },
            '>' => { chars.next(); Token::GreaterThan },
            '&' => { chars.next(); Token::And },
            '=' => { chars.next(); Token::Eq },
            '!' => {
                consume_while(&mut chars, |c| c != '\n');
                continue;
            },
            _ if c.is_whitespace() => {
                if c == '\n' {
                    line += 1;
                }
                chars.next();
                continue;
            },
            _ => return Err((format!("Unexpected character: {}", c), Location { line }))
        };
        tokens.push((token, Location { line }));
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_while() {
        let mut chars = "hello world".chars().peekable();
        let result = consume_while(&mut chars, |c| c.is_alphabetic());
        assert_eq!(result, "hello");
    }

    #[test]
    fn literal_int() {
        let tokens = tokenize("123").unwrap();
        assert_eq!(tokens, vec![(Token::Literal(LiteralKind::Int { value: 123 }), Location { line: 0 })]);
    }

    #[test]
    fn literal_float() {
        let tokens = tokenize("123.456").unwrap();
        assert_eq!(tokens, vec![(Token::Literal(LiteralKind::Float { value: 123.456 }), Location { line: 0 })]);
    }

    #[test]
    fn literal_string() {
        let tokens = tokenize("\"hello world\"").unwrap();
        assert_eq!(tokens, vec![(Token::Literal(LiteralKind::String { value: "hello world".to_string() }), Location { line: 0 })]);
    }

    #[test]
    fn literal_char() {
        let tokens = tokenize("'a'").unwrap();
        assert_eq!(tokens, vec![(Token::Literal(LiteralKind::Char { value: 'a' }), Location { line: 0 })]);
    }

    #[test]
    fn identifier() {
        let tokens = tokenize("hello").unwrap();
        assert_eq!(tokens, vec![(Token::Identifier("hello".to_string()), Location { line: 0 })]);
    }

    #[test]
    fn binary_operators() {
        let tokens = tokenize("+-*/^").unwrap();
        assert_eq!(tokens, vec![
            (Token::Plus, Location { line: 0 }),
            (Token::Minus, Location { line: 0 }),
            (Token::Multiply, Location { line: 0 }),
            (Token::Divide, Location { line: 0 }),
            (Token::Carat, Location { line: 0 })
        ]);
    }

    #[test]
    fn logical_operators() {
        let tokens = tokenize("<>&=").unwrap();
        assert_eq!(tokens, vec![
            (Token::LessThan, Location { line: 0 }),
            (Token::GreaterThan, Location { line: 0 }),
            (Token::And, Location { line: 0 }),
            (Token::Eq, Location { line: 0 })
        ]);
    }

    // all other tokens work the same way, so if the above tests pass, the rest should work too

    #[test]
    fn arrow_vs_minus() {
        let tokens = tokenize("-> -").unwrap();
        assert_eq!(tokens, vec![
            (Token::Arrow, Location { line: 0 }),
            (Token::Minus, Location { line: 0 })
        ]);
    }

    #[test]
    fn comments() {
        let tokens = tokenize("!hello world\n").unwrap();
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn checking_line_count() {
        let tokens = tokenize("hello\nworld").unwrap();
        assert_eq!(tokens, vec![
            (Token::Identifier("hello".to_string()), Location { line: 0 }),
            (Token::Identifier("world".to_string()), Location { line: 1 })
        ]);
    }
}