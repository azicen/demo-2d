use gdnative::prelude::*;

use game::*;

pub mod utils;
mod game;

#[macro_use]
pub mod macros;

fn init(handle: InitHandle) {
    handle.add_class::<init::Init>();
}

godot_init!(init);
