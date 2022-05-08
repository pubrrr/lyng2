#[allow(dead_code)]
pub struct Localization {
    lexeme: String,
    line: usize,
    column: usize,
}

#[allow(dead_code)]
pub enum SyntaxTree {
    Number(Number),
    Sum(Box<(SyntaxTree, SyntaxTree)>, Vec<SyntaxTree>),
    Product(Box<(SyntaxTree, SyntaxTree)>, Vec<SyntaxTree>),
    Subtraction(Box<(SyntaxTree, SyntaxTree)>),
    Division(Box<(SyntaxTree, SyntaxTree)>),
    Negation(Box<SyntaxTree>),
}

#[allow(dead_code)]
pub struct Number {
    value: f32,
    localization: Localization,
}
