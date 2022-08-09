use std::collections::HashMap;

use super::{Event, LinkUnit};

pub struct Dispatcher<'a> {
    handler_map: HashMap<String, LinkUnit<'a>>,
}

impl<'b, 'a: 'b> Dispatcher<'a> {
    pub fn register(&mut self, event_name: &str, node: LinkUnit<'a>) {
        self.handler_map.insert(event_name.to_string(), node);
    }

    pub fn call(&mut self, event: &'b mut dyn Event) {
        let event_name = event.event_name();
        let mut node = self.handler_map.get(event_name);

        while let Some(n) = node {
            {
                let e = event;
                n.handle(e);
            }
            node = n.next();
        }
    }
}
