use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::Chars;

use crate::ast::Localization;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ErrorMessage {
    pub message: String,
    pub localization: Localization,
    priority: Priority,
}

type Priority = u8;

impl ErrorMessage {
    pub fn new(message: String, priority: Priority, localization: Localization) -> Self {
        ErrorMessage {
            message,
            priority,
            localization,
        }
    }

    pub fn forgot_comma(message: String) -> Self {
        ErrorMessage::new(message, 121, todo!())
    }

    pub fn sign_failed(message: String, localization: Localization) -> Self {
        ErrorMessage::new(message, 70, localization)
    }

    pub fn missing_opening_parenthesis(message: String, localization: Localization) -> Self {
        ErrorMessage::new(message, 40, localization)
    }

    pub fn missing_closing_parenthesis(message: String, localization: Localization) -> Self {
        ErrorMessage::new(message, 120, localization)
    }

    pub fn empty_expression(message: String, localization: Localization) -> Self {
        ErrorMessage::new(message, 99, localization)
    }

    pub fn atom_failed(message: String, localization: Localization) -> Self {
        ErrorMessage::new(message, 10, localization)
    }

    pub fn term_failed(message: String, localization: Localization) -> Self {
        ErrorMessage::new(message, 15, localization)
    }

    pub fn expression_failed(message: String) -> Self {
        ErrorMessage::new(message, 20, todo!())
    }

    pub fn exponentiation_failed(message: String, localization: Localization) -> Self {
        ErrorMessage::new(message, 25, localization)
    }

    pub fn expected_end_of_input(message: String) -> Self {
        ErrorMessage::new(message, 110, todo!())
    }

    pub fn assignment_error(message: String) -> Self {
        ErrorMessage::new(message, 120, todo!())
    }

    pub fn no_assignment_found(message: String) -> Self {
        ErrorMessage::new(message, 0, todo!())
    }

    pub fn missing_tuple_comma() -> Self {
        ErrorMessage::new("expected ',' for tuple".to_string(), 0, Localization::new())
    }

    pub fn empty_function_arguments(input: Chars) -> Self {
        ErrorMessage::new(
            format!("expected function arguments, got {}", input.as_str()),
            80,
            todo!(),
        )
    }

    pub fn leftover(leftover: String, localization: Localization) -> Self {
        ErrorMessage {
            message: format!("expected end of input, '{leftover}' was left"),
            localization,
            priority: 0,
        }
    }

    pub fn map_message(mut self, mapper: fn(String) -> String) -> Self {
        self.message = mapper(self.message);
        self
    }
}

#[macro_export]
macro_rules! most_important_of {
    ($($error_message:expr),+ $(,)?) => {
        {
            let mut messages = vec![$($error_message),+];
            messages.reverse();
            messages.sort();
            messages.last().unwrap().clone()
        }
    };
}

impl PartialOrd for ErrorMessage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ErrorMessage {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl Display for ErrorMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<ErrorMessage> for String {
    fn from(error: ErrorMessage) -> Self {
        error.message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn most_important_of_one() {
        let error_message = ErrorMessage::missing_tuple_comma();

        let result = most_important_of!(error_message.clone());

        assert_eq!(error_message, result)
    }

    #[test]
    fn first_is_more_important() {
        let more_important = ErrorMessage::new("message1".to_string(), 100, Localization::new());
        let less_important = ErrorMessage::new("message2".to_string(), 1, Localization::new());

        let result = most_important_of!(more_important.clone(), less_important);

        assert_eq!(more_important, result)
    }

    #[test]
    fn second_is_more_important() {
        let more_important = ErrorMessage::new("message1".to_string(), 100, Localization::new());
        let less_important = ErrorMessage::new("message2".to_string(), 1, Localization::new());

        let result = most_important_of!(less_important, more_important.clone());

        assert_eq!(more_important, result)
    }

    #[test]
    fn returns_first_if_equally_important() {
        let first = ErrorMessage::new("message1".to_string(), 5, Localization::new());
        let second = ErrorMessage::new("message2".to_string(), 5, Localization::new());

        let result = most_important_of!(first.clone(), second);

        assert_eq!(first, result)
    }

    #[test]
    fn most_important_of_three() {
        let most_important = ErrorMessage::new("message1".to_string(), 100, Localization::new());
        let less_important = ErrorMessage::new("message2".to_string(), 5, Localization::new());
        let least_important = ErrorMessage::new("message3".to_string(), 1, Localization::new());

        let result = most_important_of!(less_important, most_important.clone(), least_important);

        assert_eq!(most_important, result)
    }
}
