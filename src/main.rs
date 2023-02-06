#![windows_subsystem = "windows"]
use bevy::prelude::*;
mod snake_game;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_system(input_test)
        .add_system(input_test2)
        .add_plugin(snake_game::SnakeGamePlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn input_test(input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        println!("Space pressed");
    }
}
//print any key pressed
fn input_test2(input: Res<Input<KeyCode>>) {
    for key in input.get_just_pressed() {
        println!("{:?} pressed", key);
    }
}
