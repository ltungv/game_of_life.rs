use bevy::prelude::*;
use game_of_life::{
    components::CellPosition, resources::CellBoard, state_from_file, LifePlugin, WINDOW_SIZE,
};
use std::time::Duration;

fn main() {
    let state_path: String = std::env::args().skip(1).take(1).collect();
    let (state, (width, height)) = state_from_file(state_path).unwrap();
    let mut init_board = CellBoard::new(width, height);
    init_board.patch(CellPosition(0, 0), &state, width, height);

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
            init_board,
            cycle_interval: Duration::from_millis(40),
        })
        .run();
}
