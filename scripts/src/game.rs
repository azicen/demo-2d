use gdnative::api::Node2D;
use gdnative::prelude::*;

pub mod world;

#[inherit(Node2D)]
#[derive(NativeClass)]
pub struct Game {}

#[methods]
impl Game {
    fn new(_owner: &Node2D) -> Self {
        Game {}
    }

    #[export]
    fn _ready(&mut self, _owner: &Node2D) {
        godot_print!("Hello world!");
    }

    #[export]
    fn _process(&mut self, _owner: &Node2D, _delta: f32) {}
}
