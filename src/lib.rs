use bevy::prelude::*;
use std::{path::Path, time::Duration};

pub mod components;
pub mod resources;
pub mod systems;

use self::{
    resources::{BoardCycleEvent, CellBoard, CellEntityMap, CellSize, ColorHandleMap, CycleTimer},
    systems::{cell_entities_update, cell_life_cycle, life_setup},
};

pub const WINDOW_SIZE: (f32, f32) = (600.0, 600.0);

pub struct LifePlugin {
    pub init_board: CellBoard,
    pub cycle_interval: Duration,
}

impl Plugin for LifePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<BoardCycleEvent>()
            .insert_resource(self.init_board.clone())
            .insert_resource(ColorHandleMap::new())
            .insert_resource(CellEntityMap::new())
            .insert_resource(CycleTimer(Timer::new(self.cycle_interval, true)))
            .insert_resource(CellSize(
                WINDOW_SIZE.0 / self.init_board.width as f32,
                WINDOW_SIZE.1 / self.init_board.height as f32,
            ))
            .add_startup_system(life_setup.system())
            .add_system(cell_life_cycle.system())
            .add_system(cell_entities_update.system());
    }
}

pub fn state_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<(Vec<bool>, (usize, usize))> {
    let mut state = Vec::new();
    let mut width = 0;
    let mut height = 0;

    for line in std::fs::read_to_string(path)?.lines() {
        let mut width_current = 0;
        for c in line.chars() {
            match c {
                'X' => state.push(true),
                '-' => state.push(false),
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
