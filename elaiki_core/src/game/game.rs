use std::rc::Rc;

use gdnative::api::Node2D;
use gdnative::prelude::{methods, NativeClass};

use elaiki_api::log::Helper;

use crate::elaiki::Elaiki;
use crate::worlds::World;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Game {
    ticktock_hub: Rc<crate::ticktock::Hub>,
    world: Option<World>,
}

impl Game {
    fn new(_owner: &Node2D) -> Self {
        // 初始化 Elaiki单例，使得 Elaiki单例被new
        let elaiki = Elaiki::get_instance().lock().unwrap();
        Game {
            ticktock_hub: elaiki.ticktock_hub(),
            world: None,
        }
    }
}

#[methods]
impl Game {
    // 比_ready更早执行
    #[export]
    fn _enter_tree(&mut self, _owner: &Node2D) {
        let mut elaiki = Elaiki::get_instance().lock().unwrap();
        elaiki.init(_owner);
        let log = Helper::new(elaiki.log());

        // let _world = World::new(_owner);
        // match _world {
        //     Ok(v) => {
        //         self.world = Some(v);
        //     }
        //     Err(e) => {
        //         log.error(&e.to_string());
        //     }
        // }
        log.info("Ok... init");
        // log.debug(format!("{:?}", err!("test")).as_str());
        // log.debug(err!("test").to_string().as_str());
    }

    #[export]
    fn _process(&self, _owner: &Node2D, _delta: f32) {
        self.ticktock_hub.as_ref().exec_frame();
    }

    #[export]
    fn _physics_process(&mut self, _owner: &Node2D, _delta: f64) {
        self.ticktock_hub.as_ref().exec_fixed();
    }
}
