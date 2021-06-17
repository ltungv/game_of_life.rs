use bevy::prelude::*;
use game_of_life::{state_from_file, LifePlugin};
use std::{path::PathBuf, time::Duration};
use structopt::StructOpt;

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
            width: cli_opt.window_width,
            height: cli_opt.window_height,
            resizable: false,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LifePlugin {
            cycle_interval,
            init_state,
            board_width: cli_opt.cols,
            board_height: cli_opt.rows,
        })
        .run();
}

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

    /// Width of the game's window
    #[structopt(
        name = "Window's width",
        short = "w",
        long = "width",
        default_value = "600.0"
    )]
    window_width: f32,

    /// Height of the game's window
    #[structopt(
        name = "Window's height",
        short = "h",
        long = "height",
        default_value = "600.0"
    )]
    window_height: f32,

    #[structopt(
        name = "Number of rows",
        short = "r",
        long = "rows",
        default_value = "30"
    )]
    rows: usize,

    #[structopt(
        name = "Number of columns",
        short = "c",
        long = "cols",
        default_value = "30"
    )]
    cols: usize,
}
