use elaiki_api::{entities::EntityMovable, utils::errors::*};
use elaiki_derive::{entity_attribute_macro, Entity};
use gdnative::prelude::{KinematicBody2D, Node2D, Ref, Vector2};

#[derive(Entity)]
#[entity_attribute_macro]
pub struct Player {
    pub game_obj: Ref<KinematicBody2D>,
}

impl EntityMovable for Player {
    fn move_entity(&mut self, x: f32, y: f32) {
        unsafe { self.game_obj.assume_safe() }.move_and_collide(
            Vector2::new(x, y),
            true,
            true,
            false,
        );
    }
}

impl Player {
    pub fn new(root_node: &Node2D) -> Result<Player, Error> {
        let _game_obj = root_node.get_node("KinematicBody2D");

        let game_obj = unsafe { _game_obj.unwrap().assume_safe() }
            .cast::<KinematicBody2D>()
            .unwrap()
            .claim();

        let _new = Player {
            game_obj: game_obj,
            id: 0,
        };

        Ok(_new)
    }
}
