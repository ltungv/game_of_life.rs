use crate::components::{CellPosition, CellState};
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct CellEntityMap(pub HashMap<CellPosition, Entity>);

#[derive(Default)]
pub struct ColorHandleMap(pub HashMap<String, Handle<ColorMaterial>>);

pub struct CycleTimer(pub Timer);

pub struct BoardCycleEvent {
    pub delta: Vec<(CellPosition, CellState)>,
}

pub struct CellSize {
    pub width: f32,
    pub height: f32,
}

impl FromWorld for CellSize {
    fn from_world(world: &mut World) -> Self {
        let window = world.get_resource::<WindowDescriptor>().unwrap();
        let board = world.get_resource::<CellBoard>().unwrap();
        Self {
            width: window.width / board.width as f32,
            height: window.height / board.height as f32,
        }
    }
}

#[derive(Clone)]
pub struct CellBoard {
    pub width: usize,
    pub height: usize,
    state: Vec<CellState>,
}

const DEFAULT_BOARD_STATE: ([CellState; 40 * 40], (usize, usize)) =
    ([CellState::Dead; 40 * 40], (40, 40));

impl Default for CellBoard {
    fn default() -> Self {
        let (state, (width, height)) = (Vec::from(DEFAULT_BOARD_STATE.0), DEFAULT_BOARD_STATE.1);
        Self {
            width,
            height,
            state,
        }
    }
}

impl CellBoard {
    pub fn new(width: usize, height: usize, state: Vec<CellState>) -> Self {
        Self {
            width,
            height,
            state,
        }
    }

    pub fn patch(
        &mut self,
        pos: CellPosition,
        patch: &[CellState],
        patch_width: usize,
        patch_height: usize,
    ) {
        assert!(pos.col < self.width, "Non-existent column index");
        assert!(pos.row < self.height, "Non-existent row index");

        assert!(
            pos.col + patch_width <= self.width,
            "Patch exceeds board size"
        );
        assert!(
            pos.row + patch_height <= self.height,
            "Patch exceeds board size"
        );

        for row_patch in 0..patch_height {
            for col_patch in 0..patch_width {
                let pos = CellPosition {
                    col: pos.col + col_patch,
                    row: pos.row + row_patch,
                };
                self.set(pos, patch[row_patch * patch_width + col_patch]);
            }
        }
    }

    pub fn set(&mut self, pos: CellPosition, state: CellState) {
        assert!(pos.col < self.width, "Non-existent column index");
        assert!(pos.row < self.height, "Non-existent row index");
        self.state[pos.row * self.width + pos.col] = state;
    }

    pub fn alive(&self, pos: CellPosition) -> bool {
        assert!(pos.col < self.width, "Non-existent column index");
        assert!(pos.row < self.height, "Non-existent row index");
        self.state[pos.row * self.width + pos.col] == CellState::Alive
    }

    pub fn neighbours(&self, pos: CellPosition) -> Vec<CellPosition> {
        assert!(pos.col < self.width, "Non-existent column index");
        assert!(pos.row < self.height, "Non-existent row index");

        let mut neighbours = Vec::new();
        for row_offset in -1..=1 {
            for col_offset in -1..=1 {
                if row_offset == 0 && col_offset == 0 {
                    continue;
                }

                let row_neighbour = pos.row as isize + row_offset;
                let row_neighbour = if row_neighbour < 0 {
                    self.height - 1
                } else if row_neighbour as usize >= self.height {
                    0
                } else {
                    row_neighbour as usize
                };

                let col_neighbour = pos.col as isize + col_offset;
                let col_neighbour = if col_neighbour < 0 {
                    self.width - 1
                } else if col_neighbour as usize >= self.width {
                    0
                } else {
                    col_neighbour as usize
                };

                neighbours.push(CellPosition {
                    col: col_neighbour,
                    row: row_neighbour,
                });
            }
        }
        neighbours
    }
}
