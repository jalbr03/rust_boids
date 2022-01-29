mod systems;
mod comp;
mod boids;
mod walls;
mod general;

use bevy::prelude::*;
use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
// use crate::comp::BoidWorld;

static WALL_SIZE: f32 = 10.0;

static BOID_SPEED: f32 = 2.0;
static ALIGNMENT: f32 = 1.0;
static COHESION: f32 = 0.05;
static SEPARATION: f32 = 10.0;
static WALL_AVOIDANCE: f32 = 80.0;

static BOID_COUNT: i32 = 100;
static WINDOW_WIDTH: f32 = 1900.0;
static WINDOW_HEIGHT: f32 = 1060.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())

        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Time::default())

        .add_startup_system(systems::startup)
        .add_startup_system(systems::resize_window)
        .add_system(systems::update)
        .add_system(systems::place_wall_system)
        .add_system(bevy::input::system::exit_on_esc_system)

        .run();
}