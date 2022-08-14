// 实体特征
pub trait Entity {
    fn id(&self) -> u32;
    fn set_id(&mut self, id: u32);
}
