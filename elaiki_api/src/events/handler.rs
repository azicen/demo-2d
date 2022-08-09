use super::Event;

pub trait Handler<'a> {
    fn handle(&self, event: &'a mut dyn Event);
}
