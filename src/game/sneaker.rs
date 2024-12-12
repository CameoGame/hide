use bevy::prelude::*;

use super::*;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (guard_check_target_within,));
    }
}

fn guard_check_target_within(
    mut ray_cast: MeshRayCast,
    mut text: Single<&mut Text>,
    query_source: Query<(Entity, &GlobalTransform, &Sector), With<Guard>>,
    query_target: Query<(&GlobalTransform, Option<&UnderLight>), With<Sneaker>>,
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
