pub mod input;
pub mod logic;
pub mod output;

use self::{input::Input, logic::Logic, output::Output};

pub struct Core {
    input: Input,
    logic: Logic,
    output: Output,
}

impl Core {
    pub fn new() -> Self {
        Core {
            input: Input::new(),
            logic: Logic::new(),
            output: Output::new(),
        }
    }

    pub fn run(&self) {
        println!("Core is running...");
        let data = self.input.read();
        let processed_data = self.logic.process(data);
        self.output.write(processed_data);
    }
}