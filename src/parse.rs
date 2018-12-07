use crate::lex::Token;

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

#[derive(Debug)]
pub enum ParseError {
    UnmatchedBracket
}

pub fn parse(tokens: &[Token]) -> Result<Vec<Expr>, ParseError> {
    let mut program = Vec::new();
    let mut iter = tokens.iter();
    while let Some(token) = iter.next() {
        let instruction = match token {
            Token::MoveRight => Expr::MoveRight,
            Token::MoveLeft => Expr::MoveLeft,
            Token::Increment => Expr::Increment,
            Token::Decrement => Expr::Decrement,
            Token::Output => Expr::Output,
            Token::Input => Expr::Input,
            Token::Loop => Expr::Loop(parse_loop_body(&mut iter)?),
            Token::EndLoop => return Err(ParseError::UnmatchedBracket)
        };
        program.push(instruction);
    }
    Ok(program)
}

fn parse_loop_body(iter: &mut Iterator<Item=&Token>) -> Result<Vec<Expr>, ParseError> {
    let mut body = Vec::new();
    while let Some(token) = iter.next() {
        let instruction = match token {
            Token::MoveRight => Expr::MoveRight,
            Token::MoveLeft => Expr::MoveLeft,
            Token::Increment => Expr::Increment,
            Token::Decrement => Expr::Decrement,
            Token::Output => Expr::Output,
            Token::Input => Expr::Input,
            Token::Loop => Expr::Loop(parse_loop_body(iter)?),
            Token::EndLoop => return Ok(body)
        };
        body.push(instruction);
    }
    println!("End of loop body with no closing bracket");
    Err(ParseError::UnmatchedBracket)
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
