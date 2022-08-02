use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::rc::Rc;
use std::sync::{Mutex, Once};

use elaiki_api::ticktock::TicktockScheduler;
use gdnative::prelude::Node2D;

use crate::entities::Player;

pub struct Elaiki {
    pub ticktock_scheduler: Box<TicktockScheduler>,
    player: Option<Rc<RefCell<Player>>>,
}

impl Elaiki {
    pub fn new() -> Self {
        Elaiki {
            ticktock_scheduler: Box::new(TicktockScheduler::new()),
            player: None,
        }
    }

    pub fn init(&mut self, root_node: &Node2D) {
        self.player = Some(Rc::new(RefCell::new(Player::new(root_node).unwrap())))
    }

    pub fn player(&self) -> Rc<RefCell<Player>> {
        self.player.as_ref().unwrap().clone()
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
