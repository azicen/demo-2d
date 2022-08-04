use std::{cell::RefCell, rc::Rc};

use gdnative::prelude::Vector2;

use elaiki_api::entities::EntityMovable;
use elaiki_api::ticktock::Ticktock;
use elaiki_api::utils::errors::Error;

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

impl Ticktock for TicktockMovable {
    // 执行
    fn exec(&self) -> Result<(), Error> {
        let mut player = self.player.try_borrow_mut().unwrap(); // TODO 不安全的代码

        let v2 = Vector2::new(self.x, self.y);
        player.movement(v2.x, v2.y);
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
