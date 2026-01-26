use bevy::prelude::*;
use crate::components::hitstop::Hitstop;
use crate::components::state::AttackData;
use crate::events::combat_events::HitEvent;

/// Apply hitstop to both attacker and defender when an attack connects
pub fn apply_hitstop_on_hit(
    mut commands: Commands,
    mut hit_events: EventReader<HitEvent>,
    attack_query: Query<&AttackData>,
) {
    for event in hit_events.read() {
        // Get attack data to determine hitstop duration
        if let Ok(attack_data) = attack_query.get(event.attacker) {
            // Determine hitstop duration based on hit type and attack strength
            let is_blocked = event.was_blocked;
            let is_counter = event.counter_hit;

            // Use default hitstop values based on attack type
            // Light: 9f hit / 6f block / 12f counter
            // Heavy: 13f hit / 10f block / 16f counter
            let (base_hit, base_block, base_counter) = match attack_data.attack_type {
                crate::components::state::AttackType::Light => (9, 6, 12),
                crate::components::state::AttackType::Heavy => (13, 10, 16),
                crate::components::state::AttackType::Grab => (11, 0, 14),
            };

            let hitstop_frames = if is_counter {
                base_counter
            } else if is_blocked {
                base_block
            } else {
                base_hit
            };

            debug!(
                "Applying hitstop: {} frames (blocked: {}, counter: {}, type: {:?})",
                hitstop_frames, is_blocked, is_counter, attack_data.attack_type
            );

            // Apply hitstop to attacker
            commands.entity(event.attacker).insert(Hitstop::new(hitstop_frames));

            // Apply hitstop to defender
            commands.entity(event.defender).insert(Hitstop::new(hitstop_frames));
        }
    }
}

/// Tick down hitstop timers and remove when complete
pub fn process_hitstop(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Hitstop)>,
) {
    for (entity, mut hitstop) in query.iter_mut() {
        if !hitstop.tick() {
            // Hitstop complete, remove component
            commands.entity(entity).remove::<Hitstop>();
            debug!("Hitstop complete for entity {:?}", entity);
        }
    }
}

/// Visual feedback during hitstop - subtle screen shake effect
pub fn hitstop_screen_shake(
    query: Query<&Hitstop>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    // Check if any entity is in hitstop
    let max_hitstop = query.iter()
        .map(|h| h.total_frames - h.frames_remaining)
        .max()
        .unwrap_or(0);

    if max_hitstop > 0 {
        // Apply subtle shake to camera
        for mut transform in camera_query.iter_mut() {
            let shake_amount = if max_hitstop >= 13 {
                // Heavy attack hitstop - bigger shake
                3.0
            } else if max_hitstop >= 9 {
                // Light attack hitstop - small shake
                1.5
            } else {
                0.0
            };

            // Alternating shake pattern
            let shake_dir = if (max_hitstop % 2) == 0 { 1.0 } else { -1.0 };
            transform.translation.x += shake_dir * shake_amount;
        }
    }
}

/// Reset camera position when hitstop ends
pub fn cleanup_hitstop_camera(
    query: Query<&Hitstop>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    // If no entities in hitstop, reset camera
    if query.is_empty() {
        for mut transform in camera_query.iter_mut() {
            // Snap back to zero (assuming camera should be centered)
            transform.translation.x = 0.0;
        }
    }
}
