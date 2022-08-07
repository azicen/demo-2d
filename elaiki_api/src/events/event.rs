pub trait Event {
    fn name() -> &'static str;
    fn event_name(&self) -> &'static str;
}
