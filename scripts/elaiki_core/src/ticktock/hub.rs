use std::rc::Rc;
use std::sync::Mutex;

use elaiki_api::log;
use elaiki_api::ticktock::{Dispatcher, Ticktock};

// 命令调度中心
pub struct Hub {
    // 按照帧执行的
    ticktock_frame: Mutex<Dispatcher>,

    // 按照固定时间执行的
    ticktock_fixed: Mutex<Dispatcher>,
}

impl Hub {
    pub fn new(logger: Rc<dyn log::Logger>) -> Self {
        Hub {
            ticktock_frame: Mutex::new(Dispatcher::new(logger.clone())),
            ticktock_fixed: Mutex::new(Dispatcher::new(logger.clone())),
        }
    }
}

impl Hub {
    pub fn add_frame(&self, ticktock: Box<dyn Ticktock>) {
        self.ticktock_frame.lock().unwrap().add(ticktock);
    }

    pub fn add_fixed(&self, ticktock: Box<dyn Ticktock>) {
        self.ticktock_fixed.lock().unwrap().add(ticktock);
    }

    pub fn exec_frame(&self) {
        self.ticktock_frame.lock().unwrap().exec();
    }

    pub fn exec_fixed(&self) {
        self.ticktock_fixed.lock().unwrap().exec();
    }
}
