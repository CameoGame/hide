use bevy::color::palettes::css::{DEEP_PINK, LIME};
use bevy::prelude::*;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_light);
        app.add_systems(FixedUpdate, (movement, light));
        app.add_systems(FixedUpdate, (check_target,));
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
            SpotLight {
                intensity: 200_000.0,
                color: LIME.into(),
                shadows_enabled: true,
                inner_angle: 0.6,
                outer_angle: 0.8,
                ..default()
            },
            Transform::from_xyz(1.0, 1.0, 3.0).looking_at(Vec3::new(5.0, 1.0, 0.0), Vec3::Y),
            Movable,
            Guard,
            Lighter,
        ))
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
            base_color: DEEP_PINK.into(),
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
struct Lighter;

#[derive(Component)]
struct Target;

#[derive(Component)]
struct Obstacle;

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
        *transform = transform.looking_at(Vec3::new(5.0, 1.0, 0.0), Vec3::Y);
    }
}

fn light(input: Res<ButtonInput<KeyCode>>, mut query: Single<&mut SpotLight, With<Lighter>>) {
    let spotlight = &mut query;
    if input.pressed(KeyCode::Digit1) {
        spotlight.range += 0.1;
    }
    if input.pressed(KeyCode::Digit2) {
        spotlight.range -= 0.1;
    }
    if input.pressed(KeyCode::Digit3) {
        spotlight.radius += 0.1;
    }
    if input.pressed(KeyCode::Digit4) {
        spotlight.radius -= 0.1;
    }
}

fn check_target(
    mut ray_cast: MeshRayCast,
    mut text: Single<&mut Text>,
    query_movable: Query<&Transform, With<Guard>>,
    query_target: Query<&Transform, With<Target>>,
) {
    let early_exit_test = |entity: Entity| -> bool { query_target.get(entity).is_ok() };

    for trans_movable in &query_movable {
        let pos = trans_movable.translation;
        for trans_target in &query_target {
            let direction = trans_target.translation - pos;

            // TODO: check the distance and angle within valid range
            if direction.length() > 0.0 {
                let ray = Ray3d::new(pos, Dir3::new(direction).unwrap());
                if let Some((entity, _hit)) = ray_cast
                    .cast_ray(
                        ray,
                        &RayCastSettings::default().with_early_exit_test(&early_exit_test),
                    )
                    .first()
                {
                    if query_target.get(*entity).is_ok() {
                        text.0 = String::from("yes");
                    } else {
                        text.0 = String::from("no");
                    }
                }
            }
        }
    }
}
