use bevy::app::App;
use bevy::prelude::*;

#[derive(Component)]
struct Oulvan;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Level(u8);

fn add_people(mut commands: Commands) {
    commands.spawn((Oulvan, Name("Elaina Proctor".to_string()), Level(1)));
    commands.spawn((Oulvan, Name("Renzo Hume".to_string()), Level(2)));
    commands.spawn((Oulvan, Name("Zanya Nieves".to_string()), Level(3)));
}

fn greet_people(query: Query<&Name, With<Oulvan>>) {
    for name in &query {
        println!("hello {}", name.0);
    }
}

fn hello_world() {
    println!("Hello world!");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_people)
        .add_system(hello_world)
        .add_system(greet_people)
        .run();
}
