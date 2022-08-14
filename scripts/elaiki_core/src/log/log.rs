use gdnative::prelude::godot_print;

use elaiki_api::log::{Level, Logger};

pub struct Log {}

impl Log {
    pub fn new() -> Self {
        Log {}
    }
}

impl Logger for Log {
    fn log(&self, level: Level, content: &str) {
        godot_print!("[{:5}] {}", level, content);
    }
}
