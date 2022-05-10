use bigdecimal::BigDecimal;

pub mod parser;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Localization {
    line: usize,
    column: usize,
}

impl Localization {
    fn new() -> Self {
        Self { line: 0, column: 0 }
    }

    pub fn at(line: usize, column: usize) -> Localization {
        Localization { line, column }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntaxTree {
    Variable(String), // this should be a usize
    Number(BigDecimal),
    Sum(Box<(LocalizedSyntaxNode, LocalizedSyntaxNode)>),
    Product(Box<(LocalizedSyntaxNode, LocalizedSyntaxNode)>),
    Exponent(Box<(LocalizedSyntaxNode, LocalizedSyntaxNode)>),

    Subtraction(Box<(LocalizedSyntaxNode, LocalizedSyntaxNode)>),
    Division(Box<(LocalizedSyntaxNode, LocalizedSyntaxNode)>),
    Negation(Box<LocalizedSyntaxNode>),
}

// TODO check whether necessary
// pub enum Values {
//     Variable(String),
//     Number(BigDecimal),
//     Sum(Vec<Values>),
//     Product(Vec<Values>),
//     Exponent(Vec<Values>),
//     AddInv(Box<Values>),
//     MulInv(Box<Values>),
// }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalizedSyntaxNode {
    location: Localization,
    tree: SyntaxTree,
}

impl LocalizedSyntaxNode {
    fn new(location: Localization, tree: SyntaxTree) -> Self {
        Self { location, tree }
    }

    fn number<N: Into<BigDecimal>>(location: Localization, number: N) -> Self {
        Self {
            location,
            tree: SyntaxTree::Number(number.into()),
        }
    }

    fn add(location: Localization, left: Self, right: Self) -> Self {
        Self::new(location, SyntaxTree::Sum(Box::new((left, right))))
    }

    fn mul(location: Localization, left: Self, right: Self) -> Self {
        Self::new(location, SyntaxTree::Product(Box::new((left, right))))
    }

    fn sub(location: Localization, left: Self, right: Self) -> Self {
        Self::new(location, SyntaxTree::Subtraction(Box::new((left, right))))
    }

    fn div(location: Localization, left: Self, right: Self) -> Self {
        Self::new(location, SyntaxTree::Division(Box::new((left, right))))
    }

    fn exp(location: Localization, left: Self, right: Self) -> Self {
        Self::new(location, SyntaxTree::Exponent(Box::new((left, right))))
    }

    fn neg(location: Localization, value: Self) -> Self {
        Self::new(location, SyntaxTree::Negation(Box::new(value)))
    }
}
