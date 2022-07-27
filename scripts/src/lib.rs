mod game;

use gdnative::prelude::*;

fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
}

godot_init!(init);
