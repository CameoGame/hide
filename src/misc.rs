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
            ..Default::default()
        },
        Transform::from_xyz(0.0, 20., 0.0).looking_at(Vec3::ZERO, Vec3::X),
    ));
}
