use bevy::prelude::*;

use super::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostStartup,
            (setup_floor, spawn_stage, spawn_guard, spawn_sneaker),
        );
    }
}

fn setup_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // spawn base floor
    // accessble size is 2000 * 2000
    // the collider is twich on each side because I want it overlap with the air wall.
    commands
        .spawn((
            RigidBody::Fixed,
            Collider::cuboid(2000.0, 1000.0, 2000.0),
            Transform::from_xyz(0., -1000.0, 0.),
            Visibility::Inherited,
        ))
        .with_child((
            Transform::from_xyz(0., 1000.0, 0.),
            Mesh3d(meshes.add(Plane3d::default().mesh().size(2000.0, 2000.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::WHITE,
                perceptual_roughness: 1.0,
                ..default()
            })),
        ));

    // spawn air wall
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(1000.0, 1001.0, 1000.0),
        Transform::from_xyz(2000., -1.0, 0.),
    ));

    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(1000.0, 1001.0, 1000.0),
        Transform::from_xyz(-2000., -1.0, 0.),
    ));

    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(1000.0, 1001.0, 1000.0),
        Transform::from_xyz(0., -1.0, 2000.),
    ));

    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(1000.0, 1001.0, 1000.0),
        Transform::from_xyz(0., -1.0, -2000.),
    ));
}

fn spawn_stage(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands
        .spawn((
            RigidBody::Fixed,
            Collider::cuboid(1.0, 2.5, 10.0),
            Transform::from_xyz(5.0, 2.5, 0.0),
            Obstacle,
            Actor,
        ))
        .with_child((
            Mesh3d(meshes.add(Cuboid::new(2.0, 5., 20.))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: SANDY_BROWN.into(),
                emissive: LinearRgba::new(0.0, 0.0, 0.0, 0.9),
                ..default()
            })),
        ));

    // slope
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(3.0, 3.0, 3.0),
        Transform::from_xyz(-4.0, -3.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Visibility::Inherited,
    ));
    // .with_child((
    // Mesh3d(meshes.add(Cuboid::new(2.0, 5., 20.))),
    // MeshMaterial3d(materials.add(StandardMaterial {
    //     base_color: SANDY_BROWN.into(),
    //     emissive: LinearRgba::new(0.0, 0.0, 0.0, 0.9),
    //     ..default()
    // })),
    // ));

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
            Transform::from_xyz(2.0, 1.2, 5.0).looking_at(Vec3::new(5.0, 1.0, 0.0), Vec3::Y),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
            Collider::capsule_y(0.75, 0.25),
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
            Mesh3d(meshes.add(Capsule3d::new(0.25, 1.5))),
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
        ));
}

fn spawn_sneaker(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Sneaker,
            Player,
            Transform::from_xyz(0.0, 1.2, 0.0),
            RigidBody::KinematicPositionBased,
            // LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
            KinematicCharacterController {
                offset: CharacterLength::Absolute(0.01),
                up: Vec3::Y,
                max_slope_climb_angle: PI / 4.0,
                min_slope_slide_angle: PI / 4.0,
                autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Absolute(0.3),
                    min_width: CharacterLength::Absolute(0.5),
                    include_dynamic_bodies: false,
                }),
                snap_to_ground: Some(CharacterLength::Absolute(0.33)),
                slide: true,
                translation: Some(Vec3::NEG_Y),
                ..Default::default()
            },
            Collider::capsule_y(0.75, 0.25),
        ))
        .with_child((
            Mesh3d(meshes.add(Capsule3d::new(0.25, 1.5))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: LIME.into(),
                // emissive: LinearRgba::new(10.0, 0.0, 1.0, 0.9),
                ..default()
            })),
        ));
}
