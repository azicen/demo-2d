use std::rc::Rc;

use crate::log;

use super::Ticktock;

// 命令调度器
pub struct Dispatcher {
    // 命令缓冲区，储存等待执行的命令
    buffer: Vec<Box<dyn Ticktock>>,

    // 命令历史记录，储存执行过的命令
    history: Vec<Box<dyn Ticktock>>,

    // 日志
    log: log::Helper,
}

impl Dispatcher {
    pub fn new(logger: Rc<dyn log::Logger>) -> Self {
        Dispatcher {
            buffer: Vec::new(),
            history: Vec::new(),

            log: log::Helper::new(logger),
        }
    }

    pub fn add(&mut self, ticktock: Box<dyn Ticktock>) {
        self.buffer.push(ticktock);
    }

    pub fn exec(&mut self) {
        let status = vec![true; self.buffer.len()]; // 先认为所有命令都能执行成功
        for i in 0..self.buffer.len() {
            if let Err(e) = self.buffer[i].exec() {
                self.log.error(&e.to_string());
            }
        }
        let mut iter = status.iter();
        self.buffer.retain(|_| *iter.next().unwrap()); // 保留成功的命令，将失败的排除掉

        self.history.append(&mut self.buffer); // 将执行成功的命令加入到历史记录中
    }
}
