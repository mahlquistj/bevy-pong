use bevy::{prelude::*, sprite::Wireframe2dPlugin, window::WindowResolution};

mod arena;
mod ball;
mod paddle;

// Since the game window size is fixed, we can calculate some const values
const WIN_HEIGHT: f32 = 400.0;
const WIN_WIDTH: f32 = 800.0;

const WIN_Y_END: f32 = WIN_HEIGHT / 2.0;
const WIN_X_END: f32 = WIN_WIDTH / 2.0;

const WIN_Y_START: f32 = -WIN_Y_END;
const WIN_X_START: f32 = -WIN_X_END;

const PLAYER_HEIGHT: f32 = WIN_HEIGHT / 4.0;
const PLAYER_WIDTH: f32 = WIN_WIDTH / 100.0;
const PLAYER_SPEED: f32 = 100.0;

const BALL_INIT_SPEED: f32 = 100.0;
const BALL_SPEEDUP_FACTOR: f32 = 1.05;

/// Describes anything than can collide with the ball
#[derive(Component)]
struct Collider;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    let mut app = App::new();

    log::info!("[APP] Adding bevy plugins");
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(WIN_WIDTH, WIN_HEIGHT),
            resizable: false,
            title: "Pong".to_string(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(Wireframe2dPlugin);

    log::info!("[APP] Adding setup");
    app.add_systems(Startup, setup);

    log::info!("[APP] Adding custom plugins");
    app.add_plugins(arena::ArenaPlugin)
        .add_plugins(paddle::PaddlePlugin)
        .add_plugins(ball::BallPlugin);

    log::info!("App built");

    app.run();
}
