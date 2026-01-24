# Phase 4: Photorealistic Immersive Presentation

## Phase Overview

**Goal**: Create a presentation layer that makes players feel like they are standing on the actual 1983-1986 CBS Press Your Luck set at Television City, complete with authentic CRT television effects, studio lighting, and era-appropriate visual treatment.

**Duration**: 6-8 weeks (3-4 sprints)
**Priority**: MEDIUM (enhances but doesn't block core game)
**Dependencies**: Phases 1-3 complete

### Success Criteria
- [ ] CRT shader creates authentic 1980s TV viewing experience
- [ ] Set design recognizable as CBS Television City Studio 33
- [ ] Lighting and color grading match show aesthetic
- [ ] Host/announcer presence enhances immersion
- [ ] Sound design creates "being there" atmosphere

---

## Sprint 1: CRT Television Effects (Week 1-2)

### Objectives
- Implement authentic CRT shader effects
- Create 1980s television viewing experience
- Add optional intensity controls

### Tasks

#### 1.1 CRT Shader Foundation
**Complexity**: High | **Estimate**: 3-4 days

- [ ] **Task**: Research ggez/wgpu shader pipeline
  - Understand Canvas -> Shader -> Screen flow
  - WGSL shader format for wgpu
  - Post-processing render target setup

- [ ] **Task**: Implement base CRT shader
  ```wgsl
  // CRT shader outline for WGSL
  struct CRTParams {
      scanline_intensity: f32,
      scanline_spacing: f32,
      curvature: f32,
      vignette: f32,
      phosphor_bloom: f32,
      color_bleed: f32,
  }

  @fragment
  fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
      var color = textureSample(screen_texture, sampler, in.uv);

      // Apply effects in order
      color = apply_curvature(color, in.uv);
      color = apply_scanlines(color, in.uv);
      color = apply_phosphor_bloom(color);
      color = apply_color_bleed(color);
      color = apply_vignette(color, in.uv);

      return color;
  }
  ```

- [ ] **Task**: Set up render-to-texture pipeline
  - Render game to offscreen texture
  - Apply CRT shader to texture
  - Display final result to screen

#### 1.2 Scanline Effect
**Complexity**: Medium | **Estimate**: 1-2 days

- [ ] **Task**: Implement horizontal scanlines
  - Dark lines between pixel rows
  - Authentic 480i NTSC line count
  - Fade intensity at edges

- [ ] **Task**: Add scanline animation (optional)
  - Subtle vertical scrolling
  - Simulates CRT refresh
  - Very slow rate (barely perceptible)

- [ ] **Task**: Resolution handling
  - Scale scanlines to output resolution
  - Maintain correct density
  - Handle fullscreen/windowed

#### 1.3 Barrel Distortion (Curvature)
**Complexity**: Medium | **Estimate**: 1-2 days

- [ ] **Task**: Implement screen curvature
  - CRT screens were slightly curved
  - Edges bend toward center
  - Subtle effect (not extreme)

- [ ] **Task**: Curvature parameters
  ```rust
  pub struct CurvatureSettings {
      pub horizontal: f32,  // 0.0-0.15 typical
      pub vertical: f32,    // 0.0-0.15 typical
      pub corner_size: f32, // Rounded corners
  }
  ```

#### 1.4 Phosphor Bloom Effect
**Complexity**: High | **Estimate**: 2-3 days

- [ ] **Task**: Implement bloom pass
  - Extract bright areas
  - Gaussian blur
  - Additive blend back

- [ ] **Task**: Color-specific bloom
  - CRT phosphors had specific glow characteristics
  - Red: Warm, wide bloom
  - Green: Tight, bright bloom
  - Blue: Subtle, cool bloom

- [ ] **Task**: Performance optimization
  - Downscaled bloom pass
  - Efficient blur algorithm

#### 1.5 Chromatic Aberration
**Complexity**: Low | **Estimate**: 0.5-1 day

- [ ] **Task**: Implement RGB separation
  - Slight offset of color channels
  - More pronounced at screen edges
  - Subtle effect (1-2 pixels)

- [ ] **Task**: Vignette effect
  - Darker corners
  - Gradual falloff
  - Authentic to CRT viewing

### Acceptance Criteria - Sprint 1
- [ ] CRT shader functional and toggleable
- [ ] Scanlines visible but not distracting
- [ ] Curvature creates authentic shape
- [ ] Bloom adds warmth without overwhelming
- [ ] Performance acceptable (60 FPS)

---

## Sprint 2: Studio Set Design (Week 3-4)

### Objectives
- Create CBS Television City studio aesthetic
- Design detailed Big Board presentation
- Add studio lighting effects

### Tasks

#### 2.1 Set Background Design
**Complexity**: Medium | **Estimate**: 2-3 days

- [ ] **Task**: Research CBS Television City Studio 33
  - Physical layout
  - Lighting rig positions
  - Color scheme (deep blues, purples)

- [ ] **Task**: Create layered background
  ```
  Layer 1: Deep space/abstract pattern (far back)
  Layer 2: Studio walls/curtains
  Layer 3: Lighting effects
  Layer 4: Board area backdrop
  Layer 5: The Big Board
  Layer 6: Contestant podiums
  Layer 7: UI overlays
  ```

- [ ] **Task**: Implement parallax effect (optional)
  - Subtle depth movement
  - Responds to board activity
  - Enhances 3D feel

#### 2.2 Big Board Frame Enhancement
**Complexity**: Medium | **Estimate**: 2-3 days

- [ ] **Task**: Design authentic board frame
  - Chrome/metallic trim
  - Neon accent lighting
  - Beveled edges with shadows

- [ ] **Task**: Add board lighting effects
  - Individual square lighting
  - Glow from active squares
  - Reflection on frame

- [ ] **Task**: Chase lights enhancement
  - Physical bulb appearance
  - On/off states with glow
  - Glass/plastic cover effect

#### 2.3 Podium Design
**Complexity**: Medium | **Estimate**: 2 days

- [ ] **Task**: Create detailed podium graphics
  - 3D-style podium shape
  - Player name plate area
  - Score display area
  - Whammy counter area

- [ ] **Task**: Add podium lighting
  - Spotlight on active player
  - Color tinting per player
  - Dim effect for eliminated players

- [ ] **Task**: Score display enhancement
  - LED-style digit segments
  - Rolling number animation
  - Zeroing animation for Whammy

#### 2.4 Studio Lighting System
**Complexity**: High | **Estimate**: 3 days

- [ ] **Task**: Implement dynamic lighting
  - Ambient studio lighting
  - Spotlights (moveable)
  - Board backlighting

- [ ] **Task**: Lighting states
  | State | Lighting |
  |-------|----------|
  | Idle | Ambient, soft |
  | Question | Focus on question area |
  | Spin | Dynamic, exciting |
  | Whammy | Dramatic red tint |
  | Win | Celebratory, bright |

- [ ] **Task**: Transition effects
  - Smooth lighting transitions
  - Follow game state changes
  - No jarring cuts

### Acceptance Criteria - Sprint 2
- [ ] Background resembles TV studio
- [ ] Big Board looks physical/tangible
- [ ] Podiums are detailed and functional
- [ ] Lighting enhances mood appropriately

---

## Sprint 3: Host & Announcer Presence (Week 5-6)

### Objectives
- Add Peter Tomarken and Rod Roddy presence
- Implement commentary system
- Create authentic show flow

### Tasks

#### 3.1 Host Character System
**Complexity**: Medium | **Estimate**: 2-3 days

- [ ] **Task**: Design host representation
  - Option A: Illustrated character (2D portrait)
  - Option B: Text/voice only
  - Option C: Simple silhouette
  - Recommend: Text + Voice (AI or TTS)

- [ ] **Task**: Host dialogue system
  ```rust
  pub struct HostDialogue {
      pub trigger: DialogueTrigger,
      pub lines: Vec<String>,
      pub audio_file: Option<String>,
      pub timing: DialogueTiming,
  }

  pub enum DialogueTrigger {
      GameStart,
      RoundStart(usize),
      QuestionRead,
      CorrectAnswer,
      WrongAnswer,
      SpinStart,
      BigWin(u32),  // Threshold
      Whammy(u32),  // Whammy count
      WhammyOut,
      GameOver,
  }
  ```

- [ ] **Task**: Implement dialogue manager
  - Queue dialogues
  - Prevent overlap
  - Priority system

#### 3.2 Signature Phrases
**Complexity**: Low | **Estimate**: 1-2 days

- [ ] **Task**: Implement Peter Tomarken phrases
  - "Big Bucks! No Whammies!"
  - "And STOP!"
  - "Congratulations!"
  - Player name + score announcements

- [ ] **Task**: Implement Rod Roddy phrases
  - Prize descriptions
  - "A trip to Hawaii!"
  - Player introductions

- [ ] **Task**: Text-to-speech integration (optional)
  - AI voice synthesis
  - Period-appropriate voice style
  - Fallback to text display

#### 3.3 Announcer Prize Descriptions
**Complexity**: Medium | **Estimate**: 2 days

- [ ] **Task**: Create prize description database
  ```rust
  pub struct PrizeDescription {
      pub prize_type: PrizeType,
      pub short_text: String,      // "A trip to Hawaii"
      pub long_text: String,       // "Seven days and six nights..."
      pub value_text: String,      // "Worth $3,500!"
      pub enthusiasm_level: u8,    // 1-5 for voice modulation
  }
  ```

- [ ] **Task**: Dynamic description generation
  - Cash: Dollar amount with enthusiasm scaling
  - Prizes: Pre-written descriptions
  - Specials: Explanation of what happens

#### 3.4 Show Flow Narration
**Complexity**: Medium | **Estimate**: 2 days

- [ ] **Task**: Round transition narration
  - "Let's start Round 1!"
  - "Time for the Big Board!"
  - "And now, Round 2!"

- [ ] **Task**: Contextual commentary
  - Close scores
  - Big comebacks
  - Whammy danger (3 Whammies)
  - Record-breaking wins

- [ ] **Task**: Winner announcement
  - Build-up suspense
  - Final score reveal
  - Celebration moment

### Acceptance Criteria - Sprint 3
- [ ] Host presence felt throughout game
- [ ] Key phrases trigger appropriately
- [ ] Prize descriptions enhance wins
- [ ] Show flow feels authentic

---

## Sprint 4: Atmosphere & Polish (Week 7-8)

### Objectives
- Add ambient audio atmosphere
- Implement camera/view effects
- Final presentation polish

### Tasks

#### 4.1 Ambient Audio System
**Complexity**: Medium | **Estimate**: 2-3 days

- [ ] **Task**: Studio ambiance track
  - Low-level audience murmur
  - Studio equipment hum
  - Occasional coughs/movement

- [ ] **Task**: Dynamic audience system
  - Volume responds to excitement
  - Cheers for big wins
  - Gasps for Whammies
  - Groans for eliminations

- [ ] **Task**: Music integration
  - Theme music transitions
  - Tension music during spins
  - Victory music for wins
  - Sad music for Whammy-outs

#### 4.2 Camera/View System
**Complexity**: Medium | **Estimate**: 2 days

- [ ] **Task**: Implement view modes
  - Wide shot (default): Full board + podiums
  - Board focus: Close-up during spins
  - Podium focus: Close-up during question
  - Whammy focus: Centered for animation

- [ ] **Task**: View transitions
  - Smooth camera movement
  - Appropriate timing
  - Optional: Player controls view

- [ ] **Task**: Zoom effects
  - Subtle zoom on exciting moments
  - Pull back for reveals
  - Dramatic zoom for Whammy

#### 4.3 Visual Effects Polish
**Complexity**: Medium | **Estimate**: 2-3 days

- [ ] **Task**: Confetti/particle effects
  - Big wins
  - Game winner
  - Round transitions

- [ ] **Task**: Screen transitions
  - Fade between states
  - Wipe effects (80s style)
  - Flash for reveals

- [ ] **Task**: UI animation polish
  - Score counting animations
  - Spin counter updates
  - Whammy icon animations

#### 4.4 Accessibility & Settings
**Complexity**: Low | **Estimate**: 1-2 days

- [ ] **Task**: CRT effect toggle
  - On/Off switch
  - Intensity slider
  - Individual effect controls

- [ ] **Task**: Visual accessibility
  - High contrast mode
  - Color blind options
  - Large text option

- [ ] **Task**: Audio accessibility
  - Caption/subtitle system
  - Visual audio cues
  - Volume per category

### Acceptance Criteria - Sprint 4
- [ ] Atmosphere enhances immersion
- [ ] Camera system adds dynamism
- [ ] Visual polish is consistent
- [ ] Accessibility options functional

---

## Technical Requirements

### Shader System
```rust
pub struct CRTShader {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    params_buffer: wgpu::Buffer,
    offscreen_texture: wgpu::Texture,
}

pub struct CRTParams {
    pub enabled: bool,
    pub scanline_intensity: f32,    // 0.0-1.0
    pub scanline_count: u32,        // Lines (e.g., 480)
    pub curvature_x: f32,           // 0.0-0.2
    pub curvature_y: f32,           // 0.0-0.2
    pub vignette_strength: f32,     // 0.0-1.0
    pub bloom_intensity: f32,       // 0.0-1.0
    pub chromatic_aberration: f32,  // 0.0-5.0 (pixels)
    pub noise_intensity: f32,       // 0.0-0.1
}
```

### View System
```rust
pub struct ViewState {
    pub mode: ViewMode,
    pub zoom: f32,
    pub offset: Vec2,
    pub target_zoom: f32,
    pub target_offset: Vec2,
    pub transition_speed: f32,
}

pub enum ViewMode {
    Wide,           // Default view
    BoardFocus,     // During spins
    PodiumFocus(usize),  // During questions
    WhammyFocus,    // During animation
    Custom(Vec2, f32),  // Position, zoom
}
```

---

## Research Findings

### CRT Television Characteristics (1980s)
- **Resolution**: 480i (interlaced)
- **Refresh Rate**: 29.97 fps (NTSC)
- **Phosphor Type**: Shadow mask (most common)
- **Color Temperature**: Warm (slightly red/yellow)
- **Black Level**: Not true black (dark gray/purple)
- **Bloom**: Visible on bright whites

### CBS Television City Studio 33
- **Location**: 7800 Beverly Boulevard, Los Angeles
- **Size**: Large studio, multiple game show sets
- **Lighting**: Theatrical grid, colored gels
- **Audience**: Live studio audience visible
- **Era Features**: Neon, chrome, electronic displays

### Period Visual Style (1983-1986)
- **Colors**: Bold primaries, neon accents
- **Typography**: Futuristic, angular fonts
- **Effects**: Starburst, glow, chrome reflections
- **Animation**: Electronic, pixelated transitions
- **Overall**: High-energy, flashy, optimistic

---

## Implementation Strategy

### Order of Implementation
1. CRT shader (foundation for all visuals)
2. Set background (context)
3. Board and podium enhancement
4. Lighting system
5. Host/announcer system
6. View system
7. Final polish

### Performance Considerations
- CRT shader: GPU-heavy, optimize carefully
- Multiple render passes: Minimize where possible
- Dynamic lighting: Use lookup textures
- Particle effects: Object pooling

---

## Estimated Effort

| Sprint | Focus | Story Points | Hours |
|--------|-------|--------------|-------|
| Sprint 1 | CRT Shader | 26 | 35-45 |
| Sprint 2 | Studio Design | 26 | 35-45 |
| Sprint 3 | Host/Announcer | 21 | 30-40 |
| Sprint 4 | Polish | 21 | 30-40 |
| **Total** | | **94** | **130-170** |

---

## Definition of Done

Phase 4 is complete when:
- [ ] CRT shader creates authentic 1980s feel
- [ ] Studio set design recognizable as TV studio
- [ ] Big Board looks physical and detailed
- [ ] Podiums are polished and functional
- [ ] Host/announcer presence enhances experience
- [ ] Ambient audio creates atmosphere
- [ ] View system adds dynamism
- [ ] All effects toggleable/adjustable
- [ ] Performance maintained at 60 FPS
- [ ] Accessibility options available

---

*"Press Your Luck was taped at CBS Television City, 7800 Beverly Boulevard, in Hollywood."* - End credits
