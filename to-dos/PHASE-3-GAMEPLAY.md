# Phase 3: 100% Accurate Gameplay

## Phase Overview

**Goal**: Implement perfectly authentic gameplay mechanics matching the original 1983-1986 CBS Press Your Luck, including exact board patterns, timing, rules, and prize distributions.

**Duration**: 4-6 weeks (2-3 sprints)
**Priority**: HIGH
**Dependencies**: Partial Phase 1 (audio timing), Phase 2 Whammy system

### Success Criteria
- [ ] Board patterns match documented show mechanics
- [ ] All special squares function per original rules
- [ ] Spin timing matches authentic feel (not too fast, not too slow)
- [ ] Prize values and distributions historically accurate
- [ ] Passing/earned spin mechanics identical to show

---

## Sprint 1: Board Pattern Authenticity (Week 1-2)

### Objectives
- Implement historically accurate board patterns
- Validate pattern behavior against Michael Larson research
- Add post-Larson (32-pattern) system

### Tasks

#### 1.1 Board Pattern System Overhaul
**Complexity**: High | **Estimate**: 3-4 days

- [ ] **Task**: Research original 5-pattern system (pre-Larson)
  - Document the 5 looping patterns
  - Map squares 4 and 8 (no Whammy, always +Spin)
  - Identify pattern trigger conditions

- [ ] **Task**: Implement pattern data structure
  ```rust
  pub struct BoardPattern {
      pub id: usize,
      pub sequence: Vec<PatternStep>,
      pub entry_points: Vec<usize>,  // Valid starting squares
  }

  pub struct PatternStep {
      pub square: usize,
      pub prizes: [usize; 3],  // Prize indices cycling
      pub duration_ms: u32,    // How long before next step
  }

  pub struct BoardPatternManager {
      patterns: Vec<BoardPattern>,
      current_pattern: usize,
      step_in_pattern: usize,
      use_larson_era: bool,  // Toggle for historical accuracy mode
  }
  ```

- [ ] **Task**: Implement 32-pattern system (post-Larson)
  - More randomized Whammy distribution
  - No guaranteed safe squares
  - Implemented August 1984

- [ ] **Task**: Add pattern selection mode
  - "Classic" mode: 5 patterns (vulnerable to Larson strategy)
  - "Standard" mode: 32 patterns (post-August 1984)
  - "Random" mode: True random for casual play

#### 1.2 Square Prize Assignment
**Complexity**: Medium | **Estimate**: 2-3 days

- [ ] **Task**: Document authentic prize distributions
  - Round 1 cash values: $100-$1,250
  - Round 2 cash values: $500-$5,000+
  - Special square positions
  - Whammy frequency per round

- [ ] **Task**: Implement prize cycling per square
  - Each square cycles through 3 prizes
  - Cycle timing: ~0.2 seconds per prize
  - Independent cycling (not synchronized)

- [ ] **Task**: Square-specific rules
  | Square | Round 1 | Round 2 | Notes |
  |--------|---------|---------|-------|
  | 4 | Cash +Spin | Cash +Spin | NEVER Whammy (pre-Larson) |
  | 6 | Various | Pick a Corner | Always in Round 2 |
  | 8 | Cash +Spin | Cash +Spin | NEVER Whammy (pre-Larson) |
  | 12 | Big Bucks | Big Bucks | Awards Square 4 value |

#### 1.3 Board Timing Calibration
**Complexity**: Medium | **Estimate**: 2 days

- [ ] **Task**: Analyze VHS recordings for timing
  - Light movement speed
  - Stop delay (button press to actual stop)
  - Prize reveal timing

- [ ] **Task**: Implement variable spin speed
  - Initial speed: Fast
  - Deceleration curve: Gradual
  - Final stop: Snap to center of square

- [ ] **Task**: Add "spin feel" tuning
  - Minimum spin duration (prevent instant stops)
  - Maximum spin duration (prevent endless spins)
  - "Last second" suspense timing

### Acceptance Criteria - Sprint 1
- [ ] Board patterns match historical documentation
- [ ] Larson-era patterns are exploitable (as intended)
- [ ] Post-Larson patterns are fair
- [ ] Timing feels authentic to show

---

## Sprint 2: Special Squares & Rules (Week 3-4)

### Objectives
- Implement all special square mechanics
- Validate against show rules
- Handle edge cases correctly

### Tasks

#### 2.1 Special Square: Add-A-One
**Complexity**: Medium | **Estimate**: 1-2 days

- [ ] **Task**: Implement Add-A-One logic
  - $0 becomes $10 (authentic behavior)
  - $500 becomes $1,500
  - $1,000 becomes $11,000
  - $10,000 becomes $110,000

- [ ] **Task**: Verify edge cases
  - Minimum: $0 -> $10
  - Maximum score handling
  - Display animation for digit insertion

- [ ] **Task**: Audio/visual feedback
  - Special sound effect
  - Score "morphing" animation

#### 2.2 Special Square: Pick a Corner
**Complexity**: Medium | **Estimate**: 1-2 days

- [ ] **Task**: Implement corner selection UI
  - Highlight corners (squares 0, 5, 9, 14)
  - Player input: 1, 2, 3, 4 keys
  - Timeout handling (auto-select?)

- [ ] **Task**: Corner prize assignment
  - Each corner has cycling prizes
  - Selection reveals that corner's current prize
  - Prize can be Whammy!

- [ ] **Task**: Add selection suspense
  - Brief pause before reveal
  - Corner highlight animation

#### 2.3 Special Square: Double Your Money
**Complexity**: Low | **Estimate**: 0.5-1 day

- [ ] **Task**: Implement doubling logic
  - Score * 2
  - Plus one free spin (added later in show history)

- [ ] **Task**: Handle $0 case
  - Originally: Nothing happens (poor design)
  - Later: Changed to include +1 spin (always valuable)

- [ ] **Task**: Display animation
  - Score "doubles" visually
  - Celebration effect

#### 2.4 Special Square: Big Bucks
**Complexity**: Medium | **Estimate**: 1 day

- [ ] **Task**: Implement Big Bucks transfer
  - Awards value currently showing in square #4
  - Round 1: $1,000-$1,250
  - Round 2: $5,000+

- [ ] **Task**: Position: Always square #12

- [ ] **Task**: Visual connection
  - Arrow/line from square 12 to square 4
  - Highlight square 4 value

#### 2.5 Special Square: Move One Space / Advance Two / Back Two
**Complexity**: Medium | **Estimate**: 1-2 days

- [ ] **Task**: Implement movement mechanics
  - Move One: In current pattern direction
  - Advance Two: Skip to 2 squares ahead
  - Back Two: Move 2 squares backward

- [ ] **Task**: Determine "current direction"
  - Clockwise is default
  - Track last movement direction

- [ ] **Task**: Visual transition
  - Animated movement between squares
  - Land on new square, reveal prize

#### 2.6 Special Square: $2000 or Lose a Whammy
**Complexity**: Medium | **Estimate**: 1 day

- [ ] **Task**: Implement choice UI
  - Two options displayed
  - Player selects 1 or 2

- [ ] **Task**: Whammy removal logic
  - Only valid if player has 1+ Whammies
  - Cannot go below 0 Whammies
  - If 0 Whammies, $2000 auto-selected?

- [ ] **Task**: Strategic consideration
  - More valuable at 3 Whammies (prevents elimination)
  - $2000 better at 0-1 Whammies

#### 2.7 Special Square: Take the Lead
**Complexity**: Medium | **Estimate**: 1 day

- [ ] **Task**: Calculate lead amount
  - Find current leader's score
  - Set player score to leader + $1
  - Plus one spin

- [ ] **Task**: Edge cases
  - Already in lead: Still get +$1 and spin
  - Tied: Beat tie by $1

### Acceptance Criteria - Sprint 2
- [ ] All 8 special square types functional
- [ ] Edge cases handled correctly
- [ ] Visual feedback for each special
- [ ] Audio cues match action

---

## Sprint 3: Spin Mechanics & Turn Flow (Week 5-6)

### Objectives
- Perfect the spin/pass/earned mechanics
- Implement authentic turn order
- Handle elimination correctly

### Tasks

#### 3.1 Spin Type System
**Complexity**: Medium | **Estimate**: 2 days

- [ ] **Task**: Distinguish spin types
  ```rust
  pub struct SpinInventory {
      pub earned_spins: u32,    // Won through questions/board
      pub passed_spins: u32,    // Received from opponents
  }

  impl SpinInventory {
      pub fn use_spin(&mut self) {
          // Passed spins MUST be used first
          if self.passed_spins > 0 {
              self.passed_spins -= 1;
          } else {
              self.earned_spins -= 1;
          }
      }

      pub fn can_pass(&self) -> bool {
          // Can only pass EARNED spins
          self.earned_spins > 0
      }
  }
  ```

- [ ] **Task**: Display spin breakdown
  - Show "3 Earned + 2 Passed = 5 Total"
  - Color coding: Earned (green), Passed (yellow)

- [ ] **Task**: Passing rules enforcement
  - Can only pass earned spins
  - Must pass to specific player (not choice)

#### 3.2 Passing Target Rules
**Complexity**: Medium | **Estimate**: 2 days

- [ ] **Task**: Implement passing target selection
  - Pass to the leader (highest score)
  - If you're leading: Pass to 2nd place
  - If tied for lead: Pass to tied opponent

- [ ] **Task**: Document all passing scenarios
  | Your Position | Target |
  |--------------|--------|
  | 3rd place | 1st place (leader) |
  | 2nd place | 1st place (leader) |
  | 1st place | 2nd place |
  | Tied 1st (2-way) | The other tied player |
  | Tied 1st (3-way) | 2nd in original turn order |

- [ ] **Task**: Handle edge cases
  - All players tied
  - Two players tied for 2nd
  - Player eliminated mid-round

#### 3.3 Whammy Hit Processing
**Complexity**: Medium | **Estimate**: 2 days

- [ ] **Task**: Whammy impact sequence
  1. Score resets to $0
  2. Whammy count increments
  3. Animation plays
  4. Convert passed spins to earned (if any remaining)

- [ ] **Task**: Fourth Whammy elimination
  - Special Whammy-out animation
  - Player removed from game
  - Remaining spins forfeited
  - Turn passes to next player

- [ ] **Task**: Passed spin conversion
  ```rust
  fn process_whammy_hit(&mut self, player: usize) {
      let contestant = &mut self.contestants[player];
      contestant.score = 0;
      contestant.whammies += 1;

      // IMPORTANT: Passed spins become earned after Whammy
      contestant.earned_spins += contestant.passed_spins;
      contestant.passed_spins = 0;

      if contestant.whammies >= 4 {
          contestant.eliminated = true;
      }
  }
  ```

#### 3.4 Turn Order Management
**Complexity**: Medium | **Estimate**: 1-2 days

- [ ] **Task**: Implement turn cycle
  - Round 1: Players 1 -> 2 -> 3 (fixed order)
  - Round 2: Lowest score first
  - Within turn: Spin until pass or out of spins

- [ ] **Task**: Handle mid-turn changes
  - Spin earned mid-turn (can continue)
  - Passed spins received mid-turn
  - Elimination mid-turn

- [ ] **Task**: Round transition
  - Question round -> Board round
  - Round 1 -> Round 2
  - Final scores -> Winner announcement

#### 3.5 Game State Validation
**Complexity**: Low | **Estimate**: 1 day

- [ ] **Task**: Add assertion/validation checks
  - No negative spins
  - No negative score
  - Whammy count 0-4
  - Valid player indices

- [ ] **Task**: Save/load game state
  - Serialize game state to file
  - Resume interrupted game
  - Debug state inspection

### Acceptance Criteria - Sprint 3
- [ ] Spin types correctly tracked
- [ ] Passing follows exact show rules
- [ ] Whammy processing authentic
- [ ] Turn order correct for both rounds
- [ ] Game state always valid

---

## Technical Requirements

### Gameplay Constants
```rust
// Authentic show values
pub const ROUND_1_QUESTIONS: usize = 4;
pub const ROUND_2_QUESTIONS: usize = 4;
pub const SPINS_PER_CORRECT: u32 = 3;  // Regular answer
pub const SPINS_PER_BUZZ: u32 = 3;     // Buzz-in answer
pub const MAX_WHAMMIES: u32 = 4;

// Prize value ranges (authentic)
pub const ROUND_1_CASH_MIN: u32 = 100;
pub const ROUND_1_CASH_MAX: u32 = 1250;
pub const ROUND_2_CASH_MIN: u32 = 500;
pub const ROUND_2_CASH_MAX: u32 = 5000;

// Timing (in seconds)
pub const SPIN_MIN_DURATION: f32 = 1.5;
pub const SPIN_MAX_DURATION: f32 = 15.0;
pub const PRIZE_CYCLE_RATE: f32 = 0.2;
pub const BUTTON_STOP_DELAY: f32 = 0.1;
```

### Pattern Data Format
```rust
// Pattern definition format
pub struct PatternDefinition {
    pub version: String,  // "pre_larson" or "post_larson"
    pub patterns: Vec<PatternSequence>,
}

pub struct PatternSequence {
    pub id: usize,
    pub steps: Vec<(usize, u32)>,  // (square_index, duration_ms)
}

// Example pre-Larson pattern (simplified)
// Pattern 1: 0 -> 1 -> 2 -> ... -> 17 -> 0 (clockwise)
// Pattern 2: 4 -> 8 -> 12 -> 16 -> ... (skip pattern)
```

---

## Research Findings

### Michael Larson Strategy Details
- **Discovery**: Frame-by-frame VCR analysis
- **Duration**: 6 months of study
- **Key Finding**: Only 5 patterns, predictable
- **Safe Squares**: 4 and 8 (never Whammy in Round 2)
- **Execution**: Started pattern tracking at spin 16
- **Result**: 29 consecutive successful spins

### Post-Larson Changes
- **Implementation Date**: August 1984
- **Pattern Count**: Increased to 32
- **Whammy Distribution**: More even
- **Safe Squares**: Removed guarantee
- **Contestant Notice**: None publicly announced

### Authentic Prize Values
| Square Type | Round 1 | Round 2 |
|-------------|---------|---------|
| Low Cash | $100-$300 | $500-$1,000 |
| Mid Cash | $300-$750 | $1,000-$2,500 |
| High Cash | $750-$1,250 | $2,500-$5,000 |
| Big Bucks | $1,000-$1,250 | $5,000+ |
| +Spin | +$0-$500 | +$0-$1,000 |

### Whammy Frequency
- Target: ~25% of board at any time
- Never on squares 4 and 8 (pre-Larson)
- At least one always visible
- Multiple can be visible simultaneously

---

## Implementation Strategy

### Validation Approach
1. **Document Reference**: Press Your Luck Wikia rules
2. **VHS Verification**: Compare against recordings
3. **Fan Community**: Validate with show historians
4. **Play Testing**: "Does it feel right?"

### Order of Implementation
1. Board patterns (foundation)
2. Basic spin mechanics
3. Special squares
4. Passing rules
5. Edge cases and polish

---

## Estimated Effort

| Sprint | Focus | Story Points | Hours |
|--------|-------|--------------|-------|
| Sprint 1 | Board Patterns | 21 | 30-40 |
| Sprint 2 | Special Squares | 26 | 35-45 |
| Sprint 3 | Spin Mechanics | 21 | 30-40 |
| **Total** | | **68** | **95-125** |

---

## Definition of Done

Phase 3 is complete when:
- [ ] Board patterns match historical systems
- [ ] All 8+ special squares functional
- [ ] Spin/pass/earned mechanics authentic
- [ ] Turn order correct for all scenarios
- [ ] Whammy processing accurate
- [ ] Game state always valid
- [ ] Prize values historically accurate
- [ ] Documentation complete
- [ ] Validated against show recordings

---

*"I kept landing on the same two squares over and over."* - Michael Larson, 1984
