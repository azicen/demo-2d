pub use elaiki_derive::{entity_base, event_revocable_base};

pub use self::{
    entity::Entity, entity_damageable::EntityDamageable, entity_live::EntityLiving,
    entity_movable::EntityMovable,
};

mod entity;
mod entity_damageable;
mod entity_live;
mod entity_movable;
