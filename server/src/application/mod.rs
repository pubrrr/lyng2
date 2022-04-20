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
