use crate::wfc_relation::WfcRelation;
use crate::wfc_tile_dictionary::DEFAULT_TILE;
use godot::engine::ITileMap;
use godot::engine::RandomNumberGenerator;
use godot::engine::TileMap;
use godot::engine::TileSetAtlasSource;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=TileMap)]
struct WfcMap {
    base: Base<TileMap>,
    rng: Gd<RandomNumberGenerator>,
    grid_size: Vector2i,
    tile_count: i32,
    wfc_rel: WfcRelation,
}

const ATLAS_SOURCE_ID: i32 = 0;

#[godot_api]
impl WfcMap {
    #[func]
    fn set_cell(&mut self, x: i32, y: i32, source_id: i32, atlas_coords: Vector2i) {
        self.base_mut()
            .set_cell_ex(0, Vector2i { x, y })
            .source_id(source_id)
            .atlas_coords(atlas_coords)
            .done();
    }

    #[func]
    fn generate_new(&mut self, map_size: Vector2i) {
        if self.tile_count == 0 {
            return;
        }

        let grid =
            self.wfc_rel
                .generate_wfc_grid(&mut self.rng, map_size.x as usize, map_size.y as usize);

        self.base_mut().clear_layer(0);
        for x in 0..map_size.x {
            for y in 0..map_size.y {
                if let Some(tile) = grid[x as usize][y as usize] {
                    self.set_cell(x, y, ATLAS_SOURCE_ID, Vector2i::new(tile as i32, 0));
                } else {
                    self.set_cell(x, y, ATLAS_SOURCE_ID, Vector2i::new(DEFAULT_TILE, 0));
                }
            }
        }
    }
}

#[godot_api]
impl ITileMap for WfcMap {
    fn init(base: Base<TileMap>) -> Self {
        Self {
            base,
            rng: RandomNumberGenerator::new_gd(),
            grid_size: Vector2i::new(0, 0),
            tile_count: 0,
            wfc_rel: WfcRelation::new(0),
        }
    }

    fn ready(&mut self) {
        if let Ok(tile_set_atlas_src) = self
            .base()
            .get_tileset()
            .unwrap()
            .get_source(ATLAS_SOURCE_ID)
            .unwrap()
            .try_cast::<TileSetAtlasSource>()
        {
            self.grid_size = tile_set_atlas_src.get_atlas_grid_size();
            self.tile_count = tile_set_atlas_src.get_tiles_count();
            godot_print!("Tile count: {}", self.tile_count);
            self.wfc_rel = WfcRelation::new(self.tile_count as usize);
        }
        self.generate_new(Vector2i { x: 10, y: 10 });
    }
}
