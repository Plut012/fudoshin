# Development Lessons

Quick reference for common patterns and pitfalls encountered during development.

---

## Quick Reference

| Category | Pattern | What to Do | Related Issue |
|----------|---------|------------|---------------|
| **ECS/Component Lifecycle** | Shared components between systems | Only ONE system should "own" a component at a time | [#1](#1-dash-cancel-freeze-ecs) |
| **ECS/State Transitions** | Changing entity state | Always remove old state components explicitly | [#1](#1-dash-cancel-freeze-ecs) |
| **Bevy/Commands** | Component removal timing | Commands are deferred until end of frame - watch for race conditions | [#1](#1-dash-cancel-freeze-ecs) |
| **Debugging** | Timer/counter behaving oddly | Add logging before/after mutations to detect double-ticking | [#1](#1-dash-cancel-freeze-ecs) |
| **Bevy UI/Query Filters** | Marker components in UI hierarchies | Marker must be on same entity as queried component, not parent | [#2](#2-victory-menu-not-updating-ui-queries) |
| **ECS/State Management** | Complete state resets | Prefer despawn/respawn over manual resets - use OnExit/OnEnter hooks | [#3](#3-rematch-dirty-state-state-management) |
| **Bevy/Change Detection** | Writing same value to component | Only write if value changed - avoid triggering Change cascade | [#4](#4-blocking-visual-flicker-change-detection) |

---

## Detailed Postmortems

### #1: Dash-Cancel Freeze `[ECS]` `[State-Management]`

**Symptom**: Character freezes when dash-canceling into attack (Shift+D → J)

**Root Cause**: Multiple systems ticking the same `StateTimer` component in one frame

**What Happened**:
1. Dash system added `EvadeData` + `StateTimer`
2. User pressed attack during dash
3. Attack system added new `StateTimer` but didn't remove `EvadeData`
4. Two systems now query and tick the same timer:
   - `progress_attack_phases` → `timer.tick()`
   - `progress_evade` → `timer.tick()`
5. Timer jumped: 0→2→4 instead of 0→1→2→3→4

**Solution**:
```rust
// In handle_attack_input - remove ALL conflicting components
commands.entity(entity).remove::<DashData>();
commands.entity(entity).remove::<EvadeData>();  // ← The fix
```

**Prevention Rules**:
- ✅ Remove old state components when transitioning
- ✅ Document which system "owns" each component
- ✅ Use distinct components for distinct states (don't share timers)
- ✅ Add debug logging for component lifecycle in complex systems

**Debug Technique**: Log values before/after mutations to catch double-processing
```rust
debug!("Timer TICK: {}/{} -> {}/{}", old, max, new, max);
```

---

### #2: Victory Menu Not Updating `[UI-Queries]` `[Bevy-Specific]`

**Symptom**: Victory screen menu navigation works (logs show selection changing) but UI text doesn't update visually

**Root Cause**: Marker component `VictoryUI` was on parent `NodeBundle`, but query looked for `Text` component on children

**What Happened**:
1. UI hierarchy:
   ```
   NodeBundle + VictoryUI (parent)
   ├─ TextBundle "Player 1 won" (child)
   ├─ TextBundle "> Rematch" (child)
   └─ TextBundle "  Reselect" (child)
   ```
2. Query: `Query<&mut Text, With<VictoryUI>>`
3. Parent has VictoryUI but no Text
4. Children have Text but no VictoryUI
5. Query finds **0 entities** with both components

**Solution**:
```rust
// Add marker to EACH text child, not just parent
parent.spawn((
    VictoryUI,  // ← Marker on child
    TextBundle::from_section("> Rematch", ...)
));
```

**Prevention Rules**:
- ✅ Marker components must be on the same entity as queried components
- ✅ Use `Query::iter().count()` logging to verify query matches expected entities
- ✅ For hierarchies, either: (1) add marker to children OR (2) use `Parent`/`Children` queries

**Debug Technique**: Log query result count to catch empty queries
```rust
if selection.is_changed() {
    info!("Text entities count: {}", query.iter().count());  // Was 0!
}
```

---

### #3: Rematch Dirty State `[State-Management]` `[Architecture]`

**Symptom**: Rematch doesn't start from clean state - players retain position/momentum/pressure from previous match

**Root Cause**: Tried to manually reset individual components instead of using proper state lifecycle

**What Happened**:
1. Initial rematch code:
   ```rust
   for (mut breath, mut health) in player_query.iter_mut() {
       breath.reset();      // Only resets 2 components
       health.restore_full();
   }
   next_state.set(GameState::InGame);
   ```
2. Missing resets: position, velocity, initiative, pressure, momentum, guard, chain state, etc.
3. Players spawned with leftover state from previous match

**Solution**: Use OnExit/OnEnter hooks for complete lifecycle
```rust
// In core_game.rs plugin setup
.add_systems(OnEnter(GameState::InGame), spawn_players)
.add_systems(OnExit(GameState::InGame), despawn_players)

// In menus.rs - just change state
VictoryOption::Rematch => {
    next_state.set(GameState::InGame);  // That's it!
}
```

**Prevention Rules**:
- ✅ For complete resets: despawn and respawn, don't manually reset
- ✅ Use OnExit to clean up state when leaving a game state
- ✅ Use OnEnter to initialize fresh state when entering a game state
- ✅ Let Bevy's state machine handle lifecycle - don't fight it

**Why This is Better**:
- All components reset to `::default()` automatically
- No risk of forgetting to reset a component
- Cleaner separation of concerns
- Easier to add new components without updating reset logic

---

### #4: Blocking Visual Flicker `[Change-Detection]` `[Bevy-Specific]`

**Symptom**: Character sprite flickers between silver (blocking) and red (base color) when holding block button

**Root Cause**: Unconditionally writing `velocity.0.x = 0.0` every frame triggered Bevy's change detection cascade, causing `visualize_blocking` to run continuously

**What Happened**:
1. `process_movement_input` ran every frame while blocking:
   ```rust
   if !matches!(state, CharacterState::Idle | CharacterState::Walking) {
       velocity.0.x = 0.0; // ← Writes EVERY frame, even if already 0
       continue;
   }
   ```
2. Writing to `velocity.0.x` (even with same value) marks `Velocity` as `Changed`
3. `update_movement_state` runs because it queries `Changed<Velocity>`
4. Even though it doesn't modify state (guard prevents it), having `&mut CharacterState` triggers change detection
5. `visualize_blocking` runs every frame because it queries `Changed<CharacterState>`
6. Sprite color updated every frame → visual flicker

**Solution**: Only write if value actually changes
```rust
if !matches!(state, CharacterState::Idle | CharacterState::Walking) {
    // Only set velocity to 0 if it's not already 0
    if velocity.0.x != 0.0 {
        velocity.0.x = 0.0;
    }
    continue;
}
```

**Prevention Rules**:
- ✅ Check current value before writing to avoid unnecessary change detection
- ✅ Bevy marks components as `Changed` when written to, even if value is identical
- ✅ `Changed<T>` cascades: A → B → C if systems depend on each other
- ✅ Use conditional writes: `if current != new { *component = new; }`

**Debug Technique**: Track change detection cascades with systematic logging
```rust
// Add logging to ALL systems with &mut access to suspect component
let old_value = format!("{:?}", *component);
// ... system logic ...
if format!("{:?}", *component) != old_value {
    debug!("[system_name] changed: {} -> {:?}", old_value, *component);
}
```

This reveals which system is actually modifying the component vs just querying it.

---

## How to Add New Lessons

1. Add a row to the Quick Reference table
2. Create a new numbered postmortem section with tags like `[ECS]` `[Bevy-Specific]`
3. Keep it concise - focus on what broke, why, and how to prevent it
4. Include code snippets only if they're essential
