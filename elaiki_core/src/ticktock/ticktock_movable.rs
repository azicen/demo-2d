use std::{cell::RefCell, rc::Rc};

use elaiki_api::{ticktock::TicktockBase, utils::errors::Error};
use gdnative::prelude::Vector2;

use crate::entities::Player;

pub struct TicktockMovable {
    player: Rc<RefCell<Player>>,
    x: f32,
    y: f32,
}

impl TicktockMovable {
    pub fn new(player: Rc<RefCell<Player>>, x: f32, y: f32) -> Self {
        TicktockMovable { player, x, y }
    }
}

impl TicktockBase for TicktockMovable {
    // 执行
    fn exec(&self) -> Result<(), Error> {
        let player = self.player.try_borrow_mut().unwrap();
        let game_obj = unsafe { player.game_obj.assume_safe() };

        let v2 = Vector2::new(self.x, self.y);
        game_obj.move_and_collide(v2, true, true, false);
        Ok(())
    }

    // 回滚
    fn rollback(&self) -> Result<(), Error> {
        Ok(())
    }

    // 是否已经执行
    fn is_exec(&self) -> bool {
        false
    }

    // 是否执行成功
    fn is_successful(&self) -> bool {
        false
    }
}
