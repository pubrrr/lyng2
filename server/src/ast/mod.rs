use std::collections::HashMap;
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
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Values {
    Number(BigDecimal),
    Variable(String),
    Sum(Vec<Values>),
    Product(Vec<Values>),
    Exponent(Box<Values>, Box<Values>),
}

pub fn convert_to_values(tree: &SyntaxTree) -> Values {
    match tree.clone() {
        SyntaxTree::Variable(var) => Values::Variable(var.to_string()),
        SyntaxTree::Number(num) => Values::Number(num),
        SyntaxTree::Sum(box lhs, box rhs) => {
            let lhs = convert_to_values(&lhs.tree);
            let rhs = convert_to_values(&rhs.tree);
            match (lhs, rhs) {
                (Values::Sum(mut vec1), Values::Sum(vec2)) => {
                    vec1.extend(vec2);
                    vec1.sort();
                    return Values::Sum(vec1);
                }
                (Values::Sum(mut vec1), res) => {
                    vec1.push(res);
                    vec1.sort();
                    return Values::Sum(vec1);
                }
                (res, Values::Sum(mut vec1)) => {
                    vec1.push(res);
                    vec1.sort();
                    return Values::Sum(vec1);
                }
                (left, right) => {
                    let mut tmp = vec![left, right];
                    tmp.sort();
                    return Values::Sum(tmp);
                }
            }
        }

        SyntaxTree::Product(box lhs, box rhs) => {
            let lhs = convert_to_values(&lhs.tree);
            let rhs = convert_to_values(&rhs.tree);
            match (lhs, rhs) {
                (Values::Product(mut vec1), Values::Product(vec2)) => {
                    vec1.extend(vec2);
                    vec1.sort();
                    return Values::Product(vec1);
                }
                (Values::Sum(vec1), Values::Sum(vec2)) => {
                    let mut result = vec![];
                    for e1 in vec1.clone() {
                        for e2 in vec2.clone() {
                            match (e1.clone(), e2.clone()) {
                                (Values::Product(mut vec11), Values::Product(vec22)) => {
                                    vec11.extend(vec22);
                                    vec11.sort();
                                    result.push(Values::Product(vec11));
                                }
                                (Values::Product(mut vec11), res)
                                | (res, Values::Product(mut vec11)) => {
                                    vec11.push(res);
                                    vec11.sort();
                                    result.push(Values::Product(vec11));
                                }
                                (left, right) => result.push(Values::Product(vec![left, right])),
                            }
                        }
                    }
                    result.sort();
                    return Values::Sum(result);
                }

                (Values::Product(vec1), Values::Sum(vec2))
                | (Values::Sum(vec2), Values::Product(vec1)) => {
                    let mut result = vec![];
                    for e2 in vec2 {
                        match e2 {
                            Values::Product(vec22) => {
                                let mut temp = vec1.clone();
                                temp.extend(vec22);
                                temp.sort();
                                result.push(Values::Product(temp));
                            }
                            res => {
                                let mut temp = vec1.clone();
                                temp.push(res);
                                temp.sort();
                                result.push(Values::Product(temp));
                            }
                        }
                    }
                    result.sort();
                    return Values::Sum(result);
                }
                (Values::Sum(vec1), res) | (res, Values::Sum(vec1)) => {
                    let mut result = vec![];
                    for e2 in vec1.clone() {
                        match e2.clone() {
                            Values::Product(vec22) => {
                                let mut temp = vec22.clone();
                                temp.push(res.clone());
                                temp.sort();
                                result.push(Values::Product(temp));
                            }
                            a => {
                                let mut temp = vec![res.clone(), e2];
                                temp.sort();
                                result.push(Values::Product(temp));
                            }
                        }
                    }
                    result.sort();
                    return Values::Sum(result);
                }

                (Values::Product(mut vec1), res) | (res, Values::Product(mut vec1)) => {
                    vec1.push(res);
                    vec1.sort();
                    return Values::Product(vec1);
                }

                (left, right) => return Values::Product(vec![left, right]),
            }
        }

        SyntaxTree::Division(lhs, rhs) => {
            let div = SyntaxTree::Product(
                lhs,
                Box::new(LocalizedSyntaxNode::new(
                    Localization::new(),
                    SyntaxTree::Exponent(
                        rhs,
                        Box::new(LocalizedSyntaxNode::new(
                            Localization::new(),
                            SyntaxTree::Number(BigDecimal::from(-1)),
                        )),
                    ),
                )),
            );
            return convert_to_values(&div);
        }
        SyntaxTree::Subtraction(lhs, rhs) => {
            let sub = SyntaxTree::Sum(
                lhs,
                Box::new(LocalizedSyntaxNode::new(
                    Localization::new(),
                    SyntaxTree::Product(
                        rhs,
                        Box::new(LocalizedSyntaxNode::new(
                            Localization::new(),
                            SyntaxTree::Number(BigDecimal::from(-1)),
                        )),
                    ),
                )),
            );
            return convert_to_values(&sub);
        }

        SyntaxTree::Negation(val) => {
            let prod = SyntaxTree::Product(
                val,
                Box::new(LocalizedSyntaxNode::new(
                    Localization::new(),
                    SyntaxTree::Number(BigDecimal::from(-1)),
                )),
            );
            return convert_to_values(&prod);
        }
        SyntaxTree::Exponent(box lhs, box rhs) => {
            return Values::Exponent(
                Box::new(convert_to_values(&lhs.tree)),
                Box::new(convert_to_values(&rhs.tree)),
            )
        }
    }
}

fn const_folding(expr: Values) -> Values {
    let min_one = BigDecimal::from(-1);
    let one = BigDecimal::from(1);

    match expr {
        Values::Sum(ref vec) => {
            let mut nums = vec![];
            let mut syms = HashMap::new();
            //vec.sort();
            for i in vec.iter().map(|x| const_folding(x.clone())) {
                //println!("BSum {:?}\n    {:?}\n    {:?}", i, syms, nums);

                match i.clone() {
                    Values::Number(x) => nums.push(x.to_owned()),
                    Values::Product(mut vec2) => {
                        vec2.sort_by(|a, b| b.cmp(a));
                        //println!("DEBUUUUUUUUUUG  {:?}", vec);
                        let last_val = vec2.get(vec2.len()-1).unwrap().clone().to_owned();
                        if let Values::Number(coefficient) = last_val {
                            vec2.pop();
                            vec2.sort_by(|a, b| b.cmp(a));
                            //println!("=== coeffs :{:?}\n=== vec: {:?}",&coefficient,&vec2);
                            //if syms.contains_key(&Values::Product(vec.clone())){
                                //vec.push()
                                syms.entry(Values::Product(vec2.clone()))
                                    .and_modify(|x| *x += coefficient.clone())
                                    .or_insert(coefficient);
                            /*}else{
                                vec.push(Values::Number(coefficient));
                                syms.entry(Values::Product(vec)).or_insert(one.clone());
                            }*/
                        } else {
                            //println!("ELSE === vec :{:?}",&vec2);

                            syms.entry(Values::Product(vec2))
                                .and_modify(move |x| *x += BigDecimal::from(1))
                                .or_insert(one.clone());
                        }
                    }
                    others => {
                        //println!("OTHERS :{:?}",&others);

                        syms.entry(others)
                            .and_modify(move |x| *x += BigDecimal::from(1))
                            .or_insert(one.clone());
                    }
                }
                //println!("BEnd {:?}\n    {:?}\n    {:?}", i, syms, nums);
            }
            let mut ret = vec![];
            //println!("!!!!!!!!!!!!!1 syms : {:?}",syms);
            for (key, coefficient) in syms {

                if let Values::Product(mut vec) = key.to_owned() {
                        vec.push(Values::Number(BigDecimal::from(coefficient)));
                    ret.push(Values::Product( vec));
                    
                } else {
                    ret.push(Values::Product(vec![
                        key.to_owned(),
                        Values::Number(coefficient),
                    ]))
                }
            }
            let num = nums.iter().sum();
            if num != BigDecimal::from(0) {
                ret.push(Values::Number(num));
            }
            Values::Sum(ret)
        }
        Values::Product(ref vec) => {
            let one = BigDecimal::from(1);
            let mut nums = vec![];
            let mut syms = HashMap::new();
            for i in vec.iter().map(|x| const_folding(x.clone())) {
                //println!("     Prod {:?}\n     {:?}\n     {:?}", i, syms, nums);
                match i.clone() {
                    Values::Number(x) => nums.push(x),

                    Values::Exponent(box sym, box Values::Number(y)) => {
                        let dummy = y.clone();
                        syms.entry(sym)
                            .and_modify(move |x| *x += dummy)
                            .or_insert(y);
                    }
                    others => {
                        syms.entry(others)
                            .and_modify(move |x| *x += BigDecimal::from(1))
                            .or_insert(BigDecimal::from(1));
                    }
                }
                //println!("     End  {:?}\n     {:?}\n     {:?}", i, syms, nums);
            }
            let mut ret = vec![];
            for (key, coefficient) in syms {
                /*if coefficient == BigDecimal::from(0) {
                } else if coefficient == BigDecimal::from(1) {
                    ret.push(key)
                } else {*/
                    //println!("NUUUUUM : {coefficient}");

                    ret.push(Values::Exponent(
                        Box::new(key),
                        Box::new(Values::Number(coefficient)),
                    ))
                //}
                //println!("{:?}",ret);
            }
            let num = nums.iter().fold(BigDecimal::from(1), |mut a, b| {
                a *= b;
                a
            });
            if num != BigDecimal::from(1) {
                //println!("NUUUUUM : {num}");
                ret.push(Values::Number(num));
            }
            if ret.len()==1{
                let b = ret.get(0).unwrap().to_owned();
                return b
            }
            Values::Product(ret)
            
            
        }
        //Values::Exponent(box a , box b){}
        other => other,
    }
}

mod tests2 {
    use bigdecimal::{BigDecimal, FromPrimitive};

    use super::*;
    use crate::ast::parser::parse;
    use crate::ast::Localization;

    #[test]
    fn many_additions() {
        let parse_res = parse("(x+y)*(x+y);".to_string()).unwrap();
        let r = parse_res.get(0).unwrap();

        let result = convert_to_values(&r.tree);
        println!("result  : {:?} \n\n===========\n", result);
        let result = const_folding(result);
        println!("result  : {:?}", result);
        let expected = 1;

        true;
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
