use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::misc::PlayerCamera;

use super::*;

pub struct LocalPlayerPlugin;

impl Plugin for LocalPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, input_movement);

        app.add_systems(Update, debug);
        app.add_systems(FixedUpdate, camera_follow);
    }
}

fn input_movement(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    player: Single<&mut KinematicCharacterController, With<Player>>,
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

    let mut controller = player.into_inner();
    if direction != Vec3::ZERO {
        direction = direction.normalize();
    }
    direction = direction.with_y(-1.0) * time.delta_secs() * 6.0;

    controller.translation = if let Some(trans) = controller.translation {
        Some(trans + direction)
    } else {
        Some(direction)
    };
}

// fn read_result_system(
//     time: Res<Time>,
//     mut controllers: Query<(&KinematicCharacterControllerOutput, &mut Velocity), With<Player>>,
// ) {
//     for (output, mut velocity) in controllers.iter_mut() {
//         if output.effective_translation != Vec3::ZERO {
//             println!(
//                 "Entity  moved by {:?} and touches the ground: {:?}",
//                 output.effective_translation, output.grounded
//             );

//             if !output.grounded {
//                 velocity.linvel.y -= 9.8 * time.delta_secs();
//             } else {
//                 velocity.linvel.y = 0.0;
//             }
//         }
//     }
// }

fn debug(
    player: Single<(&GlobalTransform, Option<&UnderLight>), With<Player>>,
    mut texts: Query<(&mut Text, &DebugText)>,
) {
    let (g_translation, under_light) = player.into_inner();
    for (mut text, DebugText(id)) in &mut texts {
        if *id == 0 {
            text.0 = format!(
                "trans: [x={:.6}, y={:.6}, z={:.6}]\nv: [x={:.6}, y={:.6}, z={:.6}]",
                g_translation.translation().x,
                g_translation.translation().y,
                g_translation.translation().z,
                0,
                0,
                0
            );
        } else if *id == 1 {
            text.0 = format!("{:?}", under_light);
        }
    }
}

fn camera_follow(
    player: Single<&GlobalTransform, With<Player>>,
    camera: Single<&mut Transform, With<PlayerCamera>>,
) {
    let g_trans = player.into_inner();
    let mut camera_trans = camera.into_inner();
    camera_trans.translation = g_trans.translation();
    camera_trans.translation.y += 10.0;
}
