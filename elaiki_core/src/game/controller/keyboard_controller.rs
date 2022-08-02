use gdnative::api::{GlobalConstants, Node2D};
use gdnative::prelude::*;

use crate::elaiki::Elaiki;
use crate::ticktock::TicktockMovable;
use elaiki_api::ticktock::Ticktock;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct KeyboardController {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    speed: f32,
}

#[methods]
impl KeyboardController {
    fn new(_owner: &Node2D) -> Self {
        KeyboardController {
            up: false,
            down: false,
            left: false,
            right: false,
            speed: 100.0,
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node2D) {
        godot_print!("Ok... keyboard_&controller!");
    }

    #[export]
    fn _process(&mut self, _owner: &Node2D, _delta: f32) {
        // godot_print!("_process... _delta={}", _delta);
    }

    #[export]
    fn _physics_process(&mut self, _owner: &Node2D, _delta: f64) {
        // godot_print!("_physics_process... _delta={}", _delta); // 0.01666666753590107

        let owner = _owner.cast::<KinematicBody2D>();

        match owner {
            Some(_v) => {
                let mut v2 = Vector2::new(0.0, 0.0);

                if self.left {
                    v2.x -= 1.0;
                }

                if self.right {
                    v2.x += 1.0;
                }

                if self.up {
                    v2.y -= 1.0;
                }

                if self.down {
                    v2.y += 1.0;
                }

                v2.x *= self.speed * _delta as f32;
                v2.y *= self.speed * _delta as f32;

                // owner.move_and_collide(v2, true, true, false);

                let player = Elaiki::get_instance().lock().unwrap().player();

                let mut e = Elaiki::get_instance().lock().unwrap();
                e.ticktock_scheduler.ticktock(Ticktock::Fixed(Box::new(TicktockMovable::new(player, v2.x, v2.y))));
            }
            None => {}
        }
    }

    #[export]
    fn _unhandled_input(&mut self, _owner: &Node2D, _event: Ref<InputEvent>) {}

    #[export]
    fn _unhandled_key_input(&mut self, _owner: &Node2D, _event: Ref<InputEventKey>) {
        // godot_print!("Inpot... InputEventKey!");
        let v = unsafe { _event.assume_safe() };
        let key_code = v.scancode();
        let value = v.is_pressed();

        if key_code == GlobalConstants::KEY_A {
            self.left = value;
        }

        if key_code == GlobalConstants::KEY_D {
            self.right = value;
        }

        if key_code == GlobalConstants::KEY_W {
            self.up = value;
        }

        if key_code == GlobalConstants::KEY_S {
            self.down = value;
        }
    }
}
