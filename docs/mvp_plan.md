# Fudoshin — Bevy Implementation Plan

*Building the immovable mind with clean architecture and data-driven design*

---

## Philosophy

**Data-driven, not code-driven.**
Frame data, damage values, move properties — all live in `.ron` files. Balance changes = edit text, hot-reload, test. No code compilation needed.

**Clear boundaries, simple mental model.**
Each system does one thing. Each component represents one concept. The codebase reads like the game design document.

**Extensibility by default.**
Adding a new character means creating a `.ron` file and sprites. Core systems never change.

---

## Technology Stack

- **Bevy 0.14.2** (stable)
- **Rust 1.75+** (stable toolchain)
- **Resolution:** 1280x720 (balanced for hand-drawn sprites)
- **Target:** 60 FPS locked (fighting game standard)
- **Input:** Keyboard (Phase 1), Gamepad (Phase 7)
- **Data Format:** RON (Rusty Object Notation) for all game data
- **Hot Reload:** `bevy_asset_loader` + watching file changes

### Development Tools

- **bevy_inspector_egui** — inspect entities/components live
- **bevy_editor_pls** — pause, step frames, fly camera
- **bevy_framepace** — locked 60 FPS
- **iyes_perf_ui** — FPS counter

---

## Repository Structure

```
fudoshin/
├── Cargo.toml
├── assets/
│   ├── data/                    # ← All game balance lives here
│   │   ├── characters/
│   │   │   ├── ronin.ron        # Frame data, move properties
│   │   │   ├── monk.ron
│   │   │   └── ...
│   │   ├── moves/               # Move definitions (composable)
│   │   │   ├── light_attacks.ron
│   │   │   ├── heavy_attacks.ron
│   │   │   └── grabs.ron
│   │   └── game_config.ron      # Global values (frame rates, physics)
│   ├── sprites/
│   │   └── characters/
│   ├── audio/
│   │   ├── sfx/
│   │   └── ambient/
│   └── fonts/
├── src/
│   ├── main.rs                  # Entry point, plugin registration
│   ├── components/              # ← Pure data, no logic
│   │   ├── mod.rs
│   │   ├── character.rs         # Health state, position
│   │   ├── combat.rs            # Attack, Defense, Hitbox
│   │   ├── state.rs             # CharacterState enum, StateTimer
│   │   ├── initiative.rs        # Initiative, Momentum, Desperation
│   │   └── guard.rs             # GuardMeter, GuardBreak
│   ├── systems/                 # ← Each system = one responsibility
│   │   ├── mod.rs
│   │   ├── movement.rs          # Walk, step, backdash, jump
│   │   ├── input.rs             # Input buffering, player mapping
│   │   ├── attack.rs            # Hitbox activation, damage application
│   │   ├── defense.rs           # Block, parry, evade
│   │   ├── initiative.rs        # Initiative gain/loss, frame advantage
│   │   ├── guard.rs             # Guard meter fill/drain, guard break
│   │   ├── state_machine.rs     # State transitions
│   │   ├── collision.rs         # Hitbox/hurtbox detection
│   │   ├── decisive_blow.rs     # Kill conditions, Final Parry
│   │   ├── breath.rs            # Round management, reset logic
│   │   └── frame_data.rs        # Startup/active/recovery tracking
│   ├── resources/               # ← Global game state
│   │   ├── mod.rs
│   │   ├── match_state.rs       # Current match, scores, breaths
│   │   └── frame_config.rs      # Frame timings (loaded from RON)
│   ├── events/                  # ← Decoupled communication
│   │   ├── mod.rs
│   │   ├── combat_events.rs     # HitEvent, ParryEvent, GuardBreakEvent
│   │   └── match_events.rs      # BreathTakenEvent, MatchEndEvent
│   ├── data/                    # ← Data structure definitions
│   │   ├── mod.rs
│   │   ├── character_data.rs    # Loaded from .ron
│   │   ├── move_data.rs         # Loaded from .ron
│   │   └── frame_data.rs        # Startup, active, recovery structs
│   └── plugins/                 # ← Grouped systems
│       ├── mod.rs
│       ├── core_game.rs         # Movement, combat, state machine
│       ├── combat_mechanics.rs  # Initiative, guard, decisive blow
│       └── debug.rs             # Inspector, frame stepper, hitbox viz
└── docs/
    ├── mvp_plan.md              # This file
    └── architecture.md          # ECS patterns, data flow diagrams
```

---

## Design Patterns for Clean Architecture

### 1. Data-Driven Balance

**All tunable values live in `assets/data/`**.

Example: `assets/data/characters/ronin.ron`
```ron
(
    name: "Ronin",
    moves: {
        "neutral_light": (
            startup: 6,
            active: 2,
            recovery: 10,
            damage: StateDelta(1),        // Whole → Cut
            on_block: -2,                 // Frame disadvantage
            hitbox: Rect(20, 0, 40, 60),  // x, y, w, h offset
            properties: [ChainableIntoSelf(2)], // Can chain 2x
        ),
        "neutral_heavy": (
            startup: 14,
            active: 4,
            recovery: 18,
            damage: StateDelta(2),        // Whole → Wounded
            on_block: -8,
            hitbox: Rect(30, 10, 60, 70),
            properties: [LightArmor],     // Absorbs one light hit
        ),
        // ... more moves
    },
    stance: Some((
        name: "DrawnBlade",
        entry: HoldHeavy,
        guard_drain: 0.03,              // per second
        release_move: "drawn_slash",
    )),
)
```

**To change balance:** Edit the file. Save. Hot-reload. Test. No compilation.

### 2. Component Composition

Components are **pure data**. No methods, no logic.

```rust
// src/components/character.rs
#[derive(Component)]
pub struct HealthState {
    pub current: HealthLevel,
}

#[derive(Component)]
pub struct Initiative;  // Tag component: holder has initiative

#[derive(Component)]
pub struct GuardMeter {
    pub current: f32,   // 0.0 to 1.0
    pub max: f32,
}

#[derive(Component)]
pub struct FrameTimer {
    pub elapsed: u32,   // Frame count
    pub target: u32,    // When to transition
}
```

### 3. Single-Responsibility Systems

Each system does **one thing**. Systems communicate via Events.

```rust
// src/systems/guard.rs

/// Fills guard meter when blocking attacks
pub fn fill_guard_on_block(
    mut hit_events: EventReader<HitEvent>,
    mut guard_query: Query<&mut GuardMeter>,
) {
    for event in hit_events.read() {
        if event.was_blocked {
            if let Ok(mut guard) = guard_query.get_mut(event.defender) {
                guard.current += event.guard_damage;
            }
        }
    }
}

/// Triggers guard break when meter fills
pub fn check_guard_break(
    mut commands: Commands,
    mut guard_query: Query<(Entity, &GuardMeter), Changed<GuardMeter>>,
    mut break_events: EventWriter<GuardBreakEvent>,
) {
    for (entity, guard) in guard_query.iter_mut() {
        if guard.current >= guard.max {
            break_events.send(GuardBreakEvent { entity });
            // Add Staggered component, reset guard
        }
    }
}

/// Depletes guard meter over time when not blocking
pub fn drain_guard_meter(
    time: Res<Time>,
    mut guard_query: Query<&mut GuardMeter, Without<Blocking>>,
) {
    for mut guard in guard_query.iter_mut() {
        guard.current = (guard.current - 0.05 * time.delta_seconds()).max(0.0);
    }
}
```

**Key principles:**
- Descriptive names: you know what it does from the name
- Queries are specific: only entities that need this logic
- Events decouple systems: guard break triggers stagger elsewhere
- No hidden state: all data visible in components

### 4. Plugin Organization

Group related systems into plugins for clear boundaries.

```rust
// src/plugins/combat_mechanics.rs
pub struct CombatMechanicsPlugin;

impl Plugin for CombatMechanicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<HitEvent>()
            .add_event::<ParryEvent>()
            .add_event::<GuardBreakEvent>()
            .add_systems(Update, (
                // Guard systems
                fill_guard_on_block,
                drain_guard_meter,
                check_guard_break,

                // Initiative systems
                grant_initiative_on_hit,
                apply_initiative_frame_advantage,
                decay_initiative,
            ).chain());  // Run in order
    }
}
```

### 5. Extensibility Pattern: Character Loading

New characters = new `.ron` file. Core systems unchanged.

```rust
// System that loads any character from data
pub fn spawn_character(
    commands: &mut Commands,
    character_data: &CharacterData,
    player: Player,
    position: Vec2,
) -> Entity {
    commands.spawn((
        Character,
        player,
        HealthState::new(),
        GuardMeter::default(),
        Transform::from_translation(position.extend(0.0)),
        // Load moves from character_data into MoveSet component
        MoveSet::from_data(&character_data.moves),
        // Load stance if character has one
    )).id()
}
```

Adding "The Shade" character:
1. Create `assets/data/characters/shade.ron`
2. Add sprites to `assets/sprites/characters/shade/`
3. Done. No code changes.

---

## Phase-by-Phase Implementation

### Phase 0: Project Initialization (Day 1)

**Goal:** Repository setup, dependencies, hot-reload working.

**Tasks:**
- Create Bevy project with `cargo init`
- Add dependencies:
  ```toml
  [dependencies]
  bevy = "0.14"
  bevy_inspector_egui = "0.25"
  bevy_framepace = "0.17"
  serde = { version = "1.0", features = ["derive"] }
  ron = "0.8"
  ```
- Create folder structure
- Setup hot-reload for `.ron` files
- Verify: Change a value in RON, see it update without restart

**Deliverable:** Empty window, 60 FPS, hot-reload confirmed.

---

### Phase 1: Movement Foundation (Week 1)

**Goal:** Two colored rectangles that move beautifully. Fast, responsive, crosses stage in ~2 seconds.

#### 1.1 Basic Setup
- Camera (1280x720 viewport)
- Stage boundaries (entity with collision)
- Two rectangles (Player 1 red, Player 2 blue)
- Keyboard input mapping:
  - P1: WASD movement, JKL actions
  - P2: Arrow keys movement, Numpad 1/2/3 actions

#### 1.2 Walk Movement
**Components:**
```rust
Velocity(Vec2)
MaxSpeed(f32)  // Loaded from character data
Player(u8)     // 1 or 2
```

**System:**
```rust
fn walk_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &MaxSpeed, &mut Velocity)>,
) {
    // Apply velocity from input
    // Clamp to MaxSpeed
}
```

**Data:** `assets/data/game_config.ron`
```ron
(
    walk_speed: 300.0,  // pixels per second
    stage_width: 1000.0,
)
```

**Test:** Walk left/right. Should feel snappy, cross stage in ~2 seconds.

#### 1.3 Step and Backdash
**Components:**
```rust
StepState { frames_remaining: u32 }
BackdashState { frames_remaining: u32, has_iframes: bool }
```

**System:**
```rust
fn execute_step(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut StepState)>,
) {
    // Move quickly over 6 frames
    // Remove component when done
}
```

**Test:** Step feels like commitment (brief moment you can't act). Backdash feels safe but loses space.

#### 1.4 State Machine Core
**Component:**
```rust
enum CharacterState {
    Idle,
    Walking,
    Stepping,
    Backdashing,
    // More added in Phase 2
}

struct StateTimer {
    elapsed: u32,
    duration: u32,
}
```

**System:**
```rust
fn state_machine_transitions(
    mut query: Query<(&mut CharacterState, &mut StateTimer)>,
) {
    // Tick timers
    // Transition when timer expires
}
```

**Milestone:** Two players control rectangles smoothly. Movement feels **fast and intentional**.

---

### Phase 2: Core Combat Triangle (Weeks 2-3)

**Goal:** Attack/Grab vs Block/Parry/Evade working. Exchanges feel **responsive and meaningful**.

#### 2.1 Hitbox/Hurtbox System
**Components:**
```rust
Hitbox {
    rect: Rect,
    active: bool,
    damage: StateDelta,
    properties: Vec<AttackProperty>,  // LightArmor, Unblockable, etc
}

Hurtbox {
    rect: Rect,
}
```

**System:**
```rust
fn detect_hits(
    hitbox_query: Query<(Entity, &Hitbox, &GlobalTransform)>,
    hurtbox_query: Query<(Entity, &Hurtbox, &GlobalTransform)>,
    mut hit_events: EventWriter<HitEvent>,
) {
    // AABB collision detection
    // Send HitEvent on collision
}
```

#### 2.2 Attack Basics
**States:** `Attacking { move_id, phase: AttackPhase }`
```rust
enum AttackPhase {
    Startup { frames_remaining: u32 },
    Active { frames_remaining: u32 },
    Recovery { frames_remaining: u32 },
}
```

**Data:** Loaded from character RON (see earlier example).

**System:**
```rust
fn progress_attack_animation(
    mut query: Query<(&mut CharacterState, &mut StateTimer, &mut Hitbox)>,
) {
    // Startup → Active: enable hitbox
    // Active → Recovery: disable hitbox
    // Recovery → Idle: clear state
}
```

**Frame Data (in RON):**
- Light: 6f startup, 2f active, 10f recovery
- Heavy: 14f startup, 4f active, 18f recovery

#### 2.3 Block and Guard Meter
**Components:**
```rust
Blocking { started_frame: u32 }
GuardMeter { current: f32, max: f32 }
```

**System:** (see earlier guard example)

#### 2.4 Parry
**State:** `ParryAttempt { window_remaining: u32 }`

**System:**
```rust
fn check_parry_success(
    mut hit_events: EventReader<HitEvent>,
    parry_query: Query<&ParryAttempt>,
    mut parry_events: EventWriter<ParryEvent>,
) {
    for hit in hit_events.read() {
        if parry_query.get(hit.defender).is_ok() {
            // Parry succeeded!
            parry_events.send(ParryEvent { defender: hit.defender });
            // Brief time freeze, grant initiative, etc
        }
    }
}
```

**Feel:** Parry success = screen freeze for 3 frames, distinct sound, visual flash.

#### 2.5 Grab and Evade
- Grab: Unblockable, loses to attacks (stuffed during startup)
- Evade: 4 frames i-frames, directional

**Milestone:** The triangle works. Players can have real exchanges. Reads matter.

---

### Phase 3: Initiative System (Week 4)

**Goal:** Momentum matters. Landing a hit gives tangible advantage.

#### 3.1 Initiative Tracking
**Component:**
```rust
Initiative  // Tag component
```

**Resource:**
```rust
InitiativeHolder(Option<Entity>)
```

**Systems:**
- `grant_initiative_on_hit` — Hit events give Initiative
- `grant_initiative_on_parry` — Parry events give Initiative
- `revoke_initiative_on_whiff` — Missing attacks loses Initiative

#### 3.2 Frame Advantage
**System:**
```rust
fn apply_initiative_frame_advantage(
    initiative: Res<InitiativeHolder>,
    mut query: Query<(Entity, &mut StateTimer)>,
) {
    if let Some(holder) = initiative.0 {
        if let Ok((entity, mut timer)) = query.get_mut(holder) {
            // Reduce startup by 2 frames
            timer.duration = timer.duration.saturating_sub(2);
        }
    }
}
```

**Visual:** Faint ink trail behind Initiative holder.

**Milestone:** Players feel when they're "winning" neutral. Initiative flows naturally.

---

### Phase 4: Health, Breaths, Momentum (Weeks 5-6)

**Goal:** Matches have structure. Kills are earned. Drama builds.

#### 4.1 Health States
**Component:**
```rust
enum HealthLevel {
    Whole,
    Cut,
    Wounded,
    Broken,
}
```

**System:**
```rust
fn apply_damage(
    mut hit_events: EventReader<HitEvent>,
    mut health_query: Query<&mut HealthState>,
) {
    for hit in hit_events.read() {
        if let Ok(mut health) = health_query.get_mut(hit.defender) {
            health.current = health.current.decrease_by(hit.damage);
            // Visual: change sprite/animation
        }
    }
}
```

**No health bar.** Visual tells story (stance changes, breathing visible).

#### 4.2 Breath System (Round Management)
**Resource:**
```rust
MatchState {
    p1_breaths: u8,
    p2_breaths: u8,
    state: MatchPhase,
}
```

**System:**
```rust
fn check_for_death(
    mut match_state: ResMut<MatchState>,
    health_query: Query<(&Player, &HealthState)>,
    mut breath_events: EventWriter<BreathTakenEvent>,
) {
    // If health = Defeated, decrement breaths, send event
}

fn reset_after_breath(
    mut breath_events: EventReader<BreathTakenEvent>,
    mut commands: Commands,
    // Reset positions, health, guard, initiative
) {
    // 1.5 second pause
    // Visual: "BREATH" indicator
    // Grant Momentum to killer
}
```

#### 4.3 Momentum State
**Component:**
```rust
Momentum {
    timer: f32,  // 3 seconds
    guard_bonus: f32,  // +10%
}
```

**System:**
- Start with Initiative
- Visual indicator (forward energy)
- Ends on hit or timeout

#### 4.4 Desperation State
**Trigger:** Down 0-2 in Breaths.

**Component:**
```rust
Desperation {
    damage_multiplier: f32,  // 1.15
}
```

**Effect:** Activate Final Stand immediately, +15% damage.

#### 4.5 Decisive Blow
**Conditions:** Target is (Wounded OR Broken) AND Staggered.

**System:**
```rust
fn check_decisive_blow_conditions(
    mut commands: Commands,
    health_query: Query<(&HealthState, &CharacterState)>,
    // If conditions met, add DecisiveBlowAvailable component
)
```

**Execution:**
- Heavy input becomes Decisive Blow
- 24 frame wind-up (telegraphed)
- Defender can attempt Final Parry (4f window)
- Success = instant kill

**Feel:** Silence before strike. Screen holds. Ink splatter. Death.

**Milestone:** Matches flow naturally. Momentum shifts. Comebacks possible. Kills feel **earned**.

---

### Phase 5: First Character Complete (Weeks 7-8)

**Goal:** Ronin is fully playable with all moves and Drawn Blade stance.

#### 5.1 Full Move Set
Load from `assets/data/characters/ronin.ron`:
- Neutral/Forward/Back Light
- Neutral/Forward/Back Heavy
- Grab

All frame data, hitboxes, damage defined in data.

#### 5.2 Drawn Blade Stance
**Component:**
```rust
StanceActive {
    stance_type: StanceType,
    guard_drain_rate: f32,
}
```

**System:**
- Hold Heavy → enter stance
- Drain Guard while held
- Release → fast slash
- Cancel → return to neutral

#### 5.3 Visual/Audio Polish (Placeholder)
- Simple sprite (can be stick figure with sword)
- Hit effects (white flash, ink splatter)
- Sound effects (movement, attacks, parry chime)

**Milestone:** Ronin is complete. Mirror matches work. Game loop is fully playable.

---

### Phase 6: Roster Expansion (Weeks 9-11)

**Pattern:** Each character is a new `.ron` file + sprites. No core system changes.

#### Add Characters:
1. **The Monk** — Counter-attacker, Open Palm stance
2. **The Oni** — Heavy armor, Demon's Patience stance
3. **The Shade** — Mobile mixup, Flicker stance

**Test:** Each character feels distinct. Matchups create variety.

---

### Phase 7: Audio-Visual Polish (Weeks 12-13)

**Goal:** Game looks/sounds like Fudoshin.

- Ink-brush sprites
- Stage backgrounds (minimal)
- Ambient soundscape (wind, rain, silence)
- Screen effects (hitstop, shake, slowdown)
- UI (Breath indicators only)

---

### Phase 8: Modes and Completion (Weeks 14-15)

- Versus mode (character select, best of 3)
- Training mode (frame data display, dummy control)
- Tutorial (movement, triangle, Initiative, Decisive Blow)

**Definition of Done:** Two players can sit down, learn, compete. Game feels **fast, responsive, mentally engaging**.

---

## Testing and Validation

### Every Phase:
- **Does it feel right?** (most important)
- **Is it responsive?** (input → action in 1-2 frames)
- **Is it clear?** (can you understand what happened?)

### Specific Tests:
- **Movement (Phase 1):** Cross stage in 2 seconds? Snappy?
- **Parry (Phase 2):** Feels rewarding? Visual/audio punch?
- **Initiative (Phase 3):** Can you feel who's winning?
- **Decisive Blow (Phase 4):** Tense? Earned?

### Balance Tuning Workflow:
1. Play test → identify issue
2. Open `.ron` file → change values
3. Hot reload → test immediately
4. Iterate until it feels right

---

## Open Questions

- **Audio middleware:** Use `bevy_kira_audio` or raw Bevy audio?
- **Sprite format:** Sprite sheets or individual files?
- **Online play:** Rollback netcode is Phase 9+ (post-MVP)

---

## Why This Architecture Works

1. **Separation of concerns** → Systems are simple, testable
2. **Data-driven** → Balance changes without recompilation
3. **Composable** → Mix/match components for new characters
4. **Debuggable** → Inspector shows all state live
5. **Extensible** → New characters = new data files
6. **Readable** → System names explain what they do

**The codebase should read like the design doc.**

When you open `src/systems/initiative.rs`, you see:
- `grant_initiative_on_hit`
- `revoke_initiative_on_whiff`
- `apply_initiative_frame_advantage`

Clear. Simple. Robust.

---

## Next Steps

1. Review this plan — anything unclear or missing?
2. I'll scaffold the project structure (Cargo.toml, folders, main.rs)
3. Begin Phase 1: Movement foundation
4. Iterate on feel until rectangles move beautifully
5. Build from there

Ready to begin?
