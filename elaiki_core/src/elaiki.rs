use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::rc::Rc;
use std::sync::{Mutex, Once};

use gdnative::prelude::Node2D;

use crate::entities::Player;
use crate::log::Log;
use crate::resource_manager::ResourceManager;

pub struct Elaiki {
    ticktock_hub: Rc<crate::ticktock::Hub>,
    player: Option<Rc<RefCell<Player>>>,

    log: Rc<dyn elaiki_api::log::Logger>,
    logger: elaiki_api::log::Helper,
}

impl Elaiki {
    pub fn new() -> Self {
        let log = Rc::new(Log::new());
        let logger = elaiki_api::log::Helper::new(log.clone());
        Elaiki {
            ticktock_hub: Rc::new(crate::ticktock::Hub::new(log.clone())),
            player: None,

            log,
            logger,
        }
    }

    pub fn get_instance() -> &'static Mutex<Elaiki> {
        static mut INSTANCE: MaybeUninit<Mutex<Elaiki>> = MaybeUninit::uninit();
        static ONEC: Once = Once::new();

        ONEC.call_once(|| unsafe {
            INSTANCE.as_mut_ptr().write(Mutex::new(Elaiki::new()));
        });

        unsafe { &*INSTANCE.as_ptr() }
    }
}

impl Elaiki {
    pub fn init(&mut self, root_node: &Node2D) {
        let resource_manager = ResourceManager::new(root_node).unwrap(); // TODO 为完成的不安全代码
        self.player = Some(Rc::new(RefCell::new(Player::new(
            resource_manager.player_resource(),
        ))));
        self.logger.debug("Ok... elaiki init");
    }

    pub fn ticktock_hub(&self) -> Rc<crate::ticktock::Hub> {
        Rc::clone(&self.ticktock_hub)
    }

    pub fn player(&self) -> Rc<RefCell<Player>> {
        Rc::clone(&self.player.as_ref().unwrap()) // TODO 为完成的不安全代码
    }

    pub fn log(&self) -> Rc<dyn elaiki_api::log::Logger> {
        self.log.clone()
    }
}
