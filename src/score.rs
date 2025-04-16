use super::*;

#[derive(Component, Default)]
pub struct Score(usize);

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn name(&self) -> &str {
        "Score"
    }

    fn build(&self, app: &mut App) {}
}

fn spawn_score(mut commands: Commands) {
    commands.spawn(bundle)
}

fn increment_score() {}
