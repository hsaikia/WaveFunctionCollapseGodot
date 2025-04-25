use crate::wfc_probability_map::WfcProbabilityMap;
use crate::wfc_tile_dictionary::NUM_TILES;
use godot::classes::ITileMapLayer;
use godot::classes::RandomNumberGenerator;
use godot::classes::TileMapLayer;
use godot::classes::TileSetAtlasSource;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=TileMapLayer)]
struct WfcMapLayer {
    base: Base<TileMapLayer>,
    rng: Gd<RandomNumberGenerator>,
    is_ready: bool,
    wfc_prob_map: WfcProbabilityMap,
    #[export]
    map_size: Vector2i,
    #[export]
    atlas_source_id: i32,
    #[export]
    default_tile: Vector2i,
}

#[godot_api]
impl WfcMapLayer {
    #[func]
    fn set_cell(&mut self, x: i32, y: i32, atlas_coords: Vector2i) {
        let atlas_source_id = self.atlas_source_id;
        self.base_mut()
            .set_cell_ex(Vector2i { x, y })
            .source_id(atlas_source_id)
            .atlas_coords(atlas_coords)
            .done();
    }

    #[func]
    fn generate_new(&mut self) {
        if !self.is_ready {
            return;
        }

        let grid = self.wfc_prob_map.generate_wfc_grid(
            &mut self.rng,
            self.map_size.x as usize,
            self.map_size.y as usize,
        );

        self.base_mut().clear();
        for x in 0..self.map_size.x {
            for y in 0..self.map_size.y {
                if let Some(tile) = grid[x as usize][y as usize] {
                    self.set_cell(x, y, Vector2i::new(tile as i32, 0));
                } else {
                    self.set_cell(x, y, self.default_tile);
                }
            }
        }
    }
}

#[godot_api]
impl ITileMapLayer for WfcMapLayer {
    fn init(base: Base<TileMapLayer>) -> Self {
        Self {
            base,
            rng: RandomNumberGenerator::new_gd(),
            is_ready: false,
            wfc_prob_map: WfcProbabilityMap::default(),
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
            if tile_set_atlas_src.get_tiles_count() == NUM_TILES as i32 {
                self.is_ready = true;
                self.wfc_prob_map = WfcProbabilityMap::new();
                self.generate_new();
            }
        }
    }
}
