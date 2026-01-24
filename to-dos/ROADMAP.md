# Press Your Luck - Development Roadmap

## Vision Statement

**Press Your Luck** aims to be the definitive authentic recreation of the 1983-1986 CBS game show, delivering an immersive experience that transports players back to the original television studio. This project will achieve museum-quality accuracy in gameplay mechanics, visual presentation, audio design, and character animations - making players feel like they are truly contestants on the show with Peter Tomarken and Rod Roddy.

**Ultimate Goal**: Create an experience so authentic that viewers of the original show would feel genuine nostalgia, complete with Savage Steve Holland's iconic Whammy animations, Edd Kalehoff/Score Productions-style audio, and the exact gameplay mechanics that made the show legendary.

---

## Current State Assessment (v2.0.2)

### Completed Features
- **Game Engine**: ggez 0.9 framework with solid rendering pipeline
- **Board Layout**: Authentic 18-square perimeter board with cycling prizes
- **Game Logic**: Full two-round format with question and board phases
- **Prize System**: Cash, prizes, specials, and Whammy mechanics
- **Contestant System**: 3-player support with spin passing, eliminations
- **Whammy Data**: All 79 animation types defined with catchphrases
- **Procedural Audio**: Runtime-generated sound effects (board tones, Whammy, cash)
- **UI System**: Question display, podiums, score tracking
- **Visual Effects**: Chase lights, CRT color palette, rainbow headers

### Current Limitations
1. **Whammy Animations**: Text/placeholder only - no actual animated sprites
2. **Audio**: Procedurally generated, not authentic show recordings
3. **Graphics**: Functional but not photorealistic CBS studio aesthetic
4. **Board Patterns**: Not implementing Larson-era 5-pattern system
5. **Host/Announcer**: No audio presence of Tomarken/Roddy
6. **Set Design**: Abstract representation, not accurate TV studio

---

## Phase Timeline Overview

```
Phase 1: Audio Foundation (PHASE-1-AUDIO.md)
    Est. Duration: 4-6 weeks (2-3 sprints)
    Priority: HIGH
    Dependencies: None

Phase 2: Graphics & Animations (PHASE-2-GRAPHICS.md)
    Est. Duration: 8-10 weeks (4-5 sprints)
    Priority: CRITICAL
    Dependencies: None (can parallel with Phase 1)

Phase 3: Gameplay Authenticity (PHASE-3-GAMEPLAY.md)
    Est. Duration: 4-6 weeks (2-3 sprints)
    Priority: HIGH
    Dependencies: Partial Phase 1, Phase 2 Whammy system

Phase 4: Immersive Presentation (PHASE-4-PRESENTATION.md)
    Est. Duration: 6-8 weeks (3-4 sprints)
    Priority: MEDIUM
    Dependencies: Phases 1-3 complete

Phase 5: Polish & Optimization (PHASE-5-POLISH.md)
    Est. Duration: 4-6 weeks (2-3 sprints)
    Priority: MEDIUM
    Dependencies: Phases 1-4 complete

TOTAL ESTIMATED: 26-36 weeks (~6-9 months)
```

---

## Phase Dependencies

```
                    +-----------------+
                    |  Phase 1: Audio |
                    +---------+-------+
                              |
                              v
+-------------------+   +-----+-----+   +-----------------------+
| Phase 2: Graphics |-->| Phase 3   |-->| Phase 4: Presentation |
| (Whammy anims)    |   | Gameplay  |   +-----------+-----------+
+-------------------+   +-----------+               |
                                                    v
                                          +---------+---------+
                                          | Phase 5: Polish   |
                                          +-------------------+
```

**Parallel Work Opportunities**:
- Phase 1 (Audio) and Phase 2 (Graphics) can proceed simultaneously
- Phase 3 Sprint 1 can begin while Phase 2 completes final sprints

---

## Risk Assessment

### High Risk
| Risk | Impact | Mitigation |
|------|--------|------------|
| Copyright issues with original audio | Cannot use authentic sounds | Prepare high-quality synthesized alternatives; document fair use rationale for personal/educational use |
| Whammy animation complexity | Delays, quality issues | Start with subset of 12 most iconic animations; iterate |
| ggez shader limitations | Cannot achieve CRT effects | Investigate wgpu shaders; fallback to simpler post-processing |

### Medium Risk
| Risk | Impact | Mitigation |
|------|--------|------------|
| Performance with full animations | Frame rate drops | Implement sprite atlases, LOD system, lazy loading |
| Accurate timing data unavailable | Gameplay feels "off" | Study VHS recordings frame-by-frame; iterative testing |
| Scope creep | Extended timeline | Strict sprint planning; MVP first, enhancements later |

### Low Risk
| Risk | Impact | Mitigation |
|------|--------|------------|
| Dependency updates break ggez | Build failures | Lock versions; test updates in branch |
| Asset organization complexity | Technical debt | Establish conventions early; document asset pipeline |

---

## Success Metrics

### Phase 1: Audio
- [ ] 90%+ recognition rate from show fans for key sounds
- [ ] All 18 board tones match original frequencies
- [ ] Whammy sound triggers nostalgia response

### Phase 2: Graphics
- [ ] 20+ fully animated Whammies with all keyframes
- [ ] Cell-shaded style matches Savage Steve Holland aesthetic
- [ ] Animation timing matches original show (verified via VHS)

### Phase 3: Gameplay
- [ ] All special squares function per original rules
- [ ] Board patterns match historical documentation
- [ ] Spin mechanics identical to show (passing, earned vs. passed)

### Phase 4: Presentation
- [ ] CRT shader creates authentic 1980s TV feel
- [ ] Set design recognizable as CBS Television City Studio 33
- [ ] Host/announcer audio presence (even if synthesized)

### Phase 5: Polish
- [ ] 60 FPS on mid-range hardware
- [ ] < 2 second load time
- [ ] Zero gameplay-affecting bugs
- [ ] Complete accessibility options

---

## Resource Requirements

### Assets to Source/Create
- **Audio**: Theme music, Whammy sounds, audience reactions, host clips
- **Graphics**: Whammy sprite sheets (79 animations x ~20 frames each)
- **Reference**: VHS recordings, production documents, fan archives

### Technical Skills
- Rust/ggez development
- Sprite animation and atlas creation
- Audio editing and synthesis
- WGSL/shader programming
- Game design and balancing

### External Resources
- Archive.org for theme music and sound effects
- Press Your Luck Wikia for animation documentation
- YouTube for VHS episode references
- Game show fan communities for accuracy validation

---

## File Structure

```
to-dos/
|-- ROADMAP.md              # This file - overview
|-- RESEARCH-NOTES.md       # Research findings and sources
|-- PHASE-1-AUDIO.md        # Original audio integration
|-- PHASE-2-GRAPHICS.md     # Cell-shaded Whammy animations
|-- PHASE-3-GAMEPLAY.md     # 100% accurate rules/timing
|-- PHASE-4-PRESENTATION.md # Photorealistic immersive experience
|-- PHASE-5-POLISH.md       # Final polish and optimization
```

---

## Quick Start

1. **Read RESEARCH-NOTES.md** for background on show history and technical details
2. **Review current codebase** - focus on `src/game/mod.rs` (Whammy types) and `src/audio/mod.rs`
3. **Begin Phase 1** if focusing on audio, or **Phase 2** if focusing on graphics
4. **Track progress** using task checkboxes in each phase file
5. **Update ROADMAP.md** as phases complete with actual timelines

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 2.0.2 | Current | Base implementation with procedural audio |
| 3.0.0 | Target | Phase 1-2 complete (authentic audio + Whammy animations) |
| 4.0.0 | Target | Phase 3-4 complete (full authenticity + presentation) |
| 5.0.0 | Target | Phase 5 complete (polished release candidate) |

---

*"Big Bucks! No Whammies! STOP!"* - Every contestant, 1983-1986
