use gdnative::api::Node2D;
use gdnative::prelude::*;

use crate::elaiki::Elaiki;
use crate::worlds::World;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Init {
    world: Option<World>,
    // elaiki: Box<Elaiki>,
}

#[methods]
impl Init {
    fn new(_owner: &Node2D) -> Self {
        Init {
            world: None,
            // elaiki: Box::new(Elaiki::new()),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node2D) {
        let _world = World::new(_owner);
        match _world {
            Ok(v) => {
                self.world = Some(v);
            }
            Err(e) => {
                godot_dbg!(e);
            }
        }
        Elaiki::get_instance().lock().unwrap().init(_owner);
        godot_print!("Ok... init");
    }

    #[export]
    fn _process(&mut self, _owner: &Node2D, _delta: f32) {}

    #[export]
    fn _physics_process(&mut self, _owner: &Node2D, _delta: f64) {
        // self.elaiki.ticktock_scheduler.exec();
        let mut e = Elaiki::get_instance().lock().unwrap();
        e.ticktock_scheduler.exec();
    }
}
