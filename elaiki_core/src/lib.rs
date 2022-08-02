use gdnative::prelude::*;

use game::controller;
use game::init;

pub mod elaiki;
pub mod worlds;
pub mod ticktock;
pub mod entities;
pub mod game;

fn init(handle: InitHandle) {
    handle.add_class::<init::Init>();
    handle.add_class::<controller::KeyboardController>();
}

godot_init!(init);
