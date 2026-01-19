use bevy::prelude::*;
use crate::components::character::Player;
use crate::components::combat::{Hitbox, Hurtbox};
use crate::components::state::CharacterState;
use crate::events::combat_events::HitEvent;
use crate::systems::evade::EvadeData;

/// Detect collisions between active hitboxes and hurtboxes
/// Only checks between different players (can't hit yourself)
pub fn detect_hits(
    hitbox_query: Query<(Entity, &Hitbox, &GlobalTransform, &Player)>,
    hurtbox_query: Query<(Entity, &Hurtbox, &GlobalTransform, &Player, &CharacterState, Option<&EvadeData>)>,
    mut hit_events: EventWriter<HitEvent>,
) {
    for (attacker_entity, hitbox, attacker_transform, attacker_player) in hitbox_query.iter() {
        // Skip if hitbox is not active
        if !hitbox.active {
            continue;
        }

        let hitbox_rect = hitbox.world_rect(&attacker_transform.compute_transform());

        for (defender_entity, hurtbox, defender_transform, defender_player, defender_state, evade_data) in hurtbox_query.iter() {
            // Can't hit yourself
            if attacker_player == defender_player {
                continue;
            }

            // Check if defender is invincible (evading with i-frames)
            if let Some(evade) = evade_data {
                if evade.invincible {
                    debug!("Attack missed - defender is invincible (evade i-frames)");
                    continue; // Skip this hit, they're invincible
                }
            }

            let hurtbox_rect = hurtbox.world_rect(&defender_transform.compute_transform());

            // AABB collision detection
            if rects_intersect(&hitbox_rect, &hurtbox_rect) {
                // Hit detected!
                let mut event = HitEvent::new(attacker_entity, defender_entity, hitbox.damage);

                // Check if defender is blocking
                let is_blocking = matches!(defender_state, CharacterState::Blocking);

                // Check for unblockable property
                let is_unblockable = hitbox.properties.iter().any(|p| matches!(p, crate::components::combat::AttackProperty::Unblockable));

                if is_unblockable {
                    event = event.unblockable();
                } else if is_blocking {
                    event = event.blocked();
                }

                let was_blocked = event.was_blocked;
                hit_events.send(event);

                debug!(
                    "Hit detected! {:?} hit {:?} for {} damage (blocked: {})",
                    attacker_player, defender_player, hitbox.damage, was_blocked
                );
            }
        }
    }
}

/// AABB (Axis-Aligned Bounding Box) collision detection
fn rects_intersect(a: &Rect, b: &Rect) -> bool {
    a.min.x < b.max.x && a.max.x > b.min.x && a.min.y < b.max.y && a.max.y > b.min.y
}

/// Debug visualization for hitboxes and hurtboxes
pub fn debug_draw_boxes(
    mut gizmos: Gizmos,
    hitbox_query: Query<(&Hitbox, &GlobalTransform)>,
    hurtbox_query: Query<(&Hurtbox, &GlobalTransform)>,
) {
    // Draw active hitboxes in red
    for (hitbox, transform) in hitbox_query.iter() {
        if hitbox.active {
            let rect = hitbox.world_rect(&transform.compute_transform());
            let center = (rect.min + rect.max) / 2.0;
            let size = rect.max - rect.min;

            gizmos.rect_2d(
                center,
                0.0,
                size,
                Color::srgb(1.0, 0.0, 0.0), // Red for active hitboxes
            );
        }
    }

    // Draw hurtboxes in green
    for (hurtbox, transform) in hurtbox_query.iter() {
        let rect = hurtbox.world_rect(&transform.compute_transform());
        let center = (rect.min + rect.max) / 2.0;
        let size = rect.max - rect.min;

        gizmos.rect_2d(
            center,
            0.0,
            size,
            Color::srgba(0.0, 1.0, 0.0, 0.3), // Semi-transparent green for hurtboxes
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rects_intersect() {
        let a = Rect::from_center_size(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let b = Rect::from_center_size(Vec2::new(5.0, 0.0), Vec2::new(10.0, 10.0));
        assert!(rects_intersect(&a, &b));

        let c = Rect::from_center_size(Vec2::new(20.0, 0.0), Vec2::new(10.0, 10.0));
        assert!(!rects_intersect(&a, &c));
    }
}
