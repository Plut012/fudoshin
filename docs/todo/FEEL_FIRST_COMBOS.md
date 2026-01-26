# Feel-First Combo System Implementation

## Philosophy

**Focus on FEEL, not systems complexity.**

The combo system should make the game feel:
- **Responsive** - Inputs work when you expect them to
- **Varied** - Different routes for different situations
- **Impactful** - Hits feel increasingly satisfying

**What we're NOT doing (for MVP):**
- ❌ Damage scaling (balance concern, not feel)
- ❌ Combo counter UI (polish, not critical)
- ❌ Complex tracking systems
- ❌ Per-move cancel data in RON files (can hardcode)

---

## Current State

**What works:**
- ✅ Hitstop system (9-16 frames, feels meaty)
- ✅ Hitboxes sized correctly (attacks connect reliably)
- ✅ Light → Light chain (2 hits max, on hit only)
- ✅ 7-frame cancel window during recovery
- ✅ ChainState component tracking chains

**What's missing:**
- ❌ Input buffer (must press during exact frame)
- ❌ Light → Heavy cancel (no damage route)
- ❌ Light → Grab cancel (no mixup route)
- ❌ Visual escalation through combos

---

## Implementation Plan

### Phase 1: Input Buffer (1 day)

**Goal:** Make cancels feel forgiving and responsive

**Problem:** Players must press buttons during exact 7-frame window (4 frames @ 60fps = 67ms)

**Solution:** Buffer inputs for 8 frames so early presses still work

#### Add InputBuffer Component

```rust
// src/components/combo.rs (NEW FILE)
use bevy::prelude::*;
use crate::components::state::AttackType;

/// Buffers recent button presses for lenient combo execution
#[derive(Component, Debug, Default)]
pub struct InputBuffer {
    /// Frames since Light was pressed (0 = not buffered)
    pub light: u8,

    /// Frames since Heavy was pressed
    pub heavy: u8,

    /// Frames since Grab was pressed
    pub grab: u8,
}

impl InputBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a button press (resets timer to 1)
    pub fn press(&mut self, attack_type: AttackType) {
        match attack_type {
            AttackType::Light => self.light = 1,
            AttackType::Heavy => self.heavy = 1,
            AttackType::Grab => self.grab = 1,
        }
    }

    /// Age all buffers by 1 frame
    pub fn tick(&mut self) {
        if self.light > 0 { self.light += 1; }
        if self.heavy > 0 { self.heavy += 1; }
        if self.grab > 0 { self.grab += 1; }

        // Clear buffers older than 8 frames
        if self.light > 8 { self.light = 0; }
        if self.heavy > 8 { self.heavy = 0; }
        if self.grab > 8 { self.grab = 0; }
    }

    /// Check if attack type is currently buffered
    pub fn is_buffered(&self, attack_type: AttackType) -> bool {
        let frames = match attack_type {
            AttackType::Light => self.light,
            AttackType::Heavy => self.heavy,
            AttackType::Grab => self.grab,
        };
        frames > 0 && frames <= 8
    }

    /// Consume a buffered input (clears it)
    pub fn consume(&mut self, attack_type: AttackType) {
        match attack_type {
            AttackType::Light => self.light = 0,
            AttackType::Heavy => self.heavy = 0,
            AttackType::Grab => self.grab = 0,
        }
    }
}
```

#### Add Input Recording System

```rust
// src/systems/combo.rs (RENAME from chain.rs)

use bevy::prelude::*;
use crate::components::character::Player;
use crate::components::combo::InputBuffer;
use crate::components::state::AttackType;
use crate::systems::input::CurrentInputs;

/// Record button presses into input buffer each frame
pub fn record_inputs_to_buffer(
    inputs: Res<CurrentInputs>,
    mut query: Query<(&Player, &mut InputBuffer)>,
) {
    for (player, mut buffer) in query.iter_mut() {
        let input = match player {
            Player::One => &inputs.player_one,
            Player::Two => &inputs.player_two,
        };

        // Record any button presses this frame
        if input.light_attack {
            buffer.press(AttackType::Light);
        }
        if input.heavy_attack {
            buffer.press(AttackType::Heavy);
        }
        if input.grab {
            buffer.press(AttackType::Grab);
        }
    }
}

/// Age all input buffers by 1 frame
pub fn age_input_buffers(
    mut query: Query<&mut InputBuffer>,
) {
    for mut buffer in query.iter_mut() {
        buffer.tick();
    }
}
```

#### Integration

1. Add `InputBuffer` to all character entities at spawn
2. Register systems in `core_game.rs`:
   ```rust
   .add_systems(Update, (
       record_inputs_to_buffer,
       age_input_buffers,
       // ... rest of systems
   ))
   ```

**Test:** Press Light attack 3-4 frames before cancel window opens. Cancel should still work.

**Feel improvement:** Combos feel easy and responsive, not frame-perfect timing test.

---

### Phase 2: Expand Cancel Routes (2-3 days)

**Goal:** Give players options - damage route vs mixup route

**Current:** Only Light → Light (pressure route)

**Add:**
- Light → Heavy (damage route, on hit only)
- Light → Grab (mixup route, on hit or block)

#### Modify ChainState Component

```rust
// src/systems/combo.rs (rename from chain.rs)

/// Component to track chain attack state
#[derive(Component, Debug)]
pub struct ChainState {
    /// Number of attacks in current chain (0-2)
    pub chain_count: u8,

    /// Total hits landed (for visual escalation)
    pub hit_count: u8,  // NEW

    /// Whether the last attack hit (can chain)
    pub can_chain: bool,

    /// Whether we're in the chain window
    pub in_chain_window: bool,

    /// What attack types can cancel into
    pub cancellable_into: Vec<AttackType>,  // NEW
}

impl ChainState {
    pub fn can_cancel_into(&self, attack_type: AttackType) -> bool {
        self.can_chain
            && self.in_chain_window
            && self.cancellable_into.contains(&attack_type)
    }
}
```

#### Set Cancel Options on Hit

```rust
// Modify mark_chainable_on_hit

pub fn mark_chainable_on_hit(
    mut hit_events: EventReader<HitEvent>,
    mut query: Query<(&mut ChainState, &CharacterState)>,
) {
    for event in hit_events.read() {
        // Set what can cancel into based on hit type
        if let Ok((mut chain_state, state)) = query.get_mut(event.attacker) {
            if let CharacterState::Attacking { attack_type, phase, .. } = state {
                if *attack_type == AttackType::Light && *phase == AttackPhase::Active {
                    // Light hit: Can cancel into Light, Heavy, or Grab
                    if !event.was_blocked {
                        chain_state.can_chain = true;
                        chain_state.in_chain_window = true;
                        chain_state.hit_count += 1;
                        chain_state.cancellable_into = vec![
                            AttackType::Light,
                            AttackType::Heavy,
                            AttackType::Grab,
                        ];

                        info!("Light hit! Can cancel into: Light/Heavy/Grab (hit #{})",
                            chain_state.hit_count);
                    }
                }
            }
        }
    }
}
```

#### Check Multiple Cancel Types

```rust
// Modify handle_chain_input to check all buffered inputs

pub fn handle_chain_input(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Player,
        &mut CharacterState,
        &mut ChainState,
        &InputBuffer,  // Changed from &mut
        Option<&mut StateTimer>
    )>,
) {
    for (entity, player, mut state, mut chain_state, buffer, timer) in query.iter_mut() {
        let Some(mut timer) = timer else { continue };

        if !chain_state.can_chain || !chain_state.in_chain_window {
            continue;
        }

        // Check each attack type in priority order
        // Priority: Heavy > Grab > Light (Heavy = commitment, should be intentional)

        if buffer.is_buffered(AttackType::Heavy)
            && chain_state.can_cancel_into(AttackType::Heavy)
        {
            execute_cancel(entity, &mut commands, &mut state, &mut chain_state,
                           &mut timer, AttackType::Heavy, player);
        }
        else if buffer.is_buffered(AttackType::Grab)
            && chain_state.can_cancel_into(AttackType::Grab)
        {
            execute_cancel(entity, &mut commands, &mut state, &mut chain_state,
                           &mut timer, AttackType::Grab, player);
        }
        else if buffer.is_buffered(AttackType::Light)
            && chain_state.can_cancel_into(AttackType::Light)
        {
            execute_cancel(entity, &mut commands, &mut state, &mut chain_state,
                           &mut timer, AttackType::Light, player);
        }
    }
}

// Helper function to execute cancel
fn execute_cancel(
    entity: Entity,
    commands: &mut Commands,
    state: &mut CharacterState,
    chain_state: &mut ChainState,
    timer: &mut StateTimer,
    attack_type: AttackType,
    player: &Player,
) {
    info!("Player {:?} CANCEL → {:?} (hit #{})",
        player, attack_type, chain_state.hit_count);

    // Update chain state
    chain_state.chain_count += 1;
    chain_state.can_chain = false;
    chain_state.in_chain_window = false;
    chain_state.cancellable_into.clear();

    // Transition to new attack
    *state = CharacterState::Attacking {
        attack_type,
        direction: AttackDirection::Neutral,
        phase: AttackPhase::Startup,
    };

    // Reset timer with appropriate startup
    let startup = match attack_type {
        AttackType::Light => 5,
        AttackType::Heavy => 11,
        AttackType::Grab => 10,
    };
    timer.reset(startup);

    // Spawn hitbox for new attack
    let hitbox = crate::systems::attack::create_hitbox(attack_type);
    if let Some(mut entity_commands) = commands.get_entity(entity) {
        entity_commands.insert(hitbox);
    }
}
```

**Test cases:**
1. Light → Light (should still work)
2. Light → Heavy (new, should combo)
3. Light → Grab (new, should combo)
4. Buffered Heavy during cancel window (should execute)

**Feel improvement:** Different combo routes feel distinct. Game has depth.

---

### Phase 3: Impact Escalation (1-2 days)

**Goal:** Make combos feel MORE satisfying as they continue

**Add:**
- Hit flash escalation (white → yellow → orange)
- Screen shake escalation (2px → 4px → 8px)
- Sound pitch shift (optional, if time)

#### Screen Shake Escalation

```rust
// src/systems/hitstop.rs - MODIFY existing

/// Visual feedback during hitstop - escalating screen shake
pub fn hitstop_screen_shake(
    query: Query<&Hitstop>,
    chain_query: Query<&ChainState>,  // NEW: Get combo count
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let max_hitstop = query.iter()
        .map(|h| h.total_frames - h.frames_remaining)
        .max()
        .unwrap_or(0);

    if max_hitstop == 0 { return; }

    // Get highest hit count across all chains
    let max_hits = chain_query.iter()
        .map(|c| c.hit_count)
        .max()
        .unwrap_or(0);

    for mut transform in camera_query.iter_mut() {
        // Base shake on hitstop type
        let base = if max_hitstop >= 13 { 3.0 } else { 1.5 };

        // Escalate based on combo hits
        let shake_amount = match max_hits {
            0..=1 => base,           // 1.5px or 3px
            2 => base * 1.5,         // 2.25px or 4.5px
            3 => base * 2.5,         // 3.75px or 7.5px
            _ => base * 3.0,         // 4.5px or 9px
        };

        // Alternating shake pattern
        let shake_dir = if (max_hitstop % 2) == 0 { 1.0 } else { -1.0 };
        transform.translation.x += shake_dir * shake_amount;

        debug!("Shake: {:.1}px (hit #{})", shake_amount, max_hits);
    }
}
```

#### Hit Flash Escalation

```rust
// src/systems/visual_effects.rs (NEW FILE)

use bevy::prelude::*;
use crate::systems::combo::ChainState;

/// Flash effect color based on combo count
pub fn combo_hit_flash(
    chain_query: Query<(&ChainState, &Transform), Changed<ChainState>>,
    mut gizmos: Gizmos,
) {
    for (chain_state, transform) in chain_query.iter() {
        if chain_state.hit_count == 0 { continue; }

        let (color, radius) = match chain_state.hit_count {
            1 => (Color::srgba(1.0, 1.0, 1.0, 0.8), 30.0),    // White
            2 => (Color::srgba(1.0, 1.0, 0.0, 0.9), 40.0),    // Yellow
            3 => (Color::srgba(1.0, 0.65, 0.0, 1.0), 50.0),   // Orange
            _ => (Color::srgba(1.0, 0.3, 0.0, 1.0), 60.0),    // Red-orange
        };

        // Draw flash ring (temporary, until we have sprite effects)
        let pos = transform.translation.truncate();
        gizmos.circle_2d(pos, radius, color);

        info!("Hit flash: {} ({:?})", chain_state.hit_count, color);
    }
}

/// Debug: Log combo hits
pub fn debug_combo_hits(
    query: Query<(&Player, &ChainState), Changed<ChainState>>,
) {
    for (player, chain) in query.iter() {
        if chain.hit_count > 0 {
            info!("Player {:?}: {} HIT COMBO!", player, chain.hit_count);
        }
    }
}
```

#### Sound Pitch Shift (Optional)

```rust
// If adding sound:
// Hit 1: pitch 1.0
// Hit 2: pitch 1.1
// Hit 3: pitch 1.2
// Hit 4: pitch 1.3
```

**Test:** Land 3-hit combo. Each hit should feel progressively more impactful.

**Feel improvement:** Combos feel satisfying and escalate naturally.

---

## Integration Checklist

### File Changes

**New Files:**
- [ ] `src/components/combo.rs` - InputBuffer component
- [ ] `src/systems/visual_effects.rs` - Hit flash system

**Modified Files:**
- [ ] `src/systems/chain.rs` → `src/systems/combo.rs` (rename + expand)
- [ ] `src/systems/hitstop.rs` - Add shake escalation
- [ ] `src/plugins/core_game.rs` - Register new systems
- [ ] `src/components/mod.rs` - Export combo module

### System Registration Order

```rust
// In core_game.rs
.add_systems(Update, (
    // Input phase
    record_inputs_to_buffer,
    age_input_buffers,

    // Combat phase
    detect_hits,
    mark_chainable_on_hit,
    manage_chain_window,
    handle_chain_input,  // Uses buffer

    // Visual phase
    combo_hit_flash,
    debug_combo_hits,
).chain())
```

### Spawn Changes

```rust
// Add InputBuffer to character spawn
commands.spawn((
    Character,
    Player::One,
    Health::default(),
    ChainState::default(),
    InputBuffer::default(),  // NEW
    // ... rest
));
```

---

## Testing Plan

### Manual Tests

1. **Input buffer works:**
   - Press Light during hitstop → should buffer
   - Press Heavy 3 frames before window → should still cancel

2. **Cancel routes work:**
   - Light → Light → Light (should work)
   - Light → Light → Heavy (should work)
   - Light → Light → Grab (should work)
   - Light (blocked) → Grab (should NOT work - hit only)

3. **Visual escalation works:**
   - 1-hit: Small shake, white flash
   - 2-hit: Medium shake, yellow flash
   - 3-hit: Big shake, orange flash

### Success Criteria

- [ ] Combos feel responsive (buffer makes timing forgiving)
- [ ] Combos have variety (Heavy vs Grab routes feel different)
- [ ] Combos feel impactful (shake/flash escalate)
- [ ] No execution barriers (anyone can do basic combos)

---

## What We're NOT Doing

**Skipped for MVP (can add later):**
- ❌ Damage scaling per hit (balance, not feel)
- ❌ Combo counter UI (polish)
- ❌ ComboTracker component (over-engineered)
- ❌ Counter Heavy → Light extensions (edge case)
- ❌ Per-move cancel data in RON (can hardcode)
- ❌ Complex cancel trees (keep it simple)

**Why skip these?**
- They don't improve how the game FEELS
- They're balance/polish concerns
- MVP needs feel-good, not feature-complete

---

## Timeline

| Phase | Goal | Time |
|-------|------|------|
| **1** | Input buffer | 1 day |
| **2** | Expand cancels | 2-3 days |
| **3** | Impact escalation | 1-2 days |

**Total: 4-6 days**

---

## Expected Outcome

After implementing this, players should say:
- "Combos feel smooth and responsive"
- "I can actually do them without practicing timing"
- "Landing a 3-hit feels WAY more satisfying than a single hit"
- "The game has depth - I can choose damage or mixup routes"

**That's what feel-first means.**

The game doesn't need perfect balance or systems depth yet. It needs to feel GOOD to play.
