use super::*;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn name(&self) -> &str {
        "Arena"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_walls);
    }
}

#[derive(Component)]
pub struct Wall;

fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = meshes.add(Rectangle::default());
    let color = materials.add(Color::WHITE);
    commands.spawn((
        crate::Collider,
        Wall,
        Mesh2d(shape),
        MeshMaterial2d(color),
        Transform::from_xyz(0.0, WIN_Y_START, 0.0).with_scale(Vec3::new(
            WIN_WIDTH,
            PLAYER_WIDTH,
            0.0,
        )),
    ));

    let shape = meshes.add(Rectangle::default());
    let color = materials.add(Color::WHITE);
    commands.spawn((
        crate::Collider,
        Wall,
        Mesh2d(shape),
        MeshMaterial2d(color),
        Transform::from_xyz(0.0, WIN_Y_END, 0.0).with_scale(Vec3::new(
            WIN_WIDTH,
            PLAYER_WIDTH,
            0.0,
        )),
    ));
}
