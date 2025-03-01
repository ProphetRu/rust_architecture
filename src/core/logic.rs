pub struct Logic;

impl Logic {
    pub fn new() -> Self {
        Logic
    }

    pub fn process(&self, data: String) -> String {
        data.to_uppercase()
    }
}