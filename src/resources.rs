use crate::components::{CellPosition, CellState};
use bevy::prelude::*;
use std::collections::HashMap;

pub type CellEntityMap = HashMap<CellPosition, Entity>;

pub type ColorHandleMap = HashMap<String, Handle<ColorMaterial>>;

pub struct BoardCycleEvent {
    pub delta: Vec<(CellPosition, CellState)>,
}

pub struct CycleTimer(pub Timer);

pub struct CellSize(pub f32, pub f32);

#[derive(Clone)]
pub struct CellBoard {
    pub width: usize,
    pub height: usize,
    state: Vec<CellState>,
}

impl CellBoard {
    pub fn new(width: usize, height: usize) -> Self {
        let state = vec![CellState::Dead; width * height];
        Self {
            width,
            height,
            state,
        }
    }

    pub fn patch(
        &mut self,
        CellPosition(row, col): CellPosition,
        patch: &[bool],
        patch_width: usize,
        patch_height: usize,
    ) {
        assert!(col < self.width, "Non-existent column index");
        assert!(row < self.height, "Non-existent row index");

        assert!(col + patch_width <= self.width, "Patch exceeds board size");
        assert!(
            row + patch_height <= self.height,
            "Patch exceeds board size"
        );

        for row_patch in 0..patch_height {
            for col_patch in 0..patch_width {
                let row = row + row_patch;
                let col = col + col_patch;
                self.state[row * self.width + col] = if patch[row_patch * patch_width + col_patch] {
                    CellState::Alive
                } else {
                    CellState::Dead
                };
            }
        }
    }

    pub fn cycle(&mut self) -> Vec<(CellPosition, CellState)> {
        let mut delta = vec![];
        for row in 0..self.height {
            for col in 0..self.width {
                let pos = CellPosition(row, col);
                let n_alive_neighbours: usize = self
                    .neighbours(pos)
                    .into_iter()
                    .map(|pos| if self.alive(pos) { 1 } else { 0 })
                    .sum();
                let can_live =
                    self.alive(pos) && (n_alive_neighbours == 2 || n_alive_neighbours == 3);
                let can_revive = !self.alive(pos) && n_alive_neighbours == 3;

                // RULES (https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life#Rules)
                // 1. Any live cell with two or three live neighbours survives.
                // 2. Any dead cell with three live neighbours becomes a live cell.
                // 3. All other live cells die in the next generation. Similarly, all other dead
                // cells stay dead.
                let new_cell_state = if can_live || can_revive {
                    CellState::Alive
                } else {
                    CellState::Dead
                };

                if self.state[row * self.width + col] != new_cell_state {
                    delta.push((pos, new_cell_state));
                }
            }
        }
        for &(CellPosition(row, col), state) in &delta {
            self.state[row * self.width + col] = state;
        }
        delta
    }

    pub fn alive(&self, CellPosition(row, col): CellPosition) -> bool {
        assert!(col < self.width, "Non-existent column index");
        assert!(row < self.height, "Non-existent row index");

        self.state[row * self.width + col] == CellState::Alive
    }

    fn neighbours(&self, CellPosition(row, col): CellPosition) -> Vec<CellPosition> {
        assert!(col < self.width, "Non-existent column index");
        assert!(row < self.height, "Non-existent row index");

        let mut neighbours = Vec::new();
        for row_offset in -1..=1 {
            for col_offset in -1..=1 {
                if row_offset == 0 && col_offset == 0 {
                    continue;
                }

                let row_neighbour = row as isize + row_offset;
                let row_neighbour = if row_neighbour < 0 {
                    self.height - 1
                } else if row_neighbour as usize >= self.height {
                    0
                } else {
                    row_neighbour as usize
                };

                let col_neighbour = col as isize + col_offset;
                let col_neighbour = if col_neighbour < 0 {
                    self.width - 1
                } else if col_neighbour as usize >= self.width {
                    0
                } else {
                    col_neighbour as usize
                };

                neighbours.push(CellPosition(row_neighbour, col_neighbour));
            }
        }
        neighbours
    }
}
