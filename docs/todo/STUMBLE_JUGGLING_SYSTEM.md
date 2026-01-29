# Stumble & Juggling System

## Overview

Implement a **Smash Bros-inspired juggling system** where skilled players can string together long pressure sequences through reads and positioning. Stumble creates disadvantage states that are escapable but punishable, emphasizing **"reads over reactions"**.

## Philosophy

**Key Concept:** Hits don't guarantee follow-ups, but create **advantageous situations** where skilled attackers can continue pressure through:
- Reading tech timing
- Controlling stumble direction
- Positioning for extensions
- Wall bounces for setup
- Spike finishers for payoff

**Both players feel in control:** Defender can tech/escape, attacker must earn each extension through reads.

---

## Core Mechanics

### **Flow After Hit**

```
Hit lands → Hitstop (9-13f, both frozen)
         → Hitstun (defender frozen, existing system)
         → STUMBLE (new! defender disadvantaged but can act)
```

### **Stumble State**

**Duration:** 25-35 frames base (depends on move)

**Properties:**
- Defender can act but at disadvantage
- Subtle tumble/off-balance animation
- Direction-based (backward/forward/down)
- Escapable via tech (8-frame window)
- Extendable via directional hits

**Visual:**
- Small arrow at player's feet showing stumble direction
- Subtle sway/poor guard animation
- Dark red flash on successful tech

---

## Move Properties System

### **New Component: StumbleProperty**

```rust
// src/components/combat.rs
#[derive(Clone, Debug, PartialEq)]
pub enum StumbleDirection {
    Backward,   // Most common - knocks away
    Forward,    // Rare - knocks toward attacker
    Down,       // Low tumble
}

#[derive(Clone, Debug)]
pub enum StumbleProperty {
    None,                              // No stumble (most moves)
    Launcher(StumbleDirection, u32),   // Starts stumble (direction, duration in frames)
    Extender(StumbleDirection, u32),   // Continues stumble (direction, added frames)
    Spike,                             // Hard knockdown/wallbounce finisher
}
```

### **Add to MoveData**

```rust
// src/components/movelist.rs
pub struct MoveData {
    // ... existing fields
    pub stumble_property: StumbleProperty,
}
```

### **Move Assignments**

**Launchers (start juggle):**
- Neutral Heavy → Launcher(Backward, 30f)
- Forward Heavy → Launcher(Forward, 28f)
- Down Heavy → Launcher(Down, 25f)
- Back Heavy → Launcher(Backward, 32f)

**Extenders (continue juggle):**
- Neutral Light → Extender(Backward, 15f)
- Forward Light → Extender(Forward, 15f)
- Down Light → Extender(Down, 12f)
- Back Light → None (defensive reset)

**Spikes (finisher):**
- Down Heavy during stumble → Spike
- Neutral Heavy during stumble → Spike (armored version)

**No stumble:**
- Grab → None (different purpose)
- Back Light → None (defensive reset)

---

## Stumble State Component

```rust
// src/components/stumble.rs
#[derive(Component, Debug)]
pub struct StumbleState {
    /// Frames remaining in stumble
    pub frames_remaining: u32,

    /// Direction of stumble
    pub direction: StumbleDirection,

    /// Tech window frames (8f window during stumble)
    pub tech_window_start: u32,
    pub tech_window_end: u32,

    /// Can this stumble be teched?
    pub can_tech: bool,

    /// Number of times this stumble has been extended
    pub extension_count: u8,

    /// Was this stumble caused by counter hit?
    pub from_counter_hit: bool,
}

impl StumbleState {
    pub fn new(direction: StumbleDirection, duration: u32, can_tech: bool) -> Self {
        Self {
            frames_remaining: duration,
            direction,
            tech_window_start: 5,      // Tech window opens at frame 5
            tech_window_end: 13,        // Tech window closes at frame 13 (8f window)
            can_tech,
            extension_count: 0,
            from_counter_hit: false,
        }
    }

    pub fn is_in_tech_window(&self, elapsed: u32) -> bool {
        self.can_tech && elapsed >= self.tech_window_start && elapsed <= self.tech_window_end
    }

    pub fn extend(&mut self, direction: StumbleDirection, base_frames: u32) {
        // Diminishing returns on extensions
        let scaled_frames = match self.extension_count {
            0 => base_frames,           // 15f
            1 => (base_frames * 80) / 100,  // 12f (80%)
            2 => (base_frames * 66) / 100,  // 10f (66%)
            3 => (base_frames * 53) / 100,  // 8f (53%)
            _ => 0,  // Max 4 extensions, then auto-end
        };

        self.frames_remaining += scaled_frames;
        self.direction = direction;
        self.extension_count += 1;

        // Reset tech window for new extension
        self.tech_window_start = self.frames_remaining - scaled_frames + 5;
        self.tech_window_end = self.tech_window_start + 8;
    }
}
```

---

## Implementation Plan

### **Phase 1: Basic Stumble State**

**Goal:** Add stumble state component and basic triggering

**1. Create Components**
- `src/components/stumble.rs` - StumbleState component
- Add `StumbleProperty` to `src/components/combat.rs`
- Add `stumble_property` field to `MoveData`

**2. Update Move Definitions**
- Add `stumble_property` to all moves in `Movelist::default_character()`
- Assign launchers to Heavy attacks
- Assign extenders to Light attacks
- Mark Spikes appropriately

**3. Create Stumble System**

```rust
// src/systems/stumble.rs

/// Apply stumble state when launcher hits
pub fn apply_stumble_on_hit(
    mut commands: Commands,
    mut hit_events: EventReader<HitEvent>,
    attacker_query: Query<(&CharacterState, &Movelist)>,
) {
    for event in hit_events.read() {
        if event.was_blocked {
            continue;
        }

        if let Ok((state, movelist)) = attacker_query.get(event.attacker) {
            if let CharacterState::Attacking { attack_type, direction, .. } = state {
                if let Some(move_data) = movelist.get_move(*attack_type, *direction) {
                    match &move_data.stumble_property {
                        StumbleProperty::Launcher(stumble_dir, duration) => {
                            let can_tech = !event.is_counter_hit;  // Counter hits can't be teched
                            let mut stumble = StumbleState::new(
                                stumble_dir.clone(),
                                *duration,
                                can_tech
                            );

                            if event.is_counter_hit {
                                stumble.from_counter_hit = true;
                                stumble.frames_remaining += 10;  // Bonus duration
                            }

                            commands.entity(event.defender).insert(stumble);

                            info!(
                                "Stumble applied: {:?} direction, {} frames, can_tech: {}",
                                stumble_dir, duration, can_tech
                            );
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

/// Tick down stumble duration
pub fn process_stumble(
    mut commands: Commands,
    mut query: Query<(Entity, &mut StumbleState)>,
) {
    for (entity, mut stumble) in query.iter_mut() {
        if stumble.frames_remaining > 0 {
            stumble.frames_remaining -= 1;
        } else {
            // Stumble ended naturally
            commands.entity(entity).remove::<StumbleState>();
            info!("Stumble ended naturally");
        }

        // Max extension limit
        if stumble.extension_count >= 4 {
            commands.entity(entity).remove::<StumbleState>();
            info!("Stumble ended: max extensions reached");
        }
    }
}
```

**4. Visual Feedback**

```rust
/// Show stumble direction arrow at player's feet
pub fn visualize_stumble_direction(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &StumbleState)>,
) {
    for (transform, stumble) in query.iter() {
        let pos = transform.translation.truncate();
        let feet_pos = pos + Vec2::new(0.0, -100.0);  // Below character

        // Arrow color based on tech window
        let color = if stumble.can_tech {
            Color::srgb(1.0, 0.8, 0.0)  // Yellow - can tech
        } else {
            Color::srgb(1.0, 0.3, 0.3)  // Red - cannot tech
        };

        // Draw directional arrow
        let arrow_dir = match stumble.direction {
            StumbleDirection::Backward => Vec2::new(-20.0, 0.0),
            StumbleDirection::Forward => Vec2::new(20.0, 0.0),
            StumbleDirection::Down => Vec2::new(0.0, -20.0),
        };

        // Arrow shaft
        gizmos.line_2d(feet_pos, feet_pos + arrow_dir, color);

        // Arrowhead
        let arrow_tip = feet_pos + arrow_dir;
        let perpendicular = Vec2::new(-arrow_dir.y, arrow_dir.x).normalize() * 5.0;
        gizmos.line_2d(arrow_tip, arrow_tip - arrow_dir.normalize() * 8.0 + perpendicular, color);
        gizmos.line_2d(arrow_tip, arrow_tip - arrow_dir.normalize() * 8.0 - perpendicular, color);
    }
}

/// Subtle stumble animation
pub fn visualize_stumble_state(
    mut query: Query<(&StumbleState, &mut Sprite)>,
) {
    for (stumble, mut sprite) in query.iter_mut() {
        // Subtle darkening to show disadvantage
        let darken = 0.9;
        sprite.color = Color::srgb(
            sprite.color.to_srgba().red * darken,
            sprite.color.to_srgba().green * darken,
            sprite.color.to_srgba().blue * darken,
        );
    }
}
```

---

### **Phase 2: Tech System**

**Goal:** Allow defender to escape stumble early with tech

**1. Tech Input Detection**

```rust
/// Handle tech input during stumble
pub fn handle_tech_input(
    mut commands: Commands,
    inputs: Res<CurrentInputs>,
    mut query: Query<(Entity, &Player, &mut StumbleState, &StateTimer)>,
) {
    for (entity, player, mut stumble, timer) in query.iter_mut() {
        if !stumble.can_tech {
            continue;
        }

        let input = match player {
            Player::One => &inputs.player_one,
            Player::Two => &inputs.player_two,
        };

        // Any attack button techs
        let tech_pressed = input.light_attack || input.heavy_attack || input.grab;

        if tech_pressed && stumble.is_in_tech_window(timer.elapsed) {
            // Successful tech!
            commands.entity(entity).remove::<StumbleState>();

            // Apply small frame disadvantage (attacker still has advantage)
            commands.entity(entity).insert(Initiative::new(-5));

            info!("TECH successful! -5 frames but escaped stumble");

            // Visual feedback - dark red flash
            // (Will implement in visual system)
        }
    }
}
```

**2. Tech Visual Feedback**

```rust
/// Flash dark red on successful tech
pub fn tech_flash_effect(
    mut query: Query<(&Player, &mut Sprite), (With<Character>, Added<Initiative>)>,
) {
    for (player, mut sprite) in query.iter_mut() {
        // Dark red flash
        sprite.color = Color::srgb(0.6, 0.1, 0.1);

        // Flash will fade naturally through other systems
    }
}
```

---

### **Phase 3: Stumble Extensions**

**Goal:** Allow attackers to extend stumble with directional hits

**1. Extension System**

```rust
/// Extend stumble when extender moves hit during stumble
pub fn extend_stumble_on_hit(
    mut hit_events: EventReader<HitEvent>,
    attacker_query: Query<(&CharacterState, &Movelist)>,
    mut defender_query: Query<&mut StumbleState>,
) {
    for event in hit_events.read() {
        if event.was_blocked {
            continue;
        }

        // Check if defender is in stumble state
        if let Ok(mut stumble) = defender_query.get_mut(event.defender) {
            // Check if attacker used an extender move
            if let Ok((state, movelist)) = attacker_query.get(event.attacker) {
                if let CharacterState::Attacking { attack_type, direction, .. } = state {
                    if let Some(move_data) = movelist.get_move(*attack_type, *direction) {
                        if let StumbleProperty::Extender(extend_dir, extend_frames) = &move_data.stumble_property {
                            // Extend the stumble!
                            stumble.extend(extend_dir.clone(), *extend_frames);

                            info!(
                                "Stumble extended! Direction: {:?}, Extension #{}, Added frames: {}",
                                extend_dir,
                                stumble.extension_count,
                                extend_frames
                            );
                        }
                    }
                }
            }
        }
    }
}
```

---

### **Phase 4: Wall Bounce**

**Goal:** Add wall bounce mechanic for stage positioning gameplay

**1. Wall Bounce Detection**

```rust
// src/systems/stumble.rs

/// Detect when stumbling player hits wall
pub fn detect_wall_bounce(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut StumbleState)>,
    stage_bounds: Query<&StageBounds>,  // Assuming we have stage bounds
) {
    let bounds = stage_bounds.single();

    for (entity, transform, mut stumble) in query.iter_mut() {
        let x = transform.translation.x;

        // Check if hit left or right wall
        let hit_left_wall = x <= bounds.left && matches!(stumble.direction, StumbleDirection::Backward);
        let hit_right_wall = x >= bounds.right && matches!(stumble.direction, StumbleDirection::Forward);

        if hit_left_wall || hit_right_wall {
            // WALL BOUNCE!

            // Reverse direction
            stumble.direction = match stumble.direction {
                StumbleDirection::Backward => StumbleDirection::Forward,
                StumbleDirection::Forward => StumbleDirection::Backward,
                d => d,  // Down doesn't wall bounce
            };

            // Add extra vulnerability time
            stumble.frames_remaining += 20;

            // Cannot tech during wall bounce
            stumble.can_tech = false;

            info!("WALL BOUNCE! Direction reversed, +20f vulnerability, no tech");

            // Visual/audio feedback
            // TODO: Screen shake, impact sound
        }
    }
}
```

**2. Wall Bounce Visual**

```rust
/// Visual effect for wall bounce
pub fn visualize_wall_bounce(
    query: Query<(&Transform, &StumbleState), Changed<StumbleState>>,
    mut gizmos: Gizmos,
) {
    for (transform, stumble) in query.iter() {
        if !stumble.can_tech && stumble.frames_remaining > 40 {
            // Just wall bounced (high frames + no tech)
            let pos = transform.translation.truncate();

            // Impact effect - expanding circle
            for i in 0..3 {
                let radius = 30.0 + (i as f32 * 15.0);
                gizmos.circle_2d(pos, radius, Color::srgba(1.0, 0.5, 0.0, 0.7));
            }
        }
    }
}
```

---

### **Phase 5: Spike Finisher**

**Goal:** Add high-damage spike moves that work during stumble

**1. Spike Detection**

```rust
/// Handle spike moves hitting stumbling opponents
pub fn handle_spike_finisher(
    mut commands: Commands,
    mut hit_events: EventReader<HitEvent>,
    attacker_query: Query<(&CharacterState, &Movelist)>,
    defender_query: Query<&StumbleState>,
) {
    for event in hit_events.read() {
        if event.was_blocked {
            continue;
        }

        // Check if defender is stumbling
        if let Ok(_stumble) = defender_query.get(event.defender) {
            // Check if attacker used a spike move
            if let Ok((state, movelist)) = attacker_query.get(event.attacker) {
                if let CharacterState::Attacking { attack_type, direction, .. } = state {
                    if let Some(move_data) = movelist.get_move(*attack_type, *direction) {
                        if matches!(move_data.stumble_property, StumbleProperty::Spike) {
                            // SPIKE HIT!

                            // Remove stumble state (hard knockdown)
                            commands.entity(event.defender).remove::<StumbleState>();

                            // Apply extra damage (handled by existing damage system)
                            // Spike moves should have high base damage (20-25)

                            info!("SPIKE FINISHER landed! Hard knockdown");

                            // Visual: screen freeze, impact flash
                            // TODO: Extra hitstop, special visual
                        }
                    }
                }
            }
        }
    }
}
```

---

## Integration Points

### **System Execution Order**

Add to `src/plugins/core_game.rs`:

```rust
.add_systems(Update, (
    // ... existing systems
    stumble::apply_stumble_on_hit,      // Apply stumble from launchers
    stumble::extend_stumble_on_hit,     // Extend stumble with extenders
    stumble::handle_spike_finisher,     // Detect spike finishers
).chain().run_if(in_state(GameState::InGame)))

.add_systems(Update, (
    stumble::process_stumble,           // Tick stumble duration
    stumble::handle_tech_input,         // Handle tech attempts
    stumble::detect_wall_bounce,        // Wall bounce detection
).chain().run_if(in_state(GameState::InGame)))

.add_systems(Update, (
    stumble::visualize_stumble_direction,  // Arrow at feet
    stumble::visualize_stumble_state,      // Subtle visual
    stumble::visualize_wall_bounce,        // Wall impact
    stumble::tech_flash_effect,            // Tech flash
).run_if(in_state(GameState::InGame)))
```

---

## Testing Checklist

### **Phase 1: Basic Stumble**
- [ ] Heavy attacks trigger stumble state
- [ ] Stumble duration ticks down correctly
- [ ] Stumble direction matches move property
- [ ] Counter hits prevent teching
- [ ] Arrow appears at player's feet showing direction
- [ ] Defender can still move during stumble (at disadvantage)

### **Phase 2: Tech System**
- [ ] Tech window is 8 frames (frame 5-13 of stumble)
- [ ] Pressing any attack button during window techs
- [ ] Successful tech removes stumble state
- [ ] Successful tech gives attacker +5 frame advantage
- [ ] Dark red flash on tech
- [ ] Failed tech (outside window) does nothing

### **Phase 3: Extensions**
- [ ] Light attacks extend stumble when hitting stumbling opponent
- [ ] Extension adds frames based on move property
- [ ] Stumble direction changes to match extension direction
- [ ] Diminishing returns work (15f → 12f → 10f → 8f → 0f)
- [ ] Max 4 extensions before auto-end
- [ ] Tech window resets on each extension

### **Phase 4: Wall Bounce**
- [ ] Hitting wall during stumble triggers bounce
- [ ] Direction reverses on bounce
- [ ] +20 frames added to stumble
- [ ] Cannot tech during wall bounce
- [ ] Visual impact effect appears
- [ ] Only triggers when moving toward wall

### **Phase 5: Spike**
- [ ] Spike moves deal high damage to stumbling opponents
- [ ] Spike causes hard knockdown (removes stumble)
- [ ] Spike works after wall bounce
- [ ] Visual feedback is impactful
- [ ] Cannot spike non-stumbling opponents (or deals normal damage)

### **Integration Tests**
- [ ] Full combo: Counter Heavy → Forward Light → Back Light → Wall Bounce → Spike
- [ ] Defender successfully techs and escapes
- [ ] Defender panic evades during stumble (gets punished)
- [ ] Multiple players can be stumbling simultaneously
- [ ] Stumble doesn't interfere with existing systems (hitstop, initiative, etc.)

---

## Example Juggle Sequences

### **Sequence 1: Basic Launcher → Extension**
```
1. Neutral Heavy lands → Launcher(Backward, 30f)
2. Opponent stumbles backward, tries to tech at frame 10
3. Attacker dashes forward → Forward Light (Extender, 15f)
4. Stumble extended! Now Forward direction, 15f added
5. Opponent techs successfully → -5 frames but escaped
```

### **Sequence 2: Counter Hit → Wall Bounce → Spike**
```
1. Counter Hit Neutral Heavy → Launcher(Backward, 40f, NO TECH)
2. Opponent stumbles backward (guaranteed, cannot tech)
3. Forward Light extends → Forward direction, 15f added
4. Back Light extends → Backward direction, 12f added
5. Hits wall → WALL BOUNCE! Forward direction, +20f, NO TECH
6. Down Heavy (Spike) → 25 damage, hard knockdown
Total: ~60% damage combo
```

### **Sequence 3: Read the Tech**
```
1. Neutral Heavy → Launcher(Backward, 30f)
2. Attacker waits, watches for tech
3. Opponent techs at frame 8 (dark red flash)
4. Attacker immediately attacks → catches -5 frame disadvantage
5. New launcher starts fresh juggle
```

---

## Future Enhancements (Post-MVP)

- **Air stumble:** Vertical stumble direction (true Smash Bros juggling)
- **Stage hazards:** Stumble into hazards for unique interactions
- **Character-specific spike animations:** Each character has unique spike
- **Tumble DI:** Directional influence during stumble (slight control)
- **Tech roll:** Tech with direction to roll away/toward
- **Stumble meter:** Visual meter showing remaining stumble time

---

## Notes

- Stumble complements existing hitstun/hitstop systems
- Does NOT replace chain/cancel system (they work together)
- Emphasizes **positioning and reads** over guaranteed combos
- Both players understand what's happening (clear cause/effect)
- Fits Fudoshin's "immovable mind" philosophy perfectly

**Key Design Principle:** The attacker earns each extension. The defender always has counterplay. Victory through superior understanding, not execution.
