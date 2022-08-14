use std::any::Any;

pub trait Event {
    fn event_name(&self) -> &'static str;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}
