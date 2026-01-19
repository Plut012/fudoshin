use bevy::prelude::*;
use serde::Deserialize;

/// Game configuration loaded from assets/data/game_config.ron
#[derive(Debug, Clone, Deserialize, Resource)]
pub struct GameConfig {
    // Window and rendering
    pub window_width: f32,
    pub window_height: f32,
    pub clear_color: (f32, f32, f32, f32),

    // Stage
    pub stage_width: f32,
    pub stage_height: f32,

    // Movement
    pub walk_speed: f32,
    pub step_speed: f32,
    pub step_duration: u32,
    pub backdash_speed: f32,
    pub backdash_duration: u32,
    pub backdash_iframe_duration: u32,

    // Frame timing
    pub target_fps: u32,

    // Guard
    pub guard_depletion_rate: f32,
    pub light_guard_damage: f32,
    pub heavy_guard_damage: f32,
}
