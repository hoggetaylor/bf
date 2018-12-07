#[derive(Debug, Eq, PartialEq)]
/// Represents a single token in a BrainFuck program.
pub enum Token {
    /// >
    /// Increment the data pointer to the next cell to the right.
    MoveRight,
    /// <
    /// Decrement the data pointer to the next cell to the left.
    MoveLeft,
    /// +
    /// Increment the byte at the data pointer.
    Increment,
    /// -
    /// Decrement the byte at the data pointer.
    Decrement,
    /// .
    /// Output the byte at the data pointer.
    Output,
    /// ,
    /// Accept one byte of input, storing it's value in the byte at the data pointer.
    Input,
    /// [
    /// If the byte at the data pointer is 0, jump to the next command, else jump to after the matching ']'
    Loop,
    /// ]
    /// If the byte at the data pointer is nonzero, jump back to the command after the matching '[',
    /// else continue to the next command.
    EndLoop
}

/// Tokenizes a brainfuck string.
pub fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::with_capacity(s.len());
    for c in s.chars() {
        if let Some(token) = tokenize_char(c) {
            tokens.push(token);
        }
    }
    tokens
}

fn tokenize_char(c: char) -> Option<Token> {
    let t = match c {
        '>' => Token::MoveRight,
        '<' => Token::MoveLeft,
        '+' => Token::Increment,
        '-' => Token::Decrement,
        '.' => Token::Output,
        ',' => Token::Input,
        '[' => Token::Loop,
        ']' => Token::EndLoop,
        _ => return None
    };
    Some(t)
}
