use bevy::prelude::*;

pub(super) struct MiscPlugin;

impl Plugin for MiscPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 1,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 40., 0.0).looking_at(Vec3::ZERO, Vec3::X),
    ));

    commands.spawn((
        Camera2d,
        Camera {
            order: 10,
            ..Default::default()
        },
    ));
}
