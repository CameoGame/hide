use std::f32::consts::PI;

use bevy::color::palettes::css::{LIME, SANDY_BROWN, WHITE};
use bevy::prelude::*;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_light);
        app.add_systems(FixedUpdate, (movement,));
        app.add_systems(FixedUpdate, (light_check_target_within, check_target));
    }
}

fn setup_light(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Text::new("UI"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));

    // floor plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(15.0, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        })),
    ));

    // green spot light
    commands
        .spawn((
            Actor,
            Transform::from_xyz(1.0, 1.0, 3.0).looking_at(Vec3::new(5.0, 1.0, 0.0), Vec3::Y),
            Movable,
            Guard,
            LightSource,
        ))
        .with_child((SpotLight {
            intensity: 200_000.0,
            color: WHITE.into(),
            shadows_enabled: true,
            inner_angle: PI / 6.0,
            outer_angle: PI / 4.0,
            range: 10.0,
            radius: 0.07,
            ..default()
        },))
        .with_child((
            Mesh3d(meshes.add(Capsule3d::new(0.5, 2.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: LIME.into(),
                emissive: LinearRgba::new(10.0, 0.0, 1.0, 0.9),
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

    commands.spawn((
        Target,
        Mesh3d(meshes.add(Capsule3d::new(0.5, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: LIME.into(),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));
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
#[require(Transform)]
struct Actor;

fn movement(
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
    query_source: Query<(&GlobalTransform, &LightRange), With<LightSource>>,
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

fn check_target(
    mut ray_cast: MeshRayCast,
    query_source: Query<(Entity, &GlobalTransform, &VisionRange), With<LightSource>>,
    query_target: Query<&GlobalTransform, (With<Target>, With<UnderLight>)>,
) {
    let early_exit_test = |entity: Entity| -> bool { query_target.get(entity).is_ok() };
    let double_check = |entity: Entity| -> bool { query_target.get(entity).is_ok() };
    for (entity, source, range) in &query_source {
        for target in &query_target {
            if check_target_within_range(
                &mut ray_cast,
                source,
                target,
                range,
                &early_exit_test,
                &double_check,
            ) {
                // see player
                // handle see one player
                //          and see multiple player
            }
        }
    }
}

#[derive(Component)]
struct LightRange {
    intensity: f32,
    angle: f32,
    distance: f32,
    color: Color,
}

#[derive(Component)]
struct VisionRange {
    /// full woking angle in radians
    angle: f32,

    /// if target out this distance, no effect at all
    max_distance: f32,

    /// if target in this distance, no more check
    min_distance: f32,
}

trait SectorLike {
    fn angle(&self) -> f32;
    fn max_distance(&self) -> f32;
    fn min_distance(&self) -> f32;
}

impl SectorLike for LightRange {
    fn angle(&self) -> f32 {
        self.angle
    }

    fn max_distance(&self) -> f32 {
        self.distance
    }

    fn min_distance(&self) -> f32 {
        0.0
    }
}

impl SectorLike for VisionRange {
    fn angle(&self) -> f32 {
        self.angle
    }

    fn max_distance(&self) -> f32 {
        self.max_distance
    }

    fn min_distance(&self) -> f32 {
        self.min_distance
    }
}

#[derive(Component)]
struct UnderLight;

fn check_target_within_range<'a, S: SectorLike>(
    ray_cast: &mut MeshRayCast,
    source: &GlobalTransform,
    target: &GlobalTransform,
    range: &S,
    early_exit_test: &'a impl Fn(Entity) -> bool,
    double_check: &'a impl Fn(Entity) -> bool,
) -> bool {
    let forward = source.forward();
    let direction = target.translation() - source.translation();
    let distance = direction.length();

    if distance < range.min_distance() {
        return true;
    }

    if distance < range.max_distance() && forward.angle_between(direction) < range.angle() / 2.0 {
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
