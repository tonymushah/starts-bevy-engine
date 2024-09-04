use bevy::prelude::*;

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(String);

fn add_person(mut cmd: Commands) {
    cmd.spawn_batch(vec![
        (Person, Name("Tony".into())),
        (Person, Name("Tomefy".into())),
        (Person, Name("Kanto".into())),
        (Person, Name("Tendry".into())),
    ])
}

fn fetch_named_person(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("getted person name: {}", name.0);
    }
}

pub struct BasicECS;

impl Plugin for BasicECS {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, add_person)
            .add_systems(Update, fetch_named_person);
    }
}
