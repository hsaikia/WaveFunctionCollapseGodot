use crate::wfc_relation::WfcRelation;
use godot::classes::ITileMapLayer;
use godot::classes::RandomNumberGenerator;
use godot::classes::TileMapLayer;
use godot::classes::TileSetAtlasSource;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=TileMapLayer)]
struct WfcMap {
    base: Base<TileMapLayer>,
    rng: Gd<RandomNumberGenerator>,
    tile_count: i32,
    wfc_rel: WfcRelation,
    #[export]
    map_size: Vector2i,
    #[export]
    atlas_source_id: i32,
    #[export]
    default_tile: Vector2i,
}

#[godot_api]
impl WfcMap {
    #[func]
    fn set_cell(&mut self, x: i32, y: i32, source_id: i32, atlas_coords: Vector2i) {
        self.base_mut()
            .set_cell_ex(Vector2i { x, y })
            .source_id(source_id)
            .atlas_coords(atlas_coords)
            .done();
    }

    #[func]
    fn generate_new(&mut self) {
        if self.tile_count == 0 {
            return;
        }

        let grid = self.wfc_rel.generate_wfc_grid(
            &mut self.rng,
            self.map_size.x as usize,
            self.map_size.y as usize,
        );

        self.base_mut().clear();
        for x in 0..self.map_size.x {
            for y in 0..self.map_size.y {
                if let Some(tile) = grid[x as usize][y as usize] {
                    self.set_cell(x, y, self.atlas_source_id, Vector2i::new(tile as i32, 0));
                } else {
                    self.set_cell(x, y, self.atlas_source_id, self.default_tile);
                }
            }
        }
    }
}

#[godot_api]
impl ITileMapLayer for WfcMap {
    fn init(base: Base<TileMapLayer>) -> Self {
        Self {
            base,
            rng: RandomNumberGenerator::new_gd(),
            tile_count: 0,
            wfc_rel: WfcRelation::new(0),
            map_size: Vector2i { x: 10, y: 10 },
            atlas_source_id: 0,
            default_tile: Vector2i { x: 26, y: 0 },
        }
    }

    fn ready(&mut self) {
        if let Ok(tile_set_atlas_src) = self
            .base()
            .get_tile_set()
            .unwrap()
            .get_source(self.atlas_source_id)
            .unwrap()
            .try_cast::<TileSetAtlasSource>()
        {
            self.tile_count = tile_set_atlas_src.get_tiles_count();
            godot_print!("Tile count: {}", self.tile_count);
            self.wfc_rel = WfcRelation::new(self.tile_count as usize);
        }
        self.generate_new();
    }
}
