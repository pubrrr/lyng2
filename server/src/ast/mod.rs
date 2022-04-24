pub struct Localization {
    lexeme: String,
    line: usize,
    column: usize,
}

pub enum SyntaxTree {
    Number(Number),
    Sum(Box<(SyntaxTree, SyntaxTree)>, Vec<SyntaxTree>),
    Product(Box<(SyntaxTree, SyntaxTree)>, Vec<SyntaxTree>),
    Subtraction(Box<(SyntaxTree, SyntaxTree)>),
    Division(Box<(SyntaxTree, SyntaxTree)>),
    Negation(Box<SyntaxTree>),
}

pub struct Number {
    value: f32,
    localization: Localization,
}
