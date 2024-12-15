mod level;
mod local_player;
mod character;

use std::f32::consts::PI;

use bevy::color::palettes::css::{LIME, SANDY_BROWN, WHITE};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// const LIGHT_STRONG: f32 = 10_000_000.;
const LIGHT_NORMAL: f32 = 1_000_000.;
const LIGHT_FLASHLIGHT: f32 = 400_000.;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            level::LevelPlugin,
            character::CharacterPlugin,
            local_player::LocalPlayerPlugin,
        ));

        app.add_systems(PostStartup, spawn_ui);
    }
}

#[derive(Component, Default)]
#[require(Transform, Visibility)]
struct Actor;

#[derive(Component, Default)]
#[require(Actor)]
struct Character;

#[derive(Component)]
#[require(Character)]
struct Player;

#[derive(Component)]
#[require(Character)]
struct Guard;

#[derive(Component)]
#[require(Character)]
struct Sneaker;

#[derive(Component)]
struct Obstacle;

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

#[derive(Component, Debug)]
struct UnderLight;

#[derive(Component)]
struct DebugText(pub u32);

fn spawn_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("UI"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        DebugText(0),
    ));

    commands.spawn((
        Text::new("UI"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(120.0),
            left: Val::Px(12.0),
            ..default()
        },
        DebugText(1),
    ));

    commands.spawn((
        Text::new("UI"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(240.0),
            left: Val::Px(12.0),
            ..default()
        },
        DebugText(2),
    ));
}

/// TODO: Refactor this function
/// So far, this funciton has two roles
///     1. has the target been caught
///     2. has the target been seen
/// The min distance should not be used as seen?
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

