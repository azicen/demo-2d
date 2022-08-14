use super::Level;

pub trait Logger {
    fn log(&self, level: Level, content: &str);
}
