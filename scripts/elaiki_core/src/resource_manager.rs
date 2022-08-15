use std::rc::Rc;

use gdnative::api::{TileMap, TileSet};
use gdnative::prelude::{
    GodotObject, KinematicBody2D, Node, Node2D, Ref, ResourceLoader, Sprite, SubClass, TRef,
};

use elaiki_api::utils::errors::{err, Error};

// 资源管理器，用于管理游戏资源和场景资源
pub struct ResourceManager {
    resource_loader: &'static ResourceLoader,

    player_resource: Rc<PlayerResource>,
    map_resource: Rc<MapResource>,
}

impl ResourceManager {
    pub fn new(root_node: &Node2D) -> Result<Self, Error> {
        let player_res = PlayerResource::new(root_node)?;
        let map_res = MapResource::new(root_node)?;
        Ok(ResourceManager {
            resource_loader: ResourceLoader::godot_singleton(),

            player_resource: Rc::new(player_res),
            map_resource: Rc::new(map_res),
        })
    }

    fn extraction_node<U>(root_node: &Node2D, node_path: &str) -> Result<Ref<U>, Error>
    where
        U: GodotObject + SubClass<Node>,
    {
        // 获取节点
        let node = root_node.get_node(node_path);
        // 节点是否存在
        let result = match node {
            // 节点类型是否正确
            Some(n) => match unsafe { n.assume_safe() }.cast::<U>() {
                Some(n2d) => Ok(n2d.claim()),
                None => Err(err!("node path:{} are not available.", node_path)),
            },
            None => Err(err!("node path:{} do not exist.", node_path)),
        };
        result
    }
}

impl ResourceManager {
    pub fn update_player_resource(&mut self, res: PlayerResource) {
        self.player_resource = Rc::new(res);
    }

    pub fn player_resource(&self) -> Rc<PlayerResource> {
        Rc::clone(&self.player_resource)
    }

    pub fn map_resource(&self) -> Rc<MapResource> {
        Rc::clone(&self.map_resource)
    }

    pub fn resource_loader(&self) -> &ResourceLoader {
        self.resource_loader
    }
}

/********** 玩家资源 **********/
pub struct PlayerResource {
    sprite: Ref<Sprite>,
    kinematic_body: Ref<KinematicBody2D>,
}

impl PlayerResource {
    pub fn new(root_node: &Node2D) -> Result<Self, Error> {
        // 从引擎场景中获取资源
        let kinematic_node = PlayerResource::extraction_kinematic_node(root_node)?;
        let sprite_node = PlayerResource::extraction_sprite_node(root_node)?;

        Ok(PlayerResource {
            sprite: sprite_node,
            kinematic_body: kinematic_node,
        })
    }

    fn extraction_kinematic_node(root_node: &Node2D) -> Result<Ref<KinematicBody2D>, Error> {
        ResourceManager::extraction_node(root_node, node::player::KINEMATIC_BODY)
    }

    fn extraction_sprite_node(root_node: &Node2D) -> Result<Ref<Sprite>, Error> {
        ResourceManager::extraction_node(root_node, node::player::SPRITE)
    }
}

impl PlayerResource {
    pub fn sprite(&self) -> TRef<Sprite> {
        unsafe { self.sprite.assume_safe() }
    }

    pub fn kinematic_body(&self) -> TRef<KinematicBody2D> {
        unsafe { self.kinematic_body.assume_safe() }
    }
}

/********** 地图资源 **********/
pub struct MapResource {
    tilemap: Ref<TileMap>,
    tileset: Ref<TileSet>,
}

impl MapResource {
    pub fn new(root_node: &Node2D) -> Result<Self, Error> {
        let tileset_node = unsafe { TileSet::new().assume_shared() };
        let tilemap_node = MapResource::extraction_tilemap_node(root_node)?;

        unsafe { tilemap_node.assume_safe() }
            .set_tileset(unsafe { tileset_node.assume_safe() }.claim());

        Ok(MapResource {
            tilemap: tilemap_node,
            tileset: tileset_node,
        })
    }

    fn extraction_tilemap_node(root_node: &Node2D) -> Result<Ref<TileMap>, Error> {
        ResourceManager::extraction_node(root_node, node::map::TILE_MAP)
    }
}

impl MapResource {
    pub fn tilemap(&self) -> TRef<TileMap> {
        unsafe { self.tilemap.assume_safe() }
    }

    pub fn tileset(&self) -> TRef<TileSet> {
        unsafe { self.tileset.assume_safe() }
    }
}

/********** 节点资源路径 **********/
mod node {
    pub mod player {
        // const PLAYER_NODE_NAME: &str = "Player";
        pub const KINEMATIC_BODY: &str = "Player/KinematicBody2D";
        pub const SPRITE: &str = "Player/KinematicBody2D/Sprite";
    }

    pub mod map {
        pub const TILE_MAP: &str = "Map/TileMap";
    }
}
