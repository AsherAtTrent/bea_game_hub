use bevy::prelude::*;

#[derive(Resource)]
pub struct GreetTimer(Timer);
#[derive(Component)]
pub struct Person;
#[derive(Component)]
pub struct Name(String);



pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}


pub fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Beatriz Lourenco matias".to_string())));
    commands.spawn((Person, Name("Asher Ali".to_string())));
}

pub fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0)
        }
    }
}