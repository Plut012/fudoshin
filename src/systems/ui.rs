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
