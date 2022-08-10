use super::{Event, Handler};

pub struct Listener {
    topic: String,
    handler_list: Vec<(String, Box<dyn Handler>)>,
}

impl Listener {
    pub fn new(topic: &str) -> Self {
        Listener {
            topic: topic.to_string(),
            handler_list: Vec::new(),
        }
    }
}

impl Listener {
    pub fn topic(&self) -> &str {
        &self.topic.as_str()
    }

    // 订阅事件
    pub fn subscribe(&mut self, handler_key: &str, handler: Box<dyn Handler>) {
        self.handler_list.push((handler_key.to_string(), handler));
    }

    // 退订事件
    pub fn unsubscribe(&mut self, handler_key: &str) {
        for i in 0..self.handler_list.len() {
            if self.handler_list[i].0 == handler_key {
                self.handler_list.remove(i);
                break;
            }
        }
    }

    // 发布事件
    pub fn publish<'a>(&self, event: &'a mut dyn Event) {
        for (_, handler) in self.handler_list.iter() {
            handler.handle(event);
        }
    }
}
