use crate::wfc_tile_dictionary::*;
use godot::{engine::RandomNumberGenerator, log::godot_print, obj::Gd};

pub const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub struct WfcRelation {
    pub possible_neighbors: Vec<[Vec<usize>; 4]>,
}

impl WfcRelation {
    pub fn new(num_tiles: usize) -> Self {
        if num_tiles == 0 {
            return Self {
                possible_neighbors: Vec::new(),
            };
        }

        let mut possible_neighbors: Vec<[Vec<usize>; 4]> = Vec::with_capacity(num_tiles);
        for _ in 0..num_tiles {
            let tile_neighbors: [Vec<usize>; 4] = Default::default();
            possible_neighbors.push(tile_neighbors);
        }

        assert!(num_tiles == WFC_TILE_DICT.len());

        for i in 0..num_tiles {
            for j in i..num_tiles {
                for d in 0..4 {
                    if WFC_TILE_DICT[i][d] == WFC_TILE_DICT[j][(d + 2) % 4] {
                        possible_neighbors[i][d].push(j);
                        if i != j {
                            possible_neighbors[j][(d + 2) % 4].push(i);
                        }
                    }
                }
            }
        }

        Self { possible_neighbors }
    }

    fn pick_possibility(
        rng: &mut Gd<RandomNumberGenerator>,
        grid: &[Vec<Option<usize>>],
        possibility_grid: &[Vec<Vec<usize>>],
    ) -> Option<(usize, usize)> {
        let mut min_num_possibility = usize::MAX;
        for x in 0..possibility_grid.len() {
            for y in 0..possibility_grid[x].len() {
                if grid[x][y].is_some() {
                    continue;
                }

                if possibility_grid[x][y].is_empty() {
                    return None;
                }

                let num_possibility = possibility_grid[x][y].len();
                if num_possibility < min_num_possibility {
                    min_num_possibility = num_possibility;
                }
            }
        }

        let possible_options = (0..possibility_grid.len())
            .flat_map(|x| (0..possibility_grid[x].len()).map(move |y| (x, y)))
            .filter(|&(x, y)| possibility_grid[x][y].len() == min_num_possibility)
            .collect::<Vec<_>>();

        if !possible_options.is_empty() {
            let idx = rng.randi_range(0, possible_options.len() as i32 - 1) as usize;
            return Some(possible_options[idx]);
        }

        None
    }

    fn set_and_propagate(
        &self,
        rng: &mut Gd<RandomNumberGenerator>,
        grid: &mut [Vec<Option<usize>>],
        possibility_grid: &mut [Vec<Vec<usize>>],
        x: usize,
        y: usize,
    ) {
        assert!(!possibility_grid[x][y].is_empty());
        let random_idx = rng.randi_range(0, possibility_grid[x][y].len() as i32 - 1) as usize;
        let value = possibility_grid[x][y][random_idx];
        grid[x][y] = Some(value);
        possibility_grid[x][y] = vec![value];

        let mut q = vec![(x, y)];
        while let Some((x, y)) = q.pop() {
            for (dir_idx, dir) in DIRECTIONS.iter().enumerate() {
                let nx = x as i32 + dir.0;
                let ny = y as i32 + dir.1;
                if nx < 0
                    || nx >= possibility_grid.len() as i32
                    || ny < 0
                    || ny >= possibility_grid[0].len() as i32
                {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                if possibility_grid[nx][ny].is_empty() {
                    continue;
                }

                let old_num_possibilities = possibility_grid[nx][ny].len();
                let values = possibility_grid[x][y].clone();
                let mut all_possible_values: Vec<usize> = Vec::new();
                for val in values {
                    all_possible_values.extend(&self.possible_neighbors[val][dir_idx]);
                }

                possibility_grid[nx][ny].retain(|v| all_possible_values.contains(v));

                if old_num_possibilities > possibility_grid[nx][ny].len() {
                    q.push((nx, ny));
                }
            }
        }
    }

    fn all_done(done: &[Vec<Option<usize>>]) -> bool {
        for row in done {
            for el in row {
                if el.is_none() {
                    return false;
                }
            }
        }
        true
    }

    pub fn generate_wfc_grid(
        &self,
        rng: &mut Gd<RandomNumberGenerator>,
        width: usize,
        height: usize,
    ) -> Vec<Vec<Option<usize>>> {
        let num_tiles = self.possible_neighbors.len();
        let all_neighbors = (0..num_tiles).collect::<Vec<_>>();
        let mut possibilities_grid = vec![vec![all_neighbors.clone(); height]; width];
        let mut grid = vec![vec![None; height]; width];

        while !WfcRelation::all_done(&grid) {
            if let Some((x, y)) = WfcRelation::pick_possibility(rng, &grid, &possibilities_grid) {
                self.set_and_propagate(rng, &mut grid, &mut possibilities_grid, x, y);
            } else {
                break;
            }
        }

        if !WfcRelation::all_done(&grid) {
            godot_print!("Failed to generate WFC grid");
            return self.generate_wfc_grid(rng, width, height);
        }

        godot_print!("Generated WFC grid");
        grid
    }
}
