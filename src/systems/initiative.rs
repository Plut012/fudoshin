use bevy::prelude::*;
use crate::components::initiative::Initiative;
use crate::components::character::Player;
use crate::events::combat_events::{HitEvent, ParryEvent};

/// Apply frame advantage after hits and blocks
pub fn apply_frame_advantage(
    mut hit_events: EventReader<HitEvent>,
    mut query: Query<(&mut Initiative, &Player)>,
) {
    for event in hit_events.read() {
        // Attacker gains/loses frames based on hit/block
        if let Ok((mut attacker_init, attacker_player)) = query.get_mut(event.attacker) {
            if event.was_blocked {
                // Blocked attacks give disadvantage
                let disadvantage = match event.damage {
                    1 => 2,  // Light: -2f on block
                    2 => 8,  // Heavy: -8f on block
                    _ => 2,
                };
                attacker_init.lose(disadvantage);
                info!("Player {:?} blocked, -{}f disadvantage", attacker_player, disadvantage);
            } else {
                // Successful hits give advantage
                let advantage = match event.damage {
                    1 => 4,  // Light: +4f on hit
                    2 => 6,  // Heavy: +6f on hit
                    _ => 4,
                };
                attacker_init.gain(advantage);
                info!("Player {:?} hit, +{}f advantage", attacker_player, advantage);
            }
        }

        // Defender loses frames if hit (opposite of attacker)
        if let Ok((mut defender_init, defender_player)) = query.get_mut(event.defender) {
            if !event.was_blocked {
                let disadvantage = match event.damage {
                    1 => 4,  // Light hit: -4f
                    2 => 6,  // Heavy hit: -6f
                    _ => 4,
                };
                defender_init.lose(disadvantage);
                debug!("Player {:?} got hit, -{}f disadvantage", defender_player, disadvantage);
            } else {
                // Defender gains small advantage for successful block
                let advantage = match event.damage {
                    1 => 2,  // +2f for blocking Light
                    2 => 8,  // +8f for blocking Heavy
                    _ => 2,
                };
                defender_init.gain(advantage);
                debug!("Player {:?} blocked successfully, +{}f advantage", defender_player, advantage);
            }
        }
    }
}

/// Apply massive frame advantage on successful parry
pub fn apply_parry_advantage(
    mut parry_events: EventReader<ParryEvent>,
    mut query: Query<(&mut Initiative, &Player)>,
) {
    for event in parry_events.read() {
        // Defender (parrier) gains huge advantage
        if let Ok((mut defender_init, defender_player)) = query.get_mut(event.defender) {
            defender_init.gain(12);  // +12f advantage
            info!("Player {:?} parried! +12f advantage", defender_player);
        }

        // Attacker loses frames (already staggered, but track it)
        if let Ok((mut attacker_init, attacker_player)) = query.get_mut(event.attacker) {
            attacker_init.lose(12);  // -12f disadvantage
            debug!("Player {:?} got parried, -12f disadvantage", attacker_player);
        }
    }
}

/// Tick down initiative frames each frame
pub fn tick_initiative(
    mut query: Query<&mut Initiative>,
) {
    for mut initiative in query.iter_mut() {
        initiative.tick();
    }
}

/// Prevent attacks when at frame disadvantage
/// This makes initiative meaningful - you can't act when minus
pub fn restrict_attacks_when_minus(
    mut query: Query<(&Initiative, &mut crate::components::state::CharacterState)>,
) {
    for (initiative, state) in query.iter_mut() {
        // If trying to attack while at disadvantage, it won't work
        // This is handled by the attack input system checking Initiative
        // Just log for debugging
        if initiative.has_disadvantage() && matches!(*state, crate::components::state::CharacterState::Attacking { .. }) {
            debug!("Character attacking while at disadvantage ({}f)", initiative.frames);
        }
    }
}

/// Visual indicator for initiative (arrow above character)
pub fn visualize_initiative(
    mut gizmos: Gizmos,
    query: Query<(&Initiative, &Transform, &Player)>,
) {
    for (initiative, transform, _player) in query.iter() {
        if initiative.frames == 0 {
            continue; // Neutral, no indicator
        }

        let pos = transform.translation.truncate();
        let indicator_height = 80.0; // Above character

        if initiative.has_advantage() {
            // Green up arrow for advantage
            let arrow_pos = pos + Vec2::new(0.0, indicator_height);
            gizmos.line_2d(
                arrow_pos,
                arrow_pos + Vec2::new(0.0, 15.0),
                Color::srgb(0.0, 1.0, 0.0),
            );
            // Arrow head
            gizmos.line_2d(
                arrow_pos + Vec2::new(0.0, 15.0),
                arrow_pos + Vec2::new(-5.0, 10.0),
                Color::srgb(0.0, 1.0, 0.0),
            );
            gizmos.line_2d(
                arrow_pos + Vec2::new(0.0, 15.0),
                arrow_pos + Vec2::new(5.0, 10.0),
                Color::srgb(0.0, 1.0, 0.0),
            );
        } else {
            // Red down arrow for disadvantage
            let arrow_pos = pos + Vec2::new(0.0, indicator_height);
            gizmos.line_2d(
                arrow_pos,
                arrow_pos - Vec2::new(0.0, 15.0),
                Color::srgb(1.0, 0.0, 0.0),
            );
            // Arrow head
            gizmos.line_2d(
                arrow_pos - Vec2::new(0.0, 15.0),
                arrow_pos + Vec2::new(-5.0, -10.0),
                Color::srgb(1.0, 0.0, 0.0),
            );
            gizmos.line_2d(
                arrow_pos - Vec2::new(0.0, 15.0),
                arrow_pos + Vec2::new(5.0, -10.0),
                Color::srgb(1.0, 0.0, 0.0),
            );
        }
    }
}

/// Debug: Log initiative changes
pub fn debug_initiative(
    query: Query<(&Player, &Initiative), Changed<Initiative>>,
) {
    for (player, initiative) in query.iter() {
        if initiative.frames != 0 {
            debug!("Player {:?} initiative: {}f", player, initiative.frames);
        }
    }
}
