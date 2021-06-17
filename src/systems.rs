use crate::{
    components::{CellPosition, CellState},
    resources::{BoardCycleEvent, CellBoard, CellEntityMap, CellSize, ColorHandleMap, CycleTimer},
};
use bevy::prelude::*;

pub fn life_setup(
    mut commands: Commands,
    mut cell_entities: ResMut<CellEntityMap>,
    mut color_handles: ResMut<ColorHandleMap>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board: Res<CellBoard>,
    cell_size: Res<CellSize>,
    window: Res<WindowDescriptor>,
) {
    color_handles.0.insert(
        "white".to_string(),
        materials.add(Color::rgb_u8(255, 255, 255).into()),
    );

    // Camera entity
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // Background entity
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb_u8(0, 0, 0).into()),
        sprite: Sprite::new(Vec2::new(window.width, window.height)),
        ..Default::default()
    });

    for row in 0..board.height {
        for col in 0..board.width {
            let pos = CellPosition { col, row };
            if board.alive(pos) {
                let x =
                    (-window.width / 2.0) + (col as f32 * cell_size.width + cell_size.width / 2.0);
                let y = (window.height / 2.0)
                    - (row as f32 * cell_size.height + cell_size.height / 2.0);

                // Cell entity
                let cell_new = commands
                    .spawn_bundle(SpriteBundle {
                        material: color_handles.0.get("white").unwrap().clone(),
                        sprite: Sprite::new(Vec2::new(cell_size.width, cell_size.height)),
                        transform: Transform::from_xyz(x, y, 0.0),
                        ..Default::default()
                    })
                    .id();
                cell_entities.0.insert(pos, cell_new);
            }
        }
    }
}

pub fn apply_life_cycle_rules(
    mut cycle_events: EventWriter<BoardCycleEvent>,
    mut board: ResMut<CellBoard>,
    mut cycle_timer: ResMut<CycleTimer>,
    time: Res<Time>,
) {
    if cycle_timer.0.tick(time.delta()).finished() {
        let delta: Vec<_> = (0..board.height)
            .flat_map(|row| (0..board.width).map(move |col| CellPosition { col, row }))
            .filter_map(|pos| {
                let n_alive_neighbours: usize = board
                    .neighbours(pos)
                    .into_iter()
                    .filter(|p| board.alive(*p))
                    .count();

                // RULES (https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life#Rules)
                // 1. Any live cell with two or three live neighbours survives.
                // 2. Any dead cell with three live neighbours becomes a live cell.
                // 3. All other live cells die in the next generation. Similarly, all other dead
                // cells stay dead.

                let is_alive = board.alive(pos);
                let can_live = is_alive && (n_alive_neighbours == 2 || n_alive_neighbours == 3);
                let can_revive = !is_alive && n_alive_neighbours == 3;

                if (can_live || can_revive) && !is_alive {
                    return Some((pos, CellState::Alive));
                }
                if !(can_live || can_revive) && is_alive {
                    return Some((pos, CellState::Dead));
                }
                None
            })
            .collect();

        delta.iter().for_each(|&(pos, state)| {
            board.set(pos, state);
        });
        cycle_events.send(BoardCycleEvent { delta });
    }
}

pub fn follow_life_cycle_rules(
    mut commands: Commands,
    mut cycle_events: EventReader<BoardCycleEvent>,
    mut cell_entities: ResMut<CellEntityMap>,
    cell_size: Res<CellSize>,
    color_handles: Res<ColorHandleMap>,
    window: Res<WindowDescriptor>,
) {
    for evt in cycle_events.iter() {
        for (pos, state) in &evt.delta {
            let cell_old = match state {
                CellState::Dead => cell_entities.0.remove(pos),
                CellState::Alive => {
                    let x = (-window.width / 2.0)
                        + (pos.col as f32 * cell_size.width + cell_size.width / 2.0);
                    let y = (window.height / 2.0)
                        - (pos.row as f32 * cell_size.height + cell_size.height / 2.0);

                    // Cell entity
                    let cell_new = commands
                        .spawn_bundle(SpriteBundle {
                            material: color_handles.0.get("white").unwrap().clone(),
                            sprite: Sprite::new(Vec2::new(cell_size.width, cell_size.height)),
                            transform: Transform::from_xyz(x, y, 0.0),
                            ..Default::default()
                        })
                        .id();
                    cell_entities.0.insert(*pos, cell_new)
                }
            };

            if let Some(entt) = cell_old {
                commands.entity(entt).despawn();
            }
        }
    }
}
