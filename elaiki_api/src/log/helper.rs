use std::rc::Rc;

use super::{Level, Logger};

pub struct Helper {
    logger: Rc<dyn Logger>,
}

impl Helper {
    pub fn new(logger: Rc<dyn Logger>) -> Self {
        Helper { logger }
    }

    fn log(&self, level: Level, content: &str) {
        self.logger.log(level, content);
    }

    pub fn debug(&self, content: &str) {
        self.log(Level::Debug, content);
    }

    pub fn info(&self, content: &str) {
        self.log(Level::Info, content);
    }

    pub fn warn(&self, content: &str) {
        self.log(Level::Warn, content);
    }

    pub fn error(&self, content: &str) {
        self.log(Level::Error, content);
    }

    pub fn fatal(&self, content: &str) {
        self.log(Level::Fatal, content);
    }
}
