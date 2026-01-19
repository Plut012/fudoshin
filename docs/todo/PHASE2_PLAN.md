# Phase 2: Core Combat Triangle - Implementation Plan

**Goal:** Implement attack/defense systems so two players can have meaningful exchanges.

**Success Criteria:** Triangle works (Attack>Grab>Block>Attack), parry feels rewarding, exchanges are fast and responsive.

---

## 1. Hitbox/Hurtbox System (Foundation)

**Components to add:**
```rust
Hitbox { rect: Rect, active: bool, damage: u8, properties: Vec<AttackProperty> }
Hurtbox { rect: Rect }
AttackProperty enum { LightArmor, Unblockable, ... }
```

**Systems to create:**
- `detect_collisions` - AABB collision between active hitboxes and hurtboxes
- Emit `HitEvent { attacker, defender, damage, was_blocked }`

**Test:** Spawn temporary hitbox, verify collision detection works

---

## 2. Attack State System

**Extend CharacterState:**
```rust
Attacking {
    move_type: AttackType,  // Light, Heavy, Grab
    phase: AttackPhase,     // Startup, Active, Recovery
    frames_remaining: u32,
}
```

**Components:**
```rust
FrameTimer { elapsed: u32, target: u32 }
AttackData { startup: u32, active: u32, recovery: u32 }
```

**Systems:**
- `progress_attack_phase` - Tick frames, transition Startup→Active→Recovery→Idle
- `activate_hitbox_on_active` - Enable hitbox during Active phase
- `deactivate_hitbox_on_recovery` - Disable hitbox after Active

**Test:** Light attack goes through full 6f+2f+10f cycle (18 frames total)

---

## 3. Light Attack

**Frame Data:**
- Startup: 6f
- Active: 2f
- Recovery: 10f
- Total: 18f
- Damage: 1 state (Whole→Cut)
- On block: -2f

**Input:** J key (P1), Numpad1 (P2)

**System:** `handle_attack_input` - Check state, if Idle and J pressed → enter Attacking state

**Test:** Press J, see rectangle flash (hitbox visual), timing feels snappy

---

## 4. Heavy Attack

**Frame Data:**
- Startup: 14f
- Active: 4f
- Recovery: 18f
- Total: 36f
- Damage: 2 states (Whole→Wounded)
- On block: -8f
- Property: LightArmor (absorbs one Light during startup)

**Input:** K key (P1), Numpad2 (P2)

**Test:** Heavy feels committal, different from Light

---

## 5. Block & Guard Meter

**Components:**
```rust
Blocking { started_frame: u32 }
GuardMeter { current: f32, max: f32 }
Staggered { frames_remaining: u32 }
```

**Systems:**
- `handle_block_input` - Hold block button → enter Blocking state
- `fill_guard_on_block` - HitEvent where was_blocked=true → increase GuardMeter
- `drain_guard_passive` - Slowly reduce GuardMeter when not blocking
- `check_guard_break` - GuardMeter >= max → enter Staggered state

**Guard Fill Values:**
- Light blocked: +15% (0.15)
- Heavy blocked: +35% (0.35)
- Drain rate: -5% per second

**Test:** Block 3 Heavies → guard breaks → stagger

---

## 6. Parry System

**Frame Data:**
- Startup: 2f
- Active window: 6f
- Whiff recovery: 14f

**State:**
```rust
ParryAttempt { window_remaining: u32 }
```

**Systems:**
- `handle_parry_input` - Tap block (not hold) → enter ParryAttempt
- `check_parry_success` - If hit occurs during window → ParryEvent
- `handle_parry_success` - Time freeze (3f), visual flash, grant Initiative
- `handle_parry_whiff` - Window expires → 14f recovery

**Input:** Tap Block (hold=block, tap=parry)

**Feel:** Success = screen freeze, distinct sound, massive reward

**Test:** Parry a Light attack → see time freeze, get guaranteed counter

---

## 7. Grab

**Frame Data:**
- Startup: 10f
- Active: 2f
- Recovery: 20f (whiff)
- Range: Very short (must be touching)
- Property: Unblockable
- Effect: Causes Stagger

**Systems:**
- `handle_grab_input` - L key → enter Grabbing state
- Grab hitbox beats Block, loses to any active attack hitbox

**Input:** L key (P1), Numpad3 (P2)

**Test:** Grab beats block, but gets stuffed by Light attack

---

## 8. Evade

**Frame Data:**
- Startup: 3f
- I-frames: 4f
- Recovery: 8f
- Total: 15f
- Directional: 4 directions (forward, back, up, down)

**State:**
```rust
Evading {
    direction: Vec2,
    frames_remaining: u32,
    invincible: bool,
}
```

**Systems:**
- `handle_evade_input` - Direction + Evade button → enter Evading
- `apply_evade_movement` - Move quickly in direction
- `evade_invincibility` - Ignore hits during i-frame window

**Input:** Direction + Space (P1), Direction + Numpad0 (P2)

**Test:** Evade through attack, dodge grab

---

## 9. Events System

**Create events:**
```rust
HitEvent { attacker: Entity, defender: Entity, damage: u8, was_blocked: bool }
ParryEvent { defender: Entity, attacker: Entity }
GuardBreakEvent { entity: Entity }
GrabEvent { attacker: Entity, defender: Entity }
```

**Purpose:** Decouple systems - collision detection doesn't know about guard meter

---

## 10. Testing & Polish

**Integration tests:**
1. Light vs Block → guard fills
2. Heavy vs Block → guard fills faster
3. Grab vs Block → grab wins (Stagger)
4. Light vs Grab startup → Light stuffs grab
5. Parry vs Light → time freeze, counter opportunity
6. Parry vs Grab → fail (counter hit)
7. Evade vs Grab → evade wins
8. Block 3-4 hits → guard break → Stagger

**Feel tests:**
- Do attacks feel snappy? (6f startup is fast)
- Does parry feel rewarding? (time freeze critical)
- Does the triangle make sense? (clear win conditions)
- Are exchanges fast? (~1-2 seconds per exchange)

---

## Implementation Order

1. **Hitbox/Hurtbox collision** - 30min
2. **Attack state system** - 30min
3. **Light attack** - 20min
4. **Block system** - 20min
5. **Heavy attack** - 15min
6. **Guard meter** - 20min
7. **Parry** - 30min (needs polish)
8. **Grab** - 15min
9. **Evade** - 20min
10. **Integration testing** - 30min

**Total estimated time:** 3-4 hours

---

## What Gets Deferred to Phase 3

- Initiative system (frame advantage)
- Pressure states
- Chain attacks (Light→Light)
- Move cancels

Phase 2 focus: **The core triangle must work perfectly.**

---

## Definition of Done

Two players can:
- ✅ Throw Light attacks (6f startup, fast)
- ✅ Throw Heavy attacks (14f startup, committal)
- ✅ Block attacks (guard meter fills)
- ✅ Parry attacks (6f window, high reward)
- ✅ Grab opponents (beats block)
- ✅ Evade attacks (4f i-frames)
- ✅ Guard break happens after 3-4 blocked hits
- ✅ Triangle relationships are clear and consistent

The game should feel like **real exchanges** are happening.
