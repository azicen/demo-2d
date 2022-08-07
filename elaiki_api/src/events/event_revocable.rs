use super::Event;

pub trait EventRevocable: Event {
    fn is_revoke(&self) -> bool;
    fn revoke(&mut self);
    fn set_revoke(&mut self, is: bool);
}
