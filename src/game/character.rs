use bevy::prelude::*;

use super::*;

pub(super) struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (check_character_under_light, guard_check_target_within),
        );
    }
}

fn check_character_under_light(
    mut commands: Commands,
    mut ray_cast: MeshRayCast,
    lights: Query<(&GlobalTransform, &Sector), With<LightSector>>,
    actors: Query<(Entity, &Children, &GlobalTransform), With<Character>>,
) {
    let early_exit_test = |entity: Entity| -> bool {
        for (_, children, _) in &actors {
            if children.contains(&entity) {
                return true;
            }
        }

        false
    };
    let double_check = |entity: Entity| -> bool {
        for (_, children, _) in &actors {
            if children.contains(&entity) {
                return true;
            }
        }

        false
    };

    for (entity, _, target) in &actors {
        let mut under_light = false;
        for (source, range) in &lights {
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

fn guard_check_target_within(
    mut ray_cast: MeshRayCast,
    mut texts: Query<(&mut Text, &DebugText)>,
    guards: Query<(&GlobalTransform, &Sector), With<VisionSector>>,
    sneakers: Query<(&GlobalTransform, &Children), (With<Sneaker>, With<UnderLight>)>,
) {
    let early_exit_test = |entity: Entity| -> bool {
        for (_, children) in &sneakers {
            if children.contains(&entity) {
                return true;
            }
        }

        false
    };
    let double_check = |entity: Entity| -> bool {
        for (_, children) in &sneakers {
            if children.contains(&entity) {
                return true;
            }
        }

        false
    };

    for (guard, range) in &guards {
        for (sneaker, _) in &sneakers {
            if check_target_within_range(
                &mut ray_cast,
                guard,
                sneaker,
                range,
                &early_exit_test,
                &double_check,
            ) {
                for (mut text, DebugText(id)) in &mut texts {
                    if *id == 2 {
                        text.0 = String::from("YES");
                    }
                }
                // see player
                // handle see one player
                //          and see multiple player
            } else {
                // debug
                for (mut text, DebugText(id)) in &mut texts {
                    if *id == 2 {
                        text.0 = String::from("NO");
                    }
                }
            }
        }
    }
}
