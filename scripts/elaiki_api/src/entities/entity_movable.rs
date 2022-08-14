use super::Entity;

// 可以移动的实体
pub trait EntityMovable: Entity {
    fn movement(&mut self, x: f32, y: f32);
}
