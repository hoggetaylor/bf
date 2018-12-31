use crate::parse::Expr;

pub fn optimize(program: &[Expr]) -> Vec<Expr> {
    let compressed = compress_repeated(program);
    filter_noops(compressed)
}

/// Compresses repeated additions or moves into a single operation.
fn compress_repeated(program: &[Expr]) -> Vec<Expr> {
    let mut compressed = Expr::Add(0);
    let mut compressed_program = program.iter().filter_map(|current| {
        let compressed = &mut compressed;
        match (&compressed, current) {
            (Expr::Move(a), Expr::Move(b)) => {
                *compressed = Expr::Move(*a + b);
                None
            },
            (Expr::Add(a), Expr::Add(b)) => {
                *compressed = Expr::Add(*a + b);
                None
            },
            (_, Expr::Loop(e)) => {
                let x = compressed.clone();
                *compressed = Expr::Loop(compress_repeated(e));
                Some(x)
            }
            (_, current) => {
                let x = Some(compressed.clone());
                *compressed = current.clone();
                x
            }
        }
    }).collect::<Vec<_>>();
    compressed_program.push(compressed);
    compressed_program
}

fn filter_noops(program: Vec<Expr>) -> Vec<Expr> {
    program.into_iter().filter_map(|e| {
        match e {
            Expr::Move(0) => None,
            Expr::Add(0) => None,
            Expr::Loop(e) => Some(Expr::Loop(filter_noops(e))),
            _ => Some(e)
        }
    }).collect()
}
