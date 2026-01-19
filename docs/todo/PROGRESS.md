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

## 🎯 Next: Phase 3 - Initiative & Pressure

**What it adds:**
1. **Initiative System**: Frame advantage/disadvantage tracking
2. **Pressure States**: Being "plus" gives offensive bonus
3. **Momentum**: Winning exchanges builds momentum
4. **Chain Attacks**: Cancel Light into Light on hit
5. **Counter Hits**: Extra damage/hitstun when hitting startup

**After Phase 3:** Health States (Whole → Cut → Wounded → Broken) and Breath system

---

Last updated: Phase 2 complete!
Status: Full combat triangle working - attacks, defense, parry, evade all functional
