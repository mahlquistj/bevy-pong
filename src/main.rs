use bevy::{prelude::*, sprite::Wireframe2dPlugin, window::WindowResolution};

const WIN_HEIGHT: f32 = 400.0;
const WIN_WIDTH: f32 = 800.0;

const WIN_Y_END: f32 = WIN_HEIGHT / 2.0;
const WIN_X_END: f32 = WIN_WIDTH / 2.0;

const WIN_Y_START: f32 = -WIN_Y_END;
const WIN_X_START: f32 = -WIN_X_END;

const PLAYER_HEIGHT: f32 = WIN_HEIGHT / 4.0;
const PLAYER_WIDTH: f32 = WIN_WIDTH / 100.0;

const MOVE_SPEED: f32 = 100.0;

#[derive(Component)]
pub struct Player {
    name: &'static str,
}

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Ball;

pub struct PlayerPlugin;

fn add_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // PLAYER
    let shape = meshes.add(Rectangle::new(PLAYER_WIDTH, PLAYER_HEIGHT));
    let color = materials.add(Color::WHITE);
    commands.spawn((
        Player { name: "player" },
        Mesh2d(shape),
        MeshMaterial2d(color),
        Transform::from_xyz(WIN_X_START + PLAYER_WIDTH, 0.0, 0.0),
    ));

    // BOT: TBD
    let shape = meshes.add(Rectangle::new(PLAYER_WIDTH, PLAYER_HEIGHT));
    let color = materials.add(Color::BLACK);
    commands.spawn((
        Player { name: "bot" },
        Mesh2d(shape),
        MeshMaterial2d(color),
        Transform::from_xyz(WIN_X_END - 10.0, 0.0, 0.0),
    ));
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Player)>,
) {
    for (mut transform, player) in &mut query {
        if keys.pressed(KeyCode::KeyW) {
            let up = transform.up();
            transform.translation += up * MOVE_SPEED * time.delta_secs();
        }

        if keys.pressed(KeyCode::KeyS) {
            let down = transform.down();
            transform.translation += down * MOVE_SPEED * time.delta_secs();
        }

        println!("{}: {:?}", player.name, transform.translation);
    }
}

impl Plugin for PlayerPlugin {
    fn name(&self) -> &str {
        "Players"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_player)
            .add_systems(Update, move_player);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WIN_WIDTH, WIN_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(Wireframe2dPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
