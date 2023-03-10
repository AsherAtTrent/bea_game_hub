use bevy::prelude::*;

pub const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
pub const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

pub(super) fn spawn_snake(mut commands: Commands, mut segments: ResMut<super::SnakeSegments>) {
    *segments = super::SnakeSegments(vec![
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(10.0, 10.0, 10.0),
                    ..default()
                },
                ..default()
            })
            .insert(super::SnakeHead {
                direction: super::Direction::Up,
            })
            .insert(super::SnakeSegment)
            .insert(super::Position { x: 3, y: 3 })
            .insert(super::Size::square(0.8))
            .id(),
        spawn_segment(commands, super::Position { x: 3, y: 2 }),
    ])
}

pub fn spawn_segment(mut commands: Commands, position: super::Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(super::SnakeSegment)
        .insert(position)
        .insert(super::Size::square(0.8))
        .id()
}

pub fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<super::GrowthEvent>,
    food_positions: Query<(Entity, &super::Position), With<super::Food>>,
    head_positions: Query<&super::Position, With<super::SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (food_entity, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(food_entity).despawn();
                growth_writer.send(super::GrowthEvent)
            }
        }
    }
}

pub fn snake_growth(
    commands: Commands,
    last_tail_position: ResMut<super::LastTailPosition>,
    mut segments: ResMut<super::SnakeSegments>,
    mut growth_reader: EventReader<super::GrowthEvent>,
) {
    if growth_reader.iter().next().is_some() {
        segments.push(spawn_segment(commands, last_tail_position.0.unwrap()))
    }
}
