# Menu & End Screen Design

Simple, minimalist UI system for Fudoshin that respects the game's aesthetic philosophy.

---

## Design Philosophy

**Core Principles:**
- **Minimal visual noise** - Empty space is intentional
- **Sumi-e aesthetic** - Brushstroke simplicity, ink wash style
- **Clear legibility** - Information is immediately readable
- **No distraction** - UI serves the game, never competes with it

**Inspiration:** Traditional Japanese calligraphy, Go board simplicity, Kurosawa film titles

---

## Current State

**What exists:**
- ✅ All UI uses `gizmos` (circles, rectangles) - no text rendering yet
- ✅ Breath indicators (circles)
- ✅ Health bars (colored rectangles)
- ✅ Round timer (bar + dots)
- ✅ Visual state indicators (pulses, flashes)

**What's missing:**
- ❌ Text rendering (no font system)
- ❌ Menu navigation
- ❌ Game state machine (MainMenu / InGame / Victory)
- ❌ Input handling for menus

---

## Proposed Solution: Hybrid Approach

### Option A: Pure Gizmos (Recommended for MVP)

**Pros:**
- Consistent with current implementation
- Zero dependencies
- Extremely minimalist
- Fast to implement

**Cons:**
- No text = must use symbols/shapes for everything
- Less accessible for new players
- Creative challenge to communicate clearly

**How it works:**
- Menus are geometric compositions
- Player navigates with directional inputs
- Selected option pulses/glows
- Confirmation uses attack buttons

### Option B: Add bevy_ui for Text Only

**Pros:**
- Text is clearer than symbols
- Standard UI patterns
- Better accessibility

**Cons:**
- Introduces new dependency
- Font selection matters (must fit aesthetic)
- More complex styling

**How it works:**
- Minimal text overlays (white on black)
- Gizmos for decorative elements
- Simple fade transitions

---

## Recommended: Pure Gizmos (Option A)

Keep it pure geometric until post-MVP. Add text in Phase 5 when doing art/polish pass.

---

## Screen Designs

### 1. Main Menu (Phase 5)

```
         ○                          <-- Pulsing circle (title symbol)
       ○   ○
         ○

    ━━━━━━━━━                       <-- Start Match (selected = bright)

    ━━━  ━━━                        <-- Controls (2 bars = 2 players)

    ╳                                <-- Quit (X symbol)


    [Press Light to Select]
    [Press Heavy to Confirm]
```

**Visual Language:**
- Single large circle = Start
- Two small bars = Controls (implies 2 players)
- X = Quit
- Selected item: bright + pulsing
- Unselected: dim gray

**Navigation:**
- Up/Down = cycle options
- Light attack = select/confirm
- Back button = return (where applicable)

**Implementation:**
```rust
enum MenuState {
    Start,
    Controls,
    Quit,
}

fn render_main_menu(gizmos, menu_state) {
    // Draw symbols for each option
    // Pulse selected option
    // Handle input in separate system
}
```

---

### 2. Victory Screen (Phase 4)

```

         ✓                          <-- Winner symbol (checkmark)
         ↑                          <-- Points up to winner's side


    ●  ●  ○                         <-- P1 Breaths (2/3)

    ○  ○  ○                         <-- P2 Breaths (0/3) - LOSER


    ━━━━━━━━━                       <-- Rematch (selected)

    ←                               <-- Back to menu


    [Rematch in 5s...]              <-- Auto-rematch countdown
```

**Visual Language:**
- Large checkmark above winner
- Arrow points to winner's breath display
- Winner's breaths filled, loser's empty
- Rematch option (horizontal bar)
- Back arrow (return to menu)

**Features:**
- Auto-rematch in 5 seconds (can cancel)
- Shows final breath count
- Simple, clear winner indication

**Implementation:**
```rust
#[derive(Resource)]
struct VictoryScreenState {
    winner: Player,
    p1_breaths: u8,
    p2_breaths: u8,
    rematch_countdown: f32,  // 5.0 -> 0.0
    selected_option: VictoryMenuOption,
}

enum VictoryMenuOption {
    Rematch,
    MainMenu,
}
```

---

### 3. Controls Screen (Phase 5)

```
    P1              P2

    ↑               ▲              <-- Movement (WASD / Arrows)
   ← →             ◄ ►
    ↓               ▼

    ○  ○  ○        ①  ②  ③        <-- Attacks (J/K/L vs Num1/2/3)

    ▭              ⓪              <-- Block (I vs Num0)

    ⚡ + →         ⚡ + ▲          <-- Evade (Shift+Dir)


    [Press Any Button to Return]
```

**Visual Language:**
- Arrows for directional input
- Circles for attack buttons
- Rectangle for block
- Lightning + arrow for evade
- Side-by-side P1/P2 layout mirrors game

---

## Game State Machine

```rust
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    MainMenu,
    InGame,
    Victory,
    Controls,
}
```

**State Transitions:**
```
MainMenu --> InGame (Start selected)
MainMenu --> Controls (Controls selected)
Controls --> MainMenu (Back)
InGame --> Victory (Match ends)
Victory --> InGame (Rematch)
Victory --> MainMenu (Back)
```

**System Organization:**
```rust
// In core_game.rs
app
    .init_state::<GameState>()
    // Menu systems
    .add_systems(Update, (
        main_menu_input,
        main_menu_render,
    ).run_if(in_state(GameState::MainMenu)))
    // Game systems
    .add_systems(Update, (
        // All existing game systems
    ).run_if(in_state(GameState::InGame)))
    // Victory systems
    .add_systems(Update, (
        victory_screen_input,
        victory_screen_render,
        auto_rematch_countdown,
    ).run_if(in_state(GameState::Victory)))
```

---

## Implementation Priority

### Phase 4 (Current)
**Goal:** Finish match system, add victory screen

1. ✅ Implement health/breath systems (already planned)
2. ✅ Detect match end condition
3. **Add victory screen:**
   - Basic gizmo rendering (checkmark, breath display)
   - Rematch / back to game options
   - Auto-rematch countdown
   - Transition back to match start

**Why:** Victory screen is essential for Phase 4's "complete match" goal

### Phase 5 (Post-MVP)
**Goal:** Full menu system

1. Add `GameState` enum and state machine
2. Create main menu:
   - Gizmo-based options
   - Input handling
   - Transitions to game
3. Create controls screen
4. Add pause menu (optional)
5. Polish transitions (fades, animations)

**Why:** Can playtest Phase 4 by just restarting the executable

---

## Technical Implementation Notes

### Gizmo Symbol Library

Create reusable functions for common symbols:

```rust
// In systems/ui_symbols.rs

/// Draw a checkmark symbol
pub fn draw_checkmark(gizmos: &mut Gizmos, pos: Vec2, size: f32, color: Color) {
    // Two lines forming a checkmark
    let p1 = pos + Vec2::new(-size * 0.3, 0.0);
    let p2 = pos + Vec2::new(0.0, -size * 0.5);
    let p3 = pos + Vec2::new(size * 0.5, size * 0.5);

    // Draw as connected line segments
    gizmos.line_2d(p1, p2, color);
    gizmos.line_2d(p2, p3, color);
}

/// Draw an X symbol
pub fn draw_x_symbol(gizmos: &mut Gizmos, pos: Vec2, size: f32, color: Color) {
    gizmos.line_2d(
        pos + Vec2::new(-size, -size),
        pos + Vec2::new(size, size),
        color
    );
    gizmos.line_2d(
        pos + Vec2::new(-size, size),
        pos + Vec2::new(size, -size),
        color
    );
}

/// Draw arrow pointing in direction
pub fn draw_arrow(gizmos: &mut Gizmos, pos: Vec2, dir: Vec2, size: f32, color: Color) {
    let tip = pos + dir * size;
    let base = pos - dir * size * 0.3;

    // Arrow shaft
    gizmos.line_2d(base, tip, color);

    // Arrowhead
    let perpendicular = Vec2::new(-dir.y, dir.x) * size * 0.3;
    gizmos.line_2d(tip, tip - dir * size * 0.4 + perpendicular, color);
    gizmos.line_2d(tip, tip - dir * size * 0.4 - perpendicular, color);
}

/// Draw pulsing selection indicator
pub fn draw_selection_pulse(gizmos: &mut Gizmos, pos: Vec2, time: f32) {
    let pulse = (time * 3.0).sin() * 0.5 + 0.5;
    let alpha = 0.3 + pulse * 0.4;

    gizmos.circle_2d(
        pos,
        30.0 + pulse * 10.0,
        Color::srgba(1.0, 1.0, 1.0, alpha),
    );
}
```

### Menu Input Handling

```rust
fn menu_navigation_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<MenuState>,
) {
    // Unified input for both players in menus (use P1 controls)
    if keyboard.just_pressed(KeyCode::KeyW) || keyboard.just_pressed(KeyCode::ArrowUp) {
        menu_state.previous_option();
    }
    if keyboard.just_pressed(KeyCode::KeyS) || keyboard.just_pressed(KeyCode::ArrowDown) {
        menu_state.next_option();
    }
    if keyboard.just_pressed(KeyCode::KeyJ) { // Light attack = confirm
        menu_state.confirm_selection();
    }
}
```

### Smooth Transitions

```rust
#[derive(Resource)]
struct ScreenTransition {
    fade: f32,  // 0.0 = invisible, 1.0 = fully visible
    transitioning: bool,
    target_state: Option<GameState>,
}

fn update_transitions(
    mut transition: ResMut<ScreenTransition>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    if transition.transitioning {
        transition.fade -= time.delta_seconds() * 2.0;  // Fade out

        if transition.fade <= 0.0 {
            // Switch state
            if let Some(new_state) = transition.target_state {
                next_state.set(new_state);
            }
            // Fade back in
            transition.fade = 1.0;
            transition.transitioning = false;
        }
    }
}

fn apply_fade_overlay(mut gizmos: Gizmos, transition: Res<ScreenTransition>) {
    if transition.fade < 1.0 {
        // Draw dark overlay
        gizmos.rect_2d(
            Vec2::ZERO,
            0.0,
            Vec2::new(2000.0, 2000.0),
            Color::srgba(0.0, 0.0, 0.0, 1.0 - transition.fade),
        );
    }
}
```

---

## Alternative: Minimal Text (If Needed)

If pure symbols prove too unclear in playtesting, add minimal text:

**Font Choice:**
- Monospace font (clean, readable)
- White text on black background
- No shadows or effects
- Examples: "Iosevka", "IBM Plex Mono", "Source Code Pro"

**Text Usage:**
- Victory screen: "PLAYER 1 WINS"
- Menu options: "START  CONTROLS  QUIT"
- Controls: Just button labels "WASD  ↑←↓→"

**bevy_ui Integration:**
```rust
fn spawn_victory_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        TextBundle::from_section(
            "PLAYER 1 WINS",
            TextStyle {
                font: asset_server.load("fonts/Iosevka-Regular.ttf"),
                font_size: 60.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(200.0),
            left: Val::Percent(50.0),
            ..default()
        }),
    );
}
```

---

## Decision Matrix

| Approach | Implementation Time | Fits Aesthetic | Accessibility | Recommendation |
|----------|-------------------|----------------|---------------|----------------|
| **Pure Gizmos** | 2-3 hours | ★★★★★ | ★★★☆☆ | **Phase 4** |
| **Gizmos + Minimal Text** | 4-5 hours | ★★★★☆ | ★★★★★ | **Phase 5** |
| **Full bevy_ui** | 8-10 hours | ★★☆☆☆ | ★★★★★ | Not recommended |

---

## Recommendation

**For Phase 4 (Now):**
- Implement victory screen with pure gizmos
- Use symbols library (checkmark, arrows, circles)
- Auto-rematch countdown
- Simple "restart" returns to game

**For Phase 5 (Later):**
- Add minimal text overlays for clarity
- Full state machine (MainMenu/InGame/Victory/Controls)
- Smooth fade transitions
- Controls screen

**Rationale:**
- Gizmos are already working well for in-game UI
- Adding text adds complexity and font decisions
- Can always add text later if needed
- Pure geometric design is unique and memorable

---

## Next Steps

1. Complete Phase 4 health/breath systems
2. Detect match victory condition
3. Implement basic victory screen (2-3 hours):
   - Winner indicator (checkmark + arrow)
   - Final breath display
   - "Press to restart" prompt
4. Playtest and iterate
5. Consider text addition in Phase 5 based on feedback
