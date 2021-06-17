use bevy::prelude::*;
use game_of_life::{components::CellState, resources::CellBoard, state_from_file, LifePlugin};
use std::{path::PathBuf, time::Duration};
use structopt::StructOpt;

const WINDOW_SIZE: (f32, f32) = (600.0, 600.0);

const DEFAULT_BOARD_STATE: ([CellState; 40 * 40], (usize, usize)) =
    ([CellState::Dead; 40 * 40], (40, 40));

/// It's a game of life!
#[derive(StructOpt)]
#[structopt()]
struct CliOpt {
    /// Duration (in milliseconds) to wait before moving to the next generation
    #[structopt(name = "Cycle interval", short = "t", long = "interval")]
    interval_duration_ms: Option<u64>,

    /// Path to the file contains the initial state
    #[structopt(name = "Initial state", short = "s", long = "state")]
    init_state_path: Option<PathBuf>,
}

fn main() {
    let cli_opt = CliOpt::from_args();
    let interval_duration_ms = cli_opt.interval_duration_ms.unwrap_or(40);
    let (state, (width, height)) = match cli_opt.init_state_path {
        None => (Vec::from(DEFAULT_BOARD_STATE.0), DEFAULT_BOARD_STATE.1),
        Some(p) => {
            let (state, board_size) = state_from_file(p).unwrap();
            let state = state
                .into_iter()
                .map(|b| if b { CellState::Alive } else { CellState::Dead })
                .collect();
            (state, board_size)
        }
    };

    App::build()
        .insert_resource(WindowDescriptor {
            title: "Nondeterministic Life".to_string(),
            width: WINDOW_SIZE.0,
            height: WINDOW_SIZE.1,
            resizable: false,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LifePlugin {
            init_board: CellBoard::new(width, height, state),
            cycle_interval: Duration::from_millis(interval_duration_ms),
        })
        .run();
}
