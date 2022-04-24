use serde::Serialize;

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

    pub fn run(&mut self, _input: String) -> CommandResult {
        CommandResult::Success(String::from("42"))
    }
}

pub struct Context;

#[cfg(test)]
mod tests {
    use crate::CommandResult;

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
