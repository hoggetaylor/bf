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
    let mut program = Vec::new();
    let mut stack = Vec::new();
    for token in tokens {
        let expr = match token {
            Token::MoveRight => Some(Expr::MoveRight),
            Token::MoveLeft => Some(Expr::MoveLeft),
            Token::Increment => Some(Expr::Increment),
            Token::Decrement => Some(Expr::Decrement),
            Token::Output => Some(Expr::Output),
            Token::Input => Some(Expr::Input),
            Token::Loop => {
                stack.push(Vec::new());
                None
            },
            Token::EndLoop => {
                let body = stack.pop().ok_or(ParseError::UnmatchedBracket)?;
                Some(Expr::Loop(body))
            }
        };
        if let Some(expr) = expr {
            if stack.len() > 0 {
                stack.last_mut().unwrap().push(expr);
            } else {
                program.push(expr);
            }
        }
    }
    if !stack.is_empty() {
        return Err(ParseError::UnmatchedBracket);
    }
    Ok(program)
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
