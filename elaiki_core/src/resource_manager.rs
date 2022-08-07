use std::rc::Rc;

use gdnative::prelude::{GodotObject, KinematicBody2D, Node, Node2D, Ref, Sprite, SubClass, TRef};

use elaiki_api::utils::errors::{err, Error};

// 资源管理器，用于管理游戏资源和场景资源
pub struct ResourceManager {
    player_resource: Rc<PlayerResource>,
}

impl ResourceManager {
    pub fn new(root_node: &Node2D) -> Result<Self, Error> {
        let player_res = PlayerResource::new(root_node)?;
        Ok(ResourceManager {
            player_resource: Rc::new(player_res),
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
}

/********** 玩家资源 **********/
pub struct PlayerResource {
    sprite: Ref<Sprite>,
    kinematic_body: Ref<KinematicBody2D>,
}

// const PLAYER_NODE_NAME: &str = "Player";
const PLAYER_KINEMATIC_BODY_NODE_NAME: &str = "Player/KinematicBody2D";
const PLAYER_SPRITE_NODE_NAME: &str = "Player/KinematicBody2D/Sprite";

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
        ResourceManager::extraction_node(root_node, PLAYER_KINEMATIC_BODY_NODE_NAME)
    }

    fn extraction_sprite_node(root_node: &Node2D) -> Result<Ref<Sprite>, Error> {
        ResourceManager::extraction_node(root_node, PLAYER_SPRITE_NODE_NAME)
    }
}

impl PlayerResource {
    pub unsafe fn sprite(&self) -> TRef<Sprite> {
        self.sprite.assume_safe()
    }

    pub unsafe fn kinematic_body(&self) -> TRef<KinematicBody2D> {
        self.kinematic_body.assume_safe()
    }
}

struct MapResource {}