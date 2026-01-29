# Fudoshin - Implementation Progress

## ✅ Phase 0: Project Initialization (COMPLETE)

**Setup complete:**
- Bevy 0.14 with dynamic linking for fast iteration
- Clean folder structure following data-driven architecture
- Hot-reload infrastructure ready
- Inspector tools configured (F1 to toggle)
- 60 FPS frame pacing locked

---

## ✅ Phase 1: Movement Foundation (COMPLETE)

**What's working:**
- ✅ Two player characters spawn (red/blue rectangles)
- ✅ Keyboard input for both players
- ✅ Walk movement (WASD for P1, Arrows for P2)
- ✅ Responsive, snappy controls
- ✅ Stage boundary clamping
- ✅ State machine (Idle ↔ Walking)
- ✅ Frame-perfect input processing

---

## ✅ Phase 2: Core Combat Triangle (COMPLETE)

**What's working:**

### ✅ Hitbox/Hurtbox System
- AABB collision detection
- Active hitboxes vs hurtboxes
- Debug visualization (F1 to see boxes)
- World-space positioning

### ✅ Attack System (All 3 Types)
- **Light Attack** (J / Numpad1): 6f startup, 2f active, 10f recovery
  - Small hitbox, 1 damage, -2f on block
  - 15 frames hitstun on hit
- **Heavy Attack** (K / Numpad2): 14f startup, 4f active, 18f recovery
  - Large hitbox, 2 damage, -8f on block
  - Light Armor property (absorbs one Light)
  - 25 frames hitstun on hit
- **Grab** (L / Numpad3): 10f startup, 2f active, 20f recovery
  - Short range, Unblockable property
  - Beats blocking opponents

### ✅ Block & Guard System
- **Block** (I / Numpad0): Hold to block attacks
- **Guard Meter**: Fills when blocking, drains passively
  - Light blocked: +15% guard
  - Heavy blocked: +35% guard
  - Drain: -5% per second
- **Guard Break**: Meter full → 40 frame stagger
- **Visual feedback**: Darken when blocking, gray when staggered

### ✅ Hit Reactions
- Hitstun on successful hits (can't act during)
- Red flash visual feedback
- Console logging for debugging
- Stagger state locks out actions

### ✅ Parry System
- **Parry** (Tap I / Numpad0): 6f active window
- Success: Attacker staggers 20f, defender can act immediately
- Restores 25% guard meter on success
- Visual: Bright cyan during window, white flash on success
- High risk, high reward

### ✅ Evade System
- **Evade** (Shift + Direction): Quick dash with i-frames
- 3f startup, 4f invincibility, 8f recovery (15f total)
- Fast movement (500 units/sec) in any direction
- Visual: Semi-transparent (50% during i-frames, 70% otherwise)
- Invincible during i-frame window

---

## Controls

**Player 1:**
- WASD - Movement
- J - Light Attack
- K - Heavy Attack
- L - Grab
- I - Block/Parry (tap for parry)
- Shift + Direction - Evade

**Player 2:**
- Arrow Keys - Movement
- Numpad 1 - Light Attack
- Numpad 2 - Heavy Attack
- Numpad 3 - Grab
- Numpad 0 - Block/Parry (tap for parry)
- Right Shift + Direction - Evade

**Debug:**
- F1 - Toggle inspector (see hitboxes, components)

---

## Combat Triangle Status

✅ **Attack > Grab** - Attacks stuff grab startup
✅ **Grab > Block** - Unblockable property works
✅ **Block > Attack** - Guard meter fills, eventually breaks
✅ **Hits apply hitstun** - Red flash, can't act
✅ **Parry beats Attack** - 6f window, attacker staggers
✅ **Evade beats everything** - 4f i-frames dodge all attacks

---

## How to Test

```bash
cargo run
```

1. Move characters close together (WASD / Arrows)
2. Press J to Light attack - see white flash, red hitbox
3. Hit connects - opponent flashes red, frozen briefly
4. Hold I to block - character darkens
5. Block 3-4 attacks - guard breaks, gray stagger
6. Press L (Grab) against blocking opponent - breaks through!

---

## Architecture Highlights

**Systems organized into groups:**
1. Input & movement processing
2. State progression (attack phases, stagger)
3. Physics & collision detection
4. Reactions (damage, guard, hitstun)
5. Visual feedback
6. Debug visualization

**Key files:**
- `src/systems/attack.rs` - Attack input, phase progression, hitbox activation
- `src/systems/collision.rs` - Hitbox/hurtbox detection, HitEvent emission
- `src/systems/damage.rs` - Hit reactions, hitstun application
- `src/systems/guard.rs` - Block, guard meter, guard break
- `src/components/state.rs` - CharacterState enum, AttackData
- `src/components/combat.rs` - Hitbox, Hurtbox, AttackProperty
- `src/events/combat_events.rs` - HitEvent, GuardBreakEvent, etc.

---

## ✅ Phase 3: Initiative & Pressure (COMPLETE)

**What's working:**

### ✅ Initiative System (Frame Advantage)
- Tracks +/- frames after interactions
- **Light attack hit**: +4f advantage / -4f disadvantage
- **Heavy attack hit**: +6f advantage / -6f disadvantage
- **Parry success**: +12f advantage / -12f disadvantage
- **On block**: Varies by attack (-2f to +8f)
- Visual: Green up arrow (advantage) / Red down arrow (disadvantage)
- Ticks down each frame automatically

### ✅ Pressure System
- Tracks offensive momentum with 0-3 intensity levels
- **Build pressure**: On hit, counter hit, or chain attack
- **Pressure bonuses**:
  - Level 1: +5% movement speed
  - Level 2: +10% movement speed, -1f attack startup
  - Level 3: +15% movement speed, -2f attack startup
- **Decay**: Passive drain when not attacking
- Visual: Character glows brighter at higher pressure levels

### ✅ Chain Attack System
- **Light → Light cancels**: Cancel recovery into new Light on hit
- **Chain window**: 7 frames during recovery phase
- **Max chains**: Up to 2-hit combo
- **Not chainable on block**: Only successful hits enable chaining
- Visual: Yellow circle indicator during chain window
- Integrates with pressure system for combo momentum

### ✅ Counter Hit System
- **Detects hits during startup**: Opponent vulnerable in attack startup
- **Bonus hitstun**: +10 frames on counter hit
  - Light counter hit: 25f hitstun (vs 15f normal)
  - Heavy counter hit: 35f hitstun (vs 25f normal)
- Visual: Gold/yellow flash (vs red for normal hit)
- Console logging: "COUNTER HIT!" message

### ✅ Momentum System
- **Tracks win streaks**: Levels 0-5 based on consecutive successes
- **Gain momentum**: On hits, chains, parries (double gain)
- **Lose momentum**: When hit or after inactivity
- **Bonuses at level 3+**:
  - Level 3: +10% damage, +20% guard damage
  - Level 4: +15% damage, +30% guard damage
  - Level 5: +25% damage, +50% guard damage
- **Decay**: Starts after 2 seconds of inactivity
- Visual: Expanding colored rings (green→cyan→gold)

---

## System Integration Notes

**Phase 3 systems work together:**
- Counter hit → Extra initiative advantage → Build pressure faster
- Pressure + Initiative → Chain window opportunities
- Successful chains → Build momentum
- High momentum → More damage → Easier to maintain pressure

**System execution order (per frame):**
1. Input & movement processing
2. State progression (initiative tick, momentum decay)
3. Physics & collision detection
4. Reactions (hit application, initiative/pressure/momentum changes)
5. Visual feedback (gizmos for all systems)
6. Debug logging

---

## ✅ Phase 4: Health States & Breath System (COMPLETE)

**What's Working:**
- ✅ Health system (Whole → Cut → Wounded → Broken states)
- ✅ State-based modifiers (speed, frame advantage, parry window)
- ✅ Breath system (3 stocks per match)
- ✅ Round structure with respawns
- ✅ Round timer and countdown
- ✅ Decisive Blow conditions
- ✅ Victory conditions and UI
- ✅ Match victory screen

---

## 🎯 Current: Phase 5 - Game Feel Foundation (IN PROGRESS)

**Phase 5 Status:** ~85% Complete

### 5.1 Hitstop System ✅ COMPLETE
- ✅ 9-13 frame freezes on hit
- ✅ Screen shake visual feedback
- ✅ Automatic application on hit/block/counter
- ✅ Integration with all combat systems

### 5.2 Hitbox Sizing ✅ COMPLETE
- ✅ All hitboxes increased ~50% to match Street Fighter standards
- ✅ Light attacks: 1.4-1.6x character width
- ✅ Heavy attacks: 2.0-2.5x character width
- ✅ Grab: 1.5x character width (very generous)

### 5.3 Combat Flow & Juggling ⚠️ IN PROGRESS
**Implemented:**
- ✅ Core cancel system (Light → Light/Heavy/Grab)
- ✅ 8-frame input buffer for lenient execution
- ✅ Per-move cancel customization - Data-driven cancel rules
- ✅ Variable cancel windows per move
- ✅ Visual feedback (escalating hit flashes)
- ✅ Chain state tracking

**New Direction: Stumble/Juggling System** ✅ COMPLETE
- ✅ Smash Bros-inspired juggling mechanic
- ✅ Launcher/Extender move properties
- ✅ 8-frame tech window for defender escape
- ✅ Direction-based stumble control
- ✅ Wall bounce mechanics
- ✅ Spike finishers (Neutral Heavy & Down Heavy on stumbling opponents)

See `docs/todo/STUMBLE_JUGGLING_SYSTEM.md` for full design and implementation plan.

---

## 🎉 Phase 5.3: Stumble/Juggling System COMPLETE!

**All 4 Phases Implemented:**
- ✅ **Phase 1:** Basic stumble state with launcher moves
- ✅ **Phase 2:** Tech system with 8-frame escape window
- ✅ **Phase 3:** Wall bounce mechanics with direction reversal
- ✅ **Phase 4:** Spike finishers (Neutral Heavy & Down Heavy)

**What's Working:**
- Heavy attacks launch opponents into stumble state
- Defender can tech out during 8-frame window (frame 5-13)
- Light attacks extend stumble with diminishing returns
- Wall bounces reverse direction and add +20f vulnerability
- Neutral Heavy and Down Heavy act as spike finishers when hitting stumbling opponents
- Complete visual feedback system (arrows, flashes, impact effects)

**Next Steps:** Phase 5 complete! Ready for Phase 6 (Combat Framework) or polish phase.

---

Last updated: 2026-01-29
Status: Phase 5 nearly complete! Hitstop and hitboxes done. Combo system functional with per-move cancel customization. Game feel is solid - ready for framework expansion or final polish.
