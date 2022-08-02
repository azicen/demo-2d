use elaiki_api::entities::Entity;
use elaiki_derive::{Entity, entity_attribute_macro};

#[derive(Entity)]
#[entity_attribute_macro]
pub struct TestEntity {}

fn main() {
    let mut e = TestEntity { id: 0 };
    assert_eq!(0, e.id());
    e.set_id(10);
    assert_eq!(10, e.id());
}
