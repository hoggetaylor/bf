use crate::parse::Expr;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Instruction {
    Move(i32),
    Add(i32),
    Output,
    Input,
    JmpEq(usize),
    JmpNEq(usize)
}

pub fn compile(exprs: &[Expr], offset: usize) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for e in exprs {
        let mut ins = match *e {
            Expr::MoveRight => vec![Instruction::Move(1)],
            Expr::MoveLeft => vec![Instruction::Move(-1)],
            Expr::Increment => vec![Instruction::Add(1)],
            Expr::Decrement => vec![Instruction::Add(-1)],
            Expr::Output => vec![Instruction::Output],
            Expr::Input => vec![Instruction::Input],
            Expr::Loop(ref body) => {
                let loop_begin = offset + instructions.len() + 1;
                let mut body_instructions = compile(body, loop_begin);
                let loop_end = loop_begin + body_instructions.len() + 1;
                let mut loop_instructions = vec![Instruction::JmpEq(loop_end)];
                loop_instructions.append(&mut body_instructions);
                loop_instructions.push(Instruction::JmpNEq(loop_begin));
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
            Instruction::JmpEq(4),
            Instruction::Add(1 ),
            Instruction::Output,
            Instruction::JmpNEq(1)
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
            Instruction::Add(1),
            Instruction::JmpEq(10),
            Instruction::Add(1),
            Instruction::Add(1),
            Instruction::JmpEq(8),
            Instruction::Add(1),
            Instruction::Add(1),
            Instruction::JmpNEq(5),
            Instruction::Add(1),
            Instruction::JmpNEq(2)
        ], compile(&e, 0));
    }
}
