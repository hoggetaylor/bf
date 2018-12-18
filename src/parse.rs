use crate::lex::Token;
use failure::Fail;

#[derive(Debug, Eq, PartialEq)]
pub enum Expr {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    Loop(Vec<Expr>)
}

#[derive(Debug, Fail)]
pub enum ParseError {
    #[fail(display="Unmatched Bracket")]
    UnmatchedBracket
}

pub fn parse(tokens: &[Token]) -> Result<Vec<Expr>, ParseError> {
    let mut stack = vec![Vec::new()];
    for token in tokens {
        let expr = match token {
            Token::MoveRight => Expr::MoveRight,
            Token::MoveLeft => Expr::MoveLeft,
            Token::Increment => Expr::Increment,
            Token::Decrement => Expr::Decrement,
            Token::Output => Expr::Output,
            Token::Input => Expr::Input,
            Token::Loop => {
                stack.push(Vec::new());
                continue;
            },
            Token::EndLoop => {
                let body = stack.pop().ok_or(ParseError::UnmatchedBracket)?;
                Expr::Loop(body)
            }
        };
        let exprs = stack.last_mut().ok_or(ParseError::UnmatchedBracket)?;
        exprs.push(expr);
    }

    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        Err(ParseError::UnmatchedBracket)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex::Token;

    #[test]
    fn test_parse() {
        let tokens = vec![
            Token::Increment,
            Token::Loop,
            Token::Increment,
            Token::Increment,
            Token::Increment,
            Token::EndLoop
        ];
        let parsed = parse(&tokens).unwrap();
        assert_eq!(vec![
            Expr::Increment,
            Expr::Loop(vec![
                Expr::Increment,
                Expr::Increment,
                Expr::Increment
            ])
        ], parsed);
    }
}
