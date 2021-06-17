use bevy::prelude::*;
use game_of_life::{state_from_file, LifePlugin};
use std::{path::PathBuf, time::Duration};
use structopt::StructOpt;

const WINDOW_SIZE: (f32, f32) = (600.0, 600.0);

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
    let cycle_interval = Duration::from_millis(cli_opt.interval_duration_ms.unwrap_or(40));
    let init_state = cli_opt.init_state_path.map(|p| {
        let (state, board_size) = state_from_file(p).unwrap();
        let state = state.into_iter().collect();
        (state, board_size)
    });

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
            init_state,
            cycle_interval,
        })
        .run();
}
