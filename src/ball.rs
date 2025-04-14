use bevy::ui::update;

use super::*;

pub struct BallPlugin;

#[derive(Event)]
pub struct CollisionEvent;

#[derive(Event)]
pub struct OutOfBounds;

impl Plugin for BallPlugin {
    fn name(&self) -> &str {
        "Ball"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
            .add_systems(Update, update_ball);
    }
}

#[derive(Component)]
pub struct Ball {
    velocity: Vec2,
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = meshes.add(Circle::new(30.0));
    let color = materials.add(Color::WHITE);
    let velocity = Vec2::new(BALL_INIT_SPEED, BALL_INIT_SPEED);

    commands.spawn((
        Ball { velocity },
        Mesh2d(shape),
        MeshMaterial2d(color),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn update_ball(
    ball: Single<&Transform, With<Ball>>,
    collisions: EventWriter<CollisionEvent>,
    out_of_bounds: EventWriter<OutOfBounds>,
) {
    // Check for collisions
    // Invert x/y velocity on wall collisions
    // Update ball velocity on player collisions
    // Send Collision event to use for sounds
    // Send OOB event if valid
}
