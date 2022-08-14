use gdnative::api::Node2D;
use gdnative::api::TileMap;

use elaiki_api::utils::errors::*;

use crate::worlds::tool::ToolTilemap;

// use gdnative::prelude::*;

pub struct World {
    tilemap_tool: ToolTilemap,
}

impl World {
    pub fn new(root_node: &Node2D) -> Result<World, Error> {
        let _tilemap_tool = root_node.get_node("TileMap");

        let tilemap_tool = match _tilemap_tool {
            Some(node) => {
                let _tilemap = unsafe { node.assume_safe() }.cast::<TileMap>();

                match _tilemap {
                    Some(tilemap) => Ok(ToolTilemap::new(tilemap)),
                    None => Err(Error::new("tilemap is none".to_string())),
                }
            }
            None => Err(Error::new("node is none".to_string())),
        };

        let _world = World {
            tilemap_tool: tilemap_tool?,
        };

        Ok(_world)
    }

    pub fn tilemap_tool(self) -> ToolTilemap {
        return self.tilemap_tool;
    }
}
