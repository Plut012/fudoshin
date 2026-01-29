# Counter Hit Cancel System

## ❌ IMPLEMENTATION STATUS: NOT STARTED

**Date Created:** 2026-01-29

**What's Working:**
- ✅ Counter hit detection system (bonus hitstun, gold flash)
- ✅ Basic combo cancel system (Light → Light/Heavy/Grab)
- ✅ Hitstop system with counter hit bonuses (+3 frames)
- ❌ Counter hit-specific cancel routes (NOT IMPLEMENTED)
- ❌ Counter hit cancel window extensions (NOT IMPLEMENTED)
- ❌ Enhanced visual feedback for counter cancels (NOT IMPLEMENTED)

**Dependencies:**
- Counter hit system (COMPLETE)
- Combo/cancel system (COMPLETE)
- Hitstop system (COMPLETE)

**Next Steps:**
1. Design counter hit cancel rules (which moves gain cancel ability on counter)
2. Extend MoveData to support counter-specific cancellable_into lists
3. Implement counter hit cancel detection in combo system
4. Add enhanced visual/audio feedback for counter cancels
5. Balance damage and advantage for counter hit routes

---

## Overview

Implement **counter hit-specific cancel routes** that reward players for landing hits during the opponent's startup frames. Counter hit cancels allow moves that normally can't be canceled (or have limited cancel options) to gain additional cancel routes when they land as counter hits, enabling extended combos and higher damage conversions.

This system emphasizes **offensive reads** and **frame trap mastery** - the core of Fudoshin's "reads over reactions" philosophy.

## Context

### What are Counter Hit Cancels?

When an attack hits an opponent during their attack's startup frames (a "counter hit"), the attacker gains bonus hitstun (+10 frames in Fudoshin). Counter hit cancels extend this reward by:

1. **Unlocking new cancel routes** - Moves that can't normally cancel gain cancel options
2. **Extending cancel windows** - More time to input the next move
3. **Enabling optimal punishes** - Maximum damage conversions from counter hit starters

### Industry Examples

**Guilty Gear Series:**
- Counter hits grant "Roman Cancel" option on moves that normally can't cancel
- Counter hit confirms lead to extended combos (30-50% more damage)
- Requires hit confirmation skill - "did that counter hit?"

**Street Fighter V:**
- Counter hits extend cancel windows
- Some normals only cancellable on counter hit
- Frame traps (leaving small gaps to bait counter hits) are fundamental strategy

**Under Night In-Birth:**
- Counter hits grant additional cancel options
- "GRD" system rewards offensive counter hit play
- Counter hit confirms separate skilled players from beginners

### Research Sources

- [Counter Hits in Fighting Games - Core-A Gaming](https://www.youtube.com/watch?v=_R0hbe8HZj0)
- [Frame Traps Explained - Sajam](https://www.youtube.com/watch?v=p64gSIPxJaw)
- [Street Fighter V Frame Data - FAT](https://fullmeter.com/fatonline/)

## Current State

**What exists:**
- Counter hit detection (hits during startup grant +10 hitstun bonus)
- Counter hit visual feedback (gold flash instead of red)
- Counter hit hitstop bonus (+3 frames)
- Basic cancel system (Light → Light/Heavy/Grab)

**What's missing:**
- Counter-specific cancel routes (e.g., Heavy can only cancel on counter hit)
- Extended cancel windows on counter hit
- Clear visual distinction for counter hit cancel opportunities
- Counter hit combo routes (Counter Heavy → Light → Heavy)

**Current damage comparison:**
```
Normal Heavy hit:     15-18 damage (no cancel)
Counter Heavy hit:    15-18 damage (no cancel) ← SAME!
Light → Light → Heavy: ~31 damage

Counter Heavy SHOULD enable extension:
Counter Heavy → Light → Heavy: ~46 damage (NOT POSSIBLE YET)
```

**The problem:** Counter hits grant bonus hitstun but no extra combo potential beyond existing routes.

## Goal

Create a counter hit cancel system that:

1. **Rewards offensive reads** - Counter hit starters enable optimal damage
2. **Adds route variety** - Different combo paths based on hit type
3. **Requires confirmation** - Players must recognize counter hit and adjust combo
4. **Maintains simplicity** - No complex execution, just recognition
5. **Balances risk/reward** - High damage but requires setup (frame traps)

## Design Principles

### Principle 1: Recognition Over Execution

**Philosophy:** "The skill is seeing the counter hit, not executing the cancel."

With existing hitstop (13f on counter heavy hits) and gold flash visual:
- Players have ~200ms to recognize counter hit
- Input buffer (8 frames) makes execution forgiving
- Challenge is mental (hit confirm) not mechanical (tight timing)

### Principle 2: Meaningful Damage Increase

**Philosophy:** "Counter hit conversions should feel WORTH the setup."

Damage scaling:
```
Normal routes:
- Light → Heavy:           ~23 damage (30%)
- Light → Light → Heavy:   ~31 damage (40%)

Counter hit routes:
- Counter Heavy → Light → Heavy: ~46 damage (60%)
- Counter Light → Heavy:         ~30 damage (38%)
```

Counter hit optimal combos should deal **20-30% more damage** than normal routes.

### Principle 3: Character Expression

**Philosophy:** "Different characters emphasize different counter hit routes."

Archetypes:
- **Pressure characters:** Counter Light → Light → Heavy (chain extension)
- **Punish characters:** Counter Heavy → Light → Heavy (max damage)
- **Mixup characters:** Counter Heavy → Grab (command grab conversion)

Each character's movelist defines which moves gain counter cancel options.

### Principle 4: Clear Visual Communication

**Philosophy:** "Players and spectators should know a counter hit cancel happened."

Visual escalation:
- Normal hit: Red flash
- Counter hit: **Gold flash**
- Counter hit cancel: **Gold flash + trail effect** (NEW)
- Counter combo ender: **Gold shockwave** (NEW)

## Implementation Plan

### Phase 1: Data Structure Extensions

**Goal:** Support counter-specific cancel rules in MoveData

**1. Extend MoveData Structure**

```rust
// src/components/movelist.rs
pub struct MoveData {
    // ... existing fields
    pub cancellable_into: Vec<AttackType>,         // Normal cancels
    pub counter_cancellable_into: Vec<AttackType>, // NEW: Counter-only cancels
    pub cancel_window: u32,                        // Normal cancel window
    pub counter_cancel_window: u32,                // NEW: Extended window on counter
}
```

**2. Update Default Movelist**

```rust
// Example: Heavy attacks normally can't cancel, but can on counter hit
MoveData {
    name: "Neutral Heavy",
    // ... other fields
    cancellable_into: vec![],  // No normal cancels
    counter_cancellable_into: vec![
        AttackType::Light,     // Can cancel to Light on counter
        AttackType::Heavy,     // Can cancel to Heavy on counter
    ],
    cancel_window: 0,          // No normal cancel window
    counter_cancel_window: 7,  // 7-frame window on counter hit
}
```

**3. Configure Cancel Rules**

Proposed rules for generic movelist:
```
Light attacks:
- Normal: Can cancel to Light/Heavy/Grab (existing)
- Counter: Same as normal (no change needed)

Heavy attacks:
- Normal: Cannot cancel (safety tradeoff)
- Counter: Can cancel to Light/Heavy (reward for counter hit)

Grab:
- Normal: Cannot cancel (ends combo)
- Counter: Cannot cancel (still ends combo)
```

### Phase 2: Counter Hit Detection Integration

**Goal:** Combo system recognizes counter hits and unlocks cancel routes

**1. Track Counter Hit State**

```rust
// src/components/combat.rs
#[derive(Component)]
pub struct CounterHitState {
    pub is_counter: bool,
    pub frames_remaining: u32,  // How long counter state lasts
}

// Applied when counter hit lands
impl CounterHitState {
    pub fn new() -> Self {
        Self {
            is_counter: true,
            frames_remaining: 15,  // Lasts through typical combo window
        }
    }
}
```

**2. Apply Counter State on Hit**

```rust
// src/systems/damage.rs (or counter_hit.rs)
pub fn mark_counter_hit_state(
    mut commands: Commands,
    mut hit_events: EventReader<HitEvent>,
) {
    for event in hit_events.read() {
        if event.is_counter_hit {
            // Mark attacker as having landed counter hit
            commands.entity(event.attacker).insert(CounterHitState::new());
        }
    }
}
```

**3. Decay Counter State**

```rust
pub fn decay_counter_state(
    mut commands: Commands,
    mut query: Query<(Entity, &mut CounterHitState)>,
) {
    for (entity, mut state) in query.iter_mut() {
        if state.frames_remaining > 0 {
            state.frames_remaining -= 1;
        } else {
            commands.entity(entity).remove::<CounterHitState>();
        }
    }
}
```

### Phase 3: Enhanced Cancel System

**Goal:** Allow counter-specific cancels during cancel window

**1. Check Counter State During Cancel Processing**

```rust
// src/systems/chain.rs (modify existing cancel system)
pub fn process_attack_cancels(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &AttackData,
        &StateTimer,
        &Player,
        Option<&CounterHitState>,  // NEW: Check if counter state active
    )>,
    inputs: Res<CurrentInputs>,
    input_buffer_query: Query<&InputBuffer>,
) {
    for (entity, attack_data, timer, player, counter_state) in query.iter_mut() {
        let is_counter = counter_state.is_some();

        // Determine available cancels based on counter state
        let available_cancels = if is_counter {
            // Combine normal cancels + counter-only cancels
            let mut cancels = attack_data.move_data.cancellable_into.clone();
            cancels.extend(attack_data.move_data.counter_cancellable_into.clone());
            cancels
        } else {
            // Only normal cancels available
            attack_data.move_data.cancellable_into.clone()
        };

        // Determine cancel window
        let cancel_window = if is_counter {
            attack_data.move_data.counter_cancel_window.max(
                attack_data.move_data.cancel_window
            )
        } else {
            attack_data.move_data.cancel_window
        };

        // Check if in cancel window
        let frames_into_recovery = timer.current_frame()
            .saturating_sub(attack_data.move_data.startup + attack_data.move_data.active);

        if frames_into_recovery <= cancel_window {
            // Check buffered inputs for valid cancel
            if let Ok(buffer) = input_buffer_query.get(entity) {
                if let Some(buffered_attack) = buffer.get_most_recent() {
                    if available_cancels.contains(&buffered_attack) {
                        // Valid cancel! Execute it
                        execute_cancel(
                            &mut commands,
                            entity,
                            buffered_attack,
                            is_counter,  // Pass counter state for visual feedback
                        );
                    }
                }
            }
        }
    }
}
```

**2. Mark Counter Cancels for Visual Feedback**

```rust
#[derive(Component)]
pub struct CounterCancelMarker;  // Visual feedback trigger

fn execute_cancel(
    commands: &mut Commands,
    entity: Entity,
    next_attack: AttackType,
    was_counter_cancel: bool,
) {
    // Start new attack
    // ... existing cancel logic ...

    // Mark counter cancels for enhanced visuals
    if was_counter_cancel {
        commands.entity(entity).insert(CounterCancelMarker);
    }
}
```

### Phase 4: Visual and Audio Feedback

**Goal:** Make counter hit cancels visually distinct and satisfying

**1. Enhanced Hit Flash on Counter Cancel**

```rust
// src/systems/visual_feedback.rs
pub fn counter_cancel_visual_effect(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<CounterCancelMarker>>,
) {
    for (entity, transform) in query.iter() {
        // Spawn gold trail effect
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(1.0, 0.84, 0.0, 0.8),  // Gold
                    custom_size: Some(Vec2::new(80.0, 80.0)),
                    ..default()
                },
                transform: *transform,
                ..default()
            },
            TrailEffect {
                duration: 10,  // 10-frame fade out
                fade_rate: 0.1,
            },
        ));

        // Remove marker after spawning effect
        commands.entity(entity).remove::<CounterCancelMarker>();
    }
}
```

**2. Counter Combo Ender Effect**

```rust
// When counter combo finishes with Heavy ender
pub fn counter_combo_finish_effect(
    query: Query<(&ChainState, &AttackData, &CounterHitState)>,
    // ...
) {
    // If this is a Heavy ender of a counter hit combo:
    // - Gold shockwave burst (expanding circle)
    // - Extra screen shake (5px instead of 3px)
    // - Deeper hit sound
}
```

**3. UI Indicator**

```rust
// Display "COUNTER!" text on counter hit
// Display "COUNTER COMBO!" on successful counter cancel
// Flash combo counter gold instead of white
```

### Phase 5: Balance and Tuning

**Goal:** Ensure counter hit routes feel rewarding but not overpowered

**1. Damage Scaling Adjustments**

```rust
// Counter hit combos should deal more damage, but not excessive
// Apply slight damage scaling to prevent infinites

Normal scaling:
Hit 1: 100%
Hit 2: 100%
Hit 3: 90%

Counter scaling:
Hit 1 (counter): 100%
Hit 2: 100%
Hit 3: 95%  // Less scaling on counter combos (reward!)
```

**2. Frame Advantage Tuning**

```rust
// Counter Heavy → Light cancel should be:
// - Safe on block (not punishable)
// - Slightly plus on hit (can continue pressure)
// - Risk/reward balanced (committed to Heavy startup)
```

**3. Cancel Window Testing**

```
Test different cancel windows:
- 5 frames: Tight, requires anticipation
- 7 frames: Comfortable with buffer
- 10 frames: Very lenient

Recommended: 7-frame counter cancel window (matches buffer)
```

### Phase 6: Testing and Iteration

**Goal:** Validate counter hit cancels improve depth without complexity

**Test scenarios:**
1. **Frame trap setup** - Can players create frame traps to bait counter hits?
2. **Hit confirmation** - Can players react to counter hit flash and cancel?
3. **Damage scaling** - Are counter routes worth the setup?
4. **Balance** - Do counter cancels create dominant strategies?

**Playtest goals:**
- 10 matches minimum
- Track counter hit cancel usage rate
- Measure damage increase from counter routes
- Gather feedback on visual clarity
- Tune frame windows based on execution difficulty

## Testing Checklist

**Functionality:**
- [ ] Heavy attacks can cancel to Light on counter hit
- [ ] Heavy attacks cannot cancel on normal hit
- [ ] Counter cancel window is 7 frames
- [ ] Normal cancel window unchanged
- [ ] CounterHitState persists through combo
- [ ] CounterHitState expires after 15 frames

**Visual Feedback:**
- [ ] Gold flash on counter hit (existing)
- [ ] Gold trail effect on counter cancel (new)
- [ ] Gold shockwave on counter combo ender (new)
- [ ] "COUNTER COMBO!" UI text displays (new)
- [ ] Combo counter flashes gold (new)

**Balance:**
- [ ] Counter Heavy → Light → Heavy deals ~46 damage (60% health)
- [ ] Normal Light → Light → Heavy deals ~31 damage (40% health)
- [ ] Counter routes deal 20-30% more damage than normal routes
- [ ] Counter cancels are safe on block
- [ ] Frame traps enable counter hit setups

**Integration:**
- [ ] Works with existing cancel system
- [ ] Works with input buffer
- [ ] Works with hitstop system
- [ ] Works with damage scaling
- [ ] Doesn't break chain system

**Game Feel:**
- [ ] Counter hit cancels feel earned and satisfying
- [ ] Visual feedback is clear and exciting
- [ ] Players can react to counter hit and adjust combo
- [ ] Spectators can tell when counter cancel happens
- [ ] Adds depth without complexity

## Expected Impact

**Before Counter Hit Cancels:**
- Counter hits grant bonus hitstun (+10f) and gold flash
- But no additional combo potential beyond normal routes
- Counter Heavy hit: ~15 damage (same as normal Heavy)
- Limited reward for offensive reads

**After Counter Hit Cancels:**
- Counter hits unlock additional cancel routes
- Heavy attacks gain cancel options on counter hit
- Counter Heavy → Light → Heavy: ~46 damage (3x normal Heavy)
- Frame traps become viable strategy
- Offensive reads rewarded with optimal damage
- Depth increase without execution barrier (still uses buffer)

**Gameplay changes:**
- Players set up frame traps to bait counter hits
- Hit confirmation becomes important skill expression
- Counter hit conversions separate skilled players
- More route variety (normal routes vs. counter routes)
- Offensive momentum rewarded

## Alignment with Fudoshin's Philosophy

From the design documents:

> **"Reads Over Reactions — Prediction and pattern recognition matter more than raw input speed."**

Counter hit cancels reward:
- **Offensive reads** (setting up frame traps to bait attacks)
- **Pattern recognition** (recognizing counter hit flash and adjusting)
- **Decision-making** (choosing optimal counter route)
- **Mental execution** (hit confirming, not tight inputs)

> **"Every Frame Matters — Frame advantage creates offensive/defensive rhythm."**

Counter hits are the ultimate expression of frame advantage:
- Landing counter hit means you acted first (frame advantage)
- Counter cancel rewards this advantage with extended combo
- Creates incentive to maintain offensive pressure

> **"Consequences over Complexity — Clear cause and effect, readable game state."**

Counter hit cancels have clear cause/effect:
- **Cause:** Land hit during opponent's startup (counter hit)
- **Effect:** Gold flash + unlocked cancel routes
- **Result:** Higher damage combo (46 vs. 15 damage)

**Clear visual communication:** Gold flash → Gold trail → Gold shockwave → "COUNTER COMBO!"

## Combo Route Examples

### Basic Character (The Conscript)

**Normal Routes:**
```
1. Light → Light → Heavy
   Damage: ~31 (40% health)
   Execution: Easy

2. Light → Heavy
   Damage: ~23 (30% health)
   Execution: Easy
```

**Counter Hit Routes (NEW):**
```
3. Counter Heavy → Light → Heavy
   Damage: ~46 (60% health)
   Execution: Medium (requires counter hit confirm)
   Setup: Frame trap after blocked Light

4. Counter Heavy → Heavy
   Damage: ~33 (43% health)
   Execution: Easy
   Setup: Whiff punish
```

**Comparison:**
- Normal Heavy: 15 damage
- Counter Heavy cancel route: 46 damage (3x!)
- Skill requirement: Recognizing counter hit and confirming cancel
- Execution requirement: Same buffer window as normal combos

### Pressure Character (Future)

**Counter Routes:**
```
Counter Light → Light → Light → Heavy
Damage: ~42 (55% health)
Extended chain pressure
```

### Punish Character (Future)

**Counter Routes:**
```
Counter Heavy → Heavy → Special
Damage: ~65 (85% health)
Maximum damage punish
```

## Success Metrics

The counter hit cancel system is successful if:

1. **Usage Rate:** Players attempt counter hit cancels in 20%+ of matches
2. **Conversion Rate:** Players successfully convert 50%+ of counter hit attempts
3. **Damage Increase:** Counter routes deal 20-30% more damage than normal routes
4. **Perception:** Players report counter cancels feel "earned" and "satisfying"
5. **Spectator Clarity:** Observers can tell when counter cancels happen
6. **Balance:** Counter routes don't become dominant/only strategy

## Implementation Priority

**Priority: HIGH (Phase 5 Polish)**

This feature is part of the Phase 5 completion goals (Option A - Polish Phase 5). It directly enhances:
- Combo system depth (rewards reads)
- Offensive gameplay (frame traps)
- Skill expression (hit confirmation)
- Game feel (satisfying conversions)

**Estimated time:** 1-2 days
- Phase 1-2: 4-6 hours (data structure + detection)
- Phase 3: 4-6 hours (cancel system integration)
- Phase 4: 2-3 hours (visual feedback)
- Phase 5: 2-4 hours (balance and testing)

**Dependencies:** All complete (counter hits, combos, hitstop, buffer)

## References

- `docs/todo/SHORT_COMBO_SYSTEM.md` - Combo system design
- `docs/todo/HITSTOP_IMPLEMENTATION.md` - Hitstop integration
- `docs/mvp_plan.md` - Phase 5 polish goals
- `src/systems/chain.rs` - Existing cancel system
- `src/systems/damage.rs` - Counter hit detection

**Ready to reward offensive reads.**
