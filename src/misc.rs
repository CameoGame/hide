use bevy::prelude::*;

pub(super) struct MiscPlugin;

impl Plugin for MiscPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_seconds(0.01))
            .add_systems(Startup, setup_camera);
    }
}

#[derive(Component)]
pub struct PlayerCamera;

/// Camera
/// North is X+ while East is Z+.
/// Up is Y+
fn setup_camera(mut commands: Commands) {
    commands
        .spawn((
            PlayerCamera,
            Camera3d::default(),
            Camera {
                order: 1,
                ..Default::default()
            },
            Transform::from_xyz(0.0, 10., 0.0).looking_at(Vec3::ZERO, Vec3::X),
            // Transform::from_xyz(-20.0, 20., 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            SpotLight {
            intensity: 1000000.,
            color: Color::Srgba(Srgba::rgb(1.0, 1.0, 1.0)),
            shadows_enabled: true,
            inner_angle: 0.05,
            outer_angle: 0.1,
            range: 20.0,
            radius: 0.07,
            ..default()
        }
        ));

    commands.spawn((
        Camera2d,
        Camera {
            order: 10,
            ..Default::default()
        },
    ));
}
