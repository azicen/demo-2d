use super::Entity;

// 能够受到损伤的实体
pub trait EntityDamageable: Entity {
    fn under_attack(&mut self);
}
