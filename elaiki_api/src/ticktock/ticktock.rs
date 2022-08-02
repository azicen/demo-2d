use crate::utils::errors::*;

pub enum Ticktock {
    // 帧命令，每次更新帧时被执行
    // 由 gdnative::prelude::NativeClass::_process 触发
    Frame(Box<dyn TicktockBase>),

    // 时钟命令，固定时间被执行
    // 由 gdnative::prelude::NativeClass::_physics_process 触发
    Fixed(Box<dyn TicktockBase>),
}

pub trait TicktockBase {
    // 执行
    fn exec(&self) -> Result<(), Error>;

    // 回滚
    fn rollback(&self) -> Result<(), Error>;

    // 是否已经执行
    fn is_exec(&self) -> bool;

    // 是否执行成功
    fn is_successful(&self) -> bool;
}

pub trait FrameTicktock {
    type TicktockBase;
}

pub trait FixedTicktock {
    type TicktockBase;
}
