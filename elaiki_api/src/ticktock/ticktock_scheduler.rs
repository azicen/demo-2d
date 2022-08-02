use crate::ticktock::Ticktock;
pub struct TicktockScheduler {
    wait: Vec<Ticktock>,    // 等待执行的命令
    history: Vec<Ticktock>, // 命令历史记录
}

impl TicktockScheduler {
    pub fn new() -> Self {
        TicktockScheduler {
            wait: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn ticktock(&mut self, orders: Ticktock) {
        self.wait.push(orders);
    }

    pub fn exec(&mut self) {
        let status = vec![true; self.wait.len()]; // 先认为所有命令都能执行成功
        for i in 0..self.wait.len() {
            match &self.wait[i] {
                Ticktock::Frame(v) => {
                    _ = v.exec();
                }
                Ticktock::Fixed(v) => {
                    _ = v.exec();
                }
            }
        }
        let mut iter = status.iter();
        self.wait.retain(|_| *iter.next().unwrap()); // 保留成功的命令，将失败的排除掉

        self.history.append(&mut self.wait); // 将执行成功的命令加入到历史记录中
    }
}
