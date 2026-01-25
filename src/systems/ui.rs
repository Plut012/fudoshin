use bevy::prelude::*;
use crate::components::breath::{Breath, MatchState};
use crate::components::character::Player;
use crate::components::health::Health;

/// Render breath indicators (circles) for each player
pub fn render_breath_indicators(
    mut gizmos: Gizmos,
    query: Query<(&Player, &Breath)>,
) {
    for (player, breath) in query.iter() {
        // Position based on player (left/right side of screen)
        let base_x = match player {
            Player::One => -550.0,  // Left side
            Player::Two => 450.0,   // Right side
        };
        let y = 300.0;  // Top of screen

        // Draw circles for each breath (3 total)
        for i in 0..breath.max {
            let x = base_x + (i as f32 * 35.0);
            let filled = i < breath.current;

            if filled {
                // Filled circle (has breath)
                gizmos.circle_2d(
                    Vec2::new(x, y),
                    12.0,
                    Color::srgb(1.0, 1.0, 1.0),
                );
                // Inner filled circle
                gizmos.circle_2d(
                    Vec2::new(x, y),
                    10.0,
                    Color::srgb(0.9, 0.9, 0.9),
                );
            } else {
                // Empty circle (lost breath)
                gizmos.circle_2d(
                    Vec2::new(x, y),
                    12.0,
                    Color::srgba(0.4, 0.4, 0.4, 0.5),
                );
            }
        }
    }
}

/// Render health bars for each player
pub fn render_health_bars(
    mut gizmos: Gizmos,
    query: Query<(&Player, &Health)>,
) {
    for (player, health) in query.iter() {
        // Position based on player
        let base_x = match player {
            Player::One => -550.0,
            Player::Two => 450.0,
        };
        let y = 260.0;  // Just below breath indicators
        let bar_width = 150.0;
        let bar_height = 20.0;

        // Background (empty bar)
        let bg_min = Vec2::new(base_x, y - bar_height / 2.0);
        let bg_max = Vec2::new(base_x + bar_width, y + bar_height / 2.0);
        gizmos.rect_2d(
            Vec2::new(base_x + bar_width / 2.0, y),
            0.0,
            Vec2::new(bar_width, bar_height),
            Color::srgba(0.2, 0.2, 0.2, 0.8),
        );

        // Filled bar (current health)
        let health_percent = health.current / health.max;
        let filled_width = bar_width * health_percent;

        if filled_width > 0.0 {
            gizmos.rect_2d(
                Vec2::new(base_x + filled_width / 2.0, y),
                0.0,
                Vec2::new(filled_width, bar_height - 4.0),
                health.state.color(),
            );
        }

        // Border
        gizmos.rect_2d(
            Vec2::new(base_x + bar_width / 2.0, y),
            0.0,
            Vec2::new(bar_width + 2.0, bar_height + 2.0),
            Color::srgba(0.8, 0.8, 0.8, 0.3),
        );
    }
}

/// Render round timer at top center
pub fn render_round_timer(
    mut gizmos: Gizmos,
    match_state: Option<Res<MatchState>>,
) {
    if let Some(state) = match_state {
        if !state.round_active && state.countdown > 0.0 {
            // Show countdown before round starts
            let countdown_num = state.countdown.ceil() as u32;

            // Draw countdown number indicator (simple visualization)
            for i in 0..countdown_num {
                let radius = 15.0 + (i as f32 * 5.0);
                gizmos.circle_2d(
                    Vec2::new(0.0, 200.0),
                    radius,
                    Color::srgba(1.0, 1.0, 0.0, 0.6 - (i as f32 * 0.15)),
                );
            }
        } else if state.round_active {
            // Show round timer during active round
            let time_remaining = state.round_time.max(0.0).ceil() as u32;

            // Timer bar at top center
            let bar_width = 200.0;
            let bar_height = 8.0;
            let y = 320.0;

            // Background
            gizmos.rect_2d(
                Vec2::new(0.0, y),
                0.0,
                Vec2::new(bar_width, bar_height),
                Color::srgba(0.2, 0.2, 0.2, 0.8),
            );

            // Filled portion (time remaining)
            let time_percent = state.round_time / state.max_round_time;
            let filled_width = bar_width * time_percent;

            // Color changes based on time remaining (green -> yellow -> red)
            let timer_color = if time_percent > 0.5 {
                Color::srgb(0.3, 1.0, 0.3)  // Green
            } else if time_percent > 0.25 {
                Color::srgb(1.0, 1.0, 0.3)  // Yellow
            } else {
                Color::srgb(1.0, 0.3, 0.3)  // Red
            };

            if filled_width > 0.0 {
                gizmos.rect_2d(
                    Vec2::new(-bar_width / 2.0 + filled_width / 2.0, y),
                    0.0,
                    Vec2::new(filled_width, bar_height - 2.0),
                    timer_color,
                );
            }

            // Draw simple time indicator circles
            let num_dots = (time_remaining / 10).min(6);
            for i in 0..num_dots {
                let x = -90.0 + (i as f32 * 30.0);
                gizmos.circle_2d(
                    Vec2::new(x, y + 20.0),
                    5.0,
                    Color::srgb(1.0, 1.0, 1.0),
                );
            }
        }

        // Show match over indicator
        if state.match_over {
            // Draw victory indicator (large pulsing circles)
            let time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f32();
            let pulse = (time * 2.0).sin() * 0.5 + 0.5;

            for i in 0..3 {
                let radius = 50.0 + (i as f32 * 30.0) + pulse * 20.0;
                gizmos.circle_2d(
                    Vec2::new(0.0, 0.0),
                    radius,
                    Color::srgba(1.0, 0.8, 0.0, 0.4 - (i as f32 * 0.1)),
                );
            }
        }
    }
}

/// Show "FIGHT!" or countdown text indicator
pub fn render_round_text_indicator(
    mut gizmos: Gizmos,
    match_state: Option<Res<MatchState>>,
) {
    if let Some(state) = match_state {
        // Draw visual indicator for round state
        if !state.round_active && state.countdown > 0.0 {
            // Countdown phase - draw expanding ring
            let size = 100.0 * (1.0 - (state.countdown / 3.0));
            gizmos.circle_2d(
                Vec2::new(0.0, 0.0),
                size,
                Color::srgb(1.0, 1.0, 0.0),
            );
        } else if !state.round_active && state.countdown <= 0.0 && !state.match_over {
            // "FIGHT!" moment - draw flash effect
            gizmos.circle_2d(
                Vec2::new(0.0, 0.0),
                120.0,
                Color::srgba(1.0, 0.0, 0.0, 0.8),
            );
            gizmos.circle_2d(
                Vec2::new(0.0, 0.0),
                100.0,
                Color::srgba(1.0, 1.0, 1.0, 0.9),
            );
        }
    }
}

// ============================================================================
// SYMBOL LIBRARY - Reusable geometric shapes for UI
// ============================================================================

/// Draw a checkmark symbol (victory indicator)
fn draw_checkmark(gizmos: &mut Gizmos, pos: Vec2, size: f32, color: Color) {
    // Two lines forming a checkmark
    let p1 = pos + Vec2::new(-size * 0.3, 0.0);
    let p2 = pos + Vec2::new(0.0, -size * 0.5);
    let p3 = pos + Vec2::new(size * 0.5, size * 0.5);

    // Draw as connected line segments (thicker for visibility)
    gizmos.line_2d(p1, p2, color);
    gizmos.line_2d(p2, p3, color);

    // Draw second line slightly offset for thickness
    let offset = Vec2::new(2.0, 0.0);
    gizmos.line_2d(p1 + offset, p2 + offset, color);
    gizmos.line_2d(p2 + offset, p3 + offset, color);
}

/// Draw an arrow pointing in direction
fn draw_arrow(gizmos: &mut Gizmos, pos: Vec2, dir: Vec2, size: f32, color: Color) {
    let tip = pos + dir * size;
    let base = pos - dir * size * 0.3;

    // Arrow shaft
    gizmos.line_2d(base, tip, color);

    // Arrowhead
    let perpendicular = Vec2::new(-dir.y, dir.x) * size * 0.3;
    gizmos.line_2d(tip, tip - dir * size * 0.4 + perpendicular, color);
    gizmos.line_2d(tip, tip - dir * size * 0.4 - perpendicular, color);
}

/// Draw pulsing selection indicator
fn draw_selection_pulse(gizmos: &mut Gizmos, pos: Vec2, time: f32) {
    let pulse = (time * 3.0).sin() * 0.5 + 0.5;
    let alpha = 0.3 + pulse * 0.4;

    gizmos.circle_2d(
        pos,
        30.0 + pulse * 10.0,
        Color::srgba(1.0, 1.0, 1.0, alpha),
    );
}

/// Draw horizontal bar (menu option)
fn draw_menu_bar(gizmos: &mut Gizmos, pos: Vec2, width: f32, selected: bool) {
    let color = if selected {
        Color::srgb(1.0, 1.0, 1.0)
    } else {
        Color::srgba(0.5, 0.5, 0.5, 0.6)
    };

    gizmos.rect_2d(
        pos,
        0.0,
        Vec2::new(width, 8.0),
        color,
    );
}

// ============================================================================
// VICTORY SCREEN
// ============================================================================

/// Resource to track victory screen state
#[derive(Resource, Debug)]
pub struct VictoryScreenState {
    /// Auto-rematch countdown timer
    pub rematch_countdown: f32,
    /// Selected menu option
    pub selected_option: VictoryMenuOption,
}

impl Default for VictoryScreenState {
    fn default() -> Self {
        Self {
            rematch_countdown: 5.0,
            selected_option: VictoryMenuOption::Rematch,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VictoryMenuOption {
    Rematch,
    Quit,
}

/// Render victory screen when match is over
pub fn render_victory_screen(
    mut gizmos: Gizmos,
    match_state: Option<Res<MatchState>>,
    victory_state: Option<Res<VictoryScreenState>>,
    query: Query<(&Player, &Breath)>,
) {
    let Some(state) = match_state else { return };
    if !state.match_over {
        return;
    }

    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f32();

    // Determine winner and loser
    let mut winner_player = None;
    let mut loser_player = None;
    let mut p1_breaths = 0;
    let mut p2_breaths = 0;

    for (player, breath) in query.iter() {
        match player {
            Player::One => p1_breaths = breath.current,
            Player::Two => p2_breaths = breath.current,
        }

        if breath.is_defeated() {
            loser_player = Some(*player);
        } else {
            winner_player = Some(*player);
        }
    }

    // Draw large checkmark at top center
    draw_checkmark(
        &mut gizmos,
        Vec2::new(0.0, 150.0),
        50.0,
        Color::srgb(0.2, 1.0, 0.2), // Green checkmark
    );

    // Draw arrow pointing to winner's side
    if let Some(winner) = winner_player {
        let arrow_dir = match winner {
            Player::One => Vec2::new(-1.0, 0.3),  // Point left-up to P1
            Player::Two => Vec2::new(1.0, 0.3),   // Point right-up to P2
        };

        draw_arrow(
            &mut gizmos,
            Vec2::new(0.0, 100.0),
            arrow_dir.normalize(),
            60.0,
            Color::srgb(1.0, 1.0, 0.3), // Yellow arrow
        );
    }

    // Draw final breath count (larger, more prominent)
    // P1 breaths (left side)
    let p1_x = -150.0;
    for i in 0..3 {
        let x = p1_x + (i as f32 * 40.0);
        let filled = i < p1_breaths;

        if filled {
            gizmos.circle_2d(Vec2::new(x, 50.0), 18.0, Color::srgb(1.0, 1.0, 1.0));
            gizmos.circle_2d(Vec2::new(x, 50.0), 15.0, Color::srgb(0.9, 0.9, 0.9));
        } else {
            gizmos.circle_2d(Vec2::new(x, 50.0), 18.0, Color::srgba(0.3, 0.3, 0.3, 0.5));
        }
    }

    // P2 breaths (right side)
    let p2_x = 30.0;
    for i in 0..3 {
        let x = p2_x + (i as f32 * 40.0);
        let filled = i < p2_breaths;

        if filled {
            gizmos.circle_2d(Vec2::new(x, 50.0), 18.0, Color::srgb(1.0, 1.0, 1.0));
            gizmos.circle_2d(Vec2::new(x, 50.0), 15.0, Color::srgb(0.9, 0.9, 0.9));
        } else {
            gizmos.circle_2d(Vec2::new(x, 50.0), 18.0, Color::srgba(0.3, 0.3, 0.3, 0.5));
        }
    }

    // Menu options
    let default_vs = VictoryScreenState::default();
    let vs = victory_state.as_deref().unwrap_or(&default_vs);

    // Rematch option (horizontal bar)
    let rematch_selected = vs.selected_option == VictoryMenuOption::Rematch;
    draw_menu_bar(&mut gizmos, Vec2::new(0.0, -50.0), 150.0, rematch_selected);

    if rematch_selected {
        draw_selection_pulse(&mut gizmos, Vec2::new(0.0, -50.0), time);
    }

    // Quit option (X symbol)
    let quit_selected = vs.selected_option == VictoryMenuOption::Quit;
    let quit_color = if quit_selected {
        Color::srgb(1.0, 0.3, 0.3)
    } else {
        Color::srgba(0.5, 0.5, 0.5, 0.6)
    };

    // Draw X
    let x_pos = Vec2::new(0.0, -120.0);
    let x_size = 15.0;
    gizmos.line_2d(
        x_pos + Vec2::new(-x_size, -x_size),
        x_pos + Vec2::new(x_size, x_size),
        quit_color
    );
    gizmos.line_2d(
        x_pos + Vec2::new(-x_size, x_size),
        x_pos + Vec2::new(x_size, -x_size),
        quit_color
    );

    if quit_selected {
        draw_selection_pulse(&mut gizmos, x_pos, time);
    }

    // Auto-rematch countdown indicator (dots)
    let countdown_dots = vs.rematch_countdown.ceil().min(5.0) as u32;
    for i in 0..countdown_dots {
        let x = -60.0 + (i as f32 * 30.0);
        gizmos.circle_2d(
            Vec2::new(x, -200.0),
            5.0,
            Color::srgba(1.0, 1.0, 1.0, 0.7),
        );
    }
}

/// Handle input on victory screen
pub fn victory_screen_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut victory_state: ResMut<VictoryScreenState>,
    mut match_state: ResMut<MatchState>,
    mut player_query: Query<(&mut Breath, &mut Health)>,
) {
    // Only process input if match is over
    if !match_state.match_over {
        return;
    }

    // Navigation (up/down to change option)
    if keyboard.just_pressed(KeyCode::KeyW) || keyboard.just_pressed(KeyCode::ArrowUp) {
        victory_state.selected_option = match victory_state.selected_option {
            VictoryMenuOption::Rematch => VictoryMenuOption::Quit,
            VictoryMenuOption::Quit => VictoryMenuOption::Rematch,
        };
    }

    if keyboard.just_pressed(KeyCode::KeyS) || keyboard.just_pressed(KeyCode::ArrowDown) {
        victory_state.selected_option = match victory_state.selected_option {
            VictoryMenuOption::Rematch => VictoryMenuOption::Quit,
            VictoryMenuOption::Quit => VictoryMenuOption::Rematch,
        };
    }

    // Confirm selection (Light attack = J)
    if keyboard.just_pressed(KeyCode::KeyJ) || keyboard.just_pressed(KeyCode::Numpad1) {
        match victory_state.selected_option {
            VictoryMenuOption::Rematch => {
                info!("Rematch selected! Resetting match...");

                // Reset match state
                match_state.reset();
                victory_state.rematch_countdown = 5.0;

                // Reset all player breaths and health
                for (mut breath, mut health) in player_query.iter_mut() {
                    breath.reset();
                    health.restore_full();
                }
            }
            VictoryMenuOption::Quit => {
                info!("Quit selected!");
                // For now, just exit the app
                std::process::exit(0);
            }
        }
    }
}

/// Auto-rematch countdown
pub fn victory_screen_countdown(
    mut victory_state: ResMut<VictoryScreenState>,
    mut match_state: ResMut<MatchState>,
    mut player_query: Query<(&mut Breath, &mut Health)>,
    time: Res<Time>,
) {
    if match_state.match_over && victory_state.selected_option == VictoryMenuOption::Rematch {
        victory_state.rematch_countdown -= time.delta_seconds();

        if victory_state.rematch_countdown <= 0.0 {
            info!("Auto-rematch triggered! Resetting match...");

            // Reset match state
            match_state.reset();
            victory_state.rematch_countdown = 5.0;

            // Reset all player breaths and health
            for (mut breath, mut health) in player_query.iter_mut() {
                breath.reset();
                health.restore_full();
            }
        }
    }
}
