# Fudoshin — Development Priorities

---

## Development Philosophy

Build the **feeling** first, then the features.

Fudoshin's identity lives in its moment-to-moment feel — the weight of movement, the tension of spacing, the satisfaction of a correct read. If this foundation is wrong, no amount of content will fix it.

Every development phase should produce something **playable and testable**. Two rectangles with correct movement feel is more valuable than a beautiful character with broken mechanics.

---

## Technology Choices

### Recommended: Godot Engine

**Why Godot:**
- Free and open source
- Excellent 2D support
- GDScript is approachable (Python-like)
- Active community
- Exports to all major platforms
- Good for fighting games (frame-perfect timing possible)

**Alternatives:**
- **Love2D** — Even simpler, Lua-based, very lightweight
- **Unity** — More complex but more resources available
- **Raylib** — C-based, maximum control, steeper learning curve

### Project Structure (Godot)

```
fudoshin/
├── project.godot
├── assets/
│   ├── sprites/
│   ├── audio/
│   └── fonts/
├── scenes/
│   ├── main.tscn
│   ├── character.tscn
│   ├── stage.tscn
│   └── ui/
├── scripts/
│   ├── character/
│   │   ├── state_machine.gd
│   │   ├── states/
│   │   └── hitbox.gd
│   ├── systems/
│   │   ├── initiative.gd
│   │   ├── guard.gd
│   │   └── input_buffer.gd
│   └── game/
│       ├── match.gd
│       └── round.gd
└── data/
    └── characters/
        └── ronin.json
```

---

## Phase 1: The Foundation

**Goal**: Two rectangles that feel *right* to move — fast and responsive.

### 1.1 Basic Movement (Week 1)

**Implement:**
- Walk (left/right) — fast, cross stage in ~2 seconds
- Idle state
- Basic gravity (if any platforming exists)
- Stage boundaries

**Test for:**
- Does walking feel responsive and snappy?
- Is the speed fast enough to feel active?
- Can you control space precisely?

**Target feel**: Quick but intentional. Like a fencer, not a tank.

### 1.2 Step and Backdash (Week 1-2)

**Implement:**
- Step (quick burst movement, 6f recovery)
- Backdash (quick retreat with 4f i-frames, 10f total)
- Recovery frames on both

**Test for:**
- Does Step feel like a commitment?
- Does Backdash feel like giving ground?
- Is there risk to using these?
- Are they fast enough to feel good?

### 1.3 Basic State Machine (Week 2)

**Implement:**
- Character state machine structure
- States: Idle, Walking, Stepping, Backdashing, Jumping
- State transitions
- Input buffering (basic)

**Architecture:**
```
Character
├── StateMachine
│   ├── IdleState
│   ├── WalkState
│   ├── StepState
│   ├── BackdashState
│   └── JumpState
└── InputBuffer
```

### 1.4 Two-Player Input (Week 2)

**Implement:**
- Player 1 and Player 2 input handling
- Keyboard controls for both
- (Optional) Controller support

**Default Controls:**
```
Player 1: WASD + JKL (movement + actions)
Player 2: Arrow keys + Numpad 123 (movement + actions)
```

### Phase 1 Milestone

Two rectangles that can move, step, backdash, and jump. Movement feels fast and responsive. Two players can control them simultaneously.

---

## Phase 2: The Core Triangle

**Goal**: Light, Heavy, Grab vs Block, Parry, Evade — the fundamental interaction, fast and tight.

### 2.1 Attack Framework (Week 3)

**Implement:**
- Hitbox/hurtbox system
- Attack states (startup, active, recovery)
- Basic Light attack (one move)
- Basic Heavy attack (one move)
- Hit detection and response

**Frame Data (tight and responsive):**
- Light: 6f startup, 2f active, 10f recovery
- Heavy: 14f startup, 4f active, 18f recovery

### 2.2 Block and Guard (Week 3-4)

**Implement:**
- Block state (hold to block, 1f startup)
- Blockstun (frames you're stuck blocking after hit)
- Guard meter (fills when blocking)
- Guard break → Stagger state

**Guard Values:**
- Light blocked: +15% Guard
- Heavy blocked: +35% Guard
- Guard depletes: 5% per second when not blocking

### 2.3 Parry (Week 4)

**Implement:**
- Parry input (tap block, 2f startup)
- Parry window (6 frames active)
- Parry success: brief freeze, Initiative gain
- Parry failure: 14f recovery, vulnerable

**The Feel:**
Parry success should feel *incredible*. Time freeze, distinct sound, visual flash. This is the "correct read" payoff.

### 2.4 Grab (Week 4)

**Implement:**
- Grab attack (unblockable, 10f startup)
- Grab range (short)
- Grab loses to attacks (stuffed during startup)
- Grab causes Stagger on hit

### 2.5 Evade (Week 4-5)

**Implement:**
- Evade input (direction + evade button, 3f startup)
- I-frames during evade (4 frames)
- Evade recovery (8f)
- Evade beats grabs, can dodge attacks

### Phase 2 Milestone

The fundamental triangle works:
- Attacks beat Grab
- Grab beats Block
- Block beats Attacks
- Parry beats Attacks (with skill)
- Evade beats Grabs and some Attacks

Two players can have meaningful exchanges. Reads matter. Pace feels fast.

---

## Phase 3: Initiative System

**Goal**: The player who's "winning" the exchange has tangible advantage.

### 3.1 Initiative Tracking (Week 5)

**Implement:**
- Initiative state (Player 1, Player 2, or Neutral)
- Gain Initiative: land hit, successful parry
- Lose Initiative: whiff, get blocked repeatedly, backdash

### 3.2 Initiative Effects (Week 5-6)

**Implement:**
- Frame advantage for Initiative holder (+2f on attacks)
- Visual indicator (subtle ink trail)
- Initiative decay (returns to Neutral over time without action)

### 3.3 Pressure State (Week 6)

**Implement:**
- After landing hit, enter Pressure
- During Pressure: enhanced Initiative, chain potential
- Pressure decay: faster after multiple hits
- Pressure break: after 3 consecutive hits

### Phase 3 Milestone

Players can feel when they're "winning" the neutral. Landing a hit means something beyond damage — it gives you momentum. The game has rhythm.

---

## Phase 4: Health States, Lethality, and Match Flow

**Goal**: Damage matters. The Breath system creates match structure. Momentum and Desperation add drama.

### 4.1 Health State System (Week 6-7)

**Implement:**
- Four states: Whole, Cut, Wounded, Broken
- State transitions based on damage
- Visual changes per state
- No health bar — body language only

**Transitions:**
- Light hit: one state down
- Heavy hit: two states down
- Decisive Blow: instant death (when available)

### 4.2 The Breath System (Week 7)

**Implement:**
- 3 Breaths (stocks) per player
- Death = lose a Breath, positions reset
- Health/Guard/Initiative reset between Breaths
- Brief pause between Breaths (~1.5 seconds)
- Breath counter UI (minimal)

**Flow:**
```
Kill → Freeze → Fall → "BREATH" → Reset → Pause → Continue
```

### 4.3 Momentum State (Week 7-8)

**Implement:**
- Killer gains Momentum at start of next Breath
- Momentum properties:
  - Start with Initiative
  - +10% Guard buffer
  - Visual indicator (subtle forward energy)
- Momentum ends after 3 seconds OR when hit

### 4.4 Desperation State (Week 8)

**Implement:**
- Triggers when down 0-2 in Breaths
- Desperation properties:
  - Final Stand activates immediately
  - +15% damage on all attacks
  - Visual indicator (stance shift, subtle red)
- Desperation ends when you take a Breath or lose

### 4.5 Decisive Blow Conditions (Week 8)

**Implement:**
- Track when Decisive Blow is available
- Conditions: target Wounded/Broken AND Staggered
- Heavy attack becomes Decisive Blow when conditions met

### 4.6 Decisive Blow Execution (Week 8-9)

**Implement:**
- Extended wind-up animation (20 frames)
- Distinctive visual and audio cue
- Final Parry window for defender (4 frames, tight)
- Kill confirmation: screen freeze, ink splatter, death

### 4.7 Final Stand (Week 9)

**Implement:**
- When entering Broken state, activate Final Stand
- Next successful parry is Perfect (maximum reward)
- Getting hit while Broken = instant death

### Phase 4 Milestone

Matches have structure and stakes. The Breath system creates natural pacing. Momentum rewards kills. Desperation enables comebacks. The Decisive Blow moment is tense and satisfying.

---

## Phase 5: First Character Complete

**Goal**: The Ronin is fully playable with all moves and personality.

### 5.1 Move Set (Week 8-9)

**Implement for Ronin:**
- Neutral Light, Forward Light, Back Light
- Neutral Heavy, Forward Heavy, Back Heavy
- Grab
- All frame data tuned

### 5.2 Drawn Blade Stance (Week 9)

**Implement:**
- Stance entry (hold Heavy)
- Stance hold (drains Guard)
- Stance release (fast slash)
- Stance cancel (return to neutral)

### 5.3 Character Polish (Week 9-10)

**Implement:**
- Placeholder animations (can be simple)
- Hit effects (ink splatter)
- Sound effects (movement, attacks, hits)
- Character-specific Decisive Blow visual

### Phase 5 Milestone

The Ronin is a complete character. You could ship a demo with just this character in mirror matches. The game loop is complete.

---

## Phase 6: Roster Expansion

**Goal**: Add remaining characters with distinct playstyles.

### 6.1 The Monk (Week 10-11)

- Counter-attacker archetype
- Open Palm stance (enhanced Parry window)
- Unique Decisive Blow (palm strike)

### 6.2 The Oni (Week 11-12)

- Heavy armor archetype
- Demon's Patience stance (armor on next move)
- Unique Decisive Blow (crushing overhead)

### 6.3 The Shade (Week 12-13)

- Mobile mix-up archetype
- Flicker stance (brief invisibility/reposition)
- Unique Decisive Blow (backstab)

### Phase 6 Milestone

Four distinct characters. Each rewards different playstyles. Matchups create variety.

---

## Phase 7: Audio-Visual Polish

**Goal**: The game looks and sounds like Fudoshin should.

### 7.1 Visual Identity (Week 13-14)

**Implement:**
- Ink-brush character sprites
- Stage backgrounds (minimal, evocative)
- Hit effects refinement
- UI (minimal — round indicator, that's nearly it)

### 7.2 Audio Design (Week 14-15)

**Implement:**
- Ambient soundscape (wind, breath)
- Movement sounds (footsteps, cloth)
- Attack sounds (one clean sound per type)
- Hit sounds (distinct for Light, Heavy, Decisive)
- Parry sound (the "correct read" chime)
- Silence before Decisive Blow

### 7.3 Screen Effects (Week 15)

**Implement:**
- Hit stop (brief pause on clean hits)
- Screen shake (subtle, on Heavy hits)
- Ink splatter on hits
- White flash on Decisive Blow
- Slowdown on round end

### Phase 7 Milestone

The game has its identity. Playing it *feels* like Fudoshin — minimal, tense, weighty.

---

## Phase 8: Game Modes and Polish

**Goal**: Complete package ready for players.

### 8.1 Versus Mode (Week 15-16)

**Implement:**
- Character select
- Stage select (if multiple stages)
- Best of 3 matches
- Rematch option

### 8.2 Training Mode (Week 16)

**Implement:**
- Frame data display
- Input display
- Dummy recording/playback
- Reset position button

### 8.3 Tutorial (Week 17)

**Implement:**
- Movement basics
- The offensive/defensive triangle
- Initiative explanation
- Decisive Blow tutorial

### Phase 8 Milestone

The game is complete. Two players can sit down, learn, and compete.

---

## Testing Priorities

Throughout development, test for:

### Feel Tests
- Does movement feel fast and responsive?
- Does landing a hit feel impactful?
- Does the Parry feel rewarding?
- Does the Decisive Blow feel earned?
- Do exchanges flow at the right pace?
- Does Momentum feel like an advantage without being oppressive?
- Does Desperation feel dangerous without being unfair?

### Balance Tests
- Is any option dominant?
- Does the triangle actually work?
- Are all characters viable?
- Is the game too fast or too slow?
- Are Breaths the right length (~20-40 seconds)?
- Is Momentum too strong or too weak?
- Does Desperation create comebacks without feeling random?

### Clarity Tests
- Can spectators follow what's happening?
- Can players tell who has Initiative?
- Is the health state clear?
- Do players understand why they lost?
- Is the Breath count visible and clear?
- Is Momentum/Desperation state readable?

---

## Risk Mitigation

### If Movement Doesn't Feel Right
Stop and fix it. Movement is the foundation. A game with perfect movement and placeholder everything else is better than the reverse. Make sure it feels fast and responsive.

### If the Triangle is Broken
Adjust frame data. Make options more/less punishable. Don't add new mechanics to fix — simplify.

### If Matches Are Too Fast
Add a 4th Breath. Or increase health slightly (Light does Whole→Whole with chip, not Whole→Cut). Don't slow down the action.

### If Matches Are Too Slow
This is less likely with the current design, but: reduce Guard meter capacity, make Guard break more common, increase Pressure duration.

### If Momentum is Too Strong
Reduce duration (3s → 2s). Remove Guard buffer. Make it purely Initiative.

### If Momentum is Too Weak
Increase duration (3s → 4s). Add damage bonus. Make it more visually distinct.

### If Desperation Feels Unfair (for either side)
Adjust damage bonus. Adjust Final Stand timing. The goal: player on match point should feel nervous, not helpless.

---

## Definition of Done

The game is "done" when:

1. **Core loop is satisfying** — Neutral, exchange, kill feels complete
2. **Breath system works** — Matches flow naturally with momentum and drama
3. **All characters playable** — Four distinct, balanced characters
4. **Two players can compete** — Local versus works flawlessly
5. **The feel is right** — Fast, responsive, and mentally engaging
6. **Players can learn** — Tutorial and training mode exist

Nice-to-have (post-launch):
- Online multiplayer (rollback netcode)
- Additional characters
- Additional stages
- Replay system
- Spectator mode

---

## Estimated Timeline

| Phase | Duration | Cumulative |
|-------|----------|------------|
| Phase 1: Foundation | 2 weeks | 2 weeks |
| Phase 2: Core Triangle | 3 weeks | 5 weeks |
| Phase 3: Initiative | 2 weeks | 7 weeks |
| Phase 4: Health/Breath/Momentum | 3 weeks | 10 weeks |
| Phase 5: First Character | 2 weeks | 12 weeks |
| Phase 6: Roster | 4 weeks | 16 weeks |
| Phase 7: Polish | 3 weeks | 19 weeks |
| Phase 8: Modes | 2 weeks | 21 weeks |

**Total: ~21 weeks (5 months) to complete game**

This assumes part-time development. Full-time could compress to ~3 months.

### Critical Path

The critical path is:
1. Movement feel (Week 1-2) — if this is wrong, everything is wrong
2. Core triangle (Week 3-5) — the game lives or dies here
3. Breath system (Week 7) — match flow depends on this
4. First playable character (Week 12) — proof the vision works

Everything else can be adjusted. These four milestones are non-negotiable.

---

## Claude Code Integration

Using Claude Code throughout:

### Phase 1-2
- Generate boilerplate state machine code
- Implement input buffering
- Create hitbox/hurtbox systems

### Phase 3-4
- Design Initiative tracking logic
- Implement health state transitions
- Create Decisive Blow conditions

### Phase 5-6
- Generate character data structures
- Implement stance systems
- Balance frame data

### Phase 7-8
- Create UI components
- Implement training mode features
- Debug and polish

Claude Code is a collaborator throughout — pair programming the game into existence.
