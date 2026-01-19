use bevy::prelude::*;
use crate::components::health::Health;
use crate::components::state::AttackType;
use crate::events::combat_events::HitEvent;
use crate::systems::momentum::Momentum;

/// Calculate base damage for an attack type
fn calculate_base_damage(attack_type: AttackType) -> f32 {
    match attack_type {
        AttackType::Light => 8.0,
        AttackType::Heavy => 15.0,
        AttackType::Grab => 12.0,
    }
}

/// Apply health damage from hit events
pub fn apply_health_damage(
    mut hit_events: EventReader<HitEvent>,
    mut defender_query: Query<&mut Health>,
    attacker_query: Query<&Momentum>,
) {
    for event in hit_events.read() {
        // Get base damage from attack type
        let base_damage = calculate_base_damage(event.attack_type);

        // Apply counter hit bonus (50% extra damage)
        let counter_multiplier = if event.counter_hit { 1.5 } else { 1.0 };

        // Apply momentum bonus from attacker
        let momentum_multiplier = if let Ok(momentum) = attacker_query.get(event.attacker) {
            momentum.damage_bonus()
        } else {
            1.0
        };

        // Calculate chip damage if blocked (25% of full damage)
        let block_multiplier = if event.was_blocked { 0.25 } else { 1.0 };

        // Final damage calculation
        let final_damage = base_damage * counter_multiplier * momentum_multiplier * block_multiplier;

        // Apply damage to defender
        if let Ok(mut health) = defender_query.get_mut(event.defender) {
            let old_state = health.state;
            health.take_damage(final_damage);

            // Log damage and state changes
            if health.state != old_state {
                info!(
                    "Health state changed: {:?} -> {:?} (took {:.1} damage)",
                    old_state, health.state, final_damage
                );
            } else {
                debug!(
                    "Damage applied: {:.1} HP (blocked={}, counter={}, momentum={:.2}x)",
                    final_damage, event.was_blocked, event.counter_hit, momentum_multiplier
                );
            }
        }
    }
}

/// Visual feedback for health states - change character color based on health
pub fn visualize_health_state(
    mut query: Query<(&Health, &mut Sprite), Changed<Health>>,
) {
    for (health, mut sprite) in query.iter_mut() {
        // Update sprite color based on health state
        sprite.color = health.state.color();
    }
}

/// Apply movement speed modifiers based on health state
pub fn apply_movement_speed_modifier(
    mut query: Query<(&Health, &mut crate::components::character::MaxSpeed), Changed<Health>>,
) {
    for (health, mut max_speed) in query.iter_mut() {
        // Reset to base speed (300.0) then apply health modifier
        let base_speed = 300.0;
        max_speed.0 = base_speed * health.state.movement_speed_multiplier();
    }
}

/// Apply frame advantage penalties when taking damage while wounded
pub fn apply_frame_advantage_penalty(
    mut hit_events: EventReader<crate::events::combat_events::HitEvent>,
    mut query: Query<(&Health, &mut crate::components::initiative::Initiative)>,
) {
    for event in hit_events.read() {
        if event.was_blocked {
            continue;
        }

        // Apply penalty to defender based on their health state
        if let Ok((health, mut initiative)) = query.get_mut(event.defender) {
            let penalty = health.state.frame_advantage_penalty();
            if penalty != 0 {
                initiative.frames += penalty; // Penalty is negative, so this reduces advantage
                debug!(
                    "Health state penalty applied: {:?} gets {} frame disadvantage",
                    health.state, penalty
                );
            }
        }
    }
}

/// Modify guard meter fill rate based on health state
pub fn modify_guard_fill_rate(
    mut query: Query<(&Health, &mut crate::components::guard::GuardMeter), Changed<Health>>,
) {
    for (health, mut guard) in query.iter_mut() {
        // Base fill amounts (from guard.rs)
        let light_fill = 0.15;
        let heavy_fill = 0.35;

        // Apply health state multiplier to fill rates
        let multiplier = health.state.guard_fill_multiplier();

        // Store the multiplier in the guard meter
        // Note: This assumes we'll modify guard.rs to use these values
        // For now, we'll just log it as the guard system needs to be updated
        debug!(
            "Guard fill rate modifier for {:?}: {:.1}x (light={:.2}, heavy={:.2})",
            health.state,
            multiplier,
            light_fill * multiplier,
            heavy_fill * multiplier
        );
    }
}

/// Restrict pressure building based on health state
pub fn restrict_pressure_by_health(
    mut query: Query<(&Health, &mut crate::systems::pressure::Pressure), Changed<Health>>,
) {
    for (health, mut pressure) in query.iter_mut() {
        let max_intensity = health.state.max_pressure_intensity();

        // If pressure exceeds max for this health state, cap it
        if pressure.intensity > max_intensity {
            pressure.intensity = max_intensity;
            debug!(
                "Pressure capped at {} due to {:?} health state",
                max_intensity, health.state
            );
        }
    }
}

/// Restrict momentum building for broken state
pub fn restrict_momentum_by_health(
    mut hit_events: EventReader<crate::events::combat_events::HitEvent>,
    mut query: Query<(&Health, &mut crate::systems::momentum::Momentum)>,
) {
    for event in hit_events.read() {
        // Check attacker's health state
        if let Ok((health, mut momentum)) = query.get_mut(event.attacker) {
            if !health.state.can_build_momentum() {
                // Reset momentum if in broken state
                if momentum.level > 0 {
                    momentum.level = 0;
                    debug!("Momentum reset - cannot build in Broken state");
                }
            }
        }
    }
}

/// Debug: Display current health values
pub fn debug_health_display(
    query: Query<(&Health, &crate::components::character::Player), Changed<Health>>,
) {
    for (health, player) in query.iter() {
        debug!(
            "Player {:?}: {:.1}/{:.1} HP ({:?})",
            player, health.current, health.max, health.state
        );
    }
}
