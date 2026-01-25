use bevy::prelude::*;
use crate::components::breath::MatchState;
use crate::components::character::Player;
use crate::systems::game_state::{GameState, CharacterSelection};

/// Debug: Log current game state
pub fn debug_game_state(state: Res<State<GameState>>) {
    if state.is_changed() {
        info!("Game state changed to: {:?}", state.get());
    }
}

// ============================================================================
// UI COMPONENTS - Tags for despawning
// ============================================================================

#[derive(Component)]
pub struct CharacterSelectUI;

#[derive(Component)]
pub struct VictoryUI;

// ============================================================================
// CHARACTER SELECT SCREEN
// ============================================================================

/// Spawn character select screen UI
pub fn setup_character_select(mut commands: Commands) {
    commands
        .spawn((
            CharacterSelectUI,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::srgb(0.1, 0.1, 0.12).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                CharacterSelectUI,
                TextBundle::from_section(
                    "FUDOSHIN",
                    TextStyle {
                        font_size: 60.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ).with_style(Style {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                }),
            ));

            // Subtitle
            parent.spawn((
                CharacterSelectUI,
                TextBundle::from_section(
                    "The Immovable Mind",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::srgba(0.7, 0.7, 0.7, 1.0),
                        ..default()
                    },
                ).with_style(Style {
                    margin: UiRect::bottom(Val::Px(60.0)),
                    ..default()
                }),
            ));

            // Instructions
            parent.spawn((
                CharacterSelectUI,
                TextBundle::from_section(
                    "Press J (P1) or Delete (P2) to ready up",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::srgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ).with_style(Style {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                }),
            ));

            // Player status container
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(100.0),
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                // Player 1 status
                parent.spawn((
                    CharacterSelectUI,
                    TextBundle::from_section(
                        "Player 1: ...",
                        TextStyle {
                            font_size: 24.0,
                            color: Color::srgb(0.9, 0.3, 0.3),
                            ..default()
                        },
                    ).with_text_justify(JustifyText::Center),
                ));

                // Player 2 status
                parent.spawn((
                    CharacterSelectUI,
                    TextBundle::from_section(
                        "Player 2: ...",
                        TextStyle {
                            font_size: 24.0,
                            color: Color::srgb(0.3, 0.5, 0.9),
                            ..default()
                        },
                    ).with_text_justify(JustifyText::Center),
                ));
            });
        });
}

/// Handle character select input
pub fn character_select_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut selection: ResMut<CharacterSelection>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Debug: log any key press to see if input is working
    if keyboard.get_just_pressed().next().is_some() {
        info!("Key pressed in character select!");
    }

    // P1 ready up (J key)
    if keyboard.just_pressed(KeyCode::KeyJ) {
        selection.player1_ready = !selection.player1_ready;
        info!("Player 1 ready: {}", selection.player1_ready);
    }

    // P2 ready up (Numpad1 OR Delete for laptops)
    if keyboard.just_pressed(KeyCode::Numpad1) || keyboard.just_pressed(KeyCode::Delete) {
        selection.player2_ready = !selection.player2_ready;
        info!("Player 2 ready: {}", selection.player2_ready);
    }

    // Both ready - start match
    if selection.both_ready() {
        info!("Both players ready! Starting match...");
        next_state.set(GameState::InGame);
    }
}

/// Update character select UI text
pub fn update_character_select_ui(
    selection: Res<CharacterSelection>,
    mut query: Query<&mut Text, With<CharacterSelectUI>>,
) {
    if selection.is_changed() {
        info!("Character selection changed! P1: {}, P2: {}", selection.player1_ready, selection.player2_ready);
        info!("Text entities in character select: {}", query.iter().count());
    }

    for (i, mut text) in query.iter_mut().enumerate() {
        if selection.is_changed() {
            info!("Text[{}]: {}", i, text.sections[0].value);
        }

        // Update player status text (3rd and 4th text elements)
        if i == 3 {  // P1 status
            text.sections[0].value = format!(
                "Player 1: {}",
                if selection.player1_ready { "READY" } else { "..." }
            );
        } else if i == 4 {  // P2 status
            text.sections[0].value = format!(
                "Player 2: {}",
                if selection.player2_ready { "READY" } else { "..." }
            );
        }
    }
}

/// Cleanup character select UI when leaving state
pub fn cleanup_character_select(
    mut commands: Commands,
    query: Query<Entity, With<CharacterSelectUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// ============================================================================
// VICTORY SCREEN
// ============================================================================

/// Spawn victory screen UI
pub fn setup_victory_screen(
    mut commands: Commands,
    match_state: Res<MatchState>,
    player_query: Query<(&Player, &crate::components::breath::Breath)>,
) {
    info!("Setting up victory screen UI...");

    // Determine winner
    let winner = player_query
        .iter()
        .find(|(_, breath)| !breath.is_defeated())
        .map(|(player, _)| *player);

    let winner_text = match winner {
        Some(Player::One) => "Player 1 won this time",
        Some(Player::Two) => "Player 2 won this time",
        None => "Draw",
    };

    info!("Winner: {}", winner_text);

    commands
        .spawn((
            VictoryUI,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            // Victory message
            parent.spawn((
                VictoryUI,
                TextBundle::from_section(
                    winner_text,
                    TextStyle {
                        font_size: 48.0,
                        color: Color::srgb(1.0, 1.0, 0.3),
                        ..default()
                    },
                ).with_style(Style {
                    margin: UiRect::bottom(Val::Px(60.0)),
                    ..default()
                }),
            ));

            // Rematch option
            parent.spawn((
                VictoryUI,
                TextBundle::from_section(
                    "> Rematch",
                    TextStyle {
                        font_size: 28.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ).with_style(Style {
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                }),
            ));

            // Reselect option
            parent.spawn((
                VictoryUI,
                TextBundle::from_section(
                    "  Reselect Characters",
                    TextStyle {
                        font_size: 28.0,
                        color: Color::srgba(0.7, 0.7, 0.7, 1.0),
                        ..default()
                    },
                ),
            ));
        });
}

/// Track victory menu selection
#[derive(Resource, Debug, Default)]
pub struct VictoryMenuSelection {
    pub selected: VictoryOption,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum VictoryOption {
    #[default]
    Rematch,
    Reselect,
}

/// Handle victory screen input
pub fn victory_screen_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut selection: ResMut<VictoryMenuSelection>,
    mut next_state: ResMut<NextState<GameState>>,
    mut char_selection: ResMut<CharacterSelection>,
) {
    // Debug: Check if this system is even running
    if keyboard.get_just_pressed().next().is_some() {
        info!("Victory screen - key pressed!");
    }

    // Navigate menu (W/S or Up/Down arrows)
    if keyboard.just_pressed(KeyCode::KeyW) || keyboard.just_pressed(KeyCode::ArrowUp)
        || keyboard.just_pressed(KeyCode::KeyS) || keyboard.just_pressed(KeyCode::ArrowDown) {
        selection.selected = match selection.selected {
            VictoryOption::Rematch => VictoryOption::Reselect,
            VictoryOption::Reselect => VictoryOption::Rematch,
        };
        info!("Victory menu selection: {:?}", selection.selected);
    }

    // Confirm selection (J for P1, Delete or Numpad1 for P2)
    if keyboard.just_pressed(KeyCode::KeyJ) || keyboard.just_pressed(KeyCode::Numpad1) || keyboard.just_pressed(KeyCode::Delete) {
        match selection.selected {
            VictoryOption::Rematch => {
                info!("Rematch selected! Players will be respawned fresh...");
                // Players will be despawned on exit and respawned on enter
                next_state.set(GameState::InGame);
            }
            VictoryOption::Reselect => {
                info!("Reselect characters...");
                // Reset character selection
                char_selection.reset();
                // Players will be despawned, then respawned when returning from CharacterSelect
                next_state.set(GameState::CharacterSelect);
            }
        }

        // Reset selection for next time
        selection.selected = VictoryOption::Rematch;
    }
}

/// Update victory screen UI based on selection
pub fn update_victory_menu_ui(
    selection: Res<VictoryMenuSelection>,
    mut query: Query<&mut Text, With<VictoryUI>>,
) {
    if selection.is_changed() {
        info!("Selection changed! Current: {:?}", selection.selected);
        info!("Text entities count: {}", query.iter().count());
    }

    for (i, mut text) in query.iter_mut().enumerate() {
        if selection.is_changed() {
            info!("Text[{}]: {}", i, text.sections[0].value);
        }

        if i == 1 {  // Rematch option
            if selection.selected == VictoryOption::Rematch {
                text.sections[0].value = "> Rematch".to_string();
                text.sections[0].style.color = Color::WHITE;
            } else {
                text.sections[0].value = "  Rematch".to_string();
                text.sections[0].style.color = Color::srgba(0.7, 0.7, 0.7, 1.0);
            }
        } else if i == 2 {  // Reselect option
            if selection.selected == VictoryOption::Reselect {
                text.sections[0].value = "> Reselect Characters".to_string();
                text.sections[0].style.color = Color::WHITE;
            } else {
                text.sections[0].value = "  Reselect Characters".to_string();
                text.sections[0].style.color = Color::srgba(0.7, 0.7, 0.7, 1.0);
            }
        }
    }
}

/// Cleanup victory screen UI when leaving state
pub fn cleanup_victory_screen(
    mut commands: Commands,
    query: Query<Entity, With<VictoryUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Detect when match ends and transition to victory screen
pub fn detect_match_end(
    match_state: Res<MatchState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if match_state.match_over {
        info!("Match ended! Transitioning to victory screen...");
        next_state.set(GameState::Victory);
    }
}
