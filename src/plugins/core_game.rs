use bevy::prelude::*;
use crate::components::breath::RoundEndEvent;
use crate::events::combat_events::*;
use crate::systems::{attack, breath, chain, collision, damage, evade, game_state, guard, health, hitstop, initiative, input, menus, momentum, movement, pressure, ui};
use game_state::GameState;

/// Spawn players when entering InGame state
fn spawn_players(mut commands: Commands) {
    use crate::components::breath::*;
    use crate::components::character::*;
    use crate::components::combat::*;
    use crate::components::guard::*;
    use crate::components::health::*;
    use crate::components::initiative::*;
    use crate::components::movelist::*;
    use crate::components::state::*;
    use crate::systems::chain::ChainState;
    use crate::systems::momentum::Momentum;
    use crate::systems::pressure::Pressure;

    info!("Spawning players for match...");

    // Initialize match state (starts with countdown)
    commands.insert_resource(MatchState::default());

    // Spawn Player 1 (red rectangle, left side)
    commands.spawn((
        Character,
        Player::One,
        CharacterState::Idle,
        MaxSpeed(300.0),
        Velocity::default(),
        Hurtbox::default(),
        GuardMeter::default(),
        Initiative::default(),
        Pressure::default(),
        ChainState::default(),
        Momentum::default(),
        Health::default(),
        Breath::default(),
        Movelist::default(),
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.9, 0.2, 0.2),  // Red
                custom_size: Some(Vec2::new(100.0, 200.0)),  // Increased by 25%
                ..default()
            },
            transform: Transform::from_xyz(-300.0, 0.0, 0.0),
            ..default()
        },
    ));

    // Spawn Player 2 (blue rectangle, right side)
    commands.spawn((
        Character,
        Player::Two,
        CharacterState::Idle,
        MaxSpeed(300.0),
        Velocity::default(),
        Hurtbox::default(),
        GuardMeter::default(),
        Initiative::default(),
        Pressure::default(),
        ChainState::default(),
        Momentum::default(),
        Health::default(),
        Breath::default(),
        Movelist::default(),
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.2, 0.4, 0.9),  // Blue
                custom_size: Some(Vec2::new(100.0, 200.0)),  // Increased by 25%
                ..default()
            },
            transform: Transform::from_xyz(300.0, 0.0, 0.0),
            ..default()
        },
    ));
}

/// Despawn players when exiting InGame state (for rematch/reselect)
fn despawn_players(
    mut commands: Commands,
    query: Query<Entity, With<crate::components::character::Character>>,
) {
    info!("Despawning players for clean state...");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Core game plugin - manages fundamental game systems
/// Phase 1: Movement and input handling
/// Phase 2: Combat and collision detection
pub struct CoreGamePlugin;

impl Plugin for CoreGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // State machine
            .init_state::<GameState>()

            // Resources
            .init_resource::<input::CurrentInputs>()
            .init_resource::<game_state::CharacterSelection>()
            .init_resource::<menus::VictoryMenuSelection>()

            // Events
            .add_event::<HitEvent>()
            .add_event::<ParryEvent>()
            .add_event::<GuardBreakEvent>()
            .add_event::<GrabEvent>()
            .add_event::<RoundEndEvent>()  // Phase 4: Round end event

            // Debug game state changes
            .add_systems(Update, menus::debug_game_state)

            // Character select screen systems
            .add_systems(OnEnter(GameState::CharacterSelect), menus::setup_character_select)
            .add_systems(Update, (
                menus::character_select_input,
                menus::update_character_select_ui,
            ).run_if(in_state(GameState::CharacterSelect)))
            .add_systems(OnExit(GameState::CharacterSelect), menus::cleanup_character_select)

            // Victory screen systems
            .add_systems(OnEnter(GameState::Victory), menus::setup_victory_screen)
            .add_systems(Update, (
                menus::victory_screen_input,
                menus::update_victory_menu_ui,
            ).run_if(in_state(GameState::Victory)))
            .add_systems(OnExit(GameState::Victory), menus::cleanup_victory_screen)

            // Game systems - only run during InGame state
            .add_systems(OnEnter(GameState::InGame), spawn_players)
            .add_systems(OnExit(GameState::InGame), despawn_players)
            .add_systems(Update, menus::detect_match_end.run_if(in_state(GameState::InGame)))

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
            ).chain().run_if(in_state(GameState::InGame)))
            .add_systems(Update, (
                // Hitstop processing - MUST run first before state progression
                hitstop::process_hitstop,
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
            ).chain().run_if(in_state(GameState::InGame)))
            .add_systems(Update, (
                // Physics and collision
                movement::apply_dash_movement,      // Apply dash movement
                movement::apply_attack_movement,    // Phase 4.5: Apply attack-based movement
                movement::apply_velocity,
                movement::clamp_to_stage,
                collision::detect_hits,
            ).chain().run_if(in_state(GameState::InGame)))
            .add_systems(Update, (
                // Reactions - Part 1
                hitstop::apply_hitstop_on_hit,   // Apply hitstop when hits connect
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
            ).chain().run_if(in_state(GameState::InGame)))
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
            ).chain().run_if(in_state(GameState::InGame)))
            .add_systems(Update, (
                // Visual feedback - Part 1
                hitstop::hitstop_screen_shake,    // Screen shake during hitstop
                hitstop::cleanup_hitstop_camera,  // Reset camera after hitstop
                attack::visualize_attack_phases,
                attack::visualize_attack_direction,  // Phase 4.5: Show attack direction
                guard::visualize_blocking,
                guard::parry_flash_effect,
                damage::hit_flash_feedback,
                health::visualize_health_state,  // Phase 4: Visual health state
                breath::visualize_decisive_blow_availability,  // Phase 4: Decisive blow danger
                ui::render_breath_indicators,    // Phase 4: Breath UI
                ui::render_health_bars,          // Phase 4: Health bars
            ).run_if(in_state(GameState::InGame)))
            .add_systems(Update, (
                // Visual feedback - Part 2
                ui::render_round_timer,          // Phase 4: Round timer
                ui::render_round_text_indicator, // Phase 4: Round text
                ui::render_victory_screen,       // Phase 4: Victory screen
                evade::visualize_evade,
                evade::cleanup_evade_visuals,
                movement::visualize_dash_cooldown, // Dash cooldown indicator
                initiative::visualize_initiative,
                pressure::visualize_pressure,
                momentum::visualize_momentum,
                chain::visualize_chain_window,
            ).run_if(in_state(GameState::InGame)))
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
            ).run_if(in_state(GameState::InGame)));
    }
}
