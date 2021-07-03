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

struct CountUp(Timer, f32, i32);

fn update_counters(time: Res<Time>, mut count_up: ResMut<CountUp>) {
    count_up.0.tick(time.delta_seconds());
    if count_up.0.just_finished() {
        println!("ticking counter ({})", count_up.2);
        count_up.2 += 1;
        count_up.0 = Timer::from_seconds(1.0 / count_up.1, true);
    }
}

struct CounterPlugin;

impl Plugin for CounterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(CountUp(Timer::from_seconds(1.0, true), 2.5, 0))
            .add_system(update_counters.system());
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_plugin(CounterPlugin)
        .run();
}
