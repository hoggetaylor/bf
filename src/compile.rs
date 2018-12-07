use crate::parse::Expr;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Instruction {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    Loop(usize),
    EndLoop(usize)
}

pub fn compile(exprs: &[Expr], offset: usize) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for e in exprs {
        let mut ins = match *e {
            Expr::MoveRight => vec![Instruction::MoveRight],
            Expr::MoveLeft => vec![Instruction::MoveLeft],
            Expr::Increment => vec![Instruction::Increment],
            Expr::Decrement => vec![Instruction::Decrement],
            Expr::Output => vec![Instruction::Output],
            Expr::Input => vec![Instruction::Input],
            Expr::Loop(ref body) => {
                let loop_begin = offset + instructions.len() + 1;
                let mut body_instructions = compile(body, loop_begin);
                let loop_end = loop_begin + body_instructions.len() + 1;
                let mut loop_instructions = vec![Instruction::Loop(loop_end)];
                loop_instructions.append(&mut body_instructions);
                loop_instructions.push(Instruction::EndLoop(loop_begin));
                loop_instructions
            }
        };
        instructions.append(&mut ins);
    }
    instructions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::Expr;

    #[test]
    fn test_compile() {
        let e = vec![Expr::Loop(
            vec![Expr::Increment, Expr::Output]
        )];
        assert_eq!(vec![
            Instruction::Loop(4),
            Instruction::Increment,
            Instruction::Output,
            Instruction::EndLoop(1)
        ], compile(&e, 0));
    }

    #[test]
    fn test_compile_nested_loops() {
        let e = vec![
            Expr::Increment,
            Expr::Loop(vec![
                Expr::Increment,
                Expr::Increment,
                Expr::Loop(vec![
                    Expr::Increment,
                    Expr::Increment
                ]),
                Expr::Increment
            ])
        ];
        assert_eq!(vec![
            Instruction::Increment,
            Instruction::Loop(10),
            Instruction::Increment,
            Instruction::Increment,
            Instruction::Loop(8),
            Instruction::Increment,
            Instruction::Increment,
            Instruction::EndLoop(5),
            Instruction::Increment,
            Instruction::EndLoop(2)
        ], compile(&e, 0));
    }
}
