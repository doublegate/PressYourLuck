# Phase 2: Cell-Shaded Whammy Animations

## Phase Overview

**Goal**: Create authentic, cartoon-style Whammy animations faithful to Savage Steve Holland's original designs, using cell-shaded sprite animation that matches the 1983-1986 CBS show aesthetic.

**Duration**: 8-10 weeks (4-5 sprints)
**Priority**: CRITICAL (defines the game's identity)
**Dependencies**: None (can run parallel with Phase 1)

### Success Criteria
- [ ] Minimum 20 fully animated Whammies (of 79 total)
- [ ] Animation style matches Savage Steve Holland's cell-shaded aesthetic
- [ ] Timing matches original show (2.5-4 seconds per animation)
- [ ] Smooth 60 FPS playback on mid-range hardware
- [ ] Whammy character instantly recognizable to show fans

---

## Sprint 1: Animation Infrastructure (Week 1-2)

### Objectives
- Establish sprite animation system in ggez
- Create Whammy character model sheet
- Build animation framework

### Tasks

#### 1.1 Animation System Architecture
**Complexity**: High | **Estimate**: 3-4 days

- [ ] **Task**: Design animation data structures
  ```rust
  pub struct Animation {
      pub name: String,
      pub frames: Vec<AnimationFrame>,
      pub total_duration: f32,
      pub loop_mode: LoopMode,
  }

  pub struct AnimationFrame {
      pub sprite_rect: Rect,      // Source rect in atlas
      pub duration: f32,          // Frame duration in seconds
      pub offset: Vec2,           // Position offset
      pub scale: f32,             // Scale multiplier
      pub rotation: f32,          // Rotation in radians
      pub audio_trigger: Option<String>,  // Sound to play
  }

  pub enum LoopMode {
      Once,
      Loop,
      PingPong,
  }
  ```

- [ ] **Task**: Implement sprite atlas loader
  - Load texture atlas from PNG
  - Parse atlas metadata (JSON/XML format)
  - Support multiple atlases for memory management

- [ ] **Task**: Create AnimationPlayer component
  - Play/pause/stop controls
  - Frame advancement with timing
  - Event callbacks (frame, completion)
  - Speed multiplier for slow-mo effects

#### 1.2 Whammy Character Design
**Complexity**: Medium | **Estimate**: 2-3 days

- [ ] **Task**: Create Whammy model sheet
  - Front view (primary angle)
  - 3/4 view (action poses)
  - Profile view (running, etc.)
  - Back view (as needed)

- [ ] **Task**: Define Whammy anatomy
  - Head: Round, large eyes, mischievous grin
  - Body: Small, red, simplified
  - Limbs: Simple tubes, expressive hands
  - Tail: Devil-like forked tail

- [ ] **Task**: Establish color palette
  ```
  Primary Red:     #D42020 (body main)
  Shadow Red:      #8B1515 (body shadow)
  Highlight Red:   #FF4040 (body highlight)
  Eye White:       #FFFFFF
  Eye Pupil:       #000000
  Outline:         #1A0505 (near-black for edges)
  Grin White:      #F0F0F0
  ```

- [ ] **Task**: Create cell-shading guidelines
  - Hard edges only (no gradients)
  - 2-3 tones per color
  - Bold black outlines (2-3px at 256px)
  - Consistent light source (top-left)

#### 1.3 Sprite Sheet Pipeline
**Complexity**: Medium | **Estimate**: 2 days

- [ ] **Task**: Set up asset directory
  ```
  assets/
  |-- sprites/
      |-- whammy/
          |-- atlas_01.png      # First batch of animations
          |-- atlas_01.json     # Atlas metadata
          |-- atlas_02.png      # Second batch
          |-- atlas_02.json
  ```

- [ ] **Task**: Create atlas generation script
  - Input: Individual frame PNGs
  - Output: Packed atlas + JSON metadata
  - Tool options: TexturePacker, free-tex-packer, custom script

- [ ] **Task**: Define frame size standards
  - Base resolution: 256x256 pixels per frame
  - Atlas size: 2048x2048 maximum
  - Frames per atlas: ~64 (8x8 grid)

### Acceptance Criteria - Sprint 1
- [ ] AnimationPlayer can play test animation
- [ ] Sprite atlas loads and renders correctly
- [ ] Whammy character design documented
- [ ] Pipeline from frames to atlas established

---

## Sprint 2: Core Animation Set (Week 3-4)

### Objectives
- Animate the 5 most iconic Whammies
- Establish animation quality benchmark
- Validate timing and visual style

### Tasks

#### 2.1 Priority Animation: Hammer Whammy
**Complexity**: High | **Estimate**: 2-3 days

- [ ] **Task**: Storyboard animation phases
  1. Whammy appears (pop in)
  2. Pulls out hammer
  3. Raises hammer over head
  4. Swings down with "WHAM!"
  5. Celebrates/disappears

- [ ] **Task**: Create keyframes (12-15 frames)
  - Appearance: 3 frames
  - Hammer pull: 2 frames
  - Wind-up: 3 frames
  - Strike: 4 frames
  - Exit: 3 frames

- [ ] **Task**: Add in-between frames
  - Total: ~24 frames at 10 fps = 2.4 seconds
  - Smooth motion blur on fast actions

- [ ] **Task**: Integrate audio triggers
  - "WHAM!" sound on strike frame
  - Catchphrase: "Hee hee hee! WHAM!"

#### 2.2 Priority Animation: Pogo Stick Whammy
**Complexity**: Medium | **Estimate**: 2 days

- [ ] **Task**: Storyboard animation phases
  1. Whammy on pogo stick
  2. Bouncing (loop 3-4 times)
  3. Higher bounce
  4. Smoke puff with "WHAM!"
  5. Disappears

- [ ] **Task**: Create frames
  - Bounce cycle: 4 frames (loopable)
  - Entry: 3 frames
  - Exit: 4 frames
  - Total: ~20 frames

#### 2.3 Priority Animation: Roller Skating Whammy
**Complexity**: Medium | **Estimate**: 2 days

- [ ] **Task**: Storyboard animation phases
  1. Whammy skating in
  2. Loses control (wobble)
  3. "Look out! LOOK OOOOOUUT!!!"
  4. Crashes/falls
  5. Score wipes out

- [ ] **Task**: Create frames
  - Skating loop: 4 frames
  - Wobble: 4 frames
  - Crash: 6 frames
  - Total: ~25 frames

#### 2.4 Priority Animation: TNT Whammy
**Complexity**: High | **Estimate**: 2-3 days

- [ ] **Task**: Storyboard animation phases
  1. Whammy with TNT detonator
  2. Lights fuse
  3. "This oughta do it!"
  4. Explosion (large effect)
  5. Aftermath/disappear

- [ ] **Task**: Create explosion effect
  - Separate explosion sprite layer
  - Particle-like debris frames
  - Screen shake trigger

#### 2.5 Priority Animation: Fang (Dog) Whammy
**Complexity**: High | **Estimate**: 2-3 days

- [ ] **Task**: Design Fang character
  - Menacing bulldog design
  - Matches Whammy art style
  - Expressive face

- [ ] **Task**: Storyboard animation phases
  1. Whammy walking Fang on leash
  2. Points at score
  3. "Get 'em, Fang! GET 'EM!"
  4. Fang attacks score
  5. Score destroyed

### Acceptance Criteria - Sprint 2
- [ ] 5 animations fully playable
- [ ] Consistent art style across all
- [ ] Timing matches 2.5 seconds target
- [ ] Audio integrated with animations

---

## Sprint 3: Expanded Animation Set (Week 5-6)

### Objectives
- Add 10 more animations (total: 15)
- Implement animation variants
- Add special effects (particles, screen shake)

### Tasks

#### 3.1 Set 2 Animations
**Complexity**: Medium each | **Estimate**: 4-5 days total

- [ ] **Task**: Boxer Whammy
  - Rocky parody with boxing gloves
  - Spring-loaded glove punch

- [ ] **Task**: Breakdancing Whammy
  - Boom box, breakdance moves
  - Explosion effect reuse

- [ ] **Task**: Elvis Whammy
  - "I want money!" pose
  - Back throw-out gag

- [ ] **Task**: Dollar Bill Whammy
  - Founding father costume
  - "I cannot tell a lie!"

- [ ] **Task**: Magician Whammy
  - Top hat and wand
  - Score "disappears"

#### 3.2 Set 3 Animations
**Complexity**: Medium each | **Estimate**: 4-5 days total

- [ ] **Task**: Surfer Whammy
  - Surfboard, wave
  - "Wipeout!"

- [ ] **Task**: Pilot Whammy
  - Airplane in nosedive
  - "Mayday! MAYDAY!"

- [ ] **Task**: Umpire Whammy
  - Baseball umpire gear
  - "You're OUT!"

- [ ] **Task**: Hula Whammy
  - Hawaiian dancer
  - Water splash effect

- [ ] **Task**: Bulldozer Whammy
  - Erases score digit by digit
  - Progressive reveal

#### 3.3 Special Effects System
**Complexity**: Medium | **Estimate**: 2 days

- [ ] **Task**: Implement particle system
  - Explosion particles
  - Smoke puffs
  - Sparkles/stars
  - Score digit fragments

- [ ] **Task**: Implement screen effects
  - Screen shake (Whammy impacts)
  - Flash (explosions)
  - Wipe transitions

- [ ] **Task**: Create reusable effect library
  - Explosion (multiple sizes)
  - Smoke cloud
  - Impact stars
  - Score zeroing animation

### Acceptance Criteria - Sprint 3
- [ ] 15 total animations complete
- [ ] Particle effects functional
- [ ] Screen effects enhance impact
- [ ] Reusable components documented

---

## Sprint 4: Holiday & Special Animations (Week 7-8)

### Objectives
- Add holiday-themed Whammies
- Implement Whammy-out specials
- Create animation selection system

### Tasks

#### 4.1 Holiday Whammies
**Complexity**: Medium each | **Estimate**: 3-4 days total

- [ ] **Task**: Scrooge Whammy (Christmas)
  - Cane, top hat
  - "Bah! Humbug!"

- [ ] **Task**: Vampire Whammy (Halloween)
  - Cape, fangs
  - "I vant to suck your... CASH!"

- [ ] **Task**: Leprechaun Whammy (St. Patrick's)
  - Green outfit, pot of gold
  - "Top o' the mornin'!"

- [ ] **Task**: Turkey Hunter (Thanksgiving)
  - Pilgrim hat, musket
  - Explosion gag

#### 4.2 Whammy-Out Specials (4th Whammy)
**Complexity**: High | **Estimate**: 3-4 days

- [ ] **Task**: Grim Reaper Whammy
  - Hooded robe, scythe
  - Dramatic entrance
  - Escorts money away
  - Duration: 4 seconds (longer than normal)

- [ ] **Task**: Firing Squad Whammy
  - Multiple Whammies in line
  - "Ready! Aim! FIRE!"
  - Dramatic execution

- [ ] **Task**: Angel Whammy
  - Wings, halo
  - Floats up with money
  - Heaven clouds effect

#### 4.3 Animation Selection System
**Complexity**: Medium | **Estimate**: 2 days

- [ ] **Task**: Implement animation picker
  - Random selection from pool
  - Weighting for classics (higher frequency)
  - Holiday detection (date-based)
  - Whammy-out detection (4th Whammy)

- [ ] **Task**: Animation registry
  ```rust
  pub struct WhammyAnimationRegistry {
      regular: Vec<Animation>,
      holiday: HashMap<Holiday, Vec<Animation>>,
      whammy_out: Vec<Animation>,
  }

  impl WhammyAnimationRegistry {
      pub fn get_animation(&self, context: &AnimationContext) -> &Animation {
          if context.is_fourth_whammy {
              return self.whammy_out.choose_random();
          }
          if let Some(holiday) = get_current_holiday() {
              if let Some(anims) = self.holiday.get(&holiday) {
                  // 30% chance of holiday animation
                  if rand::random::<f32>() < 0.3 {
                      return anims.choose_random();
                  }
              }
          }
          self.regular.choose_weighted_random()
      }
  }
  ```

- [ ] **Task**: Add "no repeat" logic
  - Track last 5 animations shown
  - Avoid immediate repeats
  - Reset on game start

### Acceptance Criteria - Sprint 4
- [ ] 20+ total animations complete
- [ ] Holiday system date-aware
- [ ] Whammy-out animations dramatic
- [ ] Selection system prevents repetition

---

## Sprint 5: Polish & Optimization (Week 9-10)

### Objectives
- Optimize rendering performance
- Add animation quality settings
- Complete remaining priority animations

### Tasks

#### 5.1 Performance Optimization
**Complexity**: High | **Estimate**: 3-4 days

- [ ] **Task**: Implement sprite batching
  - Batch all Whammy frames in single draw call
  - Use ggez SpriteBatch

- [ ] **Task**: Texture atlas optimization
  - Power-of-two dimensions
  - Mipmapping for scaled display
  - Compression options (if supported)

- [ ] **Task**: Animation preloading
  - Load animations on game start
  - Background loading thread
  - Memory management (unload unused)

- [ ] **Task**: Frame rate independence
  - Delta time-based animation
  - Consistent playback across hardware
  - Interpolation for smooth motion

#### 5.2 Quality Settings
**Complexity**: Medium | **Estimate**: 2 days

- [ ] **Task**: Implement quality levels
  - High: Full frame animations
  - Medium: Reduced frames
  - Low: Key frames only

- [ ] **Task**: Resolution scaling
  - Native resolution rendering
  - Scaled rendering for performance
  - Dynamic resolution based on frame rate

#### 5.3 Additional Animations (Time Permitting)
**Complexity**: Medium each | **Estimate**: 3-4 days

- [ ] **Task**: Michael Jackson Whammy
- [ ] **Task**: Beatles Whammy (4 characters)
- [ ] **Task**: Supremes Whammy (3 characters)
- [ ] **Task**: Astronaut Whammy
- [ ] **Task**: Ben Franklin Whammy

### Acceptance Criteria - Sprint 5
- [ ] 60 FPS maintained during animations
- [ ] Quality settings functional
- [ ] Memory usage acceptable (< 500MB for animations)
- [ ] All animations documented

---

## Technical Requirements

### Animation Specifications
| Parameter | Value |
|-----------|-------|
| Frame Resolution | 256x256 px |
| Frame Rate | 10-12 fps (animation) |
| Playback Rate | 60 fps (rendering) |
| Duration | 2.5-4.0 seconds |
| Format | PNG (RGBA) |
| Atlas Size | 2048x2048 max |

### Art Style Guide

#### Cell Shading Rules
1. **No gradients**: Use flat color fills
2. **Hard shadows**: Clear boundary between lit and shadow
3. **Bold outlines**: 2-3px black outline at 256px
4. **Limited palette**: Max 6 colors per character
5. **Consistent lighting**: Top-left light source

#### Whammy Design Principles
- **Expressive**: Large eyes, exaggerated poses
- **Mischievous**: Always looks pleased with himself
- **Simple**: Reads clearly at small sizes
- **Consistent**: Same base design across all animations

### Animation Timing Reference
```
| Animation Type | Duration | Frames | Notes |
|---------------|----------|--------|-------|
| Standard      | 2.5s     | 25-30  | Most animations |
| Elaborate     | 3.0s     | 30-36  | TNT, Orchestra |
| Musical       | 3.5s     | 35-42  | Beatles, Supremes |
| Whammy-Out    | 4.0s     | 40-48  | Dramatic effect |
```

---

## Asset Requirements

### Tools Needed
- **Sprite creation**: Aseprite, Photoshop, GIMP, Krita
- **Atlas packing**: TexturePacker, free-tex-packer, custom script
- **Animation preview**: Aseprite, custom viewer

### Reference Materials
- VHS recordings of original show
- Press Your Luck Wikia animation descriptions
- Savage Steve Holland's later works for style reference

### Deliverables per Animation
1. Storyboard sketch (rough poses)
2. Keyframe drawings
3. In-between frames
4. Final sprite sheet
5. Atlas metadata JSON
6. Audio trigger timing

---

## Implementation Strategy

### Order of Priority
1. **Iconic classics**: Hammer, Pogo, Roller Skating, TNT, Fang
2. **Fan favorites**: Elvis, Breakdancing, Dollar Bill
3. **Holiday specials**: Scrooge, Vampire, Leprechaun
4. **Whammy-outs**: Grim Reaper, Firing Squad, Angel
5. **Remaining**: Based on time available

### Quality Over Quantity
- Better to have 20 polished animations than 50 rough ones
- Each animation should be show-accurate
- Prioritize most-seen animations first

---

## Estimated Effort

| Sprint | Focus | Story Points | Hours |
|--------|-------|--------------|-------|
| Sprint 1 | Infrastructure | 21 | 30-40 |
| Sprint 2 | Core 5 Animations | 34 | 45-60 |
| Sprint 3 | 10 More + Effects | 34 | 45-60 |
| Sprint 4 | Holiday + Specials | 26 | 35-45 |
| Sprint 5 | Polish + Optimization | 21 | 30-40 |
| **Total** | | **136** | **185-245** |

---

## Definition of Done

Phase 2 is complete when:
- [ ] Minimum 20 fully animated Whammies implemented
- [ ] Animation style matches original show aesthetic
- [ ] All animations play at correct timing
- [ ] Particle effects enhance visual impact
- [ ] Whammy-out specials implemented
- [ ] Holiday animation system functional
- [ ] Performance meets 60 FPS target
- [ ] Quality settings accessible
- [ ] Documentation and asset pipeline complete

---

*"I animated that little fella on the most primitive computer animation system on Earth. It was steam-powered."* - Savage Steve Holland
