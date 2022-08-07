pub use elaiki_derive::{event_base, event_revocable_base};

pub use self::{event::Event, event_revocable::EventRevocable};

mod event;
mod event_revocable;
