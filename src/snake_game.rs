use bevy::{prelude::*, time::FixedTimestep};

mod food;
mod snake;

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

// A unit struct to help identify the score Text component
#[derive(Component)]
struct Score;

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Default, Deref, DerefMut, Resource)]
pub struct SnakeSegments(Vec<Entity>);

#[derive(Default, Resource)]
pub struct LastTailPosition(Option<Position>);

#[derive(Copy, PartialEq, Eq, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
#[derive(Component)]
pub struct SnakeHead {
    direction: Direction,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: i32,
    y: i32,
}
#[derive(Component)]
pub struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component)]
pub struct Food;

pub struct GrowthEvent;

struct GameOverEvent;

pub struct SnakeGamePlugin;

impl Plugin for SnakeGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_startup_system(snake::spawn_snake)
            .add_event::<GrowthEvent>()
            .add_event::<GameOverEvent>()
            .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
            .insert_resource(SnakeSegments::default())
            .insert_resource(LastTailPosition::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(food::food_spawner),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.150))
                    .with_system(snake_movement)
                    .with_system(snake::snake_eating.after(snake_movement))
                    .with_system(snake::snake_growth.after(snake::snake_eating)),
            )
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                window: WindowDescriptor {
                    title: "Snake!".to_string(),
                    width: 500.0,
                    height: 500.0,
                    ..default()
                },
                ..default()
            }))
            .add_system(snake_movement_input.before(snake_movement))
            .add_system(game_over.after(snake_movement))
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new().with_system(score).with_system(fps_counter),
            )
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new()
                    .with_system(position_translation)
                    .with_system(size_scaling),
            );
    }
}

fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => head_pos.x -= 1,
            Direction::Right => head_pos.x += 1,
            Direction::Up => head_pos.y += 1,
            Direction::Down => head_pos.y -= 1,
        };
        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= ARENA_WIDTH
            || head_pos.y as u32 >= ARENA_HEIGHT
        {
            game_over_writer.send(GameOverEvent);
        }
        if segment_positions.contains(&head_pos) {
            game_over_writer.send(GameOverEvent);
        }
        segment_positions
            .iter()
            .zip(segments.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
        *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));
    }
}

fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
            Direction::Up
        } else if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
            Direction::Down
        } else if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
            Direction::Right
        } else if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
            Direction::Left
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

//create a function that shows the fps at the top left of the screen
fn fps_counter(mut commands: Commands, time: Res<Time>, asset_server: Res<AssetServer>) {
    let fps = 1.0 / time.delta_seconds();
    commands.spawn(TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: format!("FPS: {:.2}", fps),
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            }],
            alignment: TextAlignment::TOP_LEFT,
        },
        ..Default::default()
    });
}

pub fn score(
    mut commands: Commands,
    segments: Query<Entity, With<SnakeSegment>>,
    asset_server: Res<AssetServer>,
) {
    let score: usize = segments.iter().count();

    commands.spawn(TextBundle {
        style: Style {
            align_self: AlignSelf::Auto,
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text {
            sections: vec![TextSection {
                value: format!("Score: {}", score),
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            }],
            alignment: TextAlignment::CENTER,
        },
        ..Default::default()
    });
}

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        snake::spawn_snake(commands, segments_res);
    }
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary();
    if let Some(window) = &window {
        for (sprite_size, mut transform) in q.iter_mut() {
            transform.scale = Vec3::new(
                sprite_size.width / ARENA_WIDTH as f32 * window.width(),
                sprite_size.height / ARENA_HEIGHT as f32 * window.height(),
                1.0,
            );
        }
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary();
    if let Some(window) = &window {
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                convert(pos.x as f32, window.width(), ARENA_WIDTH as f32),
                convert(pos.y as f32, window.height(), ARENA_HEIGHT as f32),
                0.0,
            );
        }
    }
}
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
