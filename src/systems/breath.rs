use bevy::prelude::*;
use crate::components::breath::{Breath, RoundEndEvent, RoundEndReason};
use crate::components::health::Health;
use crate::components::state::AttackType;
use crate::events::combat_events::HitEvent;

/// Check for decisive blow conditions and trigger breath loss
pub fn check_decisive_blow(
    mut hit_events: EventReader<HitEvent>,
    mut round_end_events: EventWriter<RoundEndEvent>,
    health_query: Query<&Health>,
    mut breath_query: Query<&mut Breath>,
) {
    for event in hit_events.read() {
        // Skip if hit was blocked
        if event.was_blocked {
            continue;
        }

        // Get defender's health
        let defender_health = match health_query.get(event.defender) {
            Ok(health) => health,
            Err(_) => continue,
        };

        // Check if defender is in Broken state (vulnerable)
        if !defender_health.is_broken() {
            continue;
        }

        // Check if attack is Heavy or Grab (not Light)
        if !matches!(event.attack_type, AttackType::Heavy | AttackType::Grab) {
            continue;
        }

        // Get attacker's health
        let attacker_health = match health_query.get(event.attacker) {
            Ok(health) => health,
            Err(_) => continue,
        };

        // Check if attacker is healthy enough (Whole or Cut)
        if !attacker_health.can_decisive_blow() {
            continue;
        }

        // ALL CONDITIONS MET - DECISIVE BLOW!
        info!("DECISIVE BLOW! {:?} defeated with {:?}", event.attack_type, defender_health.state);

        // Remove breath from defender
        if let Ok(mut breath) = breath_query.get_mut(event.defender) {
            breath.lose_breath();

            info!(
                "Breath lost! Remaining: {}/{}",
                breath.current, breath.max
            );

            // Trigger round end event
            round_end_events.send(RoundEndEvent {
                winner: event.attacker,
                reason: RoundEndReason::DecisiveBlow,
            });
        }
    }
}

/// Visual indicator that decisive blow is available
pub fn visualize_decisive_blow_availability(
    mut gizmos: Gizmos,
    query: Query<(&Health, &Transform)>,
) {
    for (health, transform) in query.iter() {
        if health.is_broken() {
            // Draw pulsing danger indicator for broken player
            let time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f32();

            let pulse = (time * 3.0).sin() * 0.5 + 0.5; // 0-1 pulse
            let alpha = 0.3 + pulse * 0.4; // 0.3-0.7 alpha

            // Draw red danger circle above character
            gizmos.circle_2d(
                transform.translation.xy() + Vec2::new(0.0, 80.0),
                30.0,
                Color::srgba(1.0, 0.0, 0.0, alpha),
            );

            // Draw smaller inner circle
            gizmos.circle_2d(
                transform.translation.xy() + Vec2::new(0.0, 80.0),
                20.0,
                Color::srgba(1.0, 0.2, 0.2, alpha * 0.8),
            );
        }
    }
}

/// Update countdown timer before round starts
pub fn tick_round_countdown(
    mut match_state: ResMut<crate::components::breath::MatchState>,
    time: Res<Time>,
) {
    if !match_state.round_active && match_state.countdown > 0.0 && !match_state.match_over {
        match_state.countdown -= time.delta_seconds();

        if match_state.countdown <= 0.0 {
            // Start the round!
            match_state.start_round();
            info!("FIGHT! Round {} begins!", match_state.round_number);
        }
    }
}

/// Update round timer during active round
pub fn tick_round_timer(
    mut match_state: ResMut<crate::components::breath::MatchState>,
    time: Res<Time>,
) {
    if match_state.round_active {
        match_state.round_time -= time.delta_seconds();

        if match_state.is_timeout() {
            info!("TIMEOUT! Round {} ended by time", match_state.round_number);
            // Will be handled by check_timeout system
        }
    }
}

/// Check for timeout and determine winner
pub fn check_timeout(
    match_state: Res<crate::components::breath::MatchState>,
    mut round_end_events: EventWriter<RoundEndEvent>,
    query: Query<(Entity, &Health, &crate::components::character::Player)>,
) {
    if match_state.round_active && match_state.is_timeout() {
        // Find player with more health
        let mut players: Vec<_> = query.iter().collect();

        if players.len() == 2 {
            let (entity1, health1, player1) = players[0];
            let (entity2, health2, player2) = players[1];

            // Player with more health wins
            let winner = if health1.current > health2.current {
                entity1
            } else if health2.current > health1.current {
                entity2
            } else {
                // Equal health - both lose a breath (first player "wins" the event)
                entity1
            };

            info!(
                "TIMEOUT! Winner: {:?} ({:.1} HP vs {:.1} HP)",
                if winner == entity1 { player1 } else { player2 },
                health1.current,
                health2.current
            );

            round_end_events.send(RoundEndEvent {
                winner,
                reason: RoundEndReason::Timeout,
            });
        }
    }
}

/// Handle round end: reset positions, health, states
pub fn handle_round_end(
    mut round_end_events: EventReader<RoundEndEvent>,
    mut match_state: ResMut<crate::components::breath::MatchState>,
    mut query: Query<(
        Entity,
        &mut Health,
        &mut Breath,
        &mut Transform,
        &mut crate::components::state::CharacterState,
        &mut crate::components::initiative::Initiative,
        &mut crate::systems::pressure::Pressure,
        &mut crate::systems::momentum::Momentum,
        &crate::components::character::Player,
    )>,
) {
    for event in round_end_events.read() {
        info!("Round ended! Reason: {:?}", event.reason);

        // Determine loser (opposite of winner)
        let mut winner_entity = event.winner;
        let mut loser_entity = Entity::PLACEHOLDER;

        for (entity, _, _, _, _, _, _, _, _) in query.iter() {
            if entity != winner_entity {
                loser_entity = entity;
                break;
            }
        }

        // Process breath loss for loser
        if let Ok((_, _, mut breath, _, _, _, _, _, player)) = query.get_mut(loser_entity) {
            breath.lose_breath();
            info!("Player {:?} lost a breath! Remaining: {}/{}", player, breath.current, breath.max);

            // Check if match is over
            if breath.is_defeated() {
                info!("Player {:?} is defeated! Match over!", player);
                match_state.end_match(winner_entity);
                return; // Don't reset if match is over
            }
        }

        // Reset all players for next round
        for (entity, mut health, _, mut transform, mut state, mut initiative, mut pressure, mut momentum, player) in query.iter_mut() {
            // Reset health to full
            health.restore_full();

            // Reset position based on player
            let spawn_x = match *player {
                crate::components::character::Player::One => -300.0,
                crate::components::character::Player::Two => 300.0,
            };
            transform.translation = Vec3::new(spawn_x, 0.0, 0.0);

            // Reset combat states
            *state = crate::components::state::CharacterState::Idle;
            initiative.reset();
            pressure.intensity = 0;
            momentum.reset();

            debug!("Player {:?} reset for next round", player);
        }

        // End current round and prepare for next
        match_state.end_round();
        info!("Round {} complete. Next round countdown starting...", match_state.round_number - 1);
    }
}

/// Check for match victory
pub fn check_match_victory(
    match_state: Res<crate::components::breath::MatchState>,
    query: Query<(&Breath, &crate::components::character::Player)>,
) {
    if match_state.match_over {
        if let Some(winner_entity) = match_state.winner {
            // Find and display winner
            for (breath, player) in query.iter() {
                if breath.is_defeated() {
                    info!("MATCH OVER! Player {:?} defeated!", player);
                } else {
                    info!("VICTORY! Player {:?} wins the match!", player);
                }
            }
        }
    }
}

/// Debug: Log breath changes
pub fn debug_breath_display(
    query: Query<(&Breath, &crate::components::character::Player), Changed<Breath>>,
) {
    for (breath, player) in query.iter() {
        info!(
            "Player {:?} breaths: {}/{}",
            player, breath.current, breath.max
        );
    }
}
