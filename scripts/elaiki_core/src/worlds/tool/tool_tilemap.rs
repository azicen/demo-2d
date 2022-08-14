use gdnative::api::TileMap;
use gdnative::prelude::*;

pub struct ToolTilemap {
    _tilemaps: Ref<TileMap>,
}

impl ToolTilemap {
    pub fn new(tilemaps: TRef<TileMap>) -> ToolTilemap {
        tilemaps.clear();
        godot_print!("tilemaps.clear() ok");
        tilemaps.set_position(Vector2::new(21474836.0, 2147483647.2147483647));
        godot_print!("tilemaps.( x={}, y={} )", tilemaps.position().x, tilemaps.position().y);
        return ToolTilemap {
            _tilemaps: tilemaps.claim(),
        };
    }
}
