#![feature(map_first_last)]

use bigdecimal::BigDecimal;
use bigdecimal::One;
use bigdecimal::Zero;

use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::ops::*;
//use bigint::{BigInt,Sign,BigUInt};
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

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Values {
    Number(BigDecimal),
    Variable(String),
    Sum(BigDecimal, BTreeMap<Values, BigDecimal>),
    Product(BigDecimal, BTreeMap<Values, BigDecimal>),
    Exponent(Box<(Values, Values)>),
}

impl From<LocalizedSyntaxNode> for Values {
    fn from(node: LocalizedSyntaxNode) -> Values {
        match node.tree {
            SyntaxTree::Variable(value) => Values::Variable(value),
            SyntaxTree::Number(value) => Values::Number(value),
            SyntaxTree::Sum(box left, box right) => Values::from(left).add(right.into()),
            SyntaxTree::Product(box left, box right) => Values::from(left).mul(right.into()),
            SyntaxTree::Exponent(box left, box right) => Values::from(left).exp(right.into()),
            SyntaxTree::Subtraction(box left, box right) => Values::from(left)
                .add(Values::from(right))
                .mul(Values::Number(BigDecimal::from(-1))),
            SyntaxTree::Division(box left, box right) => Values::from(left)
                .mul(Values::from(right).exp(Values::Number(BigDecimal::from(-1)))),
            SyntaxTree::Negation(box value) => {
                Values::from(value).mul(Values::Number(BigDecimal::from(-1)))
            }
        }
    }
}

impl Display for Values {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Values::Variable(value) => write!(f, "{}", value),
            Values::Number(value) => write!(f, "{}", value),
            Values::Sum(constant, vmap) => {
                write!(f, "( {constant} ").unwrap();
                for (k, v) in vmap {
                    write!(f, "+ {v} * {k} ").unwrap();
                }
                write!(f, ")")
            }
            Values::Product(constant, vmap) => {
                write!(f, "( {constant} ").unwrap();
                for (k, v) in vmap {
                    write!(f, "* {k} ^ {v} ").unwrap();
                }
                write!(f, ")")
            }
            Values::Exponent(box (left, right)) => write!(f, "({} ^ {})", left, right),
        }
    }
}
//const one : BigDecimal = BigDecimal::from(1);

//const two : BigDecimal = BigDecimal::from(2);

fn exponantiate(mut x: BigDecimal, mut y: BigDecimal) -> BigDecimal {
    if (&y).is_integer() {
        if (&y).rem(BigDecimal::from(2)) == BigDecimal::zero() {
            let n = y.half();

            while (&y) > &n {
                x = x.square();
                y -= BigDecimal::one();
            }
            x
        } else {
            let n = (y.clone() - BigDecimal::one()).half();
            let m = x.clone();
            while (&y) > &n {
                x = x.square();
                y -= BigDecimal::one();
            }
            x * m
        }
    } else {
        panic!("not implemented yet")
    }
}

impl Values {
    // fn entry_adder(
    //     mut map: BTreeMap<Values, BigDecimal>,
    //     pair: (&Values, &BigDecimal),
    // ) -> BTreeMap<Values, BigDecimal> {
    //     let (key, value) = pair;
    //     map.entry(key.to_owned())
    //         .and_modify(|x| *x += value)
    //         .or_insert(value.to_owned());
    //     map
    // }

    fn entry_adder<F>(
        mut map: BTreeMap<Values, BigDecimal>,
        pair: (&Values, &BigDecimal),
        fun: F,
    ) -> BTreeMap<Values, BigDecimal>
    where
        F: Fn(&mut BigDecimal, BigDecimal),
    {
        let (key, value) = pair;
        map.entry(key.to_owned())
            .and_modify(|x| fun(x, value.to_owned()))
            .or_insert(value.to_owned());
        map
    }

    fn const_folder<F>(
        mut map: BTreeMap<Values, BigDecimal>,
        value: BigDecimal,
        fun: F,
    ) -> BTreeMap<Values, BigDecimal>
    where
        F: Fn(BigDecimal, BigDecimal) -> BigDecimal,
    {
        match map.pop_first().unwrap() {
            (Values::Number(v1), v2) => map.insert(Values::Number(fun(v1, value)), v2),
            (v1, v2) => map.insert(v1, v2),
        };
        map
    }

    fn add(self, other: Values) -> Values {
        let zero: BigDecimal = BigDecimal::from(0);
        let one: BigDecimal = BigDecimal::from(1);
        let two: BigDecimal = BigDecimal::from(2);
        match (self, other) {
            (Values::Number(x), other) | (other, Values::Number(x)) if x == zero => other,
            (Values::Number(x), Values::Number(y)) => Values::Number(x + y),
            (Values::Sum(cx, x), Values::Number(y)) | (Values::Number(y), Values::Sum(cx, x)) => {
                Values::Sum(cx + y, x)
            }

            (Values::Sum(cx, x), Values::Sum(cy, y)) => Values::Sum(
                cx + cy,
                x.iter().chain(y.iter()).fold(BTreeMap::new(), |a, b| {
                    Self::entry_adder(a, b, BigDecimal::add_assign)
                }),
            ),
            (Values::Sum(cx, x), Values::Product(cy, y))
            | (Values::Product(cy, y), Values::Sum(cx, x)) => Values::Sum(
                cx,
                Self::entry_adder(
                    x,
                    (&Values::Product(BigDecimal::one(), y), &cy),
                    BigDecimal::add_assign,
                ),
            ),

            (Values::Sum(cx, x), other) | (other, Values::Sum(cx, x)) => Values::Sum(
                cx,
                Self::entry_adder(x, (&other, &one), BigDecimal::add_assign),
            ),

            (otherleft, otherright) if otherleft == otherright => {
                Values::Sum(BigDecimal::zero(), BTreeMap::from([(otherleft, two)]))
            }
            (otherleft, otherright) => Values::Sum(
                BigDecimal::zero(),
                BTreeMap::from([(otherleft, one.clone()), (otherright, one)]),
            ),
        }
    }
    fn mul(self, other: Values) -> Values {
        let zero: BigDecimal = BigDecimal::from(0);
        let one: BigDecimal = BigDecimal::from(1);
        let two: BigDecimal = BigDecimal::from(2);
        //todo!();
        match (self, other) {
            (Values::Number(x), _other) | (_other, Values::Number(x)) if x == zero => {
                Values::Number(zero)
            }
            (Values::Number(x), other) | (other, Values::Number(x)) if x == one => other,
            (Values::Number(x), Values::Product(xz, z))
            | (Values::Product(xz, z), Values::Number(x)) => Values::Product(xz * x, z),
            (Values::Number(x), Values::Number(y)) => Values::Number(x * y),
            (Values::Number(x), other) | (other, Values::Number(x)) => {
                Values::Product(x, BTreeMap::from([(other, one.clone())]))
            }

            (Values::Exponent(box (a, b)), Values::Exponent(box (c, d))) => {
                if a == c {
                    a.exp(b.add(d))
                } else if b == d {
                    a.mul(c).exp(d)
                } else {
                    Values::Product(
                        BigDecimal::one(),
                        BTreeMap::from([(a.exp(b), one.clone()), (c.exp(d), one)]),
                    )
                }
            }
            (z, Values::Exponent(box (x, y))) | (Values::Exponent(box (x, y)), z) => {
                if x == z {
                    x.exp(y.add(Values::Number(one)))
                } else {
                    Values::Product(
                        BigDecimal::one(),
                        BTreeMap::from([(z, one.clone()), (x.exp(y), one)]),
                    )
                }
            }

            (Values::Sum(cx, x), Values::Sum(cy, y)) => y
                .iter()
                .flat_map(|(y_val, y_key)| {
                    x.iter().map(move |(x_val, x_key)| {
                        x_val
                            .clone()
                            .mul(y_val.clone())
                            .mul(Values::Number(x_key * y_key.clone()))
                    })
                })
                .fold(Values::Number(cx * cy), |x: Values, y: Values| x.add(y)),
            (Values::Sum(cx, x), other) | (other, Values::Sum(cx, x)) => {
                let mut ret = Values::Number(cx);
                for (k, v) in x {
                    ret = ret.add(k.mul(other.clone()).mul(Values::Number(v)));
                }
                ret
            }
            (Values::Product(cx, x), Values::Product(cy, y)) => Values::Product(
                cx * cy,
                x.iter().chain(y.iter()).fold(BTreeMap::new(), |a, b| {
                    Self::entry_adder(a, b, BigDecimal::mul_assign)
                }),
            ),
            (Values::Product(cx, x), other) | (other, Values::Product(cx, x)) => Values::Product(
                cx,
                Self::entry_adder(x, (&other, &one), BigDecimal::mul_assign),
            ),

            (otherleft, otherright) => {
                if otherleft == otherright {
                    Values::Product(BigDecimal::one(), BTreeMap::from([(otherleft, two)]))
                } else {
                    Values::Product(
                        BigDecimal::one(),
                        BTreeMap::from([(otherleft, one.clone()), (otherright, one)]),
                    )
                }
            }
        }
    }
    fn exp(self, other: Values) -> Values {
        match (self, other) {
            (Values::Number(x), _) if x == BigDecimal::zero() => Values::Number(BigDecimal::zero()),
            (_, Values::Number(x)) if x == BigDecimal::zero() => Values::Number(BigDecimal::one()),
            (Values::Number(x), _) if x == BigDecimal::one() => Values::Number(BigDecimal::one()),
            (other, Values::Number(x)) if x == BigDecimal::one() => other,
            (Values::Number(x), Values::Number(y)) => Values::Number(exponantiate(x, y)),
            (otherleft, otherright) => Values::Exponent(Box::new((otherleft, otherright))),
        }
    }
}

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
