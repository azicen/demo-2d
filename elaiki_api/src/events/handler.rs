use super::Event;

pub trait Handler {
    fn handle<'a>(&self, event: &'a mut dyn Event);
}
