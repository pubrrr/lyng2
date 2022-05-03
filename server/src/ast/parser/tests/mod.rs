mod atoms {
    use bigdecimal::{BigDecimal, FromPrimitive};

    use crate::ast::parser::parse;
    use crate::ast::{Localization, LocalizedSyntaxNode};

    #[test]
    fn integer() {
        let result = parse("123;".to_string());

        let expected = vec![LocalizedSyntaxNode::number(Localization::at(0, 0), 123u8)];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn float() {
        let result = parse("12.340  ;".to_string());

        let expected = vec![LocalizedSyntaxNode::number(
            Localization::at(0, 0),
            BigDecimal::from_f32(12.34).unwrap(),
        )];

        assert_eq!(Ok(expected), result);
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
}

mod add {
    use crate::ast::parser::parse;
    use crate::ast::{Localization, LocalizedSyntaxNode};

    #[test]
    fn add_two_integers() {
        let result = parse("123 + 456".to_string());

        let expected = vec![LocalizedSyntaxNode::add(
            Localization::at(0, 4),
            LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
            LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
        )];

        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn add_three_integers() {
        let result = parse("123 + 456 + 789".to_string());

        let expected = vec![LocalizedSyntaxNode::add(
            Localization::at(0, 10),
            LocalizedSyntaxNode::add(
                Localization::at(0, 4),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
        )];

        assert_eq!(Ok(expected), result);
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
}

mod combined_operations {
    use crate::ast::parser::parse;
    use crate::ast::{Localization, LocalizedSyntaxNode};

    #[test]
    fn add_and_subtract() {
        let result = parse("123 + 456 - 789".to_string());

        let expected = vec![LocalizedSyntaxNode::sub(
            Localization::at(0, 10),
            LocalizedSyntaxNode::add(
                Localization::at(0, 4),
                LocalizedSyntaxNode::number(Localization::at(0, 0), 123u16),
                LocalizedSyntaxNode::number(Localization::at(0, 5), 456u16),
            ),
            LocalizedSyntaxNode::number(Localization::at(0, 11), 789u16),
        )];

        assert_eq!(Ok(expected), result);
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
