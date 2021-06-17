use bevy::prelude::*;
use components::{CellPosition, CellState};
use resources::CellBoard;
use std::{path::Path, time::Duration};

mod components;
mod resources;
mod systems;

use self::{
    resources::{BoardCycleEvent, CellEntityMap, CellSize, ColorHandleMap, CycleTimer},
    systems::{apply_life_cycle_rules, follow_life_cycle_rules, life_setup},
};

pub struct LifePlugin {
    pub cycle_interval: Duration,
    pub init_state: Option<(Vec<CellState>, (usize, usize))>,
    pub board_width: usize,
    pub board_height: usize,
}

impl Plugin for LifePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let board = match &self.init_state {
            None => CellBoard::default(),
            Some((state, (width, height))) => {
                let mut board = CellBoard::new(
                    self.board_width,
                    self.board_height,
                    vec![CellState::Dead; self.board_width * self.board_height],
                );
                let pos = CellPosition {
                    col: (self.board_width - width) / 2,
                    row: (self.board_height - height) / 2,
                };
                board.patch(pos, state, *width, *height);
                board
            }
        };
        app.add_event::<BoardCycleEvent>()
            .insert_resource(board)
            .insert_resource(CycleTimer(Timer::new(self.cycle_interval, true)))
            .init_resource::<ColorHandleMap>()
            .init_resource::<CellEntityMap>()
            .init_resource::<CellSize>()
            .add_startup_system(life_setup.system())
            .add_system(apply_life_cycle_rules.system())
            .add_system(follow_life_cycle_rules.system());
    }
}

pub fn state_from_file<P: AsRef<Path>>(
    path: P,
) -> std::io::Result<(Vec<CellState>, (usize, usize))> {
    let mut state = Vec::new();
    let mut width = 0;
    let mut height = 0;

    for line in std::fs::read_to_string(path)?.lines() {
        let mut width_current = 0;
        for c in line.chars() {
            match c {
                'X' => state.push(CellState::Alive),
                '-' => state.push(CellState::Dead),
                _ => panic!("Invalid character"),
            }
            width_current += 1;
        }
        if width != 0 {
            assert_eq!(width_current, width, "Rows' size is not uniform");
        }
        width = width_current;
        height += 1
    }

    Ok((state, (width, height)))
}
