# Phase 4: Health States & Breath System - Implementation Plan

**Goal:** Add consequences and stakes through health states, breath management, and win conditions.

**Success Criteria:** Players feel the weight of each hit, health states create distinct gameplay phases, breath system adds strategic depth, matches have clear victory conditions.

---

## Overview

Phase 3 gave us **offensive flow** - initiative, pressure, chains, and momentum. Phase 4 gives us **consequences** - every hit matters, every exchange has stakes, every round tells a story.

This is where Fudoshin becomes about **managing resources** and **finishing fights**.

**Key Philosophy:** "You don't just win by landing hits - you win by breaking their spirit."

---

## 1. Health System

**Concept:** Instead of traditional HP bars, characters have distinct health states that change gameplay.

### Health States

```rust
pub enum HealthState {
    Whole,    // 100% - Full power, no restrictions
    Cut,      // 75% - First blood, minor penalties
    Wounded,  // 50% - Significant damage, clear disadvantage
    Broken,   // 25% - Critical state, vulnerable to Decisive Blow
}
```

**Component:**
```rust
pub struct Health {
    pub current: f32,      // 0.0 - 100.0
    pub max: f32,          // 100.0
    pub state: HealthState,
}
```

### State Transitions

**Thresholds:**
- 100% → 75%: Whole → Cut (first significant hit)
- 75% → 50%: Cut → Wounded (accumulating damage)
- 50% → 25%: Wounded → Broken (critical threshold)
- 25% → 0%: Broken → Defeated (Decisive Blow)

**Visual Feedback:**
- Whole: Normal character color
- Cut: Slight desaturation, blood particles
- Wounded: More desaturated, heavy breathing effect
- Broken: Dark/faded, stagger animation, desperate state

### Damage Values

**Base Damage:**
- Light Attack: 8 damage
- Heavy Attack: 15 damage
- Grab: 12 damage
- Counter Hit: +5 damage bonus
- Momentum Bonus: Multiply by momentum.damage_bonus()

**Guard Damage (chip damage):**
- Light blocked: 2 damage
- Heavy blocked: 5 damage
- Momentum Bonus: Multiply by momentum.guard_damage_bonus()

**Frame Formula:**
```rust
let damage = base_damage
    * (if counter_hit { 1.5 } else { 1.0 })
    * momentum.damage_bonus();
```

---

## 2. State Effects

**Concept:** Each health state modifies gameplay, creating distinct phases of combat.

### Whole State (100-75%)
- No penalties
- Full movement speed
- Normal attack properties
- **Mental State:** Confident, aggressive

### Cut State (75-50%)
- -5% movement speed
- Guard meter fills 10% faster (more defensive)
- Pressure drains 20% faster
- **Mental State:** Cautious, calculating

### Wounded State (50-25%)
- -10% movement speed
- -2f frame advantage on all hits (you're hurt)
- Guard meter fills 20% faster
- Parry window reduced to 4f (from 6f)
- Cannot build Pressure above level 2
- **Mental State:** Desperate, defensive

### Broken State (25-0%)
- -15% movement speed
- -4f frame advantage on all hits
- Guard meter fills 30% faster (forced defensive)
- Parry window reduced to 3f
- Cannot build Pressure or Momentum
- Vulnerable to Decisive Blow
- **Mental State:** Survival mode

**Implementation:**
```rust
pub fn apply_health_state_effects(
    query: Query<(&Health, &mut MaxSpeed, &mut Initiative, &mut GuardMeter)>,
) {
    // Modify stats based on health state
}
```

---

## 3. Breath System (Stocks)

**Concept:** Each player has 3 "Breaths" (stocks/lives). Losing all Breaths = match loss.

### Component

```rust
pub struct Breath {
    pub current: u8,  // 0-3
    pub max: u8,      // 3
}
```

### Breath Loss Conditions

1. **Decisive Blow** (instant breath loss)
   - Attacker is at Whole/Cut state
   - Defender is at Broken state
   - Land Heavy or Grab
   - Triggers special animation + breath loss

2. **Timeout** (rare, discouraged)
   - Round timer expires (60 seconds)
   - Player with less health loses breath
   - Both lose breath if equal health

3. **Ringout** (future feature, Phase 5+)
   - Pushed outside stage boundaries
   - Instant breath loss

### Breath Respawn

**When a breath is lost:**
1. Screen freeze frame (10f pause)
2. Loser respawns at starting position
3. Winner stays in place
4. Both return to Whole state (100% health)
5. Reset all states: Initiative, Pressure, Momentum, Chains
6. 3-second countdown before next round
7. Match continues until someone reaches 0 breaths

**Component:**
```rust
pub struct MatchState {
    pub round_number: u32,
    pub round_time: f32,       // 60.0 seconds per round
    pub round_active: bool,
    pub winner: Option<Player>,
}
```

---

## 4. Decisive Blow System

**Concept:** The finishing move - only available against Broken opponents, instantly removes one breath.

### Conditions

**All must be true:**
1. ✅ Attacker is at Whole or Cut state (healthy enough)
2. ✅ Defender is at Broken state (vulnerable)
3. ✅ Attack type is Heavy or Grab (not Light)
4. ✅ Attack connects (not blocked/evaded/parried)

### Execution

**When Decisive Blow lands:**
1. Special visual effect (screen flash, slow-mo)
2. Cinematic camera zoom
3. Heavy impact sound/VFX
4. Defender loses 1 breath immediately
5. Screen freeze + transition to next round

**Component:**
```rust
pub struct DecisiveBlowState {
    pub available: bool,  // Can perform decisive blow
    pub active: bool,     // Currently executing
}
```

**Visual Indicators:**
- Attacker: Weapon/hands glow when decisive blow available
- Defender: Visual danger state (red pulsing, desperate animation)
- UI: "DANGER" text appears for broken player

---

## 5. Win Conditions & Match Flow

### Victory Conditions

1. **Breath Depletion** (primary)
   - Opponent has 0 breaths remaining
   - Instant match victory

2. **Timeout** (discouraged)
   - Round timer reaches 0
   - Player with more health wins the round
   - If equal health, both lose a breath (draw)

### Round Structure

**Round Start:**
```
1. Both players spawn at start positions
2. Health set to 100% (Whole state)
3. All combat states reset
4. 3-second countdown
5. "FIGHT!" appears
6. Round timer starts (60s)
```

**Round End:**
```
1. Breath loss triggered
2. Screen freeze (10f)
3. Round result display ("DECISIVE BLOW" / "TIMEOUT")
4. Update breath counters
5. Check for match victory
   - If match over: Victory screen
   - If match continues: Round Start
```

**Match Victory:**
```
1. Final hit lands / final breath lost
2. Extended freeze frame
3. Victory screen with stats:
   - Winner announced
   - Final breath count
   - Round count
   - Time elapsed
4. Return to main menu (Phase 5+)
```

---

## 6. UI Elements (Minimal)

**Essential HUD:**
- Breath indicators (circles/dots above character)
  - Player 1: Top left (3 circles)
  - Player 2: Top right (3 circles)
  - Filled = has breath, Empty = lost breath
- Round timer: Top center (60s countdown)
- Health bars: Below breath indicators
  - Color-coded by state (white → yellow → orange → red)
- Round counter: "Round 1", "Round 2", etc.

**In-Game Indicators:**
- "DECISIVE BLOW AVAILABLE" when opponent is Broken
- "DANGER" text pulsing for Broken player
- Round start countdown: "3... 2... 1... FIGHT!"
- Round end: "DECISIVE BLOW!" / "TIMEOUT!" / "VICTORY!"

**Implementation:**
```rust
pub fn render_breath_indicators(
    mut gizmos: Gizmos,
    query: Query<(&Player, &Breath, &Transform)>,
) {
    // Draw simple circles for breath count
}
```

---

## 7. Camera & Polish

### Camera System

**Basic Camera:**
- Side-view 2D camera (current)
- Fixed position at (0, 0) looking at stage
- Zoom level to show full stage

**Camera Effects (Phase 4.5):**
- Shake on heavy hits
- Zoom in on Decisive Blow
- Freeze frame on breath loss
- Slow motion on match point

### Audio (Placeholder)

**Sound Effects Needed:**
- Hit sounds (light/heavy/counter)
- Parry sound (clang)
- Guard break sound (crack)
- Decisive Blow sound (boom + reverb)
- Breath loss sound (dramatic sting)
- Victory fanfare

**Implementation Note:** Use bevy_audio with placeholder sounds, prepare for future sound design.

---

## 8. Testing & Balance

### Damage Balance Goals

**Time to Kill (TTK):**
- ~8-12 Light attacks to go from Whole → Broken
- ~5-7 Heavy attacks to go from Whole → Broken
- ~3-4 rounds per match on average

**Chip Damage:**
- Blocking everything = lose ~30% health per round
- Forces offensive play, can't turtle forever

**Counter Hit Damage:**
- Should feel impactful (+50% damage)
- Rewards defensive reads

**Momentum Scaling:**
- Level 5 momentum = +25% damage
- High risk/reward for momentum building

### Health State Breakpoints

**From Whole (100 HP):**
- Cut threshold: 75 HP (25 damage taken)
- Wounded threshold: 50 HP (50 damage taken)
- Broken threshold: 25 HP (75 damage taken)

**Example Scenarios:**
- 3 Heavy hits (15 dmg each) = 45 damage = Cut state
- 6 Heavy hits = 90 damage = Broken state
- 10 Light hits (8 dmg each) = 80 damage = Broken state

---

## Implementation Order

### Session 1: Health & Damage (2-3 hours)

1. **Health component & damage system** (60min)
   - Create Health component
   - Create HealthState enum
   - Apply damage from HitEvents
   - Integrate momentum damage bonuses
   - Add chip damage on blocked hits

2. **Health state transitions** (30min)
   - Check thresholds after damage
   - Transition between states
   - Visual feedback (color changes)

3. **State effects on gameplay** (45min)
   - Movement speed penalties
   - Frame advantage penalties
   - Guard meter modifications
   - Parry window changes
   - Pressure/momentum restrictions

### Session 2: Breath System (2-3 hours)

4. **Breath component & UI** (45min)
   - Create Breath component
   - Create MatchState component
   - Render breath indicators (simple circles)
   - Round timer display

5. **Decisive Blow detection** (45min)
   - Check conditions on hit
   - Special visual effect
   - Trigger breath loss
   - Screen freeze

6. **Round management** (60min)
   - Round start sequence
   - Countdown timer
   - Round end detection
   - Reset positions & states
   - Match victory detection

### Session 3: Polish & Balance (2-3 hours)

7. **UI polish** (45min)
   - Health bars with color coding
   - Danger indicators for Broken state
   - Round result text
   - Victory screen

8. **Visual effects** (45min)
   - Health state color changes
   - Decisive Blow flash effect
   - Breath loss particles
   - Hit impact effects

9. **Balance testing** (60min)
   - Adjust damage values
   - Test TTK (time to kill)
   - Verify state transitions feel good
   - Ensure chip damage is meaningful

**Total Estimated Time:** 6-9 hours

---

## Key Design Decisions

### Why Health States Instead of HP Bar?

**Pros:**
- More readable game state
- Creates distinct gameplay phases
- Adds drama and tension
- Easier to balance around breakpoints

**Cons:**
- Less granular than HP
- Requires careful threshold tuning

**Decision:** Use health states as primary system, but track numerical HP internally for precise damage calculation.

### Why 3 Breaths?

**Pros:**
- Classic fighting game format (2 out of 3 rounds)
- Enough for comebacks
- Not too long for casual play

**Cons:**
- Longer matches than single-round games

**Decision:** 3 breaths provides good match structure. Consider 1-breath mode for quick matches in Phase 5.

### Should Decisive Blow Be Automatic?

**Options:**
1. Heavy/Grab on Broken = Auto decisive blow
2. Special input required
3. Builds up "finisher meter"

**Decision:** Auto decisive blow (option 1). Keeps execution simple, focuses on reads. The challenge is getting opponent to Broken state, not executing the finisher.

### How Much Chip Damage?

**Goal:** Blocking is strong but not invincible. Can't win by only blocking.

**Formula:**
- Light chip: 2 HP (25% of attack damage)
- Heavy chip: 5 HP (33% of attack damage)

**Result:** Full round of blocking (~20 attacks) = ~50 HP chip damage = Wounded state.

---

## Technical Implementation Notes

### Damage System Architecture

```rust
// In damage.rs
pub fn apply_health_damage(
    mut hit_events: EventReader<HitEvent>,
    mut query: Query<(&mut Health, &Momentum, &Player)>,
    attacker_query: Query<&Momentum>,
) {
    for event in hit_events.read() {
        // Get base damage from attack type
        let base_damage = match event.attack_type {
            Light => 8.0,
            Heavy => 15.0,
            Grab => 12.0,
        };

        // Apply counter hit bonus
        let damage = if event.counter_hit {
            base_damage * 1.5
        } else {
            base_damage
        };

        // Apply momentum bonus
        if let Ok(momentum) = attacker_query.get(event.attacker) {
            damage *= momentum.damage_bonus();
        }

        // Apply chip damage if blocked
        if event.was_blocked {
            damage *= 0.25; // Chip damage is 25% of full damage
        }

        // Apply to defender's health
        if let Ok((mut health, _, _)) = query.get_mut(event.defender) {
            health.current -= damage;
            health.update_state();
        }
    }
}
```

### Health State Effects

```rust
// In health.rs
pub fn apply_health_state_modifiers(
    mut query: Query<(&Health, &mut MaxSpeed, &mut Initiative, &mut GuardMeter, &mut Pressure)>,
) {
    for (health, mut speed, mut initiative, mut guard, mut pressure) in query.iter_mut() {
        match health.state {
            HealthState::Whole => {
                // No penalties
            }
            HealthState::Cut => {
                speed.0 *= 0.95; // -5% speed
                guard.fill_rate *= 1.1; // +10% guard fill
                pressure.decay_rate *= 1.2; // +20% decay
            }
            HealthState::Wounded => {
                speed.0 *= 0.90; // -10% speed
                initiative.frames -= 2; // -2f disadvantage
                guard.fill_rate *= 1.2; // +20% guard fill
                pressure.max_intensity = 2; // Cap at level 2
            }
            HealthState::Broken => {
                speed.0 *= 0.85; // -15% speed
                initiative.frames -= 4; // -4f disadvantage
                guard.fill_rate *= 1.3; // +30% guard fill
                pressure.intensity = 0; // Reset pressure
                pressure.max_intensity = 0; // Can't build
            }
        }
    }
}
```

### Breath Management

```rust
// In breath.rs
pub fn check_decisive_blow(
    mut hit_events: EventReader<HitEvent>,
    mut commands: Commands,
    health_query: Query<&Health>,
    mut breath_query: Query<&mut Breath>,
) {
    for event in hit_events.read() {
        if event.was_blocked { continue; }

        // Check if defender is Broken
        let defender_health = health_query.get(event.defender).unwrap();
        if defender_health.state != HealthState::Broken {
            continue;
        }

        // Check if attack is Heavy or Grab
        if !matches!(event.attack_type, Heavy | Grab) {
            continue;
        }

        // Check if attacker is healthy enough
        let attacker_health = health_query.get(event.attacker).unwrap();
        if !matches!(attacker_health.state, Whole | Cut) {
            continue;
        }

        // DECISIVE BLOW!
        if let Ok(mut breath) = breath_query.get_mut(event.defender) {
            breath.current -= 1;

            // Trigger round end
            commands.insert_resource(RoundEndEvent {
                winner: event.attacker,
                reason: RoundEndReason::DecisiveBlow,
            });
        }
    }
}
```

---

## Expected Feel After Phase 4

**Early Round (Both at Whole):**
- Feeling each other out
- Building pressure and momentum
- Aggressive pacing

**Mid Round (Cut/Wounded):**
- More careful play
- Health advantage matters
- Defensive player turtles more

**Late Round (One player Broken):**
- Broken player plays desperately defensive
- Healthy player hunts for decisive blow
- High tension, clear win condition

**Multi-Round Match:**
- Comebacks feel earned
- Each round tells a story
- Momentum shifts between rounds

**Match Point:**
- One player at 1 breath, other at 2-3
- Maximum tension
- Every hit matters

---

## Deferred to Phase 5+

- Actual character roster with unique moves
- Stage hazards and stage-specific mechanics
- Throw tech / grab escape
- Advanced movement (dash, backdash, step)
- Special moves and supers
- Training mode
- Online multiplayer
- Replays and match recording
- Sound design and music
- Art pass (move from rectangles to sprites)
- Main menu and match setup
- Character select screen

---

## Success Metrics

After Phase 4, the game should be:
- ✅ **Playable** - Full matches with win conditions
- ✅ **Balanced** - Reasonable TTK, no infinite strategies
- ✅ **Readable** - Health states are clear and meaningful
- ✅ **Strategic** - Health advantage creates gameplay decisions
- ✅ **Exciting** - Decisive blows feel hype
- ✅ **Complete** - Core loop fully functional

**This is the MVP.** After Phase 4, Fudoshin has all core mechanics and is a "real" fighting game, ready for playtesting and iteration.

---

Last updated: Phase 4 planning
Next: Implementation begins!
