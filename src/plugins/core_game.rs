use bevy::prelude::*;
use crate::components::breath::RoundEndEvent;
use crate::events::combat_events::*;
use crate::systems::{attack, breath, chain, collision, damage, evade, guard, health, initiative, input, momentum, movement, pressure, ui};

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
            .add_event::<RoundEndEvent>()  // Phase 4: Round end event

            // Systems - split into groups due to Bevy tuple limits
            .add_systems(Update, (
                // Input and movement
                input::update_inputs,
                movement::process_movement_input,
                movement::handle_dash_input,        // Dash input handling
                attack::handle_attack_input,
                guard::handle_block_input,
                evade::handle_evade_input,
                chain::handle_chain_input,
                movement::update_movement_state,
            ).chain())
            .add_systems(Update, (
                // State progression
                breath::tick_round_countdown,      // Phase 4: Round countdown
                breath::tick_round_timer,          // Phase 4: Round timer
                attack::progress_attack_phases,
                guard::progress_stagger,
                guard::progress_parry,
                evade::progress_evade,
                movement::tick_dash_cooldown,      // Dash cooldown
                initiative::tick_initiative,
                momentum::tick_momentum,
                chain::manage_chain_window,
                attack::activate_hitboxes,
                movement::initiate_attack_movement, // Phase 4.5: Start attack movement
                movement::cleanup_attack_movement,  // Phase 4.5: Clean up finished movement
            ).chain())
            .add_systems(Update, (
                // Physics and collision
                movement::apply_dash_movement,      // Apply dash movement
                movement::apply_attack_movement,    // Phase 4.5: Apply attack-based movement
                movement::apply_velocity,
                movement::clamp_to_stage,
                collision::detect_hits,
            ).chain())
            .add_systems(Update, (
                // Reactions - Part 1
                guard::check_parry_success,
                damage::apply_hit_reactions,
                health::apply_health_damage,  // Phase 4: Apply damage to health
                health::apply_movement_speed_modifier,  // Phase 4: Health state movement penalty
                health::apply_frame_advantage_penalty,  // Phase 4: Health state frame penalty
                health::restrict_pressure_by_health,    // Phase 4: Health state pressure cap
                health::restrict_momentum_by_health,    // Phase 4: Health state momentum restriction
                breath::check_decisive_blow,            // Phase 4: Check for decisive blow
                breath::check_timeout,                  // Phase 4: Check for timeout
                breath::handle_round_end,               // Phase 4: Handle round end
                breath::check_match_victory,            // Phase 4: Check match victory
            ).chain())
            .add_systems(Update, (
                // Reactions - Part 2
                guard::fill_guard_on_block,
                guard::check_guard_break,
                guard::drain_guard_meter,
                initiative::apply_frame_advantage,
                initiative::apply_parry_advantage,
                pressure::build_pressure,
                pressure::apply_pressure_movement_bonus,
                pressure::drain_pressure_passive,
                momentum::build_momentum_on_hit,
                momentum::build_momentum_on_parry,
                chain::mark_chainable_on_hit,
            ).chain())
            .add_systems(Update, (
                // Visual feedback
                attack::visualize_attack_phases,
                attack::visualize_attack_direction,  // Phase 4.5: Show attack direction
                guard::visualize_blocking,
                guard::parry_flash_effect,
                damage::hit_flash_feedback,
                health::visualize_health_state,  // Phase 4: Visual health state
                breath::visualize_decisive_blow_availability,  // Phase 4: Decisive blow danger
                ui::render_breath_indicators,    // Phase 4: Breath UI
                ui::render_health_bars,          // Phase 4: Health bars
                ui::render_round_timer,          // Phase 4: Round timer
                ui::render_round_text_indicator, // Phase 4: Round text
                evade::visualize_evade,
                evade::cleanup_evade_visuals,
                movement::visualize_dash_cooldown, // Dash cooldown indicator
                initiative::visualize_initiative,
                pressure::visualize_pressure,
                momentum::visualize_momentum,
                chain::visualize_chain_window,
            ))
            .add_systems(Update, (
                // Debug
                movement::debug_character_state,
                attack::debug_attack_state,
                guard::debug_guard_meter,
                damage::debug_hit_events,
                health::debug_health_display,  // Phase 4: Debug health
                breath::debug_breath_display,   // Phase 4: Debug breath
                initiative::debug_initiative,
                pressure::debug_pressure,
                momentum::debug_momentum,
                chain::debug_chain_state,
                collision::debug_draw_boxes,
            ));
    }
}
