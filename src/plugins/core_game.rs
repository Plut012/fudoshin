use bevy::prelude::*;
use crate::events::combat_events::*;
use crate::systems::{attack, collision, damage, evade, guard, input, movement};

/// Core game plugin - manages fundamental game systems
/// Phase 1: Movement and input handling
/// Phase 2: Combat and collision detection
pub struct CoreGamePlugin;

impl Plugin for CoreGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<input::CurrentInputs>()

            // Events
            .add_event::<HitEvent>()
            .add_event::<ParryEvent>()
            .add_event::<GuardBreakEvent>()
            .add_event::<GrabEvent>()

            // Systems - split into groups due to Bevy tuple limits
            .add_systems(Update, (
                // Input and movement
                input::update_inputs,
                movement::process_movement_input,
                attack::handle_attack_input,
                guard::handle_block_input,
                evade::handle_evade_input,
                movement::update_movement_state,
            ).chain())
            .add_systems(Update, (
                // State progression
                attack::progress_attack_phases,
                guard::progress_stagger,
                guard::progress_parry,
                evade::progress_evade,
                attack::activate_hitboxes,
            ).chain())
            .add_systems(Update, (
                // Physics and collision
                movement::apply_velocity,
                movement::clamp_to_stage,
                collision::detect_hits,
            ).chain())
            .add_systems(Update, (
                // Reactions
                guard::check_parry_success,
                damage::apply_hit_reactions,
                guard::fill_guard_on_block,
                guard::check_guard_break,
                guard::drain_guard_meter,
            ).chain())
            .add_systems(Update, (
                // Visual feedback
                attack::visualize_attack_phases,
                guard::visualize_blocking,
                guard::parry_flash_effect,
                damage::hit_flash_feedback,
                evade::visualize_evade,
                evade::cleanup_evade_visuals,
            ))
            .add_systems(Update, (
                // Debug
                movement::debug_character_state,
                attack::debug_attack_state,
                guard::debug_guard_meter,
                damage::debug_hit_events,
                collision::debug_draw_boxes,
            ));
    }
}
