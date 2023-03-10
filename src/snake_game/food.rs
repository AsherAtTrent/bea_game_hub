use bevy::prelude::*;
use rand::random;

//use super::Position;

pub const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

pub fn food_spawner(
    mut commands: Commands,
    segment_position: Query<&super::Position, With<super::SnakeSegment>>,
    food_position: Query<&super::Position, With<super::Food>>,
) {
    let mut positions: Vec<super::Position> = Vec::new();
    for x in 0..super::ARENA_WIDTH {
        for y in 0..super::ARENA_HEIGHT {
            let mut occupied = false;
            for pos in segment_position.iter() {
                if pos.x == x as i32 && pos.y == y as i32 {
                    occupied = true;
                }
            }
            for pos in food_position.iter() {
                if pos.x == x as i32 && pos.y == y as i32 {
                    occupied = true;
                }
            }
            if !occupied {
                positions.push(super::Position {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    let random_position = positions[random::<usize>() % positions.len()];
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(super::Food)
        .insert(super::Position {
            x: random_position.x,
            y: random_position.y,
        })
        .insert(super::Size::square(0.8));
}
