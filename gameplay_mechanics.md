# Fudoshin — Gameplay Mechanics

---

## Pace Philosophy

The **weight** of Fudoshin comes from *consequences*, not *animation speed*.

Think about kendo or fencing: actual movements are explosive, fast, decisive. The tension isn't slow-motion — it's that any exchange could end it. The "stillness" is internal. Externally, the game is **crisp, responsive, and alive**.

**What is fast:**
- Movement (responsive, snappy)
- Attack startup and recovery (tight frame data)
- The rhythm of exchanges (rapid decision points)
- Match pace overall (20-40 seconds per Breath)

**What feels weighty:**
- The *commitment* of actions (you chose this, you live with it)
- The *consequences* of being read (one mistake can cost you)
- The *impact* of hits (visual/audio punch, hitstop)
- The *moment* of the Decisive Blow (brief pause for drama, then it lands)

The game feels like a conversation at the speed of thought — not a slow meditation.

---

## The Shape of a Match

A match in Fudoshin has a rhythm — like breathing, like a rapid exchange of cuts.

### The Micro-Rhythm (Within Exchanges)

```
Engage → Read → Resolve → Reset
(0.5s)   (0.3s)  (0.2s)    (0.5s)
```

Exchanges are ~1-1.5 seconds of rapid decision-making. Then brief neutral. Then another exchange.

### The Macro-Rhythm (The Breath Cycle)

```
Neutral → Pressure → Advantage → Kill → Reset
```

**Neutral**: Both players active — micro-spacing, testing range, feinting. Not standing still. Information gathering at speed.

**Pressure**: Someone has Initiative. They're pressing. The defender is looking for the gap.

**Advantage**: Accumulated reads have created a kill opportunity. One player is Wounded or their Guard is cracking.

**Kill**: The Decisive Blow window. Tension peaks. Final Parry or death.

**Reset**: A Breath is taken. Brief pause. Momentum shifts. Next Breath begins.

A full match has 3-6 of these cycles, lasting ~2-3 minutes total.

---

## Health: States of Being

Rather than a traditional health bar, players exist in **states** that reflect the reality of a duel:

| State | Meaning | Mechanical Effect |
|-------|---------|-------------------|
| **Whole** | Untouched, full capability | All options available |
| **Cut** | You've been hit, shaken | Slightly reduced guard recovery |
| **Wounded** | Serious damage sustained | Decisive Blows become available against you |
| **Broken** | One strike from death | Final Stand mechanic activates |

### State Transitions

The transition between states isn't just numbers — it's *felt*. Your character's stance shifts subtly. The soundscape changes. You know, viscerally, that you're losing.

**Typical damage:**
- Light attack: Whole → Cut, or Cut → Wounded
- Heavy attack: Whole → Wounded, or Wounded → Broken
- Decisive Blow: Any state → Defeated (if conditions met)

A clean fight might be: Light → Heavy → Decisive. Three hits. Three reads.

### Visual Communication

- **Whole**: Full posture, centered stance
- **Cut**: Slight favoring, minor tells in animation
- **Wounded**: Visible strain, stance adjusts
- **Broken**: Near-collapse posture, breathing visible

No health bars. The body tells the story.

---

## The Breath System (Match Structure)

Each player has **3 Breaths** (stocks). Lose all Breaths, lose the match.

### Breath Flow

```
Match Start
├── Breath 1: Neutral start, both players Whole
│   └── Kill → Killer gains Momentum, positions reset
├── Breath 2: Killer has Momentum advantage
│   └── Kill → Check for Desperation
├── Breath 3+: Possible Desperation if down 2-0
│   └── Exchange continues until elimination
└── Match ends when one player has 0 Breaths
```

### Between Breaths

When a Breath is taken (kill occurs):
- Positions reset (both return to starting positions)
- Health states reset (both return to Whole)
- Guard meters reset
- Initiative resets to Neutral
- Brief pause (~1.5 seconds) — the dramatic breath
- Killer gains **Momentum** state

### Match Length

- Each Breath: ~20-40 seconds
- Full match: ~2-3 minutes
- No round-win screens — flow is continuous

---

## Momentum State

The player who takes a Breath gains **Momentum** entering the next exchange.

### Momentum Properties

- Start with Initiative (immediate frame advantage)
- +10% Guard buffer (slightly more defensive cushion)
- Subtle visual indicator (faint forward-energy effect)
- Lasts for **3 seconds** or until you get hit

### Momentum Design Intent

This rewards the kill beyond just the stock damage. You enter the next exchange with an edge — but it's slight, not overwhelming. A skilled defender can weather the Momentum and reset to true neutral.

### Losing Momentum

Momentum ends when:
- 3 seconds pass
- You take any hit (blocked or not)
- You whiff an attack (you squandered the advantage)

---

## Desperation State

When a player loses 2 Breaths while their opponent has lost 0 (down 0-2), they enter **Desperation**.

### Desperation Properties

- **Final Stand activates immediately** — your next Parry is Perfect
- **+15% damage** on all attacks
- Visual indicator (character stance shifts, subtle red tint)
- Lasts until you take a Breath or lose

### Desperation Design Intent

The player on match point isn't safe. They're fighting a cornered animal. This creates:
- Comeback potential (one correct read can shift momentum)
- Tension for the leader (can't relax, must respect the threat)
- Dramatic match endings

### Desperation Risk

You're still one hit from death (Broken state). The damage boost and Perfect Parry make you dangerous, but you have no margin for error.

---

## Initiative (先手 / Sente)

This is the heartbeat of Fudoshin.

**Initiative** is who is *leading* the exchange. It's abstract but felt. The player with Initiative:
- Has subtle frame advantage (their moves come out slightly faster in trades)
- Can apply **Pressure Sequences**
- Forces the opponent to *respond* rather than act

### Visual Tell

The player with Initiative has a faint ink-wash effect trailing their movement. Subtle, but unmistakable once you know to look.

### Gaining Initiative

- Landing a clean hit
- Making a correct defensive read (parry, whiff punish)
- **Tenuki** — deliberately ignoring a small threat to make a larger play
- Controlling ma-ai (being at your preferred distance)

### Losing Initiative

- Whiffing an attack (you committed to nothing)
- Being read (your pattern was predicted)
- Retreating passively (giving ground without purpose)
- Blocking too long (you're just surviving)

### Frame Advantage

When you have Initiative:
- Your Light attacks are ~2 frames faster
- You recover ~3 frames faster from actions
- Ties in simultaneous attacks go to you

This is subtle but decisive at high level play.

---

## The Tenuki Mechanic

**Tenuki** (手抜き): In Go, playing elsewhere — ignoring your opponent's move to make a bigger play.

In Fudoshin, you can choose to **accept a light hit** in order to land a heavy attack. You're saying: "Your strike is not worth my attention."

### How It Works

- If you begin a Heavy attack and they hit you with a Light, you can choose to **absorb** it
- You take the damage (Whole → Cut)
- But your Heavy attack continues and will likely land
- Heavy beats Light in damage, so you win the exchange *if you read correctly*

### The Risk

- If they threw a Heavy too, you just ate damage for nothing
- If they grabbed, you get thrown and lose Initiative
- If they baited and blocked, you whiffed a Heavy

This is high-level play. It's the statement: "I know what you're going to do. It doesn't matter."

---

## Offensive Options

Attacks are simple but meaningful. Frame data is tight — exchanges happen fast.

| Option | Startup | Active | Recovery | On Block |
|--------|---------|--------|----------|----------|
| **Light Attack** | 6f | 2f | 10f | -2f |
| **Heavy Attack** | 14f | 4f | 18f | -8f |
| **Grab** | 10f | 2f | 20f | — |

### Directional Variants

Like Smash Bros, direction + button gives different moves:
- Neutral, Forward, Back variants for Light and Heavy
- Each character has ~6-8 total moves
- Mastery is about *when*, not *what*

### Attack Properties

**Light Attacks:**
- Fast startup (6f), low commitment
- Can chain: Light → Light (2 hit max, then forced gap)
- Low damage but builds pressure
- Safe on block (-2f, slight disadvantage)
- The jab, the poke, the conversation starter

**Heavy Attacks:**
- Moderate startup (14f), high commitment
- Single hit, no chaining
- High damage, state-changing
- Unsafe on block (-8f, punishable by Light)
- Has Light Armor (can absorb one Light and continue — the Tenuki)

**Grabs:**
- Unblockable
- Short range (must be close)
- Loses to any attack (stuffed during startup, 10f is reactable)
- Causes Stagger on hit
- Repositions opponent (can throw toward corner)

---

## Defensive Options

Defense in Fudoshin is not passive. It's where reads happen.

| Option | Startup | Window | Recovery | Beats | Loses To |
|--------|---------|--------|----------|-------|----------|
| **Block** | 1f | Hold | 3f | Attacks | Grab |
| **Parry** | 2f | 6f | 14f (whiff) | Attacks | Grab, Feints |
| **Evade** | 3f | 4f i-frames | 8f | Grabs, Attacks | Directional read |

### Block

- Hold to block attacks
- Costs Guard meter on each blocked hit
- Safe but passive — you're not winning by blocking
- **Guard Break**: If Guard meter fills, you enter Stagger state
- Recovery is fast (3f) — you can act quickly after blockstun

**Guard Meter:**
- Fills when blocking attacks
- Slowly depletes when not blocking
- Fills faster from Heavy attacks
- When full: Guard Breaks, you're Staggered

### Parry

- Active input with tight window (6 frames)
- High risk, high reward
- On success:
  - Brief time freeze (the "perfect read" visual)
  - Gain Initiative immediately
  - Guaranteed Counter opportunity (free Light attack)
  - Guard meter partially restored

**Parry Failure:**
- If you parry and they grab: Counter Hit (extra damage/stagger)
- If you parry early/late: 14f recovery, very punishable
- The risk is real — don't parry predictably

### Evade

- Directional movement with brief invincibility (4 frames i-frames)
- Can evade in 4 directions (forward, back, up, down)
- Beats grabs entirely
- Can dodge attacks if direction is correct
- Fast recovery (8f) — can punish or escape

**Evade Risk:**
- If they read your direction, they recover and punish
- Forward evade into their attack = death
- Small Initiative cost to use defensively (you gave ground mentally)

---

## The Guard System

### Guard Meter

- Visual: Subtle ink circle around character, fills as guard is damaged
- Depletes slowly when not blocking
- Fills from blocked attacks:
  - Light blocked: +15% Guard
  - Heavy blocked: +35% Guard
  - Grab: Breaks through block entirely

### Guard Break (Stagger)

When Guard meter fills:
- Character enters **Stagger** state
- Stagger lasts ~30 frames
- During Stagger:
  - Cannot block, attack, or move
  - Vulnerable to Decisive Blow (if Wounded/Broken)
  - Opponent gets guaranteed mix-up

### Guard Recovery

- Guard meter depletes ~5% per second when not blocking
- Parrying restores ~25% Guard
- Getting hit (not blocking) resets Guard to 0 (you weren't blocking anyway)

---

## Movement and Space (Ma-ai)

Movement is **responsive and snappy**. The commitment comes from *decisions*, not slow animations.

| Option | Speed | Frames | Properties |
|--------|-------|--------|------------|
| **Walk** | Fast | — | Constant, fully controllable, no commitment |
| **Step** | Quick | 6f recovery | Short burst movement, small commitment |
| **Jump** | Medium | 8f landing | Committal arc, limited aerial options |
| **Backdash** | Quick | 10f total | Fast retreat, costs Initiative, i-frames |

### Walk

- Base movement speed
- Cross the stage in ~2 seconds
- Can change direction instantly
- No recovery, always actionable
- This is your primary movement — fast and fluid

### Step

- Quick burst in input direction
- Covers ~1.5x walk distance instantly
- Small recovery (6 frames) where you can't act
- Good for closing distance or micro-adjustments
- The "I'm in" commitment

### Jump

- Committal — arc is readable
- Limited aerial options (most characters have one aerial attack)
- Landing recovery (8 frames)
- Useful for escaping corners or calling out ground threats
- Anti-aired easily if predictable

### Backdash

- Quick backwards movement with brief invincibility (4f i-frames)
- Safe escape option
- **Costs Initiative** — you gave ground
- Can't backdash repeatedly (cooldown ~15 frames)
- Total duration: 10 frames

### Spatial Rules

**No passing through opponents**: Once you're close, you must deal with the situation. Space is commitment.

**Stage size**: Compact. Takes ~2 seconds to walk corner to corner. Corners matter.

**Corner pressure**: Being cornered limits Evade options to forward/up only. Dangerous. This is where games are won.

### Neutral Movement

Neutral isn't "standing still waiting." It's:
- Micro-spacing (walking in and out of threat range)
- Stance feints (entering and exiting)
- Step baits (stepping forward to draw a reaction)
- Information gathering at speed

The *mind* is calm. The *hands* are active.

---

## The Stance System (Aji)

Each character has **one or two Stances** — held positions that create threat without committing.

### Stance Properties

When you enter a Stance:
- You're not attacking (yet)
- Certain powerful options become available
- The opponent must respect the *potential*
- Holding Stance slowly drains Guard meter (~3% per second)

### The Threat of Stances

This is **aji** — unrealized potential that influences the game. The move you *might* throw is controlling space.

From Stance you can:
- **Release**: Execute the Stance's unique attack
- **Cancel**: Return to neutral (costs small Initiative)
- **Hold**: Maintain threat (costs Guard over time)

### Example: Drawn Blade (Ronin)

- Enter by holding Heavy
- While held: threatens fast, powerful slash
- Release: Fast slash with excellent range
- The opponent sees: "If I approach carelessly, I die"
- But: holding drains Guard. Can't hold forever.

---

## The Exchange System

There are no traditional combos. The "combo" is a **sequence of reads**.

### Pressure State

When you land a hit, you enter **Pressure State**:
- Brief window where you have Initiative
- Opponent must respond to your next action
- Your attacks come out faster
- Lasts ~20 frames or until you whiff/get blocked

### The Exchange Flow

```
You act → They respond → You read their response → They respond → ...
```

**Example exchange:**

1. You land Light (read their approach) → Enter Pressure
2. You throw another Light → They block → Pressure ends, you keep Initiative
3. You throw Grab (reading their block habit) → They Evade → They gain Initiative
4. They Counter with Heavy → You Parry → You regain Initiative + Counter
5. You land Heavy Counter → They're now Wounded and Staggered
6. Decisive Blow available...

### Chain Limits

- After 2 consecutive hits, Pressure decays faster
- After 3 consecutive hits, automatic reset to neutral
- This prevents touch-of-death

You must win multiple exchanges, not one long sequence.

### What "Combo" Means Here

A 4-hit kill isn't memorized inputs.

It's: *I read jump, I read panic grab, I read their escape, I read their desperation reversal.*

Each hit is a decision. Each hit is earned.

---

## The Decisive Blow (一撃 / Ichigeki)

The killing stroke. The spike. The checkmate.

### Conditions

Decisive Blow becomes available when:
- Opponent is **Wounded** or **Broken**, AND
- Opponent is **Staggered** (Guard broken or hit during recovery)

### Execution

When conditions are met, your Heavy attack input becomes the Decisive Blow.

**Properties:**
- Longer wind-up (~24 frames, heavily telegraphed)
- Distinctive visual (character shifts into killing posture)
- Distinctive audio (the inhale before the cut, or silence)
- If it lands: **Instant kill**, regardless of current state

### The Final Parry

The opponent gets one last chance.

During the Decisive Blow wind-up, they can attempt a Parry with a **tighter window** (~4 frames, down from 6-8).

**If they parry:**
- They survive
- Situation resets to neutral
- They remain Wounded/Broken (health doesn't restore)
- They've earned a second chance through skill

**If they fail:**
- Death
- Screen holds for a moment
- Ink splatter, silence, then the cut sound
- The match point

### Decisive Blow Timing

You don't have to use Decisive Blow immediately. You can:
- Go for it right away (they might still have reactions)
- Wait a beat (let them sweat, maybe they parry early)
- Fake the timing (start wind-up, cancel, re-enter)

Mind games exist even in the kill.

---

## Final Stand

When you enter **Broken** state (one hit from death), a new mechanic activates.

### Properties

**Final Stand**: Your next successful Parry is automatically **Perfect**:
- Maximum reward
- Guaranteed Heavy Counter
- Steal Initiative completely
- Guard meter fully restored

### The Design Intent

This represents the clarity that comes at the edge of death. Fudoshin fully manifested. Nothing to lose, so you see perfectly.

### The Risk

If you fail the Parry or Block incorrectly while Broken:
- Any hit kills you immediately
- No chip damage forgiveness
- The stakes are absolute

### The Tension

The Broken player is *dangerous*:
- One correct read and they're back in the game
- But one mistake and it's over
- The opponent must respect this

---

## Stagger State

Stagger is the vulnerable state that enables Decisive Blows.

### How You Get Staggered

- Guard Break (Guard meter filled from blocking)
- Hit during recovery frames (whiff punish)
- Grabbed

### Stagger Properties

- Duration: ~30 frames
- Cannot: Block, Attack, Move, Evade
- Can: Attempt Final Parry (if Decisive Blow incoming)

### Stagger Severity

Not all Staggers are equal:
- **Light Stagger** (from Light whiff punish): ~20 frames
- **Medium Stagger** (from Grab): ~30 frames
- **Heavy Stagger** (from Guard Break): ~40 frames

Only Medium and Heavy Stagger enable Decisive Blow.

---

## Stage Design

### Dimensions

- Width: ~8 character widths
- Small enough that corners come into play
- Large enough for spacing game to exist

### Corner Mechanics

Being cornered is dangerous:
- Cannot backdash (wall)
- Evade options limited (only forward or up)
- Opponent can apply maximum pressure
- This is the "Zugzwang zone"

### Visual Design

- Minimal background elements
- Clear stage boundaries
- No visual distractions
- Negative space emphasis

---

## Match Structure

### Pre-Match

- Character select (if versus mode)
- Players appear on opposite sides of stage
- Brief stillness (~1 second)
- "READY" indicator
- Match begins

### During Match

The match flows continuously through Breaths:

**Active Play:**
- Both players engage in neutral, exchanges, pressure
- No timer (matches are naturally fast)
- Health states tracked, Initiative flows

**When a Kill Occurs:**
1. Decisive Blow or lethal damage lands
2. Brief freeze (~0.5 seconds) — the impact moment
3. Loser's character falls
4. "BREATH" indicator (shows remaining Breaths)
5. Positions reset
6. Killer gains Momentum
7. ~1.5 second pause
8. Next Breath begins immediately

**Desperation Trigger:**
- If a player is down 0-2 in Breaths
- Desperation state activates
- Visual/audio indicator
- Match continues with heightened stakes

### Match End

- Final Breath taken
- Extended freeze (~1 second)
- Winner's character performs victory gesture
- "VICTORY" indicator
- Option to rematch or return to menu

### Timing Summary

| Phase | Duration |
|-------|----------|
| Pre-match | ~2 seconds |
| Each Breath | ~20-40 seconds |
| Between Breaths | ~1.5 seconds |
| Full match (3-5 Breaths) | ~2-3 minutes |
| Post-match | ~3 seconds |

---

## Frame Data Philosophy

### Complete Frame Data Reference

| Action | Startup | Active | Recovery | On Block | Notes |
|--------|---------|--------|----------|----------|-------|
| Light Attack | 6f | 2f | 10f | -2f | Chains into itself |
| Heavy Attack | 14f | 4f | 18f | -8f | Light Armor on startup |
| Grab | 10f | 2f | 20f | — | Unblockable |
| Block | 1f | Hold | 3f | — | Costs Guard |
| Parry | 2f | 6f | 14f | — | Whiff recovery |
| Evade | 3f | 4f | 8f | — | I-frames during active |
| Step | 2f | — | 6f | — | Movement option |
| Backdash | 2f | 4f | 4f | — | I-frames during active |
| Jump | 4f | — | 8f | — | Landing recovery |

### Design Principles

- **Reactable but punishable**: Heavies (14f) can be reacted to with a read, but hesitation = you get hit
- **Tight windows reward skill**: Parry window (6f) is learnable but demands precision
- **Safe isn't free**: Light on block (-2f) is "safe" but you don't get to act first
- **Whiffs are death**: Missing attacks leaves you wide open
- **Nothing is unreactable**: A focused player can respond to anything with the right read
- **Everything is punishable**: If you're predictable, you die

### The Pace of Exchanges

A typical exchange:
```
Player 1 Light (6f) → Hit → Pressure (20f window)
├── P1 follows with Light → P2 blocks → Small advantage to P1
├── P1 follows with Grab → P2 evades → Neutral reset
└── P1 follows with Heavy → P2 parries → P2 has Initiative
```

Each decision point is ~10-15 frames. Exchanges last 1-2 seconds. Then reset. Then go again.

---

## Summary

Fudoshin's mechanics create a game where:

1. **Every hit is a decision** — not a memorized input
2. **Defense is active** — you're reading, not waiting
3. **Position matters** — space, Guard, Initiative are all resources
4. **The kill is earned** — through accumulated advantage, not lucky hits
5. **Tension is constant** — low health means every exchange could be your last
6. **Pace is fast** — exchanges happen in seconds, matches in minutes
7. **Momentum shifts** — Breaths create natural dramatic arcs
8. **Comebacks are possible** — Desperation keeps matches tense to the end

The game is fast. Your mind must be still.
