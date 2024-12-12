use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::*;

pub struct LocalPlayerPlugin;

impl Plugin for LocalPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (input_movement, debug));
    }
}

fn input_movement(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    _time: Res<Time>,
    player: Single<Entity, With<Player>>,
) {
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

    let entity = player.into_inner();
    if direction != Vec3::ZERO {
        commands.entity(entity).insert(ExternalForce {
            force: direction.normalize() * 15.0,
            ..Default::default()
        });
    } else {
        commands.entity(entity).insert(ExternalForce::default());
    }
}

fn debug(player: Single<(&GlobalTransform,), With<Player>>, mut text: Single<&mut Text>) {
    let (g_translation,) = player.into_inner();
    text.0 = format!("{}", g_translation.translation());
}
