pub struct Output;

impl Output {
    pub fn new() -> Self {
        Output
    }

    pub fn write(&self, data: String) {
        println!("Output: {}", data);
    }
}