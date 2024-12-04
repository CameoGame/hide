use std::f32::consts::PI;

use bevy::color::palettes::css::{LIME, SANDY_BROWN, WHITE};
use bevy::prelude::*;

const LIGHT_STRONG: f32 = 10_000_000.;
const LIGHT_NORMAL: f32 = 1_000_000.;
const LIGHT_FLASHLIGHT: f32 = 400_000.;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostStartup,
            (spawn_stage, spawn_guard, spawn_player, spawn_ui),
        );
        app.add_systems(FixedUpdate, (input_movement,));
        app.add_systems(
            FixedUpdate,
            (light_check_target_within, guard_check_target_within),
        );
    }
}

#[derive(Component)]
struct Movable;

#[derive(Component)]
struct Guard;

#[derive(Component)]
struct LightSource;

#[derive(Component)]
struct Target;

#[derive(Component)]
struct Obstacle;

#[derive(Component)]
#[require(Transform, Visibility)]
struct Actor;

#[derive(Component)]
#[require(Transform)]
struct Sector {
    /// half woking angle in radians
    angle: f32,

    /// if target out this distance, no effect at all
    max_distance: f32,

    /// if target in this distance, no more check
    min_distance: f32,
}

#[derive(Component)]
struct LightSector;

#[derive(Component)]
struct VisionSector;

#[derive(Component)]
struct UnderLight;

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
        Mesh3d(meshes.add(Cuboid::new(2.0, 5., 2.))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: SANDY_BROWN.into(),
            emissive: LinearRgba::new(0.0, 0.0, 0.0, 0.9),
            ..default()
        })),
        Transform::from_xyz(5.0, 2.5, 0.0),
        Obstacle,
    ));

    // fixed light
    commands
        .spawn((
            Actor,
            Transform::from_xyz(10.0, 5.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
            LightSource,
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
            Actor,
            Transform::from_xyz(1.0, 1.0, 5.0).looking_at(Vec3::new(5.0, 1.0, 0.0), Vec3::Y),
            Guard,
            LightSource,
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

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Target,
        Movable,
        Mesh3d(meshes.add(Capsule3d::new(0.5, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: LIME.into(),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("UI"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn input_movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Movable>>,
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

        transform.translation += time.delta_secs() * 3.0 * direction;
    }
}

fn light_check_target_within(
    mut commands: Commands,
    mut ray_cast: MeshRayCast,
    query_source: Query<(&GlobalTransform, &Sector), With<LightSector>>,
    query_target: Query<(Entity, &GlobalTransform), With<Target>>,
) {
    let early_exit_test = |entity: Entity| -> bool { query_target.get(entity).is_ok() };
    let double_check = |entity: Entity| -> bool { query_target.get(entity).is_ok() };

    for (entity, target) in &query_target {
        let mut under_light = false;
        for (source, range) in &query_source {
            under_light |= check_target_within_range(
                &mut ray_cast,
                source,
                target,
                range,
                &early_exit_test,
                &double_check,
            );
        }

        if under_light {
            commands.entity(entity).insert(UnderLight);
        } else {
            commands.entity(entity).remove::<UnderLight>();
        }
    }
}

fn guard_check_target_within(
    mut ray_cast: MeshRayCast,
    mut text: Single<&mut Text>,
    query_source: Query<(Entity, &GlobalTransform, &Sector), With<VisionSector>>,
    query_target: Query<(&GlobalTransform, Option<&UnderLight>), With<Target>>,
) {
    let early_exit_test = |entity: Entity| -> bool { query_target.get(entity).is_ok() };
    let double_check =
        |entity: Entity| -> bool { query_target.get(entity).is_ok_and(|(_, opt)| opt.is_some()) };
    for (_entity, source, range) in &query_source {
        for (target, _under_light) in &query_target {
            if check_target_within_range(
                &mut ray_cast,
                source,
                target,
                range,
                &early_exit_test,
                &double_check,
            ) {
                text.0 = String::from("YES");
                // see player
                // handle see one player
                //          and see multiple player
            } else {
                // debug
                if _under_light.is_some() {
                    text.0 = String::from("NO");
                } else {
                    text.0 = String::from("DARK");
                }
            }
        }
    }
}

fn check_target_within_range(
    ray_cast: &mut MeshRayCast,
    source: &GlobalTransform,
    target: &GlobalTransform,
    range: &Sector,
    early_exit_test: &impl Fn(Entity) -> bool,
    double_check: &impl Fn(Entity) -> bool,
) -> bool {
    let forward = source.forward();
    let direction = target.translation() - source.translation();
    let distance = direction.length();

    if distance < range.min_distance {
        return true;
    }

    if distance < range.max_distance && forward.angle_between(direction) < range.angle {
        if let Ok(dir) = Dir3::new(direction) {
            let ray = Ray3d::new(source.translation(), dir);
            if let Some((entity, _hit)) = ray_cast
                .cast_ray(
                    ray,
                    &RayCastSettings::default().with_early_exit_test(&early_exit_test),
                )
                .first()
            {
                return double_check(*entity);
            }
        }
    }

    false
}
