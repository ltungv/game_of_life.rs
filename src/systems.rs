use crate::{
    components::{CellPosition, CellState},
    resources::{BoardCycleEvent, CellBoard, CellEntityMap, CellSize, ColorHandleMap, CycleTimer},
    WINDOW_SIZE,
};
use bevy::prelude::*;

pub fn life_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cell_entities: ResMut<CellEntityMap>,
    mut color_handles: ResMut<ColorHandleMap>,
    board: Res<CellBoard>,
    cell_size: Res<CellSize>,
) {
    color_handles.insert(
        "white".to_string(),
        materials.add(Color::rgb_u8(255, 255, 255).into()),
    );

    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // Background
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb_u8(0, 0, 0).into()),
        sprite: Sprite::new(Vec2::new(WINDOW_SIZE.0, WINDOW_SIZE.1)),
        ..Default::default()
    });

    for row in 0..board.height {
        for col in 0..board.width {
            if board.alive(CellPosition(row, col)) {
                let x = (-WINDOW_SIZE.1 / 2.0) + (col as f32 * cell_size.0 + cell_size.0 / 2.0);
                let y = (WINDOW_SIZE.0 / 2.0) - (row as f32 * cell_size.1 + cell_size.1 / 2.0);

                cell_entities.insert(
                    CellPosition(row, col),
                    commands
                        .spawn_bundle(SpriteBundle {
                            material: color_handles.get("white").unwrap().clone(),
                            sprite: Sprite::new(Vec2::new(cell_size.0, cell_size.1)),
                            transform: Transform::from_xyz(x, y, 0.0),
                            ..Default::default()
                        })
                        .id(),
                );
            }
        }
    }
}

pub fn cell_life_cycle(
    mut cycle_events: EventWriter<BoardCycleEvent>,
    mut board: ResMut<CellBoard>,
    mut cycle_timer: ResMut<CycleTimer>,
    time: Res<Time>,
) {
    if cycle_timer.0.tick(time.delta()).finished() {
        let delta = board.cycle();
        cycle_events.send(BoardCycleEvent { delta });
    }
}

pub fn cell_entities_update(
    mut commands: Commands,
    mut cycle_events: EventReader<BoardCycleEvent>,
    mut cell_entities: ResMut<CellEntityMap>,
    color_handles: Res<ColorHandleMap>,
    cell_size: Res<CellSize>,
) {
    for evt in cycle_events.iter() {
        for (pos, state) in &evt.delta {
            match state {
                CellState::Alive => {
                    let x =
                        (-WINDOW_SIZE.1 / 2.0) + (pos.1 as f32 * cell_size.0 + cell_size.0 / 2.0);
                    let y =
                        (WINDOW_SIZE.0 / 2.0) - (pos.0 as f32 * cell_size.1 + cell_size.1 / 2.0);

                    if let Some(entt) = cell_entities.insert(
                        *pos,
                        commands
                            .spawn_bundle(SpriteBundle {
                                material: color_handles.get("white").unwrap().clone(),
                                sprite: Sprite::new(Vec2::new(cell_size.0, cell_size.1)),
                                transform: Transform::from_xyz(x, y, 0.0),
                                ..Default::default()
                            })
                            .id(),
                    ) {
                        commands.entity(entt).despawn();
                    }
                }
                CellState::Dead => {
                    if let Some(entt) = cell_entities.remove(pos) {
                        commands.entity(entt).despawn();
                    }
                }
            }
        }
    }
}
