use core::convert::identity;
use core::str::Chars;
use std::str::FromStr;

use bigdecimal::BigDecimal;
use log::debug;
use parser_combinator::either::Either;
use parser_combinator::either::Either3;
use parser_combinator::pair::Pair;
use parser_combinator::parser::{match_anything, match_character, match_literal, Parser};
use parser_combinator::repeated::RepeatedParser;
use parser_combinator::triple::Triple;
use parser_combinator::*;

use error::ErrorMessage;

use crate::ast::ExpressionSyntaxTree;
use crate::ast::Localization;
//use crate::ast::LocalizedSyntaxNode;
use crate::ast::Node;
use crate::ast::SyntaxTree;
use crate::most_important_of;

use super::EAtom;
use super::EBracketedExpression;
use super::EDiv;
use super::EExp;
use super::EExpression;
use super::EFun;
use super::EMul;
use super::ENeg;
use super::ENum;
use super::ESub;
use super::ESum;
use super::EVar;

mod error;
#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
struct State{
    start:Localization,
    end:Localization,

}

trait  Parsable<'a,T> where
    
    T:Parse<'a,Chars<'a>,State,ExpressionSyntaxTree,ErrorMessage>
{
    
    fn parse(self,s:State) -> ParseResult<'a>;
}


impl <'a> Parsable<'a,EExpression> for Chars<'a> where
{
    fn parse(self,s:State)-> ParseResult<'a>  {
        EExpression.parse(self, s)
    }
}

impl <'a> Parsable<'a,EExpression> for &'a str where
{
    fn parse(self,s:State)-> ParseResult<'a>  {
        EExpression.parse(self.chars(), s)
    }
}


impl State{
    fn new() ->State {
        State{start:Localization::new(),end:Localization::new()}
    }

    fn transit_generator<'a>(n:usize,m:usize) -> impl Fn(State) -> State{
         move |state: State| State {
            start: state.end,
            end: Localization { line: state.end.line+n, column: state.end.column+m },
        }
    }

}

fn state_trans(x:State)->State{x}

type ParseResult<'a> =
    Result<(ExpressionSyntaxTree, State, Chars<'a>), ErrorMessage>;

pub fn parse(input: String) -> Result<Vec<SyntaxTree>, ErrorMessage> {
    todo!()
}

impl<'a> Parse<'a, Chars<'a>, State,ExpressionSyntaxTree, ErrorMessage> for ExpressionSyntaxTree {
    fn parse(
        &self,
        input: Chars<'a>,state:State,
    ) -> ParseResult<'a> {
        todo!()
    }
}

impl <'b > Parse<'b, Chars<'b>,State, ExpressionSyntaxTree, ErrorMessage> for EVar
{
    fn parse(
        &self,
        input: Chars<'b>,state:State,
    ) -> ParseResult<'b> {
        let transformer = move |letters: Vec<char>,curr_state:State| {
            ExpressionSyntaxTree::variable(
                state.end,
                curr_state.end,
                letters.iter().collect(),
            )
        };

        match_anything(State::transit_generator(1, 0))
            .validate(
                |character: &char| character.is_alphabetic(),
                "alphabetic character".to_string(),
            )
            .one_or_more()
            .transform_with_state(transformer)
            .with_error(|err, input| ErrorMessage::new(err, 0, Localization::new()))
            .parse(input,state)
    }
}

impl<'a> Parse<'a, Chars<'a>,State, ExpressionSyntaxTree, ErrorMessage> for ENum {
    fn parse(
        &self,
        input: Chars<'a>,state:State,
    ) -> ParseResult<'a> {
        let to_integer = |characters: Vec<char>| {
            characters
                .iter()
                .fold(String::new(), |mut result, character| {
                    result.push(*character);
                    result
                })
        };

        let parse_natural_numbers = match_anything(State::transit_generator(1, 0))
            .validate(
                |character: &char| character.is_numeric(),
                "numeric character".to_string(),
            )
            .one_or_more()
            .transform(to_integer);

        let parse_natural_numbers_num = match_anything(State::transit_generator(1, 0))
            .validate(
                |character: &char| character.is_numeric(),
                "numeric character".to_string(),
            )
            .one_or_more()
            .transform(to_integer)
            .transform_with_state(move |num,curr_state| {
                ExpressionSyntaxTree::number(
                    state.end,
                    curr_state.end,
                    BigDecimal::from_str(&*num).unwrap(),
                )
            });

        let separator_parser = match_literal(".".chars(),State::transit_generator(1, 0))
            .or_else(match_literal("e-".chars(),State::transit_generator(2, 0)))
            .or_else(match_literal("e".chars(),State::transit_generator(1, 0)))
            .with_error(|_, input| {
                format!(
                    "expected decimal separator '.' or 'e', got {} ",
                    input.collect::<String>(),
                )
            });

        let to_float = move |(leading, separator, fractional): (String, Chars, String),curr_state:State| {
            let num = format!("{}{}{}", leading, separator.as_str(), fractional.as_str());
            ExpressionSyntaxTree::number(
                state.end,
                curr_state.end,
                BigDecimal::from_str(&*num).unwrap(),
            )
        };

        Triple::new(
            parse_natural_numbers.clone(),
            separator_parser,
            parse_natural_numbers,
        )
        .transform_with_state(to_float)
        
        .with_error(|error, _| {
            ErrorMessage::new(
                error.fold(identity, identity, identity),
                1,
                Localization::new(),
            )
        }).or_else(parse_natural_numbers_num)
            .with_error(|(a,b), _| {
                ErrorMessage::new(
                    "number failed".to_string(),
                    1,
                    Localization::new(),
                )})
        .parse(input,state)
    }
}

fn whitespace<'a>(
    input: Chars<'a>,state:State,
) -> parser_combinator::ParseResult<Chars<'a>,State, char, ErrorMessage> {
    

    let space = match_anything(State::transit_generator(1, 0)).validate(
        |character: &char| character == &' ',
        "alphabetic character".to_string(),
    );
    let newline = match_anything(State::transit_generator(0, 1)).validate(
        |character: &char| character == &'\n',
        "alphabetic character".to_string(),
    );

    space
        .or_else(newline)
        .with_error(|(a, b), _| ErrorMessage::new("{a} {b}".to_string(),0,Localization::at(69, 420)))
        .parse(input, state)

}

type BNode<M> = Node<M, (ExpressionSyntaxTree, ExpressionSyntaxTree)>;

impl<'a> Parse<'a, Chars<'a>,State, ExpressionSyntaxTree, ErrorMessage> for ESub {
    fn parse(
        &self,
        input: Chars<'a>,state:State,
    ) -> ParseResult<'a> {
        let transformer =
            move |(mut first, vec): (ExpressionSyntaxTree, Vec<(State, ExpressionSyntaxTree)>)| {
                if vec.is_empty() {
                    first
                } else {
                    for (curr_state, second) in vec {
                        first = ExpressionSyntaxTree::sub(
                            curr_state.start,
                            curr_state.end,
                            first,
                            second,
                        )
                    }
                    first
                }
            };

        ESum.pair(
            match_literal("-".chars(),State::transit_generator(1, 0))
                .transform_with_state(|_,curr_state:State|curr_state).pair(Self).zero_or_more()
        )
            .transform(transformer)
            .with_error(|err, input| match err {
                Either::Left(err) => err,
                Either::Right(Either::Left(err)) => ErrorMessage::new(err, 1, Localization::new()),
                Either::Right(Either::Right(err)) => err,
            })
            .parse(input,state)
    }
}

impl<'a> Parse<'a, Chars<'a>,State, ExpressionSyntaxTree, ErrorMessage> for ESum {
    fn parse(
        &self,
        input: Chars<'a>,state:State,
    ) -> ParseResult<'a> {
        let transformer =
            move |(mut first, vec): (ExpressionSyntaxTree, Vec<(State, ExpressionSyntaxTree)>)| {
                if vec.is_empty() {
                    first
                } else {
                    for (curr_state, second) in vec {
                        first = ExpressionSyntaxTree::add(
                            curr_state.start,
                            curr_state.end,
                            first,
                            second,
                        )
                    }
                    first
                }
            };

        EMul.pair(match_literal("+".chars(), State::transit_generator(1, 0)).transform_with_state(|_,s|s).pair(Self).zero_or_more())
            .transform(transformer)
            .with_error(|err, input| match err {
                Either::Left(err) => err,
                Either::Right(Either::Left(err)) => ErrorMessage::new(err, 1, Localization::new()),
                Either::Right(Either::Right(err)) => err,
            })
            .parse(input,state)
    }
}

impl<'a> Parse<'a, Chars<'a>,State, ExpressionSyntaxTree, ErrorMessage> for EMul {
    fn parse(
        &self,
        input: Chars<'a>,state:State,
    ) -> ParseResult<'a> {
        let transformer =
            move |(mut first, vec): (ExpressionSyntaxTree, Vec<(State, ExpressionSyntaxTree)>)| {
                if vec.is_empty() {
                    first
                } else {
                    for (curr_state, second) in vec {
                        first = ExpressionSyntaxTree::mul(
                            curr_state.start,
                            curr_state.end,
                            first,
                            second,
                        )
                    }
                    first
                }
            };

        EDiv.pair(match_literal("*".chars(), State::transit_generator(1, 0)).transform_with_state(|_,s|s).pair(Self).zero_or_more())
            .transform(transformer)
            .with_error(|err, input| match err {
                Either::Left(err) => err,
                Either::Right(Either::Left(err)) => ErrorMessage::new(err, 1, Localization::new()),
                Either::Right(Either::Right(err)) => err,
            })
            .parse(input,state)
    }
}

impl<'a> Parse<'a, Chars<'a>,State, ExpressionSyntaxTree, ErrorMessage> for EDiv {
    fn parse(
        &self,
        input: Chars<'a>,state:State,
    ) -> ParseResult<'a> {
        let transformer =
            move |(mut first, vec): (ExpressionSyntaxTree, Vec<(State, ExpressionSyntaxTree)>)| {
                if vec.is_empty() {
                    first
                } else {
                    for (curr, second) in vec {
                        first = ExpressionSyntaxTree::div(
                            curr.start,
                            curr.end,
                            first,
                            second,
                        )
                    }
                    first
                }
            };

        EExp.pair(match_literal("/".chars(),State::transit_generator(1, 0)).transform_with_state(|_,s|s).pair(Self).zero_or_more())
            .transform(transformer)
            .with_error(|err, input| match err {
                Either::Left(err) => err,
                Either::Right(Either::Left(err)) => ErrorMessage::new(err, 1, Localization::new()),
                Either::Right(Either::Right(err)) => err,
            })
            .parse(input,state)
    }
}

impl<'a> Parse<'a, Chars<'a>,State, ExpressionSyntaxTree, ErrorMessage> for EExp {
    fn parse(
        &self,
        input: Chars<'a>,state:State,
    ) -> ParseResult<'a> {
        let exponentiation_operator_parser =
            match_literal("^".chars(), State::transit_generator(1, 0)).with_error(|_, input: Chars| {
                ErrorMessage::exponentiation_failed(format!(
                    "expected operator ^, got {}",
                    input.collect::<String>()
                ))
            }).transform_with_state(|_,s|s);

        let exponent_parser =
            RepeatedParser::zero_or_more(Pair::new(exponentiation_operator_parser, Self))
                .with_error(|error, _| error.reduce());

        Pair::new(ENeg, exponent_parser)
            .transform(move |(x, y)| {
                y.into_iter().fold(x, |left, (s, right)| {
                    ExpressionSyntaxTree::exp(s.start, s.end, left, right)
                })
            })
            .with_error(|error, _| error.reduce())
            .parse(input,state)
    }
}

impl<'a> Parse<'a, Chars<'a>,State, ExpressionSyntaxTree, ErrorMessage> for ENeg {
    fn parse(
        &self,
        input: Chars<'a>,state:State,
    ) -> ParseResult<'a> {
        let error_mapper = |(sign_error, expression_in_brackets_error), input: Chars| {
            let message = format!(
                "expected '-' or {}, got '{}'",
                expression_in_brackets_error,
                input.collect::<String>(),
            );

            let new_message = match sign_error {
                Either::Left(_) => ErrorMessage::sign_failed(message, Localization::new()),
                Either::Right(message) => message,
            };

            most_important_of!(new_message, expression_in_brackets_error)
        };

        let prefix_parser = match_literal("-".chars(), State::transit_generator(1, 0))
            .pair(Self)
            .transform(move |(op, x)| {
                ExpressionSyntaxTree::neg(Localization::new(), Localization::new(), x)
            });

        prefix_parser
            .or_else(EBracketedExpression)
            .with_error(error_mapper)
            .skip(whitespace)
            .parse(input,state)
    }
}

impl<'a> Parse<'a, Chars<'a>,State, ExpressionSyntaxTree, ErrorMessage> for EBracketedExpression {
    fn parse(
        &self,
        input: Chars<'a>,state:State,
    ) -> ParseResult<'a> {
        let error_mapper =
            |(expression_in_brackets_error, atom_error), _| match expression_in_brackets_error {
                Either3::Left(_) => most_important_of!(
                    ErrorMessage::missing_opening_parenthesis(format!("'(' or {}", atom_error)),
                    atom_error
                ),
                Either3::Middle(message) => message,
                Either3::Right(_) => ErrorMessage::missing_closing_parenthesis(
                    "missing closing parenthesis".to_string(),
                ),
            };

        Triple::new(
            match_literal("(".chars(), State::transit_generator(1, 0)),
            EExpression,
            match_literal(")".chars(), State::transit_generator(1, 0)),
        )
        .second()
        .or_else(EAtom)
        .with_error(error_mapper)
        .parse(input,state)
    }
}

impl<'a> Parse<'a, Chars<'a>,State, ExpressionSyntaxTree, ErrorMessage> for EAtom {
    fn parse(
        &self,
        input: Chars<'a>,state:State,
    ) -> ParseResult<'a> {
        EFun.or_else(ENum).with_error(|(err, _), _| err)
        .or_else(EVar)
            .with_error(|(err, _), _| err)
            .skip(whitespace)
            .parse(input,state)
    }
}

impl<'a> Parse<'a, Chars<'a>,State, ExpressionSyntaxTree, ErrorMessage> for EExpression {
    fn parse(
        &self,
        input: Chars<'a>,state:State,
    ) -> ParseResult<'a> {
        if !input.clone().any(|char| !char.is_whitespace()) {
            return Err(ErrorMessage::empty_expression(format!(
                "expected expression, got '{}'",
                input.collect::<String>()
            )));
        }
        ESub.parse(input,state)
    }
}


impl<'a> Parse<'a, Chars<'a>,State, ExpressionSyntaxTree, ErrorMessage> for EFun {
    fn parse(
        &self,
        input: Chars<'a>,state:State,
    ) -> ParseResult<'a> {
        let tuple = Triple::new(
            match_literal("(".chars(), State::transit_generator(1, 0)),
                EExpression
                    .separated_by(match_literal(",".chars(), state_trans))
                    .transform(|(x, y)| {let mut ret = vec![x];ret.extend(y.iter().map(|(a, b)| b.to_owned()));ret}),
            match_literal(")".chars(), State::transit_generator(1, 0)),
            )
                .second();

        match_anything(State::transit_generator(1, 0))
                .validate(
                    |character: &char| character.is_alphabetic(),
                    "alphabetic character".to_string(),
                )
                .one_or_more()
            .pair(tuple).transform_with_state(move |(x, y),curr_state|
                {
                    ExpressionSyntaxTree::fun(state.end,curr_state.end ,x.into_iter().collect(), y)
                }).with_error(|_,_| ErrorMessage::new("func failed".to_string(),1,Localization::new())).parse(input,state)
        }

    }

#[test]
fn pof() {
    let state = State::new();
    let a = "-f(1,x+x,2*2/4,5*(2+4))+1.2+x+y-z+c-t+1.3*1-6/6".chars().parse(state);
    println!("{:?}", a);
    assert!(false)
}
/*
fn parse_sign(input: CharWrapper) -> ParseResult {
    let error_mapper = |(sign_error, expression_in_brackets_error), input: CharWrapper| {
        let message = format!(
            "expected '-' or {}, got '{}'",
            expression_in_brackets_error,
            input.chars.collect::<String>(),
        );

        let new_message = match sign_error {
            Either::Left(_) => ErrorMessage::sign_failed(message, input.end),
            Either::Right(message) => message,
        };

        most_important_of!(new_message, expression_in_brackets_error)
    };

    let prefix_parser = match_literal(CharWrapper::new("-".chars()), state_trans)
        .pair(parse_sign)
        .transform(move |(op, x)| LocalizedSyntaxNode::neg(op.end, x));

    prefix_parser
        .or_else(parse_expression_in_brackets)
        .with_error(error_mapper)
        .skip(whitespace)
        .parse(input,state)
}

fn parse_expression_in_brackets(input: CharWrapper) -> ParseResult {
    let error_mapper =
        |(expression_in_brackets_error, atom_error), _| match expression_in_brackets_error {
            Either3::Left(_) => most_important_of!(
                ErrorMessage::missing_opening_parenthesis(format!("'(' or {}", atom_error)),
                atom_error
            ),
            Either3::Middle(message) => message,
            Either3::Right(_) => {
                ErrorMessage::missing_closing_parenthesis("missing closing parenthesis".to_string())
            }
        };

    Triple::new(
        match_literal(CharWrapper::new("(".chars()), state_trans),
        parse_expression,
        match_literal(CharWrapper::new(")".chars()), state_trans),
    )
    .second()
    .or_else(parse_atom)
    .with_error(error_mapper)
    .parse(input,state)
}

fn parse_atom(input: CharWrapper) -> ParseResult {
    let integer_parser = parse_natural_numbers.transform(move |numeric_string| {
        LocalizedSyntaxNode::number(input.start, BigDecimal::from_str(&*numeric_string).unwrap())
    });

    let float_parser = parse_float.transform(move |x| LocalizedSyntaxNode::number(input.start, x));

    float_parser
        .or_else(integer_parser)
        .with_error(|_, _| ErrorMessage::atom_failed("a number".to_string()))
        //.or_else(parse_function_call)
        .or_else(parse_identifier)
        .with_error(|(err, _), _| err)
        .skip(whitespace)
        .parse(input,state)
}

fn parse_float(
    input: CharWrapper,
) -> parser_combinator::ParseResult<CharWrapper, BigDecimal, String> {
    let separator_parser = match_literal(CharWrapper::new(".".chars()), state_trans)
        .or_else(match_literal(CharWrapper::new("e-".chars())), state_trans)
        .or_else(match_literal(CharWrapper::new("e".chars())), state_trans)
        .with_error(|_, input| {
            format!(
                "expected decimal separator '.' or 'e', got {} ",
                input.chars.as_str(),
            )
        })
        .transform(|y| y.chars);

    let to_float = |(leading, separator, fractional): (String, Chars, String)| {
        let num = format!("{}{}{}", leading, separator.as_str(), fractional.as_str());
        BigDecimal::from_str(&*num).unwrap()
    };

    Triple::new(
        parse_natural_numbers,
        separator_parser,
        parse_natural_numbers,
    )
    .transform(to_float)
    .with_error(|error, _| error.fold(identity, identity, identity))
    .parse(input,state)
}

fn parse_natural_numbers(
    input: CharWrapper,
) -> parser_combinator::ParseResult<CharWrapper, String, String> {
    let to_integer = |characters: Vec<char>| {
        characters
            .iter()
            .fold(String::new(), |mut result, character| {
                result.push(*character);
                result
            })
    };

    match_anything()
        .validate(
            |character: &char| character.is_numeric(),
            "numeric character".to_string(),
        )
        .one_or_more()
        .transform(to_integer)
        .parse(input,state)
}
/*
*/
/*

fn semicolon(
    input: CharWrapper,
) -> parser_combinator::ParseResult<CharWrapper, char, ErrorMessage> {
    match_anything()
        .validate(
            |character| character == &';',
            "expected semicolon".to_string(),
        )
        .with_error(|err, input: CharWrapper| ErrorMessage::new(err, 0, input.end))
        .parse(input,state)
}
*/
*/
