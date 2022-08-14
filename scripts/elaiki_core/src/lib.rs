use gdnative::prelude::*;

pub mod elaiki;
pub mod worlds;
pub mod ticktock;
pub mod entities;
pub mod log;
pub mod resource_manager;
pub mod game;

fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
    handle.add_class::<game::controller::KeyboardController>();
}

godot_init!(init);
