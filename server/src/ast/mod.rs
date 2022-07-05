#![feature(map_first_last)]

use bigdecimal::BigDecimal;
use bigdecimal::One;
use bigdecimal::Zero;
use std::collections::HashMap;

use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
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
pub struct Node<Dummy,T>{
    value:T,
    location:(Localization,Localization),
    phantom:PhantomData<Dummy>,
}



impl <Dummy,T> Node<Dummy,T>{

    fn new(starts: Localization,ends:Localization, tree: T) -> Self {
        Self {value:tree,location:(starts,ends),phantom:PhantomData }
    }
    fn starts_at(&mut self,pos:Localization){
        self.location.0=pos
        
    }
    fn ends_at(&mut self,pos:Localization){
        self.location.1=pos
            
    }

    fn at(&mut self,start:Localization,end:Localization){
        self.location=(start,end)
    }



}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EVar;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ENum;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EFun;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ESum;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EMul;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ESub;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EExp;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EDiv;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ENeg;


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EExpression;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EAtom;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EBracketedExpression;



#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExpressionSyntaxTree {
    Variable(Node<EVar,String>),
    Number(Node<ENum,BigDecimal>),
    Fun(Node<EFun,(String, Vec<ExpressionSyntaxTree>)>),

    Sum(Box<Node<ESum,(ExpressionSyntaxTree,ExpressionSyntaxTree)>>),
    Product(Box<Node<EMul,(ExpressionSyntaxTree,ExpressionSyntaxTree)>>),
    
    Exponent(Box<Node<EExp,(ExpressionSyntaxTree,ExpressionSyntaxTree)>>),

    Subtraction(Box<Node<ESub,(ExpressionSyntaxTree,ExpressionSyntaxTree)>>),
    Division(Box<Node<EDiv,(ExpressionSyntaxTree,ExpressionSyntaxTree)>>),
    Negation(Box<Node<ENeg,ExpressionSyntaxTree>>),
}

//TODO Knuth=Bendix algo (Given x+0 = 0, x-x=0 ,.... Reduce x+x-x)
//#[derive(Clone, Debug, Eq, PartialEq)]
//pub struct UserRuleScheme(Vec<(ExpressionSyntaxTree,ExpressionSyntaxTree)>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntaxTree {
    Expression(ExpressionSyntaxTree),
    //UserRule(UserRuleScheme,ExpressionSyntaxTree),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Values {
    Number(BigDecimal),
    Variable(String),
    Sum(BigDecimal, BTreeMap<Values, BigDecimal>),
    Product(BigDecimal, BTreeMap<Values, BigDecimal>),
    Exponent(Box<(Values, Values)>),
}

pub struct Context {
    variable_dict: HashMap<String, (ExpressionSyntaxTree, Localization)>,
    fun_dict: HashMap<String, (ExpressionSyntaxTree, Localization)>,
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
                let map = vmap.clone();
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
                if vect.is_empty(){
                    write!(f,"0")

                }else 
{                vect.reverse();
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
                )}
            }
            Values::Product(constant, vmap) => {
                let  map = vmap.clone();
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
                                        match supscriptmap.get(&x) {
                                            None => ret,
                                            Some(r) => r.to_owned(),
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


trait Eval<Evaler,Ctx,Output>{
    fn eval(&self,ctx:Ctx) -> Output; 
}


struct SimpleEvaluator;

impl Eval<SimpleEvaluator,&mut Context,Values> for SyntaxTree{
    fn eval(&self, ctx: &mut Context) -> Values {
        match self{
            SyntaxTree::Expression(expr) => expr.eval(ctx),
        }
        
    }
}
impl Eval<SimpleEvaluator,&Context,Values> for ExpressionSyntaxTree{

fn eval(&self, ctx: &Context) -> Values {
        match &self {
            ExpressionSyntaxTree::Number(value) => {
                Values::Number(value.value.to_owned())
            }
            ExpressionSyntaxTree::Fun(value) =>

                {
                    todo!()

                },

            ExpressionSyntaxTree::Variable(Node{value, location:_ ,phantom:_}) => {
                Values::Variable(value.to_owned())
            }

            ExpressionSyntaxTree::Sum(box Node{value:(left,right), location:_,phantom:_ }) => {
                left.eval(ctx).add(right.eval(ctx))
            }
            ExpressionSyntaxTree::Product(box Node{value:(left,right), location:_,phantom:_ }) => {
                left.eval(ctx).mul(right.eval(ctx))
            }
            ExpressionSyntaxTree::Exponent(
                box Node{value:(left,right), location:_,phantom:_ }
            ) => {
                left.eval(ctx).exp(right.eval(ctx))
            }
            ExpressionSyntaxTree::Subtraction(
                box Node{value:(left,right), location:_,phantom:_ }
            ) => {
                left.eval(ctx).add(
                    right.eval(ctx)
                        .mul(Values::Number(BigDecimal::from(-1))),
                )
            }
            ExpressionSyntaxTree::Division(box Node{value:(left,right), location:_,phantom:_ }) => {
                left.eval(ctx).mul(
                    right.eval(ctx)
                        .exp(Values::Number(BigDecimal::from(-1))),
                )
            }
            ExpressionSyntaxTree::Negation(
                box Node{value:value, location:_ ,phantom:_}
            ) => {
                value.eval(ctx).mul(Values::Number(BigDecimal::from(-1)))
            }
        }
    }

}


impl ExpressionSyntaxTree {
    





    fn number(starts: Localization,ends:Localization, num: BigDecimal) -> Self {
            
            ExpressionSyntaxTree::Number(Node::new(starts, ends, num))
        }
    

    
    fn variable(starts: Localization,ends:Localization, name: String) -> Self {
            ExpressionSyntaxTree::Variable(Node::new(starts, ends, name))
        
    }

    fn add(starts: Localization,ends:Localization, left: Self, right: Self) -> Self {
        ExpressionSyntaxTree::Sum(Box::new(
               Node::new(
                   starts, ends,(left,right))
               )
           )
    }

    fn mul(starts: Localization,ends:Localization, left: Self, right: Self) -> Self {
        ExpressionSyntaxTree::Product(Box::new(
      Node::new(
                starts, ends,(left,right))
            )
    )
    }

    fn sub(starts: Localization,ends:Localization, left: Self, right: Self) -> Self {
        ExpressionSyntaxTree::Subtraction(Box::new(
                Node::new(
                    starts, ends,(left,right))
                )
            )
    }

    fn div(starts: Localization,ends:Localization, left: Self, right: Self) -> Self {
            ExpressionSyntaxTree::Division(
                Box::new(Node::new(
                    starts, ends,(left,right))
                )
            )
    }

    fn exp(starts: Localization,ends:Localization, left: Self, right: Self) -> Self {
            ExpressionSyntaxTree::Exponent(
                Box::new(Node::new(
                    starts, ends,(left,right))
                )
            )
    }

    fn neg(starts: Localization,ends:Localization, value: Self) -> Self
    {
        ExpressionSyntaxTree::Negation(
            Box::new(Node::new(
                starts, ends,value
            )))
    }

    fn fun(starts: Localization,ends:Localization, name: String,vec: Vec<Self>) -> Self
    {
        ExpressionSyntaxTree::Fun(
            Node::new(
                starts, ends,(name,vec)
            ))
    }

 
}

impl Display for SyntaxTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxTree::Expression(a) => write!(f, "{a}"),
        }
    }
}

impl Display for ExpressionSyntaxTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionSyntaxTree::Variable(Node{value, location:_ , phantom:_  }) => write!(f, "{}", value),
            ExpressionSyntaxTree::Fun(Node{value:(name,value), location:_ , phantom:_  }
            ) => write!(
                f,
                "{name}({})",
                value
                    .iter()
                    .map(|x| format!("{x}"))
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            ExpressionSyntaxTree::Number(Node{value, location:_ , phantom:_  }) => write!(f, "{}", value),
            ExpressionSyntaxTree::Sum(box Node{value:(left,right), location:_ , phantom:_  }) => write!(f, "({} + {})", left, right),
            ExpressionSyntaxTree::Product(box Node{value:(left,right), location:_ , phantom:_  }) => write!(f, "({} * {})", left, right),
            ExpressionSyntaxTree::Exponent(box Node{value:(left,right), location:_ , phantom:_  }) => write!(f, "({} ^ {})", left, right),
            ExpressionSyntaxTree::Subtraction(box Node{value:(left,right), location:_ , phantom:_  }) => write!(f, "({} - {})", left, right),
            ExpressionSyntaxTree::Division(box Node{value:(left,right), location:_ , phantom:_  }) => write!(f, "({} / {})", left, right),
            ExpressionSyntaxTree::Negation(box Node{value, location:_ , phantom:_  }) => write!(f, "-{}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let under_test = ExpressionSyntaxTree::neg(
            Localization::new(),Localization::new(),
            ExpressionSyntaxTree::add(
                Localization::new(),Localization::new(),
                ExpressionSyntaxTree::number(Localization::new(),Localization::new(), BigDecimal::from(1)),
                ExpressionSyntaxTree::mul(
                    Localization::new(),Localization::new(),
                    ExpressionSyntaxTree::number(Localization::new(),Localization::new(), BigDecimal::from(2)),
                    ExpressionSyntaxTree::exp(
                        Localization::new(),Localization::new(),
                        ExpressionSyntaxTree::number(Localization::new(),Localization::new(), BigDecimal::from(3)),
                        ExpressionSyntaxTree::sub(
                            Localization::new(),Localization::new(),
                            ExpressionSyntaxTree::number(Localization::new(),Localization::new(), BigDecimal::from(4)),
                            ExpressionSyntaxTree::div(
                                Localization::new(),Localization::new(),
                                ExpressionSyntaxTree::number(Localization::new(), Localization::new(),BigDecimal::from(5)),
                                ExpressionSyntaxTree::variable(Localization::new(), Localization::new(),"x".to_string()),
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
        let mut ctx = Context {
            variable_dict: HashMap::new(),
            fun_dict: HashMap::new(),
        };
        let result: Values = parse("x + x".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .eval(&mut ctx);

        let expected = "2 x";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn add_several_variables_and_convert_to_values() {
        let mut ctx = Context {
            variable_dict: HashMap::new(),
            fun_dict: HashMap::new(),
        };
        let result: Values = parse("x+y+3+y+2*x+y+2-4*y".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .eval(&mut ctx);

        let expected = "5 + 3 x - y";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn mul_several_variables_and_convert_to_values() {
        let mut ctx = Context {
            variable_dict: HashMap::new(),
            fun_dict: HashMap::new(),
        };
        let result: Values = parse("x*x*y*y ".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .eval(&mut ctx);

        let expected = "x² y²";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn mul_two_variables_add_and_convert_to_values() {
        let mut ctx = Context {
            variable_dict: HashMap::new(),
            fun_dict: HashMap::new(),
        };
        let result: Values = parse("x*y + y*x + 2*y*x + 3*x*y".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .eval(&mut ctx);

        let expected = "7 x y"; //; "7 x y";//; "( 0 + 7 * ( 1 * x ^ 1 * y ^ 1 ) )";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn mul_several_variables_and_convert_to_valuesa() {
        let mut ctx = Context {
            variable_dict: HashMap::new(),
            fun_dict: HashMap::new(),
        };
        let result: Values = parse("x*y * y*x * 2*y*x * 3*x*y".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .eval(&mut ctx);

        let expected = "6 x⁴ y⁴"; //; "( 6 * x ^ 4 * y ^ 4 )";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn mul_several_variables_and_add_and_convert_to_values2() {

        let mut ctx = Context {
            variable_dict: HashMap::new(),
            fun_dict: HashMap::new(),
        };
        let result: Values =
            parse("x*y * y*x * 2*y*x * 3*x*y + 5*x*y * 4*y*x * 2*y*x * 3*x*y".to_string())
                .unwrap()
                .pop()
                .unwrap()
            .eval(&mut ctx);

        let expected = "126 x⁴ y⁴";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn mul_several_variables_and_add_and_convert_to_values3() {
        let mut ctx = Context {
            variable_dict: HashMap::new(),
            fun_dict: HashMap::new(),
        };
        let result: Values =
            parse("x*y * y*x * y*2*x * x*y*3 + 5*x*y * 4*y*x * 2*y*x * 3*x*y".to_string())
                .unwrap()
                .pop()
                .unwrap()
            .eval(&mut ctx);

        let expected = "126 x⁴ y⁴";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn square_of_sum() {
        let mut ctx = Context {
            variable_dict: HashMap::new(),
            fun_dict: HashMap::new(),
        };
        let result: Values = parse("(x+y)*(x+y)".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .eval(&mut ctx);

        let expected = "2 x y + x² + y²";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn for_ranja() {
        let mut ctx = Context {
            variable_dict: HashMap::new(),
            fun_dict: HashMap::new(),
        };
        let result: Values = parse("(x-y)*(x+y)+(y-x)*(x+y)".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .eval(&mut ctx);

        let expected = "x² - y²";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn diff_of_square() {
        let mut ctx = Context {
            variable_dict: HashMap::new(),
            fun_dict: HashMap::new(),
        };
        let result: Values = parse("(x+y)*(x-y)".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .eval(&mut ctx);

        let expected = "x² - y²";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn diff_of_cubes() {
        let mut ctx = Context {
            variable_dict: HashMap::new(),
            fun_dict: HashMap::new(),
        };
        let result: Values = parse("(x-y)*(x^2+y^2+x*y)".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .eval(&mut ctx);

        let expected = "x³ - y³";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn cube_of_sum() {
        let mut ctx = Context {
            variable_dict: HashMap::new(),
            fun_dict: HashMap::new(),
        };
        let result: Values = parse("(x+y)*(x+y)*(x+y)".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .eval(&mut ctx);

        let expected = "3 x y² + 3 x² y + x³ + y³";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn cube_of_diff() {
        let mut ctx = Context {
            variable_dict: HashMap::new(),
            fun_dict: HashMap::new(),
        };
        let result: Values = parse("(x-y)*(x-y)*(x-y)".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .eval(&mut ctx);

        let expected = "3 x y² + -3 x² y + x³ - y³";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn fourth_of_sum() {
        let mut ctx = Context {
            variable_dict: HashMap::new(),
            fun_dict: HashMap::new(),
        };
        let result: Values = parse("(x+y)*(x+y)*(x+y)*(x+y)".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .eval(&mut ctx);

        let expected = "4 x y³ + 6 x² y² + 4 x³ y + x⁴ + y⁴";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn fifth_of_sum() {
        let mut ctx = Context {
            variable_dict: HashMap::new(),
            fun_dict: HashMap::new(),
        };
        let result: Values = parse("(x+y)*(x+y)*(x+y)*(x+y)*(x+y)".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .eval(&mut ctx);

        let expected = "5 x y⁴ + 10 x² y³ + 10 x³ y² + 5 x⁴ y + x⁵ + y⁵";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }

    #[test]
    fn sixth_of_sum() {
        let mut ctx = Context {
            variable_dict: HashMap::new(),
            fun_dict: HashMap::new(),
        };
        let result: Values = parse("(x+y)*(x+y)*(x+y)*(x+y)*(x+y)*(x+y)".to_string())
            .unwrap()
            .pop()
            .unwrap()
            .eval(&mut ctx);

        let expected = "6 x y⁵ + 15 x² y⁴ + 20 x³ y³ + 15 x⁴ y² + 6 x⁵ y + x⁶ + y⁶";
        let mut result_text = String::new();
        write!(result_text, "{result}").unwrap();
        assert_eq!(expected, result_text)
    }
}
