mod core;

use crate::core::Core;

fn main() {
    let app_core = Core::new();
    app_core.run();
}