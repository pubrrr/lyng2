use core::convert::identity;
use core::str::Chars;
use std::str::FromStr;

use bigdecimal::BigDecimal;
use parser_combinator::either::Either;
use parser_combinator::either::Either3;
use parser_combinator::pair::Pair;
use parser_combinator::parser::{match_anything, match_character, match_literal};
use parser_combinator::repeated::RepeatedParser;
use parser_combinator::triple::Triple;
use parser_combinator::*;

use error::ErrorMessage;

use crate::ast::Localization;
use crate::ast::LocalizedSyntaxNode;
use crate::ast::SyntaxTree;
use crate::most_important_of;

mod error;
#[cfg(test)]
mod tests;

type ParseResult<'a> =
    parser_combinator::ParseResult<'a, CharWrapper<'a>, LocalizedSyntaxNode, ErrorMessage>;

#[derive(Clone, Debug)]
struct CharWrapper<'a> {
    chars: Chars<'a>,
    start: Localization,
    end: Localization,
}

impl<'a> CharWrapper<'a> {
    fn new(chars: Chars<'a>) -> CharWrapper<'a> {
        Self {
            chars,
            start: Localization::new(),
            end: Localization::new(),
        }
    }
}

impl<'a> Iterator for CharWrapper<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        self.start = self.end;
        self.end.column += 1;
        let next = self.chars.next();

        if next == Some('\n') {
            self.end.line += 1;
            self.end.column = 0;
        }
        next
    }
}

pub fn parse(input: String) -> Result<Vec<LocalizedSyntaxNode>, ErrorMessage> {
    let (result, leftover) = parse_expression
        .separated_by(match_character(';'))
        .with_error(|err, _| {
            err.fold(identity, |err2| {
                err2.fold(ErrorMessage::forgot_comma, identity)
            })
        })
        .with_error(|error, _| error.map_message(|message| format!("Syntax Error: {}", message)))
        .transform(move |(first, rest)| {
            let mut result = vec![first];
            for (_, fragment) in rest {
                result.push(fragment);
            }
            result
        })
        .skip(whitespace)
        .skip(semicolon)
        .parse(CharWrapper::new(input.chars()))?;

    let leftover_string: String = leftover.chars.collect();
    if !leftover_string.is_empty() {
        return Err(ErrorMessage::leftover(leftover_string, leftover.end));
    }

    Ok(result)
}

fn parse_expression(input: CharWrapper) -> ParseResult {
    if input.clone().filter(|char| !char.is_whitespace()).count() == 0 {
        return Err(ErrorMessage::empty_expression(format!(
            "expected expression, got '{}'",
            input.chars.collect::<String>()
        )));
    }

    let add_or_subtract = match_literal(CharWrapper::new("+".chars()))
        .or_else(match_literal(CharWrapper::new("-".chars())))
        .with_error(|_, input: CharWrapper| {
            ErrorMessage::term_failed(format!(
                "expected operator + or -, got {}",
                input.chars.collect::<String>()
            ))
        })
        .peek_and_transform(|mut x, y| {
            x.end = y.start;
            x
        });

    let build_tree =
        move |(mut x, y): (LocalizedSyntaxNode, Vec<(CharWrapper, LocalizedSyntaxNode)>),
              rest: CharWrapper| {
            for (operator, syntax_tree) in y {
                println!("{:?}  {:?}", operator, rest);
                let op = &*operator.chars.collect::<String>();
                match op {
                    "+" => x = LocalizedSyntaxNode::add(operator.end, x, syntax_tree),
                    "-" => x = LocalizedSyntaxNode::sub(operator.end, x, syntax_tree),
                    _ => panic!("this should never happen"),
                }
            }
            x
        };

    parse_term
        .separated_by(add_or_subtract)
        .with_error(|error, _| error.fold(identity, |separator_error| separator_error.reduce()))
        .peek_and_transform(build_tree)
        .parse(input)
}

fn parse_term(input: CharWrapper) -> ParseResult {
    let multiply_or_divide = match_literal(CharWrapper::new("*".chars()))
        .or_else(match_literal(CharWrapper::new("/".chars())))
        .with_error(|_, input: CharWrapper| {
            ErrorMessage::term_failed(format!(
                "expected operator * or /, got {}",
                input.collect::<String>()
            ))
        })
        .peek_and_transform(|mut x, y| {
            x.end = y.start;
            x
        });

    let build_tree =
        move |(mut x, y): (LocalizedSyntaxNode, Vec<(CharWrapper, LocalizedSyntaxNode)>)| {
            for (operator, syntax_tree) in y {
                println!("{:?}", operator);
                let op = &*operator.chars.collect::<String>();
                match op {
                    "*" => x = LocalizedSyntaxNode::mul(operator.end, x, syntax_tree),
                    "/" => x = LocalizedSyntaxNode::div(operator.end, x, syntax_tree),
                    _ => panic!("this should never happen"),
                }
            }
            x
        };

    parse_exponent
        .separated_by(multiply_or_divide)
        .with_error(|error, _| error.fold(identity, |separator_error| separator_error.reduce()))
        .transform(build_tree)
        .parse(input)
}

fn parse_exponent(input: CharWrapper) -> ParseResult {
    let exponentiation_operator_parser = match_literal(CharWrapper::new("^".chars()))
        .with_error(|_, input: CharWrapper| {
            ErrorMessage::exponentiation_failed(format!(
                "expected operator ^, got {}",
                input.chars.collect::<String>()
            ))
        })
        .peek_and_transform(|mut x, y| {
            x.end = y.start;
            x
        });

    let exponent_parser =
        RepeatedParser::zero_or_more(Pair::new(exponentiation_operator_parser, parse_exponent))
            .with_error(|error, _| error.reduce());

    Pair::new(parse_sign, exponent_parser)
        .transform(move |(x, y)| {
            y.into_iter().fold(x, |left, (op, right)| {
                LocalizedSyntaxNode::exp(op.end, left, right)
            })
        })
        .with_error(|error, _| error.reduce())
        .parse(input)
}

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

    let prefix_parser = match_literal(CharWrapper::new("-".chars()))
        .pair(parse_sign)
        .transform(move |(op, x)| LocalizedSyntaxNode::neg(op.end, x));

    prefix_parser
        .or_else(parse_expression_in_brackets)
        .with_error(error_mapper)
        .skip(whitespace)
        .parse(input)
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
        match_literal(CharWrapper::new("(".chars())),
        parse_expression,
        match_literal(CharWrapper::new(")".chars())),
    )
    .second()
    .or_else(parse_atom)
    .with_error(error_mapper)
    .parse(input)
}

fn parse_atom(input: CharWrapper) -> ParseResult {
    let integer_parser = parse_natural_numbers.transform(move |numeric_string| {
        LocalizedSyntaxNode::number(input.start, BigDecimal::from_str(&*numeric_string).unwrap())
    });

    let float_parser = parse_float.transform(move |x| LocalizedSyntaxNode::number(input.start, x));

    float_parser
        .or_else(integer_parser)
        .with_error(|_, _| ErrorMessage::atom_failed("a number".to_string()))
        .or_else(parse_identifier)
        .with_error(|(err, _), _| err)
        .skip(whitespace)
        .parse(input)
}

fn parse_float(
    input: CharWrapper,
) -> parser_combinator::ParseResult<CharWrapper, BigDecimal, String> {
    let separator_parser = match_literal(CharWrapper::new(".".chars()))
        .or_else(match_literal(CharWrapper::new("e-".chars())))
        .or_else(match_literal(CharWrapper::new("e".chars())))
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
    .parse(input)
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
        .parse(input)
}

fn parse_identifier(input: CharWrapper) -> ParseResult {
    let transformer = move |letters: Vec<char>| LocalizedSyntaxNode {
        tree: SyntaxTree::Variable(letters.into_iter().collect()),
        location: input.end,
    };

    match_anything()
        .validate(
            |character: &char| character.is_alphabetic(),
            "alphabetic character".to_string(),
        )
        .one_or_more()
        .transform(transformer)
        .with_error(|err, input: CharWrapper| ErrorMessage::new(err, 0, input.end))
        .parse(input)
}

fn whitespace(
    input: CharWrapper,
) -> parser_combinator::ParseResult<CharWrapper, char, ErrorMessage> {
    match_anything()
        .validate(
            |character: &char| character == &' ' || character == &'\n',
            "alphabetic character".to_string(),
        )
        .with_error(|err, input: CharWrapper| ErrorMessage::new(err, 0, input.end))
        .parse(input)
}

fn semicolon(
    input: CharWrapper,
) -> parser_combinator::ParseResult<CharWrapper, char, ErrorMessage> {
    match_anything()
        .validate(
            |character| character == &';',
            "expected semicolon".to_string(),
        )
        .with_error(|err, input: CharWrapper| ErrorMessage::new(err, 0, input.end))
        .parse(input)
}
