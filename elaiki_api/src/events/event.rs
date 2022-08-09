pub trait Event {
    fn event_name(&self) -> &'static str;
}
