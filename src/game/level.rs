use bevy::prelude::*;

use super::*;

pub struct GameLevelPlugin;

impl Plugin for GameLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, (spawn_stage, spawn_guard, spawn_sneaker));
    }
}

fn spawn_stage(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // floor plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(120.0, 50.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        })),
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 5., 20.))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: SANDY_BROWN.into(),
            emissive: LinearRgba::new(0.0, 0.0, 0.0, 0.9),
            ..default()
        })),
        Transform::from_xyz(5.0, 2.5, 0.0),
        Obstacle,
        Actor,
    ));

    // fixed light
    commands
        .spawn((
            Actor,
            Transform::from_xyz(10.0, 5.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        .with_child((
            SpotLight {
                intensity: LIGHT_NORMAL,
                color: WHITE.into(),
                shadows_enabled: true,
                inner_angle: PI / 4.0,
                outer_angle: PI / 3.0,
                range: 12.5,
                radius: 0.07,
                ..default()
            },
            Sector {
                angle: PI / 3.0,
                max_distance: 10.0,
                min_distance: 0.0,
            },
            LightSector,
        ));
}

fn spawn_guard(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Guard,
            Transform::from_xyz(1.0, 1.0, 5.0).looking_at(Vec3::new(5.0, 1.0, 0.0), Vec3::Y),
        ))
        .with_child((
            SpotLight {
                intensity: LIGHT_FLASHLIGHT,
                color: WHITE.into(),
                shadows_enabled: true,
                inner_angle: PI / 8.0,
                outer_angle: PI / 4.0,
                range: 10.0,
                radius: 0.07,
                ..default()
            },
            Sector {
                angle: PI / 4.0,
                max_distance: 8.0,
                min_distance: 0.0,
            },
            LightSector,
        ))
        .with_child((
            Mesh3d(meshes.add(Capsule3d::new(0.5, 2.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: LIME.into(),
                emissive: LinearRgba::new(10.0, 0.0, 1.0, 0.9),
                ..default()
            })),
            Sector {
                angle: PI / 3.0,
                max_distance: 20.0,
                min_distance: 1.0,
            },
            VisionSector,
            // debug
            // SpotLight {
            //     intensity: LIGHT_STRONG,
            //     color: GREEN.into(),
            //     // shadows_enabled: true,
            //     inner_angle: PI / 3.0,
            //     outer_angle: PI / 3.0,
            //     range: 20.0,
            //     radius: 0.07,
            //     ..default()
            // },
        ));
}

fn spawn_sneaker(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((Sneaker, Player, Transform::from_xyz(0.0, 1.0, 0.0)))
        .with_child((
            Mesh3d(meshes.add(Capsule3d::new(0.5, 2.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: LIME.into(),
                emissive: LinearRgba::new(10.0, 0.0, 1.0, 0.9),
                ..default()
            })),
        ));
}
