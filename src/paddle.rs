use super::*;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn name(&self) -> &str {
        "Players"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_paddles)
            .add_systems(Update, (move_cpu, move_player));
    }
}

#[derive(Component)]
pub struct Paddle;

fn add_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = meshes.add(Rectangle::default());
    let color = materials.add(Color::WHITE);
    commands.spawn((
        crate::Collider,
        Player,
        Paddle,
        Mesh2d(shape),
        MeshMaterial2d(color),
        Transform::from_xyz(WIN_X_START + PLAYER_WIDTH, 0.0, 0.0).with_scale(Vec3::new(
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
            0.0,
        )),
    ));

    let shape = meshes.add(Rectangle::default());
    let color = materials.add(Color::BLACK);
    commands.spawn((
        crate::Collider,
        Cpu,
        Paddle,
        Mesh2d(shape),
        MeshMaterial2d(color),
        Transform::from_xyz(WIN_X_END - PLAYER_WIDTH, 0.0, 0.0).with_scale(Vec3::new(
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
            0.0,
        )),
    ));
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut transform: Single<&mut Transform, With<Player>>,
) {
    if keys.pressed(KeyCode::KeyW) {
        let up = transform.up();
        move_direction(&mut transform, up, &time);
    }

    if keys.pressed(KeyCode::KeyS) {
        let down = transform.down();
        move_direction(&mut transform, down, &time);
    }
}

fn move_cpu(
    time: Res<Time>,
    mut transform: Single<&mut Transform, With<Cpu>>,
    ball: Single<&mut Transform, (With<crate::ball::Ball>, Without<Cpu>)>,
) {
    if ball.translation.y > transform.translation.y {
        let up = transform.up();
        move_direction(&mut transform, up, &time);
    }

    if ball.translation.y < transform.translation.y {
        let down = transform.down();
        move_direction(&mut transform, down, &time);
    }
}

#[inline(always)]
fn move_direction(transform: &mut Transform, direction: Dir3, time: &Time) {
    transform.translation += direction * PLAYER_SPEED * time.delta_secs();
}
