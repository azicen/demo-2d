use std::rc::Rc;

use gdnative::prelude::Vector2;

use elaiki_api::entities::EntityMovable;
use elaiki_derive::{Entity, entity_attribute_macro};

use crate::resource_manager::PlayerResource;

#[derive(Entity)]
#[entity_attribute_macro]
pub struct Player {
    resource: Rc<PlayerResource>,
    // pub game_obj: Ref<KinematicBody2D>,
}

impl EntityMovable for Player {
    fn movement(&mut self, x: f32, y: f32) {
        unsafe { self.resource.kinematic_body() }.move_and_collide(
            Vector2::new(x, y),
            true,
            true,
            false,
        );
    }
}

impl Player {
    pub fn new(player_resource: Rc<PlayerResource>) -> Player {
        Player {
            resource: player_resource,
            id: 0,
        }
    }
}
