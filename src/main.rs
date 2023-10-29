use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn hello_world() {
    println!("Hello world!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Tony".to_string())));
    commands.spawn((Person, Name("Akari".to_string())));
    commands.spawn((Person, Name("Tsukasa".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>){
    for name in &query {
        println!("hello {}!", name.0)
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}
