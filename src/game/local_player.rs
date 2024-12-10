use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity;
// use bevy_rapier3d::prelude::*;

use super::*;

pub struct LocalPlayerPlugin;

impl Plugin for LocalPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (input_movement,));
    }
}

fn input_movement(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player: Single<(Entity, &mut Transform), With<Player>>,
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

    if direction != Vec3::ZERO {
        let (entity, mut transform) = player.into_inner();
        transform.translation += direction.normalize() * time.delta_secs() * 10.0;
        // let next_potision = transform.translation() + direction.normalize();
        // commands.entity(entity).insert(Velocity {
        //     linvel: direction.normalize(),
        //     ..Default::default()
        // });
        // .insert(ExternalForce {
        //     force: direction.normalize(),
        //     ..Default::default()
        // });
    }
}
