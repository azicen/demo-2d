use std::rc::Rc;

use gdnative::api::{GlobalConstants, Node2D};
use gdnative::prelude::*;

use elaiki_api::log::Helper;

use crate::elaiki::Elaiki;
use crate::ticktock::TicktockMovable;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct KeyboardController {
    ticktock_hub: Rc<crate::ticktock::Hub>,

    up: bool,
    down: bool,
    left: bool,
    right: bool,
    speed: f32,
}

impl KeyboardController {
    fn new(_owner: &Node2D) -> Self {
        let elaiki = Elaiki::get_instance().lock().unwrap();

        KeyboardController {
            ticktock_hub: elaiki.ticktock_hub(),

            up: false,
            down: false,
            left: false,
            right: false,
            speed: 100.0,
        }
    }
}

#[methods]
impl KeyboardController {
    #[export]
    fn _ready(&mut self, _owner: &Node2D) {
        let elaiki = Elaiki::get_instance().lock().unwrap();
        let log = Helper::new(elaiki.log());
        log.info("Ok... keyboard_controller!");
    }

    #[export]
    fn _physics_process(&mut self, _owner: &Node2D, _delta: f64) {
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

        self.ticktock_hub
            .add_fixed(Box::new(TicktockMovable::new(player, v2.x, v2.y)));
    }

    #[export]
    fn _unhandled_input(&mut self, _owner: &Node2D, _event: Ref<InputEvent>) {}

    #[export]
    fn _unhandled_key_input(&mut self, _owner: &Node2D, _event: Ref<InputEventKey>) {
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
