use super::{Event, Handler};

pub struct LinkUnit<'a> {
    next: Option<&'a LinkUnit<'a>>,
    handler: &'a dyn Handler<'a>,
}

impl<'a> LinkUnit<'a> {
    pub fn new(handler: &'a dyn Handler<'a>) -> Self {
        LinkUnit {
            next: None,
            handler,
        }
    }
}

impl<'a> LinkUnit<'a> {
    pub fn next(&self) -> Option<&'a LinkUnit<'a>> {
        match self.next {
            Some(node) => Some(node),
            None => None,
        }
    }

    pub fn set_next(&mut self, next: &'a LinkUnit<'a>) {
        self.next = Some(next);
    }

    pub fn handle(&self, event: &'a mut dyn Event) {
        self.handler.handle(event);
    }
}
