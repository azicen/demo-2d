use gdnative::api::Node2D;
use gdnative::prelude::*;

use crate::core::world::World;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Init {
    world: Option<World>,
}

#[methods]
impl Init {
    fn new(_owner: &Node2D) -> Self {
        Init { world: None }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node2D) {
        godot_print!("Hello world!");

        let _world = World::new(_owner);
        match _world {
            Ok(v) => {
                self.world = Some(v);
            }
            Err(e) => {
                godot_dbg!(e);
            }
        }
    }

    #[export]
    fn _process(&mut self, _owner: &Node2D, _delta: f32) {}
}
