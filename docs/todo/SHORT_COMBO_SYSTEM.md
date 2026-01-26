# Short Combo System (2-4 Hits)

## Overview

Implement a **chunky, satisfying short combo system** that emphasizes impact over execution. Combos should be 2-4 hits maximum, deal significant damage, and feel earned rather than memorized. This aligns with Fudoshin's core philosophy: "2-4 clean hits can kill."

## Context

### Fudoshin's Combat Framework

Fudoshin uses a three-phase combat structure:
1. **Build Advantage** - Neutral game, accumulate pressure/initiative
2. **Stagger** - Break through defense (Guard Break, Counter Hit, etc.)
3. **Finish** - Decisive Blow for the kill

**Short combos fit into Phase 1 (Build Advantage)** and **Phase 3 (Execution Combo finish type)**.

### Industry Research

**Games That Excel at Short Combos:**

**Samurai Shodown:**
- Combos are short (2-4 hits) but damage is ridiculously high
- Single hard slash = 30%+ damage
- Focus on spacing and prediction over execution
- "Poking may be strong, but a well-placed fierce slash will ruin anyone's day more than most combos will"

**Fantasy Strike:**
- Combos are "very easy, short, and simple"
- Most common combo: 3 hits (Jump A → A → cancel B)
- 8-frame input buffer makes execution trivial
- Focus on **when** to combo, not **how**

**Street Fighter:**
- Classic SF2: 4-5 hit combos standard
- Modern SF: 5-8 hits average
- Shorter combos = more decision points per match
- Player preference: "Combos should be limited to 4-5 hits"

### Research Sources

- [Samurai Shodown Review - Kotaku](https://kotaku.com/samurai-shodown-is-slow-and-deliberate-in-the-best-way-1835805404)
- [Fantasy Strike Basics](https://www.fantasystrike.com/basics)
- [The Design of Combos and Chains - Game Developer](https://www.gamedeveloper.com/design/the-design-of-combos-and-chains)
- [What's the Point of Combos? - CritPoints](https://critpoints.net/2019/11/11/whats-the-point-of-combos-in-fighting-games/)

## Current State

**Existing Chain System:**
- Light → Light chain (cancels on hit)
- 2 hits total
- Simple and functional

**What's Missing:**
- No cancel windows beyond Light → Light
- No Heavy attack confirms
- No counter hit extensions
- No link timing (only chains)
- No combo variety/routes
- Damage per combo seems low for "lethal brevity" philosophy

**Current Damage (estimated):**
- Light attack: ~8 damage
- Heavy attack: ~15 damage
- Light → Light: ~16 damage (too low for a full combo)

## Goal

Create a combo system that:

1. **Keeps combos short** (2-4 hits maximum)
2. **Makes each hit chunky** (high damage, satisfying feedback)
3. **Simple execution** (generous buffers, no 1-frame links)
4. **Strategic depth** (route variety for different situations)
5. **Clear risk/reward** (commitment vs. damage tradeoffs)
6. **Aligns with combat phases** (advantage building combos vs. finish combos)

## Design Principles

### Principle 1: High Damage Per Hit

**Philosophy:** "Each hit should hurt."

Following Samurai Shodown's model:
- 2-hit combo: 25-35% damage
- 3-hit combo: 35-50% damage
- 4-hit combo: 50-70% damage (optimal punish)
- Single heavy hit: 15-25% damage

**Implication:** 2-4 combos across a round = kill

### Principle 2: Simple Inputs

**Philosophy:** "The challenge is WHEN, not HOW."

Following Fantasy Strike's accessibility:
- No 1-frame links
- Generous input buffer (6-8 frames)
- Chain system over strict links
- Cancel windows clearly defined
- No execution barriers

### Principle 3: Meaningful Routes

**Philosophy:** "Different situations demand different combos."

Combo routes should offer choices:
- **Damage optimization** - Maximum damage
- **Meter gain** - Build resources
- **Positioning** - Corner carry, knockback
- **Safety** - End in plus frames or knockdown
- **Reset** - End early for mixup opportunity

### Principle 4: Clear Confirms

**Philosophy:** "You should know if it hit."

With hitstop implemented:
- Easy to confirm hit vs. block
- Visual/audio feedback clear
- Generous cancel windows during hitstop
- No guessing if combo will work

### Principle 5: Respect the Three Phases

**Advantage Building Combos:**
- Deal 20-40% damage
- Build pressure/initiative
- Don't kill, but set up Stagger

**Execution Combo (Finish):**
- Guaranteed kill on Staggered opponent
- 2-4 hit sequence
- Character-specific animation
- Final Parry opportunity on last hit

## Implementation Plan

### Phase 1: Core Cancel System

**Goal:** Expand beyond Light → Light chains

**1. Define Cancel Rules**

```rust
// src/components/movelist.rs
pub struct MoveData {
    // ... existing fields
    pub cancellable_into: Vec<AttackType>, // What this move can cancel into
    pub cancel_window: u32,                // Frames of cancel window
}
```

**2. Basic Cancel Tree**

```
Light attacks:
- Light → Light (existing, on hit only)
- Light → Heavy (on hit only, damage route)
- Light → Grab (on hit/block, mixup route)

Heavy attacks:
- Heavy → Special (future: when specials exist)
- Counter Heavy → Light (counter hit only, extension)

Grab:
- Grab → (no cancels, ends combo)
```

**3. Implement Cancel System**

```rust
// src/systems/attack.rs or new src/systems/combos.rs
pub fn handle_attack_cancels(
    inputs: Res<CurrentInputs>,
    mut query: Query<(&AttackData, &StateTimer, &Player)>,
    // Check if in cancel window and new attack input pressed
) {
    // If in cancel window and valid cancel input:
    // - End current attack early
    // - Start new attack immediately
    // - Mark as "combo" for damage scaling
}
```

### Phase 2: Input Buffer System

**Goal:** Make combos easy to execute

**1. Create Buffer Component**

```rust
// src/components/input_buffer.rs
#[derive(Component)]
pub struct InputBuffer {
    pub buffered_inputs: Vec<(AttackType, u32)>, // (input, frames_ago)
    pub buffer_window: u32, // Default: 6-8 frames
}
```

**2. Buffer Input Collection**

```rust
pub fn collect_buffered_inputs(
    mut query: Query<(&PlayerInput, &mut InputBuffer)>,
) {
    // Store recent inputs (last 8 frames)
    // Oldest inputs expire
}
```

**3. Consume Buffered Inputs**

```rust
pub fn process_buffered_inputs(
    mut query: Query<(&mut InputBuffer, &AttackData, &StateTimer)>,
) {
    // When in cancel window, check if buffered input is valid
    // If yes, consume buffer and execute cancel
}
```

### Phase 3: Damage Scaling

**Goal:** Prevent infinite combos, encourage short combos

**1. Add Combo Counter**

```rust
#[derive(Component)]
pub struct ComboState {
    pub hit_count: u32,
    pub damage_dealt: f32,
    pub scaling_factor: f32,
}
```

**2. Scaling Formula**

```
No scaling for first 2 hits
Light scaling for hits 3-4 (90% damage)
Heavy scaling for hits 5+ (70% damage) - shouldn't happen

Formula: damage * (1.0 - (hit_count - 2) * 0.1).max(0.7)
```

**3. Reset Combo State**

```rust
// Reset when:
// - Combo ends (no follow-up)
// - Opponent blocks
// - Opponent gets hit but not in combo
```

### Phase 4: Combo Route Examples

**Goal:** Define character-agnostic combo templates

**Basic Routes:**

```
1. Light Confirm → Damage
   Input: Light → Light → Heavy
   Damage: ~35%
   Risk: Medium (Heavy is unsafe)
   Use: Confirming stray Light hit into damage

2. Heavy Punish → Extension
   Input: Counter Heavy → Light → Heavy
   Damage: ~50%
   Risk: High (requires counter hit)
   Use: Optimal punish on opponent's whiff

3. Light Pressure → Mixup
   Input: Light → Light → Grab
   Damage: ~30%
   Risk: Low (Grab safe on tech)
   Use: Conditioning opponent to block

4. Counter Light → Quick
   Input: Counter Light → Heavy
   Damage: ~40%
   Risk: Medium
   Use: Frame trap conversion
```

**Advanced Routes (Character-Specific):**

```
Pressure Character:
- Light → Light → Light (3-hit chain)
- Focus on volume and Guard damage

Punish Character:
- Counter Heavy → Heavy (2-hit, max damage)
- Focus on optimal damage per opening

Burst Character:
- Any hit → Special → Super
- Focus on momentum swings
```

### Phase 5: Integration with Combat Phases

**1. Advantage Building Combos**

During Phase 1 (Build Advantage):
- Combos deal 20-40% damage
- Build Initiative on hit
- Increase Pressure meter
- Erode opponent's Guard meter
- Don't trigger Stagger (unless Counter Hit special case)

**2. Execution Combo (Finish)**

During Phase 3 (Finish):
- Triggered when opponent is Staggered
- Automatic 2-4 hit sequence
- Character-specific animation
- Guaranteed kill
- Final Parry opportunity on last hit only

**Implementation:**

```rust
pub fn trigger_execution_combo(
    mut commands: Commands,
    query: Query<(Entity, &Player), With<StaggeredState>>,
    input: Res<CurrentInputs>,
) {
    // If opponent Staggered and Decisive Blow input pressed:
    // 1. Lock both players into cutscene mode
    // 2. Play character-specific combo animation
    // 3. Deal damage on each hit
    // 4. Final hit has 4-frame parry window
    // 5. If not parried, opponent loses Breath
}
```

### Phase 6: Visual and Audio Feedback

**Goal:** Make every hit in combo feel impactful

**1. Hitstop per Hit**

```rust
// Each hit in combo gets hitstop
// Scaling down slightly for multi-hits:
Hit 1: Full hitstop (9f for light, 13f for heavy)
Hit 2: 80% hitstop
Hit 3: 60% hitstop
Hit 4: 60% hitstop
Final hit: EXTRA hitstop (1.5x normal)
```

**2. Screen Shake per Hit**

```rust
// Escalating shake through combo
Hit 1: 2px shake
Hit 2: 3px shake
Hit 3: 5px shake
Hit 4: 8px shake
```

**3. Combo Counter UI**

```
- Display hit count during combo
- Display total damage
- Flash "COUNTER!" on counter hit starter
- Special flash/sound on 4-hit optimal
```

**4. Sound Design**

```
Hit 1: Sharp impact sound
Hit 2: Higher pitch impact
Hit 3: Deeper, meatier impact
Hit 4: Massive CRACK sound
Combo end: Brief silence, then whoosh
```

### Phase 7: Balance and Tuning

**Goal:** Ensure combos feel earned, not cheap

**1. Damage Values**

Test and adjust:
```
Light attack: 8-10 damage
Heavy attack: 18-22 damage
Grab: 15 damage

Light → Light: 16-18 damage (20% health)
Light → Heavy: 28-32 damage (35% health)
Light → Light → Heavy: 36-42 damage (45% health)
Counter Heavy → Light → Heavy: 48-54 damage (60% health)
```

**2. Cancel Window Tuning**

```
Light → Light: 4 frames (easy)
Light → Heavy: 3 frames (medium)
Counter extensions: 5 frames (generous reward)
```

**3. Risk/Reward**

```
Safe combos: Lower damage (Light → Light)
Risky combos: Higher damage (Light → Heavy)
Optimal combos: Maximum damage but require setup (Counter)
```

## Testing Checklist

**Execution Feel:**
- [ ] Combos feel responsive (buffer system working)
- [ ] No dropped combos from execution errors
- [ ] Cancel windows feel consistent
- [ ] Hitstop makes confirms easy

**Damage Balance:**
- [ ] 2-hit combos deal ~30% damage
- [ ] 3-hit combos deal ~45% damage
- [ ] 4-hit combos deal ~60% damage
- [ ] Single hits still viable (15-25%)

**Strategic Depth:**
- [ ] Different combo routes have clear purposes
- [ ] Players choose routes based on situation
- [ ] Risk/reward feels balanced
- [ ] Optimal combos feel earned, not free

**Integration:**
- [ ] Combos work during Advantage Building phase
- [ ] Don't break existing systems (Initiative, Pressure, Guard)
- [ ] Execution Combo finish works on Staggered opponents
- [ ] Final Parry opportunity works correctly

**Game Feel:**
- [ ] Each hit feels chunky and satisfying
- [ ] Combo enders feel conclusive
- [ ] Visual/audio feedback clear
- [ ] Spectators can follow what's happening

## Expected Impact

**Before:**
- Only Light → Light chain exists
- Combos feel limited and samey
- Low damage per combo
- No route variety

**After:**
- Multiple combo routes per character
- Each combo feels impactful and earned
- 2-4 hits can deal 30-60% damage
- Strategic choices during combos
- Aligns with "lethal brevity" philosophy
- Matches Samurai Shodown's chunky, high-damage feel

## Alignment with Fudoshin's Philosophy

From the design documents:

> **"Lethal Brevity — Matches are short and intense. 2-4 clean hits can kill."**

This combo system achieves exactly that:
- 2-hit combo = 30% damage
- 3-hit combo = 45% damage
- 4-hit combo = 60% damage
- ~3 combos per round = kill

> **"Reads Over Reactions — Prediction and pattern recognition matter more than raw input speed."**

Achieved through:
- Simple execution (no execution barriers)
- Generous buffers
- Focus on WHEN to combo, not HOW
- Different routes for different reads

> **"Simple Inputs, Deep Decisions — No complex motion inputs or memorized combo strings."**

Achieved through:
- Chain/cancel system (no strict links)
- Maximum 4 hits (no long memorization)
- Clear cancel rules
- Buffer system makes execution trivial

This positions Fudoshin perfectly between:
- **Samurai Shodown's** high damage, low combo focus
- **Fantasy Strike's** accessibility and simplicity
- **Street Fighter's** combo variety and strategic depth

The result: A unique fighting game where every hit matters, combos are earned through reads rather than execution, and matches are decided by superior understanding rather than mechanical skill.
