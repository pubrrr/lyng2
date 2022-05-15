use std::ops::{Add, Sub};

use bigdecimal::BigDecimal;

use crate::ast::parser::error::ErrorMessage;
use crate::ast::{Localization, LocalizedSyntaxNode, SyntaxTree};

impl From<(usize, usize)> for Localization {
    fn from((line, column): (usize, usize)) -> Self {
        Localization::at(line, column)
    }
}

impl From<usize> for Localization {
    fn from(column: usize) -> Self {
        Localization::at(0, column)
    }
}

#[derive(Debug, Clone)]
struct SyntaxTreeMatcher {
    localization_matcher: Option<Localization>,
    node_matcher: NodeMatcher,
}

impl SyntaxTreeMatcher {
    fn at<L: Into<Localization>>(mut self, localization: L) -> Self {
        self.localization_matcher = Some(localization.into());
        self
    }

    fn assert_matches(self, result: Result<Vec<LocalizedSyntaxNode>, ErrorMessage>) {
        let result = result.unwrap();
        assert_eq!(1, result.len());
        result[0].assert_matches(self);
    }
}

impl Add for SyntaxTreeMatcher {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        SyntaxTreeMatcher {
            localization_matcher: None,
            node_matcher: NodeMatcher::Sum(Box::from(self), Box::from(rhs)),
        }
    }
}

impl Sub for SyntaxTreeMatcher {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        SyntaxTreeMatcher {
            localization_matcher: None,
            node_matcher: NodeMatcher::Subtraction(Box::from(self), Box::from(rhs)),
        }
    }
}

#[derive(Debug, Clone)]
enum NodeMatcher {
    Number(BigDecimal),
    Sum(Box<SyntaxTreeMatcher>, Box<SyntaxTreeMatcher>),
    Subtraction(Box<SyntaxTreeMatcher>, Box<SyntaxTreeMatcher>),
}

fn number<N: Into<BigDecimal>>(number: N) -> SyntaxTreeMatcher {
    SyntaxTreeMatcher {
        localization_matcher: None,
        node_matcher: NodeMatcher::Number(number.into()),
    }
}

impl LocalizedSyntaxNode {
    fn assert_matches(&self, expected: SyntaxTreeMatcher) {
        match expected.localization_matcher {
            None => {}
            Some(localization) => assert_eq!(
                localization, self.location,
                "actual: {self:?},\nexpected: {expected:?}"
            ),
        };

        let output = expected.node_matcher.clone();
        match (&self.tree, expected.node_matcher) {
            (SyntaxTree::Number(actual), NodeMatcher::Number(expected)) => {
                assert_eq!(*actual, expected)
            }
            (SyntaxTree::Sum(inner), NodeMatcher::Sum(expected_left, expected_right))
            | (
                SyntaxTree::Subtraction(inner),
                NodeMatcher::Subtraction(expected_left, expected_right),
            ) => {
                let (actual_left, actual_right) = &**inner;
                actual_left.assert_matches(*expected_left);
                actual_right.assert_matches(*expected_right);
            }
            (_, _) => panic!("{:?} does not match {:?}", self.tree, output),
        }
    }
}

mod atoms {
    use bigdecimal::{BigDecimal, FromPrimitive};

    use crate::ast::parser::parse;
    use crate::ast::parser::tests::number;
    use crate::ast::Localization;

    #[test]
    fn integer() {
        let result = parse("123;".to_string());

        let expected = number(123).at(0);

        expected.assert_matches(result);
    }

    #[test]
    fn float() {
        let result = parse("12.340  ;".to_string());

        let expected = number(BigDecimal::from_f32(12.34).unwrap()).at(0);

        expected.assert_matches(result);
    }

    #[test]
    fn malformed_float() {
        let result = parse("12.34.56 ;".to_string());

        let message = result.expect_err("12.34.56 is not a number");
        assert_eq!(
            "expected end of input, '.56 ;' was left".to_string(),
            message.message
        );
        assert_eq!(Localization::at(0, 5), message.localization);
    }

    #[test]
    fn empty_input_is_invalid() {
        let result = parse(" ".to_string());

        let message = result.expect_err("empty input is not allowed");
        assert_eq!(
            "Syntax Error: expected expression, got ''".to_string(),
            message.message
        );
        assert_eq!(Localization::at(0, 0), message.localization);
    }

    #[test]
    fn only_linebreak_is_invalid() {
        let result = parse("\n".to_string());

        let message = result.expect_err("empty input is not allowed");
        assert_eq!(
            "Syntax Error: expected expression, got ''".to_string(),
            message.message
        );
        assert_eq!(Localization::at(0, 0), message.localization);
    }
}

mod add {
    use crate::ast::parser::parse;
    use crate::ast::parser::tests::number;
    use crate::ast::Localization;

    #[test]
    fn add_two_integers() {
        let result = parse("123 + 456".to_string());

        let expected = (number(123).at(0) + number(456).at(5)).at(4);

        expected.assert_matches(result);
    }

    #[test]
    fn add_three_integers() {
        let result = parse("123 + 456 + 789".to_string());

        let expected = ((number(123).at(0) + number(456).at(5)).at(4) + number(789).at(11)).at(10);

        expected.assert_matches(result);
    }

    #[test]
    fn add_with_missing_first_summand() {
        let result = parse(" + 456".to_string());

        let message = result.expect_err("12.34.56 is not a number");
        assert_eq!(
            "Syntax Error: expected '-' or '(' or a number, got '+ 456'".to_string(),
            message.message
        );
        assert_eq!(Localization::at(0, 1), message.localization);
    }

    #[test]
    fn add_with_missing_second_summand() {
        let result = parse("123 + ".to_string());

        let message = result.unwrap_err();
        assert_eq!(
            "Syntax Error: expected '-' or '(' or a number, got ''".to_string(),
            message.message
        );
        assert_eq!(Localization::at(0, 6), message.localization);
    }
}

mod mul {
    use crate::ast::parser::parse;
    use crate::ast::{Localization, LocalizedSyntaxNode};

    #[test]
    fn mul_two_integers() {
        let result = parse("123 * 456".to_string());

        let expected = vec![LocalizedSyntaxNode::mul(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn mul_three_integers() {
        let result = parse("123 * 456 * 789".to_string());

        let expected = vec![LocalizedSyntaxNode::mul(
            Localization::at(0, 10),
            LocalizedSyntaxNode::mul(
                Localization::at(0, 4),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn mul_with_missing_first_argument() {
        let result = parse(" * 456".to_string());

        let message = result.expect_err("12.34.56 is not a number");
        assert_eq!(
            "Syntax Error: expected '-' or '(' or a number, got '* 456'".to_string(),
            message.message
        );
        assert_eq!(Localization::at(0, 1), message.localization);
    }

    #[test]
    fn mul_with_missing_second_factor() {
        let result = parse("123 * ".to_string());

        let message = result.unwrap_err();
        assert_eq!(
            "Syntax Error: expected '-' or '(' or a number, got ''".to_string(),
            message.message
        );
        assert_eq!(Localization::at(0, 6), message.localization);
    }
}

mod div {
    use crate::ast::parser::parse;
    use crate::ast::{Localization, LocalizedSyntaxNode};

    #[test]
    fn div_two_integers() {
        let result = parse("123 / 456".to_string());

        let expected = vec![LocalizedSyntaxNode::div(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn div_three_integers() {
        let result = parse("123 / 456 / 789".to_string());

        let expected = vec![LocalizedSyntaxNode::div(
            Localization::at(0, 10),
            LocalizedSyntaxNode::div(
                Localization::at(0, 4),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn div_with_missing_first_argument() {
        let result = parse(" / 456".to_string());

        let message = result.expect_err("12.34.56 is not a number");
        assert_eq!(
            "Syntax Error: expected '-' or '(' or a number, got '/ 456'".to_string(),
            message.message
        );
        assert_eq!(Localization::at(0, 1), message.localization);
    }

    #[test]
    fn div_with_missing_divisor() {
        let result = parse("123 / ".to_string());

        let message = result.unwrap_err();
        assert_eq!(
            "Syntax Error: expected '-' or '(' or a number, got ''".to_string(),
            message.message
        );
        assert_eq!(Localization::at(0, 6), message.localization);
    }
}

mod exp {
    use crate::ast::parser::parse;
    use crate::ast::{Localization, LocalizedSyntaxNode};

    #[test]
    fn exp_two_integers() {
        let result = parse("123 ^ 456".to_string());

        let expected = vec![LocalizedSyntaxNode::exp(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn exp_three_integers() {
        let result = parse("123 ^ 456 ^ 789".to_string());

        let expected = vec![LocalizedSyntaxNode::exp(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::exp(
                Localization::at(0, 10),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn exp_with_missing_first_argument() {
        let result = parse(" ^ 456".to_string());

        let message = result.expect_err("12.34.56 is not a number");
        assert_eq!(
            "Syntax Error: expected '-' or '(' or a number, got '^ 456'".to_string(),
            message.message
        );
        assert_eq!(Localization::at(0, 1), message.localization);
    }
}

mod subtract {
    use crate::ast::parser::parse;
    use crate::ast::{Localization, LocalizedSyntaxNode};

    #[test]
    fn subtract_two_integers() {
        let result = parse("123 - 456".to_string());

        let expected = vec![LocalizedSyntaxNode::sub(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn subtract_twice() {
        let result = parse("123 - 456 - 789".to_string());

        let expected = vec![LocalizedSyntaxNode::sub(
            Localization::at(0, 10),
            LocalizedSyntaxNode::sub(
                Localization::at(0, 4),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn subtract_with_missing_second_summand() {
        let result = parse("123 - ".to_string());

        let message = result.expect_err("12.34.56 is not a number");
        assert_eq!(
            "Syntax Error: expected '-' or '(' or a number, got ''".to_string(),
            message.message
        );
        assert_eq!(Localization::at(0, 6), message.localization);
    }

    #[test]
    fn subtract_with_missing_subtrahend() {
        let result = parse("123 - ".to_string());

        let message = result.unwrap_err();
        assert_eq!(
            "Syntax Error: expected '-' or '(' or a number, got ''".to_string(),
            message.message
        );
        assert_eq!(Localization::at(0, 6), message.localization);
    }
}

mod combined_operations {
    use crate::ast::parser::parse;
    use crate::ast::parser::tests::number;
    use crate::ast::{Localization, LocalizedSyntaxNode};

    #[test]
    fn add_and_subtract() {
        let result = parse("123 + 456 - 789".to_string());

        let expected = ((number(123).at(0) + number(456).at(5)).at(4) - number(789).at(11)).at(10);

        expected.assert_matches(result);
    }

    #[test]
    fn add_and_subtract_many_times() {
        let result = parse("1 + 2 - 3 + 4 - 5 + 6 - 7".to_string());

        let first_sum = (number(1) + number(2)).at(2);
        let first_difference = (first_sum - number(3)).at(6);
        let expected = (first_difference + number(4) - number(5) + number(6) - number(7)).at(22);

        expected.assert_matches(result)
    }

    #[test]
    fn add_and_multiply() {
        let result = parse("123 + 456 * 789".to_string());

        let expected = vec![LocalizedSyntaxNode::add(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::mul(
                Localization::at(0, 10),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn add_and_divide() {
        let result = parse("123 + 456 / 789".to_string());

        let expected = vec![LocalizedSyntaxNode::add(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::div(
                Localization::at(0, 10),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn add_and_exp() {
        let result = parse("123 + 456 ^ 789".to_string());

        let expected = vec![LocalizedSyntaxNode::add(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::exp(
                Localization::at(0, 10),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn sub_and_add() {
        let result = parse("123 - 456 + 789".to_string());

        let expected = vec![LocalizedSyntaxNode::add(
            Localization::at(0, 10),
            LocalizedSyntaxNode::sub(
                Localization::at(0, 4),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn sub_and_multiply() {
        let result = parse("123 - 456 * 789".to_string());

        let expected = vec![LocalizedSyntaxNode::sub(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::mul(
                Localization::at(0, 10),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn sub_and_divide() {
        let result = parse("123 - 456 / 789".to_string());

        let expected = vec![LocalizedSyntaxNode::sub(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::div(
                Localization::at(0, 10),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn sub_and_exp() {
        let result = parse("123 - 456 ^ 789".to_string());

        let expected = vec![LocalizedSyntaxNode::sub(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::exp(
                Localization::at(0, 10),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn multiply_and_add() {
        let result = parse("123 * 456 + 789".to_string());

        let expected = vec![LocalizedSyntaxNode::add(
            Localization::at(0, 10),
            LocalizedSyntaxNode::mul(
                Localization::at(0, 4),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn multiply_and_sub() {
        let result = parse("123 * 456 - 789".to_string());

        let expected = vec![LocalizedSyntaxNode::sub(
            Localization::at(0, 10),
            LocalizedSyntaxNode::mul(
                Localization::at(0, 4),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn multiply_and_div() {
        let result = parse("123 * 456 / 789".to_string());

        let expected = vec![LocalizedSyntaxNode::div(
            Localization::at(0, 10),
            LocalizedSyntaxNode::mul(
                Localization::at(0, 4),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn multiply_and_exp() {
        let result = parse("123 * 456 ^ 789".to_string());

        let expected = vec![LocalizedSyntaxNode::mul(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::exp(
                Localization::at(0, 10),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn div_and_add() {
        let result = parse("123 / 456 + 789".to_string());

        let expected = vec![LocalizedSyntaxNode::add(
            Localization::at(0, 10),
            LocalizedSyntaxNode::div(
                Localization::at(0, 4),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn div_and_sub() {
        let result = parse("123 / 456 - 789".to_string());

        let expected = vec![LocalizedSyntaxNode::sub(
            Localization::at(0, 10),
            LocalizedSyntaxNode::div(
                Localization::at(0, 4),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn div_and_mul() {
        let result = parse("123 / 456 * 789".to_string());

        let expected = vec![LocalizedSyntaxNode::mul(
            Localization::at(0, 10),
            LocalizedSyntaxNode::div(
                Localization::at(0, 4),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn div_and_exp() {
        let result = parse("123 / 456 ^ 789".to_string());

        let expected = vec![LocalizedSyntaxNode::div(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::exp(
                Localization::at(0, 10),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn exp_and_add() {
        let result = parse("123 ^ 456 + 789".to_string());

        let expected = vec![LocalizedSyntaxNode::add(
            Localization::at(0, 10),
            LocalizedSyntaxNode::exp(
                Localization::at(0, 4),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn exp_and_sub() {
        let result = parse("123 ^ 456 - 789".to_string());

        let expected = vec![LocalizedSyntaxNode::sub(
            Localization::at(0, 10),
            LocalizedSyntaxNode::exp(
                Localization::at(0, 4),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn exp_and_div() {
        let result = parse("123 ^ 456 / 789".to_string());

        let expected = vec![LocalizedSyntaxNode::div(
            Localization::at(0, 10),
            LocalizedSyntaxNode::exp(
                Localization::at(0, 4),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn exp_and_mul() {
        let result = parse("123 ^ 456 * 789".to_string());

        let expected = vec![LocalizedSyntaxNode::mul(
            Localization::at(0, 10),
            LocalizedSyntaxNode::exp(
                Localization::at(0, 4),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn paren_add_and_subtract() {
        let result = parse("(123 + 456) - 789".to_string());

        let expected = vec![LocalizedSyntaxNode::sub(
            Localization::at(0, 12),
            LocalizedSyntaxNode::add(
                Localization::at(0, 5),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 13), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn paren_add_and_multiply() {
        let result = parse("(123 + 456) * 789".to_string());

        let expected = vec![LocalizedSyntaxNode::mul(
            Localization::at(0, 12),
            LocalizedSyntaxNode::add(
                Localization::at(0, 5),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 13), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn paren_add_and_divide() {
        let result = parse("(123 + 456) / 789".to_string());

        let expected = vec![LocalizedSyntaxNode::div(
            Localization::at(0, 12),
            LocalizedSyntaxNode::add(
                Localization::at(0, 5),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 13), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn paren_add_and_exp() {
        let result = parse("(123 + 456) ^ 789".to_string());

        let expected = vec![LocalizedSyntaxNode::exp(
            Localization::at(0, 12),
            LocalizedSyntaxNode::add(
                Localization::at(0, 5),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 13), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn paren_sub_and_add() {
        let result = parse("(123 - 456) + 789".to_string());

        let expected = vec![LocalizedSyntaxNode::add(
            Localization::at(0, 12),
            LocalizedSyntaxNode::sub(
                Localization::at(0, 5),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 13), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn paren_sub_and_multiply() {
        let result = parse("(123 - 456) * 789".to_string());

        let expected = vec![LocalizedSyntaxNode::mul(
            Localization::at(0, 12),
            LocalizedSyntaxNode::sub(
                Localization::at(0, 5),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 13), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn paren_sub_and_divide() {
        let result = parse("(123 - 456) / 789".to_string());

        let expected = vec![LocalizedSyntaxNode::div(
            Localization::at(0, 12),
            LocalizedSyntaxNode::sub(
                Localization::at(0, 5),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 13), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn paren_sub_and_exp() {
        let result = parse("(123 - 456) ^ 789".to_string());

        let expected = vec![LocalizedSyntaxNode::exp(
            Localization::at(0, 12),
            LocalizedSyntaxNode::sub(
                Localization::at(0, 5),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 13), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn paren_multiply_and_add() {
        let result = parse("(123 * 456) + 789".to_string());

        let expected = vec![LocalizedSyntaxNode::add(
            Localization::at(0, 12),
            LocalizedSyntaxNode::mul(
                Localization::at(0, 5),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 13), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn paren_multiply_and_sub() {
        let result = parse("(123 * 456) - 789".to_string());

        let expected = vec![LocalizedSyntaxNode::sub(
            Localization::at(0, 12),
            LocalizedSyntaxNode::mul(
                Localization::at(0, 5),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 13), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn paren_multiply_and_div() {
        let result = parse("(123 * 456) / 789".to_string());

        let expected = vec![LocalizedSyntaxNode::div(
            Localization::at(0, 12),
            LocalizedSyntaxNode::mul(
                Localization::at(0, 5),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 13), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn paren_multiply_and_exp() {
        let result = parse("(123 * 456) ^ 789".to_string());

        let expected = vec![LocalizedSyntaxNode::exp(
            Localization::at(0, 12),
            LocalizedSyntaxNode::mul(
                Localization::at(0, 5),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 13), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn paren_div_and_add() {
        let result = parse("(123 / 456) + 789".to_string());

        let expected = vec![LocalizedSyntaxNode::add(
            Localization::at(0, 12),
            LocalizedSyntaxNode::div(
                Localization::at(0, 5),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 13), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn paren_div_and_sub() {
        let result = parse("(123 / 456) - 789".to_string());

        let expected = vec![LocalizedSyntaxNode::sub(
            Localization::at(0, 12),
            LocalizedSyntaxNode::div(
                Localization::at(0, 5),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 13), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn paren_div_and_mul() {
        let result = parse("(123 / 456) * 789".to_string());

        let expected = vec![LocalizedSyntaxNode::mul(
            Localization::at(0, 12),
            LocalizedSyntaxNode::div(
                Localization::at(0, 5),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 13), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn paren_div_and_exp() {
        let result = parse("(123 / 456) ^ 789".to_string());

        let expected = vec![LocalizedSyntaxNode::exp(
            Localization::at(0, 12),
            LocalizedSyntaxNode::div(
                Localization::at(0, 5),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 13), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn paren_exp_and_add() {
        let result = parse("(123 ^ 456) + 789".to_string());

        let expected = vec![LocalizedSyntaxNode::add(
            Localization::at(0, 12),
            LocalizedSyntaxNode::exp(
                Localization::at(0, 5),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 13), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn paren_exp_and_sub() {
        let result = parse("(123 ^ 456) - 789".to_string());

        let expected = vec![LocalizedSyntaxNode::sub(
            Localization::at(0, 12),
            LocalizedSyntaxNode::exp(
                Localization::at(0, 5),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 13), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn paren_exp_and_div() {
        let result = parse("(123 ^ 456) / 789".to_string());

        let expected = vec![LocalizedSyntaxNode::div(
            Localization::at(0, 12),
            LocalizedSyntaxNode::exp(
                Localization::at(0, 5),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 13), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn paren_exp_and_mul() {
        let result = parse("(123 ^ 456) * 789".to_string());

        let expected = vec![LocalizedSyntaxNode::mul(
            Localization::at(0, 12),
            LocalizedSyntaxNode::exp(
                Localization::at(0, 5),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 13), 789u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn add_and_subtract_paren() {
        let result = parse("123 + (456 - 789)".to_string());

        let expected = vec![LocalizedSyntaxNode::add(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::sub(
                Localization::at(0, 11),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 12), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn add_and_multiply_paren() {
        let result = parse("123 + (456 * 789)".to_string());

        let expected = vec![LocalizedSyntaxNode::add(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::mul(
                Localization::at(0, 11),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 12), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn add_and_divide_paren() {
        let result = parse("123 + (456 / 789)".to_string());

        let expected = vec![LocalizedSyntaxNode::add(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::div(
                Localization::at(0, 11),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 12), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn add_and_exp_paren() {
        let result = parse("123 + (456 ^ 789)".to_string());

        let expected = vec![LocalizedSyntaxNode::add(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::exp(
                Localization::at(0, 11),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 12), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn sub_and_add_paren() {
        let result = parse("123 - (456 + 789)".to_string());

        let expected = vec![LocalizedSyntaxNode::sub(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::add(
                Localization::at(0, 11),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 12), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn sub_and_multiply_paren() {
        let result = parse("123 - (456 * 789)".to_string());

        let expected = vec![LocalizedSyntaxNode::sub(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::mul(
                Localization::at(0, 11),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 12), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn sub_and_divide_paren() {
        let result = parse("123 - (456 / 789)".to_string());

        let expected = vec![LocalizedSyntaxNode::sub(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::div(
                Localization::at(0, 11),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 12), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn sub_and_exp_paren() {
        let result = parse("123 - (456 ^ 789)".to_string());

        let expected = vec![LocalizedSyntaxNode::sub(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::exp(
                Localization::at(0, 11),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 12), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn multiply_and_add_paren() {
        let result = parse("123 * (456 + 789)".to_string());

        let expected = vec![LocalizedSyntaxNode::mul(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::add(
                Localization::at(0, 11),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 12), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn multiply_and_sub_paren() {
        let result = parse("123 * (456 - 789)".to_string());

        let expected = vec![LocalizedSyntaxNode::mul(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::sub(
                Localization::at(0, 11),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 12), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn multiply_and_div_paren() {
        let result = parse("123 * (456 / 789)".to_string());

        let expected = vec![LocalizedSyntaxNode::mul(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::div(
                Localization::at(0, 11),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 12), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn multiply_and_exp_paren() {
        let result = parse("123 * (456 ^ 789)".to_string());

        let expected = vec![LocalizedSyntaxNode::mul(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::exp(
                Localization::at(0, 11),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 12), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn div_and_add_paren() {
        let result = parse("123 / (456 + 789)".to_string());

        let expected = vec![LocalizedSyntaxNode::div(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::add(
                Localization::at(0, 11),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 12), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn div_and_sub_paren() {
        let result = parse("123 / (456 - 789)".to_string());

        let expected = vec![LocalizedSyntaxNode::div(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::sub(
                Localization::at(0, 11),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 12), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn div_and_mul_paren() {
        let result = parse("123 / (456 * 789)".to_string());

        let expected = vec![LocalizedSyntaxNode::div(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::mul(
                Localization::at(0, 11),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 12), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn div_and_exp_paren() {
        let result = parse("123 / (456 ^ 789)".to_string());

        let expected = vec![LocalizedSyntaxNode::div(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::exp(
                Localization::at(0, 11),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 12), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn exp_and_add_paren() {
        let result = parse("123 ^ (456 + 789)".to_string());

        let expected = vec![LocalizedSyntaxNode::exp(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::add(
                Localization::at(0, 11),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 12), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn exp_and_sub_paren() {
        let result = parse("123 ^ (456 - 789)".to_string());

        let expected = vec![LocalizedSyntaxNode::exp(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::sub(
                Localization::at(0, 11),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 12), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn exp_and_div_paren() {
        let result = parse("123 ^ (456 / 789)".to_string());

        let expected = vec![LocalizedSyntaxNode::exp(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::div(
                Localization::at(0, 11),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 12), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn exp_and_mul_paren() {
        let result = parse("123 ^ (456 * 789)".to_string());

        let expected = vec![LocalizedSyntaxNode::exp(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::mul(
                Localization::at(0, 11),
                LocalizedSyntaxNode::number(Localization::at(0, 6), 456u16),
                LocalizedSyntaxNode::number(Localization::at(0, 12), 789u16),
            ),
        )];

        assert_eq!(Ok(expected), result);
    }
}
