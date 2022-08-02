use super::EntityDamageable;

// 活着的生物
pub trait EntityLiving: EntityDamageable {
    fn kill(&mut self);
}
