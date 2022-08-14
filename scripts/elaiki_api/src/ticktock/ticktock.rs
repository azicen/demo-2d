use crate::utils::errors::Error;

pub trait Ticktock {
    // 执行
    fn exec(&self) -> Result<(), Error>;

    // 回滚
    fn rollback(&self) -> Result<(), Error>;

    // 是否已经执行
    fn is_exec(&self) -> bool;

    // 是否执行成功
    fn is_successful(&self) -> bool;
}
