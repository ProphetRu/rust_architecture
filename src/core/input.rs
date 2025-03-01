pub struct Input;

impl Input {
    pub fn new() -> Self {
        Input
    }

    pub fn read(&self) -> String {
        "example data".to_string()
    }
}