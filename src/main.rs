//! Hello

use bevy::prelude::*;

struct Person;
struct Name(String);

fn add_ppl(commands: &mut Commands) {
    commands
        .spawn((Person, Name("Elaina Proctor".into())))
        .spawn((Person, Name("Renzo Hume".into())))
        .spawn((Person, Name("Zayna Nieves".into())));
}

struct GreetTimer(Timer);
fn greet_ppl(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}

struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_ppl.system())
            .add_system(greet_ppl.system());
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
