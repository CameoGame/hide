use bevy::prelude::*;

use super::*;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (check_character_within, ));
    }
}

fn check_character_within(
    mut commands: Commands,
    mut ray_cast: MeshRayCast,
    query_source: Query<(&GlobalTransform, &Sector), With<LightSector>>,
    query_target: Query<(Entity, &GlobalTransform), With<Character>>,
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

            if under_light {
                break;
            }
        }

        if under_light {
            commands.entity(entity).insert(UnderLight);
        } else {
            commands.entity(entity).remove::<UnderLight>();
        }
    }
}
