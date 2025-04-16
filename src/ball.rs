use super::*;
use crate::rng::FloatRng;
use bevy::math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume};

pub struct BallPlugin;

#[derive(Default, Event)]
pub struct CollisionEvent;

#[derive(Default, Event)]
pub struct OutOfBoundsEvent;

impl Plugin for BallPlugin {
    fn name(&self) -> &str {
        "Ball"
    }

    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_event::<OutOfBoundsEvent>()
            .add_systems(Startup, spawn_ball)
            .add_systems(Update, (apply_velocity, check_collisions).chain());
    }
}

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rng: GlobalEntropy<WyRand>,
) {
    let shape = meshes.add(Circle::new(PLAYER_WIDTH));
    let color = materials.add(Color::WHITE);

    let initial_x_speed = BALL_INIT_SPEED;
    let initial_y_speed = BALL_INIT_SPEED * rng.next_percentage_clamped(0.75..=1.0);

    let velocity = Vec2::new(-initial_x_speed, initial_y_speed);

    commands.spawn((
        Ball,
        Velocity(velocity),
        Mesh2d(shape),
        MeshMaterial2d(color),
        Transform::from_xyz(0.0, rng.next_f32_range(WIN_Y_START..=WIN_Y_END), 0.0),
    ));
}

fn apply_velocity(query: Single<(&mut Transform, &Velocity), With<Ball>>, time: Res<Time>) {
    let (mut transform, velocity) = query.into_inner();
    transform.translation.x += velocity.x * time.delta_secs();
    transform.translation.y += velocity.y * time.delta_secs();
}

fn check_collisions(
    ball: Single<(&Transform, &mut Velocity), With<Ball>>,
    colliders: Query<(&Transform, Option<&crate::paddle::Paddle>), With<crate::Collider>>,
    mut col_events: EventWriter<CollisionEvent>,
    mut rng: GlobalEntropy<WyRand>,
) {
    let (ball_transform, mut ball_velocity) = ball.into_inner();

    // Check for collisions
    let ball_bounds = BoundingCircle::new(ball_transform.translation.truncate(), PLAYER_WIDTH / 2.);
    for (collision, maybe_paddle) in colliders
        .iter()
        .filter_map(|(c_trans, paddle)| ball_collision(&ball_bounds, c_trans).map(|c| (c, paddle)))
    {
        // Send Collision event to use for sounds
        col_events.send_default();

        // Invert x/y velocity on wall collisions
        match collision {
            Collision::Top | Collision::Bottom => ball_velocity.y *= -1.0,
            Collision::Left | Collision::Right => ball_velocity.x *= -1.0,
        }

        // Update ball velocity on player collisions
        if maybe_paddle.is_some() {
            ball_velocity.0 *= 1.0 + rng.next_percentage();
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

// FROM BEVY BREAKOUT EXAMPLE
// Returns `Some` if `ball` collides with `bounding_box`.
// The returned `Collision` is the side of `bounding_box` that `ball` hit.
fn ball_collision(ball_bounds: &BoundingCircle, collider: &Transform) -> Option<Collision> {
    let hitbox = Aabb2d::new(
        collider.translation.truncate(),
        collider.scale.truncate() / 2.,
    );

    if !ball_bounds.intersects(&hitbox) {
        return None;
    }

    let closest = hitbox.closest_point(ball_bounds.center());
    let offset = ball_bounds.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}
