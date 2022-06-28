#![feature(map_first_last)]

use bigdecimal::BigDecimal;
use bigdecimal::One;
use bigdecimal::Zero;
use std::collections::HashMap;

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
                .add(Values::from(right).mul(Values::Number(BigDecimal::from(-1)))),
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
        let supscriptmap = HashMap::from([
            ('0', '⁰'),
            ('1', '¹'),
            ('2', '²'),
            ('3', '³'),
            ('4', '⁴'),
            ('5', '⁵'),
            ('6', '⁶'),
            ('8', '⁸'),
            ('7', '⁷'),
            ('9', '⁹'),
            ('-', '⁻'),
        ]);
        match self {
            Values::Variable(value) => write!(f, "{}", value),
            Values::Number(value) => write!(f, "{}", value),
            Values::Sum(constant, vmap) => {
                let mut vect = vec![];
                let mut map = vmap.clone();
                if constant != &BigDecimal::zero() {
                    vect.push(format!("{constant}"));
                }
                for (k, v) in map {
                    if v != BigDecimal::zero() && k != Values::Number(BigDecimal::zero()) {
                        if v == BigDecimal::one() {
                            vect.push(format!("{k}")); //write!(f, "+ {k} ").unwrap();
                        } else if v == BigDecimal::from(-1) {
                            vect.push(format!(" - {k}")); //write!(f, "+ {k} ").unwrap();
                        } else {
                            vect.push(format!("{v} {k}")); //write!(f, "+ {v} * {k} ").unwrap();
                        }
                    }
                }

                vect.reverse();
                write!(f, "{}", vect.pop().unwrap());
                vect.reverse();
                write!(
                    f,
                    "{}",
                    vect.iter()
                        .map(|x| {
                            if x.chars().nth(1).unwrap() == '-' {
                                x.to_owned()
                            } else {
                                format!(" + {x}")
                            }
                        })
                        .collect::<String>()
                )
            }
            Values::Product(constant, vmap) => {
                let mut map = vmap.clone();
                if constant == &BigDecimal::zero() {
                    write!(f, "0")
                } else {
                    let mut vect = vec![];
                    if constant != &BigDecimal::one() {
                        vect.push(format!("{constant}"));
                    }
                    for (k, v) in map {
                        if v == BigDecimal::zero() {
                        } else if v == BigDecimal::one() {
                            vect.push(format!("{k}"));
                        } else {
                            let elem = format!(
                                "{k}{}",
                                format!("{v}")
                                    .chars()
                                    .map(|x| {
                                        let ret = x.clone();
                                        match supscriptmap.get(&x){
                                            None => ret,
                                            Some(r) => r.to_owned()
                                        }

                                    })
                                    .collect::<String>()
                            );
                            vect.push(elem);
                        }
                    }
                    write!(f, "{}", vect.join(" "))
                }
            }
            Values::Exponent(box (left, right)) => write!(f, "({}) ^ ({})", left, right),
        }
    }
}
//const one : BigDecimal = BigDecimal::from(1);

//const two : BigDecimal = BigDecimal::from(2);

fn exponantiate(mut x: BigDecimal, mut y: BigDecimal) -> BigDecimal {
    if (&y).is_integer() {
        if (&y).rem(BigDecimal::from(2)) == BigDecimal::zero() {
            let n = y.half();

            while y > n {
                x = x.square();
                y -= BigDecimal::one();
            }
            x
        } else {
            let n = (y.clone() - BigDecimal::one()).half();
            let m = x.clone();
            while y > n {
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
            .or_insert_with(|| value.to_owned());
        map
    }

    fn add(self, other: Values) -> Values {
        let zero: BigDecimal = BigDecimal::from(0);
        let one: BigDecimal = BigDecimal::from(1);
        let two: BigDecimal = BigDecimal::from(2);
        match (self, other) {
            (Values::Number(x), other) | (other, Values::Number(x)) if x == zero => {
                //println!("add zero {other}");
                other
            }
            (Values::Number(x), Values::Number(y)) => {
                //println!("add num num {x} {y}");
                Values::Number(x + y)
            }
            (Values::Sum(cx, x), Values::Number(y)) | (Values::Number(y), Values::Sum(cx, x)) => {
                //println!("add sum num {:?} {:?}  {:?} ", cx, x, y);
                Values::Sum(cx + y, x)
            }

            (Values::Sum(cx, x), Values::Sum(cy, y)) => {
                //println!("add sum sum {:?} {:?}  {:?} {:?}", cx, x, cy, y);
                Values::Sum(
                    cx + cy,
                    x.iter().chain(y.iter()).fold(BTreeMap::new(), |a, b| {
                        Self::entry_adder(a, b, BigDecimal::add_assign)
                    }),
                )
            }
            (Values::Sum(cx, x), Values::Product(cy, mut y))
            | (Values::Product(cy, mut y), Values::Sum(cx, x))
                if y.len() == 1 =>
            {
                //println!("add sum prod len 1 {:?} {:?}  {:?} {:?}", cx, x, cy, y);
                if let Some((mut key, val)) = y.pop_first() {
                    key = key.exp(Values::Number(val));
                    Values::Sum(
                        cx,
                        Self::entry_adder(x, (&key, &cy), BigDecimal::add_assign),
                    )
                } else {
                    unreachable!()
                }
            }
            (Values::Sum(cx, x), Values::Product(cy, y))
            | (Values::Product(cy, y), Values::Sum(cx, x)) => {
                //println!("add sum prod {:?} {:?}  {:?} {:?}", cx, x, cy, y);
                //println!("Y === {:?}", y);
                let res = Values::Sum(
                    cx,
                    Self::entry_adder(
                        x.clone(),
                        (&Values::Product(BigDecimal::one(), y), &cy),
                        BigDecimal::add_assign,
                    ),
                );
                //println!("R === {:?}", &x);
                res
            }

            (Values::Sum(cx, x), other) | (other, Values::Sum(cx, x)) => {
                //println!("add sum other {:?} {:?}  {:?} ", cx, x, other);
                Values::Sum(
                    cx,
                    Self::entry_adder(x, (&other, &one), BigDecimal::add_assign),
                )
            }

            (Values::Product(cx, x), Values::Product(cy, y))
            | (Values::Product(cy, y), Values::Product(cx, x))
                if x == y =>
            {
                Values::Sum(zero, BTreeMap::from([(Values::Product(one, x), cx + cy)]))
            }
            (Values::Product(cx, x), Values::Product(cy, y)) => Values::Sum(
                zero,
                BTreeMap::from([
                    (Values::Product(one.clone(), x), cx),
                    (Values::Product(one, y), cy),
                ]),
            ),

            (Values::Product(cx, mut x), other) | (other, Values::Product(cx, mut x)) => {
                //println!("add product other {:?} {:?}  {:?} ", cx, x, other);
                let map = BTreeMap::from([(other, one.clone())]);
                if x.len() == 1 {
                    if let Some((mut key, val)) = x.pop_first() {
                        key = key.exp(Values::Number(val));
                        Values::Sum(
                            zero,
                            Self::entry_adder(map, (&key, &cx), BigDecimal::add_assign),
                        )
                    } else {
                        unreachable!()
                    }
                } else {
                    Values::Sum(
                        zero,
                        Self::entry_adder(
                            map,
                            (&Values::Product(one, x), &cx),
                            BigDecimal::add_assign,
                        ),
                    )
                }
            }

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
                Values::Product(x, BTreeMap::from([(other, one)]))
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
                } else if let Values::Number(num_val) = y {
                    Values::Product(BigDecimal::one(), BTreeMap::from([(z, one), (x, num_val)]))
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
                    Self::entry_adder(a, b, BigDecimal::add_assign)
                }),
            ),
            (Values::Product(cx, x), other) | (other, Values::Product(cx, x)) => Values::Product(
                cx,
                Self::entry_adder(x, (&other, &one), BigDecimal::add_assign),
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
            (x, Values::Number(y)) => Values::Product(BigDecimal::one(), BTreeMap::from([(x, y)])),
            (Values::Exponent(box (a, b)), c) => a.exp(b.mul(c)),
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

    use bigdecimal::{BigDecimal, FromPrimitive};

    use crate::ast::parser::parse;
    use crate::ast::Localization;
    use std::fmt::Write;

    #[test]
    fn add_two_variables_and_convert_to_values() {
        let result: Values = parse("x + x".to_string()).unwrap().pop().unwrap().into();

        let expected = "2 x";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn add_several_variables_and_convert_to_values() {
        let result: Values = parse("x+y+3+y+2*x+y+2-4*y".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .into();

        let expected = "5 + 3 x - y";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn mul_several_variables_and_convert_to_values() {
        let result: Values = parse("x*x*y*y ".to_string()).unwrap().pop().unwrap().into();

        let expected = "x² y²";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn mul_two_variables_add_and_convert_to_values() {
        let result: Values = parse("x*y + y*x + 2*y*x + 3*x*y".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .into();

        let expected = "7 x y"; //; "7 x y";//; "( 0 + 7 * ( 1 * x ^ 1 * y ^ 1 ) )";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn mul_several_variables_and_convert_to_valuesa() {
        let result: Values = parse("x*y * y*x * 2*y*x * 3*x*y".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .into();

        let expected = "6 x⁴ y⁴"; //; "( 6 * x ^ 4 * y ^ 4 )";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn mul_several_variables_and_add_and_convert_to_values2() {
        let result: Values =
            parse("x*y * y*x * 2*y*x * 3*x*y + 5*x*y * 4*y*x * 2*y*x * 3*x*y".to_string())
                .unwrap()
                .pop()
                .unwrap()
                .into();

        let expected = "126 x⁴ y⁴";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn mul_several_variables_and_add_and_convert_to_values3() {
        let result: Values =
            parse("x*y * y*x * y*2*x * x*y*3 + 5*x*y * 4*y*x * 2*y*x * 3*x*y".to_string())
                .unwrap()
                .pop()
                .unwrap()
                .into();

        let expected = "126 x⁴ y⁴";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn square_of_sum() {
        let result: Values = parse("(x+y)*(x+y)".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .into();

        let expected = "2 x y + x² + y²";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn diff_of_square() {
        let result: Values = parse("(x+y)*(x-y)".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .into();

        let expected = "x² - y²";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn diff_of_cubes() {
        let result: Values = parse("(x-y)*(x^2+y^2+x*y)".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .into();

        let expected = "x³ - y³";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn cube_of_sum() {
        let result: Values = parse("(x+y)*(x+y)*(x+y)".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .into();

        let expected = "3 x y² + 3 x² y + x³ + y³";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn cube_of_diff() {
        let result: Values = parse("(x-y)*(x-y)*(x-y)".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .into();

        let expected = "3 x y² + -3 x² y + x³ - y³";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn fourth_of_sum() {
        let result: Values = parse("(x+y)*(x+y)*(x+y)*(x+y)".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .into();

        let expected = "4 x y³ + 6 x² y² + 4 x³ y + x⁴ + y⁴";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn fifth_of_sum() {
        let result: Values = parse("(x+y)*(x+y)*(x+y)*(x+y)*(x+y)".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .into();

        let expected = "5 x y⁴ + 10 x² y³ + 10 x³ y² + 5 x⁴ y + x⁵ + y⁵";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn sixth_of_sum() {
        let result: Values = parse("(x+y)*(x+y)*(x+y)*(x+y)*(x+y)*(x+y)".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .into();

        let expected ="6 x y⁵ + 15 x² y⁴ + 20 x³ y³ + 15 x⁴ y² + 6 x⁵ y + x⁶ + y⁶";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }
}
