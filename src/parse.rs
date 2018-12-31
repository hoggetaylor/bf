use crate::lex::Token;
use failure::Fail;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expr {
    Move(i32),
    Add(i32),
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
            Token::MoveRight => Expr::Move(1),
            Token::MoveLeft => Expr::Move(-1),
            Token::Increment => Expr::Add(1),
            Token::Decrement => Expr::Add(-1),
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
            Expr::Add(1),
            Expr::Loop(vec![
                Expr::Add(1),
                Expr::Add(1),
                Expr::Add(1)
            ])
        ], parsed);
    }

    #[test]
    fn test_no_loop() {
        let tokens = vec![
            Token::Increment,
            Token::Increment,
            Token::Increment,
            Token::Increment
        ];
        let parsed = parse(&tokens).unwrap();
        assert_eq!(vec![
            Expr::Add(1),
            Expr::Add(1),
            Expr::Add(1),
            Expr::Add(1)
        ], parsed);
    }
}
