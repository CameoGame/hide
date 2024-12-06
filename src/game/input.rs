use bevy::prelude::*;

use super::*;

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (input_movement,));
    }
}

fn input_movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in &mut query {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::ArrowUp) {
            direction.x += 1.0;
        }
        if input.pressed(KeyCode::ArrowDown) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::ArrowLeft) {
            direction.z -= 1.0;
        }
        if input.pressed(KeyCode::ArrowRight) {
            direction.z += 1.0;
        }

        if direction != Vec3::ZERO {
            transform.translation += time.delta_secs() * 3.0 * direction.normalize();
        }
    }
}
