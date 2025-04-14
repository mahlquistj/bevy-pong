use super::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn name(&self) -> &str {
        "Players"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_player)
            .add_systems(Update, (move_cpu, move_player));
    }
}

#[derive(Component)]
pub struct Cpu;

#[derive(Component)]
pub struct Player;

fn add_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = meshes.add(Rectangle::new(PLAYER_WIDTH, PLAYER_HEIGHT));
    let color = materials.add(Color::WHITE);
    commands.spawn((
        Player,
        Mesh2d(shape),
        MeshMaterial2d(color),
        Transform::from_xyz(WIN_X_START + PLAYER_WIDTH, 0.0, 0.0),
    ));

    let shape = meshes.add(Rectangle::new(PLAYER_WIDTH, PLAYER_HEIGHT));
    let color = materials.add(Color::BLACK);
    commands.spawn((
        Cpu,
        Mesh2d(shape),
        MeshMaterial2d(color),
        Transform::from_xyz(WIN_X_END - 10.0, 0.0, 0.0),
    ));
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut transform: Single<&mut Transform, With<Player>>,
) {
    if keys.pressed(KeyCode::KeyW) {
        let up = transform.up();
        transform.translation += up * PLAYER_SPEED * time.delta_secs();
    }

    if keys.pressed(KeyCode::KeyS) {
        let down = transform.down();
        transform.translation += down * PLAYER_SPEED * time.delta_secs();
    }
}

fn move_cpu(time: Res<Time>, mut transform: Single<&mut Transform, With<Cpu>>) {
    // Do nothing for now
}
