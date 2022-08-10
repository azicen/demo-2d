use std::collections::HashMap;
use std::rc::Rc;

use crate::log;

use super::{Event, Handler, Listener};

pub struct Dispatcher {
    listener_map: HashMap<String, Listener>,

    // 日志
    log: log::Helper,
}

impl Dispatcher {
    pub fn new(logger: Rc<dyn log::Logger>) -> Self {
        Dispatcher {
            listener_map: HashMap::new(),

            log: log::Helper::new(logger),
        }
    }
}

impl Dispatcher {
    pub fn subscribe(&mut self, topic: &str, handler_key: &str, handler: Box<dyn Handler>) {
        if let None = self.listener_map.get(topic) {
            self.listener_map
                .insert(topic.to_string(), Listener::new(topic));
        }
        let listener = self.listener_map.get_mut(topic).unwrap();
        listener.subscribe(handler_key, handler);
    }

    pub fn unsubscribe(&mut self, topic: &str, handler_key: &str) {
        match self.listener_map.get_mut(topic) {
            Some(listener) => listener.unsubscribe(handler_key),
            None => self.log.warn("unsubscribe from topics that do not exist."),
        }
    }

    pub fn publish<'a>(&self, event: &'a mut dyn Event) {
        let topic = event.event_name();
        match self.listener_map.get(topic) {
            Some(listener) => listener.publish(event),
            None => self.log.warn("publish non-existent event topics."),
        }
    }
}
