# Phase 1: Original Audio Integration

## Phase Overview

**Goal**: Replace procedurally-generated audio with authentic Press Your Luck sounds from the original 1983-1986 CBS show, creating an immediately recognizable audio experience for anyone familiar with the series.

**Duration**: 4-6 weeks (2-3 sprints)
**Priority**: HIGH
**Dependencies**: None (can run parallel with Phase 2)

### Success Criteria
- [ ] Players who watched original show recognize sounds immediately
- [ ] Whammy foghorn triggers nostalgia response
- [ ] Board tones match original musical sequence
- [ ] Theme music authentically represents show
- [ ] All audio plays without latency or artifacts

---

## Sprint 1: Audio Research & Infrastructure (Week 1-2)

### Objectives
- Establish audio asset pipeline
- Source authentic sound files
- Set up audio file management system

### Tasks

#### 1.1 Audio Asset Research & Sourcing
**Complexity**: Medium | **Estimate**: 3-4 days

- [ ] **Task**: Download theme music from Archive.org
  - Source: tvtunes_3043 (main theme)
  - Source: tvtunes_31176 (board sound)
  - Source: tvtunes_10932 (closing theme)
  - Source: tvtunes_10967 (prize cue)
  - Source: tvtunes_29638 (full theme)

- [ ] **Task**: Source Whammy foghorn sound
  - Check soundboard sites (Myinstants, 101soundboards, Voicemod)
  - Extract from YouTube VHS recordings if needed
  - Document "hiccup" artifact (authentic to original)

- [ ] **Task**: Source audience reaction sounds
  - Cheers (big wins)
  - Gasps (Whammy hits)
  - Groans (eliminations)
  - Consider royalty-free alternatives if originals unavailable

- [ ] **Task**: Document copyright status
  - Research fair use for personal/educational projects
  - Identify which sounds are CBS property vs. stock libraries
  - Note: BBC Records and Sound Ideas library effects used in show

#### 1.2 Audio Asset Organization
**Complexity**: Low | **Estimate**: 1 day

- [ ] **Task**: Create asset directory structure
  ```
  assets/
  |-- audio/
      |-- theme/
      |   |-- main_theme.ogg
      |   |-- closing_theme.ogg
      |   |-- board_music.ogg
      |-- effects/
      |   |-- whammy_foghorn.ogg
      |   |-- cash_register.ogg
      |   |-- prize_fanfare.ogg
      |   |-- board_stop.ogg
      |-- board_tones/
      |   |-- tone_00.ogg ... tone_17.ogg
      |-- audience/
      |   |-- cheer_01.ogg ... cheer_03.ogg
      |   |-- gasp_01.ogg ... gasp_03.ogg
      |-- whammy/
          |-- catchphrase_hammer.ogg
          |-- catchphrase_pogo.ogg
          |-- ... (per animation)
  ```

- [ ] **Task**: Create audio manifest file
  - JSON/TOML file listing all audio assets
  - Include duration, loop points, volume recommendations

#### 1.3 Audio Engine Refactor
**Complexity**: Medium | **Estimate**: 2-3 days

- [ ] **Task**: Modify AudioEngine to support file loading
  - Add file-based sound loading alongside procedural generation
  - Implement fallback to procedural if file missing
  - Update `generate_sounds()` to `load_sounds()` pattern

- [ ] **Task**: Implement audio asset caching
  - Load audio on game start
  - Cache SoundData for quick playback
  - Handle loading errors gracefully

- [ ] **Task**: Create audio configuration system
  - Volume levels per category (music, effects, voice)
  - Mute toggles
  - Save/load preferences

### Acceptance Criteria - Sprint 1
- [ ] Asset directory structure exists and documented
- [ ] At least theme music and Whammy foghorn sourced
- [ ] AudioEngine can load files from disk
- [ ] Fallback to procedural audio works

---

## Sprint 2: Core Sound Replacement (Week 3-4)

### Objectives
- Replace key procedural sounds with authentic files
- Implement Whammy-specific audio system
- Add theme music integration

### Tasks

#### 2.1 Whammy Audio System
**Complexity**: High | **Estimate**: 3-4 days

- [ ] **Task**: Replace Whammy foghorn
  - Load authentic two-note synth foghorn
  - Ensure timing matches animation start
  - Test "hiccup" artifact doesn't cause issues

- [ ] **Task**: Implement Whammy catchphrase system
  - Create mapping from WhammyAnimationType to audio file
  - Play catchphrase at appropriate animation moment
  - Support voice clips if available (Bill Carruthers voice)

- [ ] **Task**: Add Whammy animation-specific SFX
  - Research which animations had specific sounds (boxing bell, boombox explosion, etc.)
  - Source from Sound Ideas / BBC Records equivalents
  - Map sounds to animation phases

#### 2.2 Board Audio Enhancement
**Complexity**: Medium | **Estimate**: 2-3 days

- [ ] **Task**: Replace board tones with authentic sounds
  - Compare procedural tones to show recordings
  - If recordings available, use those
  - If not, fine-tune procedural frequencies to match

- [ ] **Task**: Implement board stop sound
  - Source authentic mechanical "chunk"
  - Time precisely with animation stop
  - Add subtle reverb for studio feel

- [ ] **Task**: Add board spin ambient sound
  - Underlying tension during active spin
  - Smooth loop point for seamless playback
  - Volume ducking for other sounds

#### 2.3 Theme Music Integration
**Complexity**: Medium | **Estimate**: 2 days

- [ ] **Task**: Implement theme music player
  - Play intro theme at game start
  - Support looping board music during play
  - Smooth transitions between tracks

- [ ] **Task**: Add prize/win music cues
  - Cash win fanfare
  - Prize reveal music
  - Winner celebration theme

- [ ] **Task**: Implement music volume ducking
  - Duck music during voice clips
  - Restore after clip completes
  - Configurable duck amount

### Acceptance Criteria - Sprint 2
- [ ] Whammy foghorn plays authentic sound
- [ ] Board tones match original show
- [ ] Theme music plays at appropriate times
- [ ] All sound transitions smooth

---

## Sprint 3: Polish & Audience Immersion (Week 5-6)

### Objectives
- Add audience reaction sounds
- Implement host/announcer presence
- Fine-tune audio mixing

### Tasks

#### 3.1 Audience Reaction System
**Complexity**: Medium | **Estimate**: 3 days

- [ ] **Task**: Implement audience cheer system
  - Trigger on big wins (> $1000)
  - Trigger on special squares
  - Vary intensity based on win size
  - Randomize from pool of variations

- [ ] **Task**: Implement audience gasp system
  - Trigger on Whammy hits
  - Trigger on close calls (near-Whammy)
  - Add tension buildup before elimination

- [ ] **Task**: Add ambient audience murmur
  - Low-level background during gameplay
  - Increases during exciting moments
  - Optional (can be toggled off)

#### 3.2 Host/Announcer Audio (Optional Enhancement)
**Complexity**: High | **Estimate**: 2-3 days

- [ ] **Task**: Research voice synthesis options
  - Text-to-speech with period-appropriate voice
  - Consider ElevenLabs/other AI voice for Peter Tomarken style
  - Fallback: text-only announcements

- [ ] **Task**: Implement key announcer phrases
  - "Big Bucks! No Whammies!"
  - Player names and scores
  - Round transitions
  - Winner announcement

- [ ] **Task**: Add Rod Roddy-style prize descriptions
  - Prize reveal narration
  - "A trip to Hawaii!" style excitement
  - Optional feature (can be toggled)

#### 3.3 Audio Mixing & Polish
**Complexity**: Medium | **Estimate**: 2 days

- [ ] **Task**: Implement comprehensive audio mixing
  - Master volume control
  - Category volumes (music, effects, voice, audience)
  - Prevent audio clipping
  - Spatial audio for stereo positioning (optional)

- [ ] **Task**: Add audio settings menu
  - Volume sliders
  - Mute toggles
  - Audio quality options

- [ ] **Task**: Performance optimization
  - Preload frequently-used sounds
  - Unload unused audio during transitions
  - Memory usage monitoring

### Acceptance Criteria - Sprint 3
- [ ] Audience reacts appropriately to game events
- [ ] Audio mixing prevents clipping
- [ ] Settings menu functional
- [ ] Performance acceptable (no audio lag)

---

## Technical Requirements

### Dependencies
```toml
[dependencies]
# Existing
ggez = "0.9"
rodio = "0.17"  # Via ggez

# May need for format conversion
# symphonia = "0.5"  # If format issues arise
```

### File Format Specifications
- **Format**: OGG Vorbis (primary), WAV (fallback)
- **Sample Rate**: 44,100 Hz
- **Channels**: Mono for effects, Stereo for music
- **Bit Depth**: 16-bit minimum

### Audio System Architecture
```rust
pub struct AudioEngine {
    // Asset-based sounds
    asset_sounds: HashMap<String, Source>,

    // Procedural fallbacks
    procedural_sounds: HashMap<String, Source>,

    // Configuration
    config: AudioConfig,

    // Playback state
    playing_music: Option<Source>,
    audience_ambient: Option<Source>,
}

pub struct AudioConfig {
    pub master_volume: f32,
    pub music_volume: f32,
    pub effects_volume: f32,
    pub voice_volume: f32,
    pub audience_volume: f32,
    pub use_authentic_sounds: bool,
}
```

---

## Research Findings

### Authentic Sound Sources
| Sound | Source | Status |
|-------|--------|--------|
| Theme Music | Archive.org tvtunes | Available |
| Board Sound | Archive.org tvtunes_31176 | Available |
| Prize Cue | Archive.org tvtunes_10967 | Available |
| Whammy Foghorn | Soundboard sites | Available |
| Audience Sounds | YouTube/stock libraries | Research needed |

### Original Show Audio Notes
- First 7 episodes: No Whammy foghorn
- First 7 episodes: CBS "ding" for passing spins
- First 22 episodes: Board sound at lower volume
- Sound Ideas library used for many Whammy animation effects

### Legal Considerations
- Theme music: Likely CBS property
- Stock library sounds: Check licensing
- Audience sounds: Generic crowd sounds widely available
- Recommendation: Include fallback to procedural for distribution

---

## Implementation Strategy

### Order of Implementation
1. **Week 1**: Set up asset structure, source key sounds
2. **Week 2**: Refactor AudioEngine for file loading
3. **Week 3**: Replace Whammy and board sounds
4. **Week 4**: Add theme music and transitions
5. **Week 5**: Implement audience system
6. **Week 6**: Polish, mixing, settings

### Testing Approach
- A/B comparison with VHS recordings
- Blind testing with show fans
- Latency measurement
- Memory usage profiling

### Fallback Strategy
If authentic sounds unavailable:
1. Procedural generation (current implementation)
2. High-quality synthesized alternatives
3. Royalty-free period-appropriate sounds

---

## Estimated Effort

| Sprint | Tasks | Story Points | Hours |
|--------|-------|--------------|-------|
| Sprint 1 | Research & Infrastructure | 13 | 20-25 |
| Sprint 2 | Core Sound Replacement | 21 | 30-40 |
| Sprint 3 | Polish & Immersion | 18 | 25-35 |
| **Total** | | **52** | **75-100** |

---

## Definition of Done

Phase 1 is complete when:
- [ ] All key sounds replaced with authentic or high-quality alternatives
- [ ] Theme music plays at appropriate game states
- [ ] Whammy sound instantly recognizable
- [ ] Audience reactions enhance immersion
- [ ] Audio settings accessible to player
- [ ] No audio latency or quality issues
- [ ] Documentation updated
- [ ] Code reviewed and tested

---

*"Big Bucks! No Whammies! STOP!"* - Peter Tomarken
