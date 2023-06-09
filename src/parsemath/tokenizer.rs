use std::iter::Peekable;
use std::str::Chars;

enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
    Caret,
    LeftParen,
    RightParen,
    Num(f64),
    EOF,
    InvalidChar

}
// pub struct TokenizerV1 {
//     expr: &str
// }
// The problem with this implementation, is that it wouldn't allow us to easily do the iteration we need to do, over the string slice.

// In order to do this, we have to make use of the Chars struct from the standard library.

pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>

    // Chars provides an iterator over string slices, which will allow us to use `peek()`, to look at the next char in the expresion.
        // pub struct Peekable<I: Iterator> {
        //     iter: I,
        //     peeked: Option<Option<I::Item>>
        // }
        
        // impl<I: Iterator> Peekable<T> {
        //     pub(in crate::iter) fn new(iter: I) -> Peekable<I> {
        //         Peekable {iter, peeked: None}
        //     }
        // }
}

impl<'a> Tokenizer<'a> {
    fn new(new_expr: &'a str) -> Self {
        Tokenizer {
            expr: new_expr.chars().peekable()
        }
    }

}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let next_char = self.expr.next();

        match next_char {
            Some('0'..'9') => {
                let mut number = next_char?.to_string();

                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_numeric() || next_char == &'.' {
                        number.push(self.expr.next()?)
                    } else if next_char == &'(' {
                        return None;
                    } else {
                        break;
                    }
                }

                Some(Token::Num(number.parse::<f64>().unwrap()))
            }
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::RightParen),
            Some(')') => Some(Token::LeftParen),
            None => Some(Token::EOF),
            Some(_) => InvalidChar
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos_int() {
        let mut tokenizer = Tokenizer::new("34");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.0))
    }
    #[test] 
    fn test_dec_num() {
        let mut tokenizer = Tokenizer::new("34.5");
        assert_eq!(tokenizer.next.unwrap(), Token::Num(34.5))
    }

    #[test]
    #[ignore]
    fn test_invalid_char() {
        let mut tokenizer = Tokenizer::new("%#@");

        assert_eq!(tokenizer.next().unwrap(), Token::InvalidChar)
    }
}
