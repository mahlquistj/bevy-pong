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

fn spawn_walls(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let top_shape = meshes.add(Rectangle::new(WIN_WIDTH, PLAYER_WIDTH));
    let bot_shape = meshes.add(Rectangle::new(WIN_WIDTH, PLAYER_WIDTH));

    commands.spawn((
        Wall,
        Mesh2d(top_shape),
        Transform::from_xyz(0.0, WIN_Y_START, 0.0),
    ));

    commands.spawn((
        Wall,
        Mesh2d(bot_shape),
        Transform::from_xyz(0.0, WIN_Y_END, 0.0),
    ));
}
