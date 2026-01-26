# Fudoshin — MVP Implementation Plan

*Building the immovable mind with clean architecture and feel-first development*

---

## Philosophy

**Feel first, features second.**
The game's identity lives in its moment-to-moment feel — the weight of hits, the tension of spacing, the satisfaction of a correct read. If this foundation is wrong, no amount of content will fix it.

**Data-driven, not code-driven.**
Frame data, damage values, move properties — all live in `.ron` files. Balance changes = edit text, hot-reload, test. No code compilation needed.

**Vertical slice over horizontal sprawl.**
One complete character that feels incredible is more valuable than four half-finished characters. Build deep, then scale wide.

**Extensibility by default.**
The framework should support diverse character archetypes (Combat Phases: Advantage → Stagger → Finish) without core system changes.

---

## Technology Stack

- **Bevy 0.14.2** (stable)
- **Rust 1.75+** (stable toolchain)
- **Resolution:** 1280x720 (balanced for hand-drawn sprites)
- **Target:** 60 FPS locked (fighting game standard)
- **Input:** Keyboard (MVP), Gamepad (post-MVP)
- **Data Format:** RON (Rusty Object Notation) for all game data
- **Hot Reload:** `bevy_asset_loader` + watching file changes

### Development Tools

- **bevy_inspector_egui** — inspect entities/components live
- **bevy_framepace** — locked 60 FPS
- **F1 debug view** — hitboxes, hurtboxes, frame data

---

## Repository Structure

```
fudoshin/
├── Cargo.toml
├── assets/
│   ├── data/                    # ← All game balance lives here
│   │   ├── characters/
│   │   │   └── conscript.ron    # Frame data, move properties
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
│   │   ├── character.rs         # Health state, position
│   │   ├── combat.rs            # Attack, Defense, Hitbox
│   │   ├── state.rs             # CharacterState enum, StateTimer
│   │   ├── hitstop.rs           # Hitstop component (NEW)
│   │   └── guard.rs             # GuardMeter, GuardBreak
│   ├── systems/                 # ← Each system = one responsibility
│   │   ├── movement.rs          # Walk, step, backdash, evade
│   │   ├── attack.rs            # Hitbox activation, damage application
│   │   ├── hitstop.rs           # Hitstop freeze system (NEW)
│   │   ├── combo.rs             # Cancel system, input buffer (NEW)
│   │   ├── guard.rs             # Guard meter fill/drain, guard break
│   │   └── collision.rs         # Hitbox/hurtbox detection
│   ├── data/                    # ← Data structure definitions
│   │   ├── character_data.rs    # Loaded from .ron
│   │   └── move_data.rs         # Loaded from .ron
│   └── plugins/
│       ├── core_game.rs         # Movement, combat, state machine
│       └── debug.rs             # Inspector, hitbox viz
└── docs/
    ├── mvp_plan.md              # This file
    ├── combat_phases.md         # Character design framework
    ├── roster.md                # 10-character roster
    └── todo/
        ├── HITSTOP_IMPLEMENTATION.md
        ├── HITBOX_HURTBOX_SIZING.md
        └── SHORT_COMBO_SYSTEM.md
```

---

## Current State (Phases 0-4 Complete ✅)

### What's Working

**Phase 0:** ✅ Project setup, Bevy 0.14, 60 FPS locked, hot-reload ready

**Phase 1:** ✅ Movement foundation
- Walk, step, backdash, evade
- Responsive, snappy controls
- Stage boundaries

**Phase 2:** ✅ Core combat triangle
- 3 attack types (Light, Heavy, Grab)
- Block with guard meter
- 6-frame parry window
- Evade with i-frames
- Hitbox/hurtbox collision

**Phase 3:** ✅ Initiative & pressure
- Frame advantage tracking
- Pressure state with bonuses
- Chain attacks (Light → Light)
- Counter hit system
- Momentum tracking

**Phase 4:** ✅ Health, breaths, rounds
- Health states (Whole → Cut → Wounded → Broken)
- Breath system (3 stocks per match)
- Round management with countdown/timer
- Decisive Blow conditions
- Victory conditions and UI

### What's Missing

**Critical game feel gaps:**
- ❌ Hitstop/freeze frames on hits (industry standard)
- ❌ Hitbox sizes 30-50% too small
- ❌ Combo system limited (only Light → Light)
- ❌ No input buffer
- ❌ No damage scaling

**Framework gaps:**
- ❌ Data-driven character loading (RON files exist but not loaded)
- ❌ Stagger method variety (only Guard Break implemented)
- ❌ Finish type variety (only Standard implemented)
- ❌ Character-specific mechanics framework

**Content gaps:**
- ❌ All characters use same generic movelist
- ❌ No character-specific stances fully implemented
- ❌ No distinct character identities

---

## Phase-by-Phase Implementation

### Phase 5: Game Feel Foundation (2-3 weeks)

**Goal:** Hits feel meaty, attacks connect reliably, combos are satisfying

**Priority: CRITICAL** — Without this, the game doesn't feel like Fudoshin

#### 5.1 Hitstop System (Week 1)

**Reference:** `docs/todo/HITSTOP_IMPLEMENTATION.md`

**Why critical:** "The single most important feature for making hits feel chunky and satisfying"

**Implementation:**

```rust
// src/components/hitstop.rs
#[derive(Component)]
pub struct Hitstop {
    pub frames_remaining: u32,
    pub total_frames: u32,
}

// Add to MoveData in src/components/movelist.rs
pub struct MoveData {
    // ... existing fields
    pub hitstop_on_hit: u32,      // 9f for lights, 13f for heavies
    pub hitstop_on_block: u32,    // 6f for lights, 10f for heavies
    pub hitstop_on_counter: u32,  // Add 3f bonus
}
```

**Systems to add:**

```rust
// src/systems/hitstop.rs

/// Apply hitstop to both attacker and defender on hit
pub fn apply_hitstop_on_hit(
    mut commands: Commands,
    mut hit_events: EventReader<HitEvent>,
) {
    for event in hit_events.read() {
        let frames = calculate_hitstop(&event);

        // Freeze both entities
        commands.entity(event.attacker).insert(Hitstop {
            frames_remaining: frames,
            total_frames: frames,
        });
        commands.entity(event.defender).insert(Hitstop {
            frames_remaining: frames,
            total_frames: frames,
        });
    }
}

/// Tick down hitstop, remove when complete
pub fn process_hitstop(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Hitstop)>,
) {
    for (entity, mut hitstop) in query.iter_mut() {
        if hitstop.frames_remaining > 0 {
            hitstop.frames_remaining -= 1;
        } else {
            commands.entity(entity).remove::<Hitstop>();
        }
    }
}

/// Prevent state progression during hitstop
pub fn freeze_during_hitstop(
    mut query: Query<&mut StateTimer, With<Hitstop>>,
) {
    // Don't tick timers on frozen entities
}
```

**Integration:**
- Modify existing systems to skip entities with `Hitstop` component
- Add screen shake during hitstop (2px for lights, 5px for heavies)
- Allow cancels during hitstop window (enables tight combo timing)

**Industry standard values:**
- Light attacks: 8-9 frames
- Heavy attacks: 13-14 frames
- Counter hits: Add 3 frames
- Blocked hits: Reduce by 3 frames
- Parries: 10-12 frames

**Test:** Every hit should feel chunky with clear impact moment

#### 5.2 Hitbox Sizing (Week 1)

**Reference:** `docs/todo/HITBOX_HURTBOX_SIZING.md`

**Why critical:** Current hitboxes are 30-50% smaller than Street Fighter standards, making attacks feel unrewarding

**Changes to `src/components/movelist.rs`:**

```rust
// Light attacks: 1.0x → 1.5x character width
Jab:        Vec2::new(80.0, 80.0)   → Vec2::new(120.0, 95.0)
Dash Jab:   Vec2::new(90.0, 80.0)   → Vec2::new(130.0, 95.0)
Low Poke:   Vec2::new(85.0, 55.0)   → Vec2::new(125.0, 65.0)
Step Jab:   Vec2::new(75.0, 80.0)   → Vec2::new(115.0, 95.0)

// Heavy attacks: 1.5x → 2.0-2.5x character width
Heavy:      Vec2::new(120.0, 105.0) → Vec2::new(170.0, 130.0)
Lunge:      Vec2::new(130.0, 105.0) → Vec2::new(190.0, 130.0)
Sweep:      Vec2::new(145.0, 45.0)  → Vec2::new(200.0, 50.0)
Counter:    Vec2::new(110.0, 105.0) → Vec2::new(160.0, 130.0)

// Grab: 0.8x → 1.5x character width (most important!)
Grab:       Vec2::new(65.0, 105.0)  → Vec2::new(120.0, 120.0)
```

**Implementation:**
1. Update all hitbox sizes in `Movelist::default_character()`
2. Adjust offsets if needed (push hitboxes further forward)
3. Test with F1 debug view
4. Verify hitboxes don't overlap own hurtbox

**Test:** Attacks should feel generous, grabs should be threatening, spacing should still matter

#### 5.3 Short Combo System (Weeks 2-3)

**Reference:** `docs/todo/SHORT_COMBO_SYSTEM.md`

**Why critical:** "2-4 clean hits can kill" is core philosophy, but only Light → Light exists

**Combo routes to implement:**

```
1. Light Confirm:  Light → Light → Heavy  (35-40% damage)
2. Heavy Punish:   Counter Heavy → Light → Heavy  (50-60% damage)
3. Light Pressure: Light → Light → Grab  (30% damage + setup)
4. Quick Damage:   Light → Heavy  (30% damage)
```

**New components:**

```rust
// src/components/combo.rs
#[derive(Component)]
pub struct InputBuffer {
    pub buffer: Vec<(AttackType, u8)>,  // (input, frames_ago)
    pub window: u8,  // 6-8 frames
}

#[derive(Component)]
pub struct ComboState {
    pub hit_count: u32,
    pub damage_dealt: f32,
    pub scaling: f32,  // 1.0 → 0.9 → 0.8 for hits 1-3
}

// Add to MoveData
pub struct MoveData {
    // ... existing fields
    pub cancellable_into: Vec<AttackType>,
    pub cancel_window_frames: u32,  // Frames during recovery you can cancel
}
```

**New systems:**

```rust
// src/systems/combo.rs

/// Buffer recent inputs
pub fn buffer_inputs(
    mut query: Query<(&CurrentInput, &mut InputBuffer)>,
) {
    // Store last 6-8 frames of inputs
    // Age out old inputs
}

/// Handle attack cancels during recovery
pub fn process_cancels(
    mut query: Query<(&AttackData, &StateTimer, &InputBuffer, &Player)>,
    mut commands: Commands,
) {
    // If in cancel window and buffered input is valid:
    // - End current attack early
    // - Start new attack immediately
    // - Increment combo counter
}

/// Apply damage scaling
pub fn apply_combo_scaling(
    mut query: Query<&mut ComboState>,
    mut hit_events: EventReader<HitEvent>,
) {
    // No scaling hits 1-2
    // 90% damage hit 3
    // 80% damage hit 4+
}

/// Reset combo state
pub fn reset_combos(
    mut query: Query<&mut ComboState>,
    // Reset on block, neutral return, timeout
) {}
```

**Cancel rules:**
- Light → Light (existing, on hit only)
- Light → Heavy (new, on hit only)
- Light → Grab (new, on hit or block)
- Counter Heavy → Light (new, counter hit only)

**Visual feedback:**
- Hit flash escalation (white → yellow → gold)
- Screen shake scaling (2px → 5px → 8px)
- Combo counter UI ("2 HIT!")
- Sound pitch shift per hit

**Test:** Combos should feel earned and impactful, not memorized

#### Phase 5 Milestone

✅ **Game Feel Complete**
- Hits have 8-13f hitstop (meaty impact)
- Hitboxes 50% larger (attacks connect reliably)
- 4 combo routes working (Light → Heavy, Light → Grab, etc.)
- 2-4 hit combos deal 30-60% damage
- Screen shake and visual feedback escalate through combos

**The game should feel better than most fighting games at this point.**

---

### Phase 6: Combat Framework (2-3 weeks)

**Goal:** System supports diverse character archetypes without code duplication

**Reference:** `docs/combat_phases.md` — Build Advantage → Stagger → Finish framework

#### 6.1 Stagger Method Variety (Week 1)

**Core insight:** Multiple paths to Stagger enable character diversity

**Implementation:**

```rust
// src/components/combat.rs
#[derive(Component, Debug)]
pub enum StaggerSource {
    GuardBreak,      // Existing: Guard meter full
    CounterHit,      // New: Specific moves Stagger on counter
    CommandGrab,     // New: Special grab that Staggers
    ArmorTrade,      // New: Armored move absorbs + connects
}

#[derive(Component)]
pub struct Stagger {
    pub frames_remaining: u32,
    pub source: StaggerSource,
}

// Add to AttackProperty enum
pub enum AttackProperty {
    Unblockable,
    LightArmor,
    StaggersOnCounter,   // NEW: Causes Stagger when lands as counter hit
    CommandGrab,         // NEW: Unblockable grab that Staggers
    ArmorStaggers,       // NEW: Staggers if absorbs hit and connects
}
```

**New stagger triggers:**

```rust
// src/systems/damage.rs

/// Check for counter hit stagger
pub fn apply_counter_stagger(
    mut hit_events: EventReader<HitEvent>,
    attack_query: Query<&AttackData>,
    mut commands: Commands,
) {
    for event in hit_events.read() {
        if event.is_counter_hit {
            if let Ok(attack) = attack_query.get(event.attacker) {
                if attack.properties.contains(&AttackProperty::StaggersOnCounter) {
                    commands.entity(event.defender).insert(Stagger {
                        frames_remaining: 30,
                        source: StaggerSource::CounterHit,
                    });
                }
            }
        }
    }
}

/// Check for armor trade stagger
pub fn apply_armor_trade_stagger(
    // If armored move absorbs hit and connects, stagger opponent
) {}
```

**Stagger duration by source:**
- Guard Break: 40 frames (existing)
- Counter Hit: 30 frames
- Command Grab: 35 frames
- Armor Trade: 30 frames

**Test:** Multiple paths to Stagger should feel distinct

#### 6.2 Finish Type Variety (Week 1)

**Core insight:** Different kill animations/mechanics enable character expression

**Implementation:**

```rust
// src/components/combat.rs
#[derive(Clone, Copy, Debug)]
pub enum FinishType {
    Standard,          // Single heavy strike (20f wind-up, 4f parry window)
    ExecutionCombo,    // Auto 3-4 hit sequence
    Grapple,           // Command grab kill (no parry possible)
    Counter,           // Counter stance, attack = death
}

// Add to CharacterData
pub struct CharacterData {
    pub name: String,
    pub movelist: Movelist,
    pub stance: Option<StanceData>,
    pub finish_type: FinishType,  // NEW
}
```

**For MVP:**
- Implement **Standard** finish (existing)
- Build framework interface for others
- Document how to add ExecutionCombo/Grapple/Counter

**System interface:**

```rust
// src/systems/decisive_blow.rs

/// Trigger appropriate finish based on character's finish_type
pub fn execute_decisive_blow(
    character_data: &CharacterData,
    target: Entity,
) {
    match character_data.finish_type {
        FinishType::Standard => execute_standard_finish(target),
        FinishType::ExecutionCombo => execute_combo_finish(target),
        FinishType::Grapple => execute_grapple_finish(target),
        FinishType::Counter => execute_counter_finish(target),
    }
}
```

**Test:** Framework exists, Standard finish works, others can be added without touching core code

#### 6.3 Character Mechanic Plugin System (Week 2)

**Core insight:** Characters need unique systems (Penance, Feral, Toxin) without polluting core

**Implementation:**

```rust
// src/plugins/character_mechanics.rs

/// Trait for character-specific mechanics
pub trait CharacterMechanic: Send + Sync + 'static {
    fn update(&mut self, ctx: &MechanicContext);
    fn on_hit(&mut self, hit: &HitEvent);
    fn on_block(&mut self, block: &BlockEvent);
    fn damage_modifier(&self) -> f32;  // Multiplier for damage dealt
}

/// Context provided to mechanics
pub struct MechanicContext {
    pub entity: Entity,
    pub time: f32,
    pub health: &Health,
    pub guard: &GuardMeter,
}

// Example mechanic (for future Flagellant character)
#[derive(Component)]
pub struct PenanceMechanic {
    pub meter: f32,  // 0.0 - 1.0
    pub decay_rate: f32,
}

impl CharacterMechanic for PenanceMechanic {
    fn update(&mut self, ctx: &MechanicContext) {
        // Decay over time
        self.meter = (self.meter - self.decay_rate * ctx.time).max(0.0);
    }

    fn on_hit(&mut self, hit: &HitEvent) {
        // Build Penance when taking damage (not blocking)
        if !hit.was_blocked {
            self.meter = (self.meter + 0.2).min(1.0);
        }
    }

    fn damage_modifier(&self) -> f32 {
        if self.meter >= 1.0 {
            2.0  // Double damage at full Penance
        } else {
            1.0
        }
    }
}
```

**Generic system:**

```rust
// src/systems/character_mechanics.rs

/// Update all active character mechanics
pub fn update_character_mechanics(
    mut query: Query<&mut dyn CharacterMechanic>,
    time: Res<Time>,
) {
    // Call update on all active mechanics
}
```

**For MVP:**
- Build the **interface** (trait + plugin pattern)
- Don't implement specific mechanics yet
- Prove concept with simple example

**Test:** Can add new mechanic without editing core combat systems

#### 6.4 Data-Driven Character Loading (Week 2-3)

**Core insight:** Characters should be data, not code

**RON file format:**

```rust
// assets/data/characters/conscript.ron
(
    name: "The Conscript",
    archetype: Pressure,

    moves: {
        "neutral_light": (
            startup: 5,
            active: 2,
            recovery: 10,
            damage: 8.0,
            on_block: -2,
            hitbox_offset: Vec2(40.0, 0.0),
            hitbox_size: Vec2(120.0, 95.0),
            hitstop_on_hit: 9,
            hitstop_on_block: 6,
            cancellable_into: [Light, Heavy],
            properties: [],
        ),
        // ... 8 more moves
    },

    stance: Some((
        type: DrillForm,
        guard_damage_reduction: 0.7,
    )),

    finish_type: Standard,
)
```

**Loading system:**

```rust
// src/data/character_data.rs
use serde::Deserialize;

#[derive(Deserialize, Asset, TypePath)]
pub struct CharacterData {
    pub name: String,
    pub archetype: String,
    pub moves: HashMap<String, MoveData>,
    pub stance: Option<StanceData>,
    pub finish_type: FinishType,
}

// src/systems/character_loader.rs
pub fn load_character_from_file(
    asset_server: Res<AssetServer>,
) -> Handle<CharacterData> {
    asset_server.load("data/characters/conscript.ron")
}

pub fn spawn_character_from_data(
    commands: &mut Commands,
    character_data: &CharacterData,
    player: Player,
    position: Vec2,
) -> Entity {
    commands.spawn((
        Character,
        player,
        Health::default(),
        GuardMeter::default(),
        Movelist::from_data(&character_data.moves),
        Transform::from_translation(position.extend(0.0)),
        // ... other components
    )).id()
}
```

**Hot reload:**
- Use Bevy's asset system
- Detect changes to `.ron` files
- Reload character data without restart
- Apply to existing entities

**Test:** Editing `conscript.ron` changes character without recompiling

#### Phase 6 Milestone

✅ **Framework Complete**
- Multiple stagger methods (Guard Break, Counter Hit, Command Grab, Armor Trade)
- Finish type system (interface for 4 types, Standard implemented)
- Character mechanic plugin interface (extensible without core changes)
- Data-driven character loading (RON → CharacterData → Entity)
- Hot reload working

**The system can now support 10 diverse characters without code changes.**

---

### Phase 7: The Conscript (2-3 weeks)

**Goal:** One complete, balanced character that feels incredible

**Why The Conscript?**

From `docs/roster.md`:
- Balanced archetype (good at everything, great at nothing)
- Tutorial character (players learn fundamentals through him)
- Standard patterns (Pressure → Guard Break → Standard finish)
- Simple stance (Drill Form: defensive, reduces guard damage)

**Perfect MVP character:** Proves core systems without exotic mechanics

#### 7.1 Complete Movelist (Week 1)

**Implement all 9 moves with proper data:**

```ron
// assets/data/characters/conscript.ron
(
    name: "The Conscript",
    archetype: Pressure,

    moves: {
        // === LIGHT ATTACKS ===

        "neutral_light": (
            name: "Jab",
            startup: 5,
            active: 2,
            recovery: 10,
            damage: 8.0,
            on_block: -2,
            hitbox_offset: Vec2(40.0, 0.0),
            hitbox_size: Vec2(120.0, 95.0),
            hitstop_on_hit: 9,
            hitstop_on_block: 6,
            cancellable_into: [Light, Heavy],
            properties: [],
        ),

        "forward_light": (
            name: "Advancing Jab",
            startup: 4,
            active: 2,
            recovery: 10,
            damage: 6.0,
            on_block: -2,
            hitbox_offset: Vec2(50.0, 0.0),
            hitbox_size: Vec2(130.0, 95.0),
            hitstop_on_hit: 9,
            cancellable_into: [Light, Heavy],
            properties: [],
            movement: Some(Forward(50.0, 15.0)),  // distance, speed
        ),

        "down_light": (
            name: "Low Poke",
            startup: 6,
            active: 2,
            recovery: 11,
            damage: 7.0,
            on_block: -3,
            hitbox_offset: Vec2(40.0, -30.0),
            hitbox_size: Vec2(125.0, 65.0),
            hitstop_on_hit: 9,
            cancellable_into: [Light],
            properties: [],
        ),

        "back_light": (
            name: "Retreating Jab",
            startup: 5,
            active: 2,
            recovery: 9,
            damage: 6.0,
            on_block: 1,  // Safe on block!
            hitbox_offset: Vec2(35.0, 0.0),
            hitbox_size: Vec2(115.0, 95.0),
            hitstop_on_hit: 9,
            cancellable_into: [],  // Can't cancel (defensive)
            properties: [],
            movement: Some(Back(30.0, 15.0)),
        ),

        // === HEAVY ATTACKS ===

        "neutral_heavy": (
            name: "Overhead Strike",
            startup: 11,
            active: 4,
            recovery: 18,
            damage: 18.0,
            on_block: -8,
            hitbox_offset: Vec2(50.0, 10.0),
            hitbox_size: Vec2(170.0, 130.0),
            hitstop_on_hit: 13,
            hitstop_on_block: 10,
            cancellable_into: [],
            properties: [LightArmor],  // Absorbs one light hit
        ),

        "forward_heavy": (
            name: "Lunging Strike",
            startup: 9,
            active: 4,
            recovery: 18,
            damage: 15.0,
            on_block: -6,
            hitbox_offset: Vec2(60.0, 10.0),
            hitbox_size: Vec2(190.0, 130.0),
            hitstop_on_hit: 13,
            cancellable_into: [],
            properties: [],
            movement: Some(Forward(80.0, 15.0)),
        ),

        "down_heavy": (
            name: "Sweep",
            startup: 13,
            active: 4,
            recovery: 20,
            damage: 20.0,
            on_block: -10,
            hitbox_offset: Vec2(50.0, -35.0),
            hitbox_size: Vec2(200.0, 50.0),
            hitstop_on_hit: 14,
            cancellable_into: [],
            properties: [],
        ),

        "back_heavy": (
            name: "Defensive Strike",
            startup: 10,
            active: 4,
            recovery: 16,
            damage: 16.0,
            on_block: -4,  // Safer than normal heavy
            hitbox_offset: Vec2(45.0, 0.0),
            hitbox_size: Vec2(160.0, 130.0),
            hitstop_on_hit: 13,
            cancellable_into: [],
            properties: [StaggersOnCounter],  // Staggers on counter hit!
            movement: Some(Back(40.0, 15.0)),
        ),

        // === GRAB ===

        "neutral_grab": (
            name: "Command Grab",
            startup: 10,
            active: 2,
            recovery: 20,
            damage: 12.0,
            on_block: 0,
            hitbox_offset: Vec2(35.0, 0.0),
            hitbox_size: Vec2(120.0, 120.0),
            hitstop_on_hit: 11,
            cancellable_into: [],
            properties: [Unblockable, CommandGrab],  // Causes Stagger
        ),
    },

    stance: Some((
        type: DrillForm,
        entry: HoldBlock,  // Hold block for 10 frames to enter
        guard_damage_reduction: 0.7,  // Take 30% less guard damage
        movement_speed: 0.8,  // Can walk slowly in stance
    )),

    finish_type: Standard,
)
```

**Frame data tuning:**
- Test pressure flow (Light → Light → mixup)
- Test punish windows (whiff Heavy = punishable)
- Ensure risk/reward feels fair
- All combos should work (Light → Heavy, Light → Grab, etc.)

#### 7.2 Drill Form Stance (Week 2)

**Implementation:**

```rust
// src/components/stance.rs
#[derive(Component, Debug)]
pub enum Stance {
    DrillForm {
        guard_damage_reduction: f32,
        movement_speed: f32,
        active: bool,
    }
}

// src/systems/stance.rs

/// Enter stance when conditions met
pub fn enter_drill_form(
    mut query: Query<(&Player, &mut Stance, &CharacterState)>,
    input: Res<CurrentInputs>,
) {
    // Hold block for 10 frames → enter Drill Form
}

/// Apply stance effects
pub fn apply_drill_form_effects(
    mut guard_events: EventReader<HitEvent>,
    stance_query: Query<&Stance>,
) {
    // Reduce guard damage by 30% while in stance
}

/// Exit stance
pub fn exit_drill_form(
    mut query: Query<&mut Stance>,
    input: Res<CurrentInputs>,
) {
    // Release block or press attack → exit stance
}
```

**Visual:**
- Shield raised, centered posture
- Slight glow on shield when active
- Movement animation slower

**Purpose:** Teaches stance mechanics without complex options

#### 7.3 Visual Identity (Week 2-3)

**Art requirements:**
- Sprite per state (Idle, Walk, Attack, Block, Stagger, Death)
- Can be simple but distinctive
- **Shield on arm** (visual identifier)
- Clear silhouette (medium build, balanced stance)

**Color palette:**
- Armor: Gray/steel blue
- Shield: Bronze/copper
- Accent: Red cloth

**Effects:**
- Hit sparks (white flash on hit, yellow on counter)
- Guard effect (blue shield glow on block)
- Stance indicator (persistent shield glow in Drill Form)
- Decisive Blow wind-up (screen darkens, character glows)

**Audio:**
- 4 hit sounds (light, heavy, counter, decisive)
- 2 movement sounds (footstep, dash)
- Parry chime (bell-like)
- Stance sounds (enter/exit)
- Grab sound (heavier impact)

**Test:** Character feels like "The Conscript" (soldier, practical, reliable)

#### 7.4 Balance & Polish (Week 3)

**Balance testing:**
- Mirror match should be 50/50 skill-based
- All 9 moves should be useful in different situations
- Combos should deal 30-60% damage depending on route
- Stance should feel valuable but not mandatory
- No dominant strategy

**Polish:**
- All visual feedback working
- All audio triggering correctly
- Hitstop feels consistent
- Combos flow naturally
- UI shows all relevant info (breaths, health state, round timer)

**Playtesting goals:**
- 10 matches minimum
- Identify frustrations
- Tune frame data
- Adjust damage values
- Refine combo routes

#### Phase 7 Milestone

✅ **The Conscript Complete**
- All 9 moves implemented and balanced
- Drill Form stance functional
- Complete visual/audio identity
- Loaded from `conscript.ron` data file
- Mirror match is fun, strategic, and balanced
- Game feels incredible to play

**The vertical slice is complete. Ship it.**

---

## MVP Definition of Done

The MVP is complete when these criteria are met:

### ✅ Game Feel Checklist
- [ ] Hits have 8-13f hitstop (feels meaty and impactful)
- [ ] Hitboxes 50% larger (attacks connect reliably)
- [ ] Light → Light → Heavy combo deals 35-40% damage
- [ ] Light → Heavy combo deals 28-32% damage
- [ ] Screen shake escalates through combos (2px → 5px → 8px)
- [ ] Hit flash escalates through combos (white → yellow → gold)
- [ ] Combo counter displays hit count
- [ ] Every hit feels satisfying

### ✅ Framework Checklist
- [ ] Stagger triggers from 4 sources (Guard Break, Counter Hit, Command Grab, Armor Trade)
- [ ] Finish type system supports 4 types (Standard implemented, others documented)
- [ ] Character mechanic plugin system exists (trait + example)
- [ ] Character loads from RON file
- [ ] Hot reload works for character data
- [ ] New attack properties can be added without touching core systems

### ✅ Conscript Checklist
- [ ] All 9 moves implemented with correct frame data
- [ ] All combo routes work (Light → Light → Heavy, Light → Heavy, Light → Grab, Counter → combo)
- [ ] Drill Form stance reduces guard damage by 30%
- [ ] Character has distinct visual identity (shield, soldier aesthetic)
- [ ] Audio feedback for all actions
- [ ] Mirror match is balanced and fun

### ✅ Match Flow Checklist
- [ ] Match starts with character select (even if only Conscript available)
- [ ] 3 breaths per player
- [ ] Rounds last 20-40 seconds
- [ ] Full match lasts 2-3 minutes
- [ ] Momentum state works (killer has advantage next breath)
- [ ] Desperation state works (down 0-2 has damage boost + Final Stand)
- [ ] Decisive Blow available when conditions met
- [ ] Final Parry window works (4f tight timing)
- [ ] Victory screen shows winner

### ✅ Player Experience Checklist
- [ ] Two players can sit down and play immediately
- [ ] Controls are responsive (input → action in 1-2 frames)
- [ ] Kills feel earned through accumulated advantage
- [ ] Comebacks are possible through Desperation/Final Stand
- [ ] Players can identify what went wrong when they lose
- [ ] Spectators can follow what's happening
- [ ] F1 debug view shows hitboxes, frame data, game state

---

## Revised Timeline

| Phase | Focus | Estimated Time |
|-------|-------|----------------|
| **5** | Game Feel (hitstop, hitboxes, combos) | 2-3 weeks |
| **6** | Combat Framework (stagger variety, finish types, data loading) | 2-3 weeks |
| **7** | The Conscript (complete character) | 2-3 weeks |

**Total MVP:** 6-9 weeks (1.5-2 months)

**Deliverable:** A vertical slice that proves Fudoshin works

---

## Post-MVP Scaling

Once the MVP is complete, adding characters becomes straightforward:

### Character Addition Workflow

1. **Design** (1-2 days)
   - Choose Advantage/Stagger/Finish combination from Combat Phases
   - Design unique mechanic (if needed)
   - Draft movelist

2. **Data File** (1 day)
   - Create `assets/data/characters/{name}.ron`
   - Define 9 moves with frame data
   - Configure stance (if any)
   - Set finish type

3. **Character Mechanic** (2-3 days, if unique)
   - Implement mechanic using plugin system
   - Examples: Penance (Flagellant), Feral (Beast), Toxin (Apothecary)

4. **Art** (3-5 days)
   - Sprites for 6 states (Idle, Walk, Attack, Block, Stagger, Death)
   - Effects (hit sparks, stance indicators)
   - Audio (4-5 sounds)

5. **Balance** (2-3 days)
   - Test vs all existing characters
   - Tune frame data, damage, hitboxes
   - Refine unique mechanic

**Estimated time per character:** 1-2 weeks with established framework

### Roster Roadmap Post-MVP

**Phase 8: Second Character Trio** (3-4 weeks)
- The Butcher (Burst/Command Grab/Grapple)
- The Duchess (Conditioning/Counter Hit/Standard)
- Mycella (Setplay/Trap Trigger/Trap Trigger)

**Phase 9: Third Character Trio** (3-4 weeks)
- The Flagellant (Punish/Armor Trade/Counter)
- The Courier (Pressure/Guard Break/Execution Combo)
- The Effigy (Conditioning/Stance Punish/Transformation)

**Phase 10: Final Character Quartet** (4-5 weeks)
- The Apothecary (Attrition/Guard Break/Standard)
- The Revenant (Attrition/Armor Trade/Standard)
- The Beast (Burst/Counter Hit/Transformation)
- Additional character TBD

**Full 10-character roster:** 10-13 additional weeks after MVP

---

## Testing and Validation

### Phase 5 Tests (Game Feel)
- Does hitstop make hits feel meaty?
- Do attacks connect at expected ranges?
- Do combos flow naturally?
- Is input buffer forgiving enough?
- Does damage scaling feel right?

### Phase 6 Tests (Framework)
- Can you add a new stagger method without editing core systems?
- Can you add a new finish type without breaking existing code?
- Does hot reload work for character data?
- Can you add a character mechanic as a plugin?

### Phase 7 Tests (Conscript)
- Are all 9 moves useful in different situations?
- Does Drill Form feel valuable?
- Is mirror match balanced?
- Do players understand why they won/lost?
- Can new players pick up the controls in 5 minutes?

### Balance Tuning Workflow

1. **Playtest** — Two people play 10+ matches
2. **Identify** — What feels unfair, frustrating, or dominant?
3. **Edit** — Open `conscript.ron`, change values
4. **Reload** — Hot reload applies changes
5. **Test** — Play 5 more matches
6. **Iterate** — Repeat until balanced

**No compilation required** — balance iteration is fast

---

## Architecture Principles

These principles from the original plan remain valid:

### 1. Data-Driven Balance
All tunable values live in `.ron` files. Balance changes = edit text, hot-reload, test.

### 2. Component Composition
Components are pure data. No methods, no logic.

### 3. Single-Responsibility Systems
Each system does one thing. Systems communicate via Events.

### 4. Plugin Organization
Group related systems into plugins for clear boundaries.

### 5. Extensibility by Default
New characters = new data file + optional mechanic plugin. Core systems never change.

**The codebase should read like the design doc.**

---

## Why This Plan Works

### Compared to Original MVP Plan

**Old approach:**
- 4 half-finished characters
- Placeholder feel until Phase 7
- Framework built "as needed"
- 15+ weeks

**New approach:**
- 1 complete character
- Industry-standard feel from day 1
- Framework proven extensible
- 6-9 weeks

### Risk Mitigation

**Old risk:** Build 4 characters → discover feel is wrong → rewrite everything

**New risk:** Build foundation → validate with 1 character → scale only when proven

### Value Proposition

After 6-9 weeks, you have:
- A game that feels better than most fighting games
- A framework that supports 10 diverse characters
- One complete, polished character
- Proof that the vision works

**Then you scale.**

---

## Immediate Next Steps

1. ✅ Review this refactored plan
2. Start Phase 5.1: Implement hitstop system
3. Continue Phase 5.2: Fix hitbox sizes
4. Build Phase 5.3: Expand combo system
5. Validate game feel improvements
6. Move to Phase 6: Build framework
7. Complete Phase 7: Finish The Conscript

**Goal:** Vertical slice that proves Fudoshin in 6-9 weeks

---

## References

- `docs/combat_phases.md` — Character design framework
- `docs/roster.md` — 10-character roster designs
- `docs/gameplay_mechanics.md` — Complete mechanics reference
- `docs/todo/HITSTOP_IMPLEMENTATION.md` — Hitstop/freeze frames
- `docs/todo/HITBOX_HURTBOX_SIZING.md` — Industry-standard hitbox sizes
- `docs/todo/SHORT_COMBO_SYSTEM.md` — 2-4 hit combo design

**Ready to build the immovable mind.**
