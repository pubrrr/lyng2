use std::fmt::{Display, Formatter};

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
    Variable(String),
    Number(BigDecimal),
    Sum(Box<LocalizedSyntaxNode>, Box<LocalizedSyntaxNode>),
    Product(Box<LocalizedSyntaxNode>, Box<LocalizedSyntaxNode>),
    Exponent(Box<LocalizedSyntaxNode>, Box<LocalizedSyntaxNode>),

    Subtraction(Box<LocalizedSyntaxNode>, Box<LocalizedSyntaxNode>),
    Division(Box<LocalizedSyntaxNode>, Box<LocalizedSyntaxNode>),
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

    #[cfg(test)]
    fn variable(location: Localization, name: String) -> Self {
        Self::new(location, SyntaxTree::Variable(name))
    }

    fn add(location: Localization, left: Self, right: Self) -> Self {
        Self::new(location, SyntaxTree::Sum(Box::new(left), Box::new(right)))
    }

    fn mul(location: Localization, left: Self, right: Self) -> Self {
        Self::new(
            location,
            SyntaxTree::Product(Box::new(left), Box::new(right)),
        )
    }

    fn sub(location: Localization, left: Self, right: Self) -> Self {
        Self::new(
            location,
            SyntaxTree::Subtraction(Box::new(left), Box::new(right)),
        )
    }

    fn div(location: Localization, left: Self, right: Self) -> Self {
        Self::new(
            location,
            SyntaxTree::Division(Box::new(left), Box::new(right)),
        )
    }

    fn exp(location: Localization, left: Self, right: Self) -> Self {
        Self::new(
            location,
            SyntaxTree::Exponent(Box::new(left), Box::new(right)),
        )
    }

    fn neg(location: Localization, value: Self) -> Self {
        Self::new(location, SyntaxTree::Negation(Box::new(value)))
    }
}

impl Display for LocalizedSyntaxNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tree)
    }
}

impl Display for SyntaxTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxTree::Variable(value) => write!(f, "{}", value),
            SyntaxTree::Number(value) => write!(f, "{}", value),
            SyntaxTree::Sum(left, right) => write!(f, "({} + {})", left, right),
            SyntaxTree::Product(left, right) => write!(f, "({} * {})", left, right),
            SyntaxTree::Exponent(left, right) => write!(f, "({} ^ {})", left, right),
            SyntaxTree::Subtraction(left, right) => write!(f, "({} - {})", left, right),
            SyntaxTree::Division(left, right) => write!(f, "({} / {})", left, right),
            SyntaxTree::Negation(value) => write!(f, "-{}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let under_test = LocalizedSyntaxNode::neg(
            Localization::new(),
            LocalizedSyntaxNode::add(
                Localization::new(),
                LocalizedSyntaxNode::number(Localization::new(), 1),
                LocalizedSyntaxNode::mul(
                    Localization::new(),
                    LocalizedSyntaxNode::number(Localization::new(), 2),
                    LocalizedSyntaxNode::exp(
                        Localization::new(),
                        LocalizedSyntaxNode::number(Localization::new(), 3),
                        LocalizedSyntaxNode::sub(
                            Localization::new(),
                            LocalizedSyntaxNode::number(Localization::new(), 4),
                            LocalizedSyntaxNode::div(
                                Localization::new(),
                                LocalizedSyntaxNode::number(Localization::new(), 5),
                                LocalizedSyntaxNode::variable(Localization::new(), "x".to_string()),
                            ),
                        ),
                    ),
                ),
            ),
        );

        let expected = "-(1 + (2 * (3 ^ (4 - (5 / x)))))".to_string();

        assert_eq!(expected, format!("{}", under_test));
    }
}
