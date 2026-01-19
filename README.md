# Fudoshin - The Immovable Mind

A minimalist fighting game where victory is earned through superior perception, not execution.

## Current Status: Phase 3 - Initiative & Pressure ✅

**Phase 0 Complete:** ✅ Project Setup
**Phase 1 Complete:** ✅ Movement Foundation
**Phase 2 Complete:** ✅ Core Combat Triangle (Attacks, Block, Parry, Evade)
**Phase 3 Complete:** ✅ Initiative & Pressure (Frame Advantage, Chains, Counter Hits, Momentum)

**What's Working:**
- Full movement system with stage boundaries
- 3 attack types (Light, Heavy, Grab) with proper frame data
- Block system with guard meter and guard breaks
- 6-frame parry window that staggers attackers
- Evade with i-frames and directional movement
- Frame advantage tracking (+/- initiative)
- Pressure state with movement/attack bonuses
- Chain attacks (Light → Light cancels on hit)
- Counter hit system (+10f hitstun, gold flash)
- Momentum tracking with win streak bonuses

## Quick Start

```bash
# Run the game
cargo run

# Press F1 in-game to toggle inspector and see hitboxes
```

## Controls

### Player 1
- **WASD** - Movement
- **J** - Light Attack
- **K** - Heavy Attack
- **L** - Grab
- **I** - Block/Parry (tap for parry)
- **Shift + Direction** - Evade

### Player 2
- **Arrow Keys** - Movement
- **Numpad 1** - Light Attack
- **Numpad 2** - Heavy Attack
- **Numpad 3** - Grab
- **Numpad 0** - Block/Parry (tap for parry)
- **Right Shift + Direction** - Evade

### Debug
- **F1** - Toggle inspector (see hitboxes, components, gizmos)

## Visual Feedback

The game uses color-coded visual feedback to communicate game state:

- **Red flash** - Normal hit
- **Gold/yellow flash** - Counter hit (hit during startup)
- **White flash** - Successful parry
- **Cyan** - Parry active window
- **Green arrows** - Frame advantage (you can act first)
- **Red arrows** - Frame disadvantage (opponent acts first)
- **Yellow circle** - Chain window active (can cancel into next Light)
- **White glow** - Pressure state (brighter = higher pressure)
- **Colored rings** - Momentum level (green/cyan/gold for levels 3/4/5)
- **Semi-transparent** - Evade i-frames active
- **Red hitboxes** - Active attack hitboxes (press F1 to see)
- **Green hurtboxes** - Vulnerable areas (press F1 to see)

## Architecture

- **Data-Driven:** Balance values configured in code (assets system planned)
- **ECS Pattern:** Clean separation of components, systems, and resources
- **Frame-Perfect:** 60 FPS locked timing for fighting game precision
- **Event-Driven:** Decoupled communication through HitEvent, ParryEvent, etc.

See `docs/todo/PROGRESS.md` for detailed implementation status and `docs/todo/PHASE3_PLAN.md` for Phase 3 details.

## Next Steps: Phase 4

The next phase will add:
- **Health States** - Whole → Cut → Wounded → Broken progression
- **Breath System** - 3 stocks per match
- **Win Conditions** - Decisive Blow and round victory
- **Actual Damage** - Integrate momentum bonuses with health system

## Development

```bash
# Fast compilation checks
cargo check

# Run the game
cargo run

# Run with optimizations (faster, but slower compile)
cargo run --release
```

## Philosophy

> The immovable mind wins. Not because it's faster. Because it sees clearly.

**Core Design Pillars:**
- **Reads over Reactions** - Victory through prediction, not execution speed
- **Depth from Simplicity** - Few moves, infinite mind games
- **Every Frame Matters** - Frame advantage creates offensive/defensive rhythm
- **Consequences over Complexity** - Clear cause and effect, readable game state

Read more in `docs/gameplay_mechanics.md` and `docs/dev_priorities.md`

## Documentation

- `docs/todo/PROGRESS.md` - Complete implementation progress tracker
- `docs/todo/PHASE3_PLAN.md` - Phase 3 detailed plan and completion status
- `docs/gameplay_mechanics.md` - Core gameplay systems explained
- `docs/roster.md` - Character concepts and movesets
