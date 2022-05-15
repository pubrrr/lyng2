use serde::Serialize;

use crate::ast::parser::parse;

#[derive(Serialize)]
pub enum CommandResult {
    Success(String),
    Error(String),
}

pub struct Application;

impl Application {
    pub fn create() -> Application {
        Application
    }

    pub fn run(&mut self, input: String) -> CommandResult {
        match parse(input) {
            Ok(result) => CommandResult::Success(
                result
                    .iter()
                    .map(|result| format!("{result}"))
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
            Err(error) => CommandResult::Error(error.message),
        }
    }
}

pub struct Context;

#[cfg(test)]
mod tests {
    use crate::application::CommandResult;

    #[test]
    fn deserialize_success() {
        let result = CommandResult::Success("success message".to_string());

        let actual = ron::to_string(&result).unwrap();

        let expected = "Success(\"success message\")".to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_failure() {
        let result = CommandResult::Error("error message".to_string());

        let actual = ron::to_string(&result).unwrap();

        let expected = "Error(\"error message\")".to_string();
        assert_eq!(expected, actual);
    }
}
