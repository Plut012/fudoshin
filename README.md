# Fudoshin - The Immovable Mind

A minimalist fighting game where victory is earned through superior perception, not execution.

## Current Status: Phase 1 - Movement Foundation

**Phase 0 Complete:** ✅
- Project structure established
- Bevy engine configured
- Data-driven architecture in place
- Hot-reload ready

**Phase 1 In Progress:** 🚧
- Movement systems implementation

## Quick Start

```bash
# Run the game
cargo run

# Debug mode (press F1 in-game to toggle inspector)
cargo run
```

## Controls (Coming in Phase 1)

### Player 1
- WASD - Movement
- J - Light Attack
- K - Heavy Attack
- L - Grab

### Player 2
- Arrow Keys - Movement
- Numpad 1 - Light Attack
- Numpad 2 - Heavy Attack
- Numpad 3 - Grab

## Architecture

- **Data-Driven:** All balance values in `assets/data/*.ron`
- **ECS Pattern:** Clean separation of components, systems, and resources
- **Hot-Reload:** Edit data files, see changes instantly
- **60 FPS Locked:** Frame-perfect fighting game timing

See `docs/mvp_plan.md` for full implementation plan.

## Development

```bash
# Fast compilation checks
cargo check

# Run with optimizations
cargo run --release
```

## Philosophy

> The immovable mind wins. Not because it's faster. Because it sees clearly.

Read more: `overview.md`, `gameplay_mechanics.md`, `dev_priorities.md`
