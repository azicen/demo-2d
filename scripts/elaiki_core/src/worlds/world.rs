use std::rc::Rc;

use gdnative::prelude::{AsArg, Texture};

use crate::resource_manager::MapResource;

use super::Land;

pub struct World {
    resource: Rc<MapResource>,
}

impl World {
    pub fn new(map_resource: Rc<MapResource>) -> World {
        World {
            resource: map_resource,
        }
    }
}

impl World {
    pub fn update_tile(&self) {
        // self.resource.tileset()
    }
}

// 注册各种资源的实现
// 这是不安全的，只被允许在程序初始化阶段使用
// TODO 考虑之后使用“锁”来强化“只允许初始化阶段使用”和“禁止异步调用”的行为
impl World {
    // register_land 注册地皮数据
    // TODO 考虑之后使用一个数据类型来注册地皮数据，替代直接使用的基础数据类型，使意义更明确
    // TODO 未完成: 禁止异步调用
    pub unsafe fn register_land(
        &self,
        id: u32,
        name: String,
        short_name: String,
        texture: impl AsArg<Texture>,
    ) {
        Land::register(id, name, short_name);
        self.resource.tileset().create_tile(id as i64);
        self.resource.tileset().tile_set_texture(id as i64, texture);
    }
}
