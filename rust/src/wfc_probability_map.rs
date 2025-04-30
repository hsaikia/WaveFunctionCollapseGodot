use crate::wfc_tile_dictionary::{DIRECTIONS, NUM_TILES, WFC_TILE_DICT};
use godot::{classes::RandomNumberGenerator, global::godot_print, obj::Gd};

type TileIdx = usize;

#[derive(Clone)]
pub enum State {
    Wave(Vec<TileIdx>),
    Collapsed(TileIdx),
}

impl State {
    fn collapse_random(&mut self, rng: &mut Gd<RandomNumberGenerator>) {
        assert!(matches!(self, State::Wave(_)));
        match self {
            State::Wave(values) => {
                let random_idx = rng.randi_range(0, values.len() as i32 - 1) as usize;
                *self = State::Collapsed(values[random_idx]);
            }
            _ => (),
        }
    }

    fn is_collapsed(&self) -> bool {
        matches!(self, State::Collapsed(_))
    }

    fn values(&self) -> Vec<usize> {
        match self {
            State::Collapsed(val) => {
                vec![*val]
            }
            State::Wave(values) => values.clone(),
        }
    }
}

#[derive(Default)]
pub struct WfcProbabilityMap {
    possible_neighbors: Vec<[Vec<TileIdx>; 4]>,
    pub grid: Vec<Vec<State>>,
}

impl WfcProbabilityMap {
    pub fn new(width: usize, height: usize) -> Self {
        let mut possible_neighbors: Vec<[Vec<TileIdx>; 4]> = Vec::with_capacity(NUM_TILES);
        for _ in 0..NUM_TILES {
            let tile_neighbors: [Vec<TileIdx>; 4] = Default::default();
            possible_neighbors.push(tile_neighbors);
        }

        for i in 0..NUM_TILES {
            for j in i..NUM_TILES {
                for d in 0..4 {
                    // Check if opposite side (d and (d + 2) % 4) of a direction has the same connection type
                    if WFC_TILE_DICT[i][d] == WFC_TILE_DICT[j][(d + 2) % 4] {
                        possible_neighbors[i][d].push(j);
                        if i != j {
                            possible_neighbors[j][(d + 2) % 4].push(i);
                        }
                    }
                }
            }
        }

        let all_tile_indices = (0..NUM_TILES).collect::<Vec<_>>();
        let grid = vec![vec![State::Wave(all_tile_indices.clone()); height]; width];

        Self {
            possible_neighbors,
            grid,
        }
    }

    fn reset(&mut self) {
        let all_tile_indices = (0..NUM_TILES).collect::<Vec<_>>();
        let width = self.grid.len();
        let height = self.grid.first().unwrap_or(&vec![]).len();
        self.grid = vec![vec![State::Wave(all_tile_indices.clone()); height]; width];
    }

    /// Pick (one of) the grid locations which has the minimum possible valid tile options
    /// Most constrained grid location is picked as that likely preserves the most number of options for other cells
    fn pick_possibility(&self, rng: &mut Gd<RandomNumberGenerator>) -> Option<(usize, usize)> {
        let mut min_num_possibility = usize::MAX;
        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                match &self.grid[x][y] {
                    State::Wave(possible_values) => {
                        if possible_values.is_empty() {
                            return None;
                        }
                        min_num_possibility = min_num_possibility.min(possible_values.len());
                    }
                    _ => {}
                }
            }
        }

        let possible_options = (0..self.grid.len())
            .flat_map(|x| (0..self.grid[x].len()).map(move |y| (x, y)))
            .filter(|&(x, y)| match &self.grid[x][y] {
                State::Wave(values) => values.len() == min_num_possibility,
                _ => false,
            })
            .collect::<Vec<_>>();

        if !possible_options.is_empty() {
            let idx = rng.randi_range(0, possible_options.len() as i32 - 1) as usize;
            return Some(possible_options[idx]);
        }

        None
    }

    /// Sets the grid position at (x, y) to a random tile and propagates the dependencies from that choice to neighboring tiles
    fn set_and_propagate(&mut self, rng: &mut Gd<RandomNumberGenerator>, x: usize, y: usize) {
        self.grid[x][y].collapse_random(rng);

        let mut q = vec![(x, y)];
        while let Some((curr_x, curr_y)) = q.pop() {
            for (dir_idx, dir) in DIRECTIONS.iter().enumerate() {
                let nx = curr_x as i32 + dir.0;
                let ny = curr_y as i32 + dir.1;
                if nx < 0
                    || nx >= self.grid.len() as i32
                    || ny < 0
                    || ny >= self.grid[0].len() as i32
                {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                let possible_values_nx_ny: Vec<usize> = self.grid[curr_x][curr_y]
                    .values()
                    .iter()
                    .flat_map(|val| self.possible_neighbors[*val][dir_idx].clone())
                    .collect();
                match &mut self.grid[nx][ny] {
                    State::Wave(possibilities) => {
                        let old_num_possibilities = possibilities.len();
                        possibilities.retain(|v| possible_values_nx_ny.contains(v));

                        if old_num_possibilities > possibilities.len() {
                            q.push((nx, ny));
                        }
                    }
                    State::Collapsed(_) => {
                        continue;
                    }
                }
            }
        }
    }

    fn all_collapsed(&self) -> bool {
        self.grid
            .iter()
            .all(|row| row.iter().all(|s| s.is_collapsed()))
    }

    fn all_same(&self) -> bool {
        let mut count: [usize; NUM_TILES] = [0; NUM_TILES];
        for row in &self.grid {
            for cell in row {
                match cell {
                    State::Collapsed(x) => {
                        count[*x] += 1;
                    }
                    _ => (),
                }
            }
        }
        let grid_cells = self.grid.len() * self.grid.first().unwrap_or(&vec![]).len();
        count.iter().filter(|x| **x == grid_cells).count() > 0
    }

    pub fn generate_wfc_grid(
        &mut self,
        rng: &mut Gd<RandomNumberGenerator>,
        width: usize,
        height: usize,
        retries: i32,
    ) -> bool {
        if retries == 0 {
            // Failed to generate for all retry attempts
            return false;
        }

        self.reset();
        while !self.all_collapsed() {
            if let Some((x, y)) = self.pick_possibility(rng) {
                self.set_and_propagate(rng, x, y);
            } else {
                break;
            }
        }

        if !self.all_collapsed() || self.all_same() {
            godot_print!("Failed to generate WFC grid");
            return self.generate_wfc_grid(rng, width, height, retries - 1);
        }

        godot_print!("Generated WFC grid with {} tries remaining.", retries - 1);
        true
    }
}
