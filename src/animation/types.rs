//! # Animation Type Definitions
//!
//! Core types for the animation system including frames, sequences, and loop modes.

use ggez::graphics::Rect;
use ggez::mint::Vector2;

// =============================================================================
// ANIMATION FRAME
// =============================================================================

/// A single frame of animation with sprite data and timing
#[derive(Debug, Clone)]
pub struct AnimationFrame {
    /// Source rectangle in the sprite atlas (UV coordinates)
    pub sprite_rect: Rect,

    /// Duration this frame is displayed (seconds)
    pub duration: f32,

    /// Position offset from animation origin
    pub offset: Vector2<f32>,

    /// Scale multiplier for this frame
    pub scale: f32,

    /// Rotation in radians
    pub rotation: f32,

    /// Optional audio trigger when this frame starts
    pub audio_trigger: Option<String>,

    /// Optional particle effect trigger
    pub particle_trigger: Option<String>,
}

impl Default for AnimationFrame {
    fn default() -> Self {
        Self {
            sprite_rect: Rect::new(0.0, 0.0, 1.0, 1.0),
            duration: 1.0 / 24.0, // 24 FPS default
            offset: Vector2 { x: 0.0, y: 0.0 },
            scale: 1.0,
            rotation: 0.0,
            audio_trigger: None,
            particle_trigger: None,
        }
    }
}

impl AnimationFrame {
    /// Create a new animation frame from sprite rect
    pub fn new(sprite_rect: Rect, duration: f32) -> Self {
        Self {
            sprite_rect,
            duration,
            ..Default::default()
        }
    }

    /// Create frame with offset
    pub fn with_offset(mut self, x: f32, y: f32) -> Self {
        self.offset = Vector2 { x, y };
        self
    }

    /// Create frame with scale
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    /// Create frame with rotation
    pub fn with_rotation(mut self, radians: f32) -> Self {
        self.rotation = radians;
        self
    }

    /// Create frame with audio trigger
    pub fn with_audio(mut self, trigger: &str) -> Self {
        self.audio_trigger = Some(trigger.to_string());
        self
    }

    /// Create frame with particle trigger
    pub fn with_particles(mut self, trigger: &str) -> Self {
        self.particle_trigger = Some(trigger.to_string());
        self
    }
}

// =============================================================================
// LOOP MODE
// =============================================================================

/// How an animation repeats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopMode {
    /// Play once and stop on last frame
    Once,

    /// Loop continuously from start
    Loop,

    /// Alternate forward and backward
    PingPong,

    /// Play once and hide when complete
    OnceAndHide,
}

impl Default for LoopMode {
    fn default() -> Self {
        Self::Once
    }
}

// =============================================================================
// ANIMATION
// =============================================================================

/// A complete animation sequence
#[derive(Debug, Clone)]
pub struct Animation {
    /// Unique animation identifier
    pub name: String,

    /// Human-readable display name
    pub display_name: String,

    /// All frames in sequence
    pub frames: Vec<AnimationFrame>,

    /// Total animation duration (calculated from frames)
    pub total_duration: f32,

    /// How the animation loops
    pub loop_mode: LoopMode,

    /// Priority for animation blending (higher = more important)
    pub priority: u8,

    /// Whether this animation can be interrupted
    pub interruptible: bool,

    /// Optional screen shake on start
    pub screen_shake: Option<(f32, f32)>, // (intensity, duration)

    /// Optional screen flash on start
    pub screen_flash: Option<(f32, f32, f32, f32)>, // (r, g, b, duration)

    /// Atlas ID this animation uses
    pub atlas_id: String,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            name: String::new(),
            display_name: String::new(),
            frames: Vec::new(),
            total_duration: 0.0,
            loop_mode: LoopMode::Once,
            priority: 0,
            interruptible: true,
            screen_shake: None,
            screen_flash: None,
            atlas_id: String::from("default"),
        }
    }
}

#[allow(dead_code)]
impl Animation {
    /// Create a new animation with name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            display_name: name.replace('_', " ").to_uppercase(),
            ..Default::default()
        }
    }

    /// Add a frame to the animation
    pub fn add_frame(&mut self, frame: AnimationFrame) {
        self.total_duration += frame.duration;
        self.frames.push(frame);
    }

    /// Set loop mode
    pub fn with_loop_mode(mut self, mode: LoopMode) -> Self {
        self.loop_mode = mode;
        self
    }

    /// Set priority
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Set interruptible flag
    pub fn with_interruptible(mut self, interruptible: bool) -> Self {
        self.interruptible = interruptible;
        self
    }

    /// Add screen shake effect
    pub fn with_screen_shake(mut self, intensity: f32, duration: f32) -> Self {
        self.screen_shake = Some((intensity, duration));
        self
    }

    /// Add screen flash effect
    pub fn with_screen_flash(mut self, r: f32, g: f32, b: f32, duration: f32) -> Self {
        self.screen_flash = Some((r, g, b, duration));
        self
    }

    /// Set atlas ID
    pub fn with_atlas(mut self, atlas_id: &str) -> Self {
        self.atlas_id = atlas_id.to_string();
        self
    }

    /// Calculate total duration from frames
    pub fn recalculate_duration(&mut self) {
        self.total_duration = self.frames.iter().map(|f| f.duration).sum();
    }

    /// Get frame at given time
    pub fn get_frame_at_time(&self, time: f32) -> Option<&AnimationFrame> {
        if self.frames.is_empty() {
            return None;
        }

        let time = match self.loop_mode {
            LoopMode::Once | LoopMode::OnceAndHide => time.min(self.total_duration),
            LoopMode::Loop => time % self.total_duration,
            LoopMode::PingPong => {
                let cycle = time % (self.total_duration * 2.0);
                if cycle > self.total_duration {
                    self.total_duration * 2.0 - cycle
                } else {
                    cycle
                }
            }
        };

        let mut elapsed = 0.0;
        for frame in &self.frames {
            elapsed += frame.duration;
            if time < elapsed {
                return Some(frame);
            }
        }

        self.frames.last()
    }

    /// Get current frame index at given time
    pub fn get_frame_index_at_time(&self, time: f32) -> usize {
        if self.frames.is_empty() {
            return 0;
        }

        let time = match self.loop_mode {
            LoopMode::Once | LoopMode::OnceAndHide => time.min(self.total_duration),
            LoopMode::Loop => time % self.total_duration,
            LoopMode::PingPong => {
                let cycle = time % (self.total_duration * 2.0);
                if cycle > self.total_duration {
                    self.total_duration * 2.0 - cycle
                } else {
                    cycle
                }
            }
        };

        let mut elapsed = 0.0;
        for (i, frame) in self.frames.iter().enumerate() {
            elapsed += frame.duration;
            if time < elapsed {
                return i;
            }
        }

        self.frames.len() - 1
    }

    /// Check if animation is complete at given time
    pub fn is_complete(&self, time: f32) -> bool {
        match self.loop_mode {
            LoopMode::Once | LoopMode::OnceAndHide => time >= self.total_duration,
            LoopMode::Loop | LoopMode::PingPong => false,
        }
    }

    /// Create frames from a grid-based sprite sheet
    pub fn from_grid(
        name: &str,
        cols: u32,
        rows: u32,
        frame_count: u32,
        frame_duration: f32,
    ) -> Self {
        let mut anim = Self::new(name);
        let frame_w = 1.0 / cols as f32;
        let frame_h = 1.0 / rows as f32;

        for i in 0..frame_count {
            let col = i % cols;
            let row = i / cols;
            let rect = Rect::new(
                col as f32 * frame_w,
                row as f32 * frame_h,
                frame_w,
                frame_h,
            );
            anim.add_frame(AnimationFrame::new(rect, frame_duration));
        }

        anim
    }
}

// =============================================================================
// ANIMATION DEFINITION BUILDER
// =============================================================================

/// Builder for creating animations with fluent API
pub struct AnimationBuilder {
    animation: Animation,
    current_time: f32,
}

impl AnimationBuilder {
    /// Create a new animation builder
    pub fn new(name: &str) -> Self {
        Self {
            animation: Animation::new(name),
            current_time: 0.0,
        }
    }

    /// Set display name
    pub fn display_name(mut self, name: &str) -> Self {
        self.animation.display_name = name.to_string();
        self
    }

    /// Set atlas
    pub fn atlas(mut self, atlas_id: &str) -> Self {
        self.animation.atlas_id = atlas_id.to_string();
        self
    }

    /// Set loop mode
    pub fn loop_mode(mut self, mode: LoopMode) -> Self {
        self.animation.loop_mode = mode;
        self
    }

    /// Set priority
    pub fn priority(mut self, priority: u8) -> Self {
        self.animation.priority = priority;
        self
    }

    /// Set interruptible
    pub fn interruptible(mut self, value: bool) -> Self {
        self.animation.interruptible = value;
        self
    }

    /// Add screen shake
    pub fn screen_shake(mut self, intensity: f32, duration: f32) -> Self {
        self.animation.screen_shake = Some((intensity, duration));
        self
    }

    /// Add screen flash
    pub fn screen_flash(mut self, r: f32, g: f32, b: f32, duration: f32) -> Self {
        self.animation.screen_flash = Some((r, g, b, duration));
        self
    }

    /// Add a frame from grid position
    pub fn frame_grid(
        mut self,
        col: u32,
        row: u32,
        cols: u32,
        rows: u32,
        duration: f32,
    ) -> Self {
        let frame_w = 1.0 / cols as f32;
        let frame_h = 1.0 / rows as f32;
        let rect = Rect::new(
            col as f32 * frame_w,
            row as f32 * frame_h,
            frame_w,
            frame_h,
        );
        self.animation.add_frame(AnimationFrame::new(rect, duration));
        self.current_time += duration;
        self
    }

    /// Add frames from grid range
    pub fn frames_grid(
        mut self,
        start_frame: u32,
        count: u32,
        cols: u32,
        rows: u32,
        duration_per_frame: f32,
    ) -> Self {
        let frame_w = 1.0 / cols as f32;
        let frame_h = 1.0 / rows as f32;

        for i in 0..count {
            let frame_idx = start_frame + i;
            let col = frame_idx % cols;
            let row = frame_idx / cols;
            let rect = Rect::new(
                col as f32 * frame_w,
                row as f32 * frame_h,
                frame_w,
                frame_h,
            );
            self.animation.add_frame(AnimationFrame::new(rect, duration_per_frame));
            self.current_time += duration_per_frame;
        }
        self
    }

    /// Add a custom frame
    pub fn frame(mut self, frame: AnimationFrame) -> Self {
        self.current_time += frame.duration;
        self.animation.add_frame(frame);
        self
    }

    /// Build the animation
    pub fn build(self) -> Animation {
        self.animation
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_frame_default() {
        let frame = AnimationFrame::default();
        assert!((frame.duration - 1.0 / 24.0).abs() < 0.0001);
        assert_eq!(frame.scale, 1.0);
    }

    #[test]
    fn test_animation_from_grid() {
        let anim = Animation::from_grid("test", 4, 4, 16, 1.0 / 24.0);
        assert_eq!(anim.frames.len(), 16);
        assert!((anim.total_duration - 16.0 / 24.0).abs() < 0.0001);
    }

    #[test]
    fn test_animation_get_frame() {
        let anim = Animation::from_grid("test", 4, 1, 4, 0.25);

        let frame0 = anim.get_frame_at_time(0.0).unwrap();
        assert!((frame0.sprite_rect.x - 0.0).abs() < 0.0001);

        let frame2 = anim.get_frame_at_time(0.5).unwrap();
        assert!((frame2.sprite_rect.x - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_animation_loop_mode() {
        let mut anim = Animation::from_grid("test", 4, 1, 4, 0.25);
        anim.loop_mode = LoopMode::Loop;

        // At time 1.0, should wrap to 0.0
        let frame = anim.get_frame_at_time(1.0).unwrap();
        assert!((frame.sprite_rect.x - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_animation_builder() {
        let anim = AnimationBuilder::new("test_anim")
            .display_name("Test Animation")
            .loop_mode(LoopMode::PingPong)
            .priority(5)
            .frames_grid(0, 8, 8, 1, 1.0 / 24.0)
            .build();

        assert_eq!(anim.name, "test_anim");
        assert_eq!(anim.display_name, "Test Animation");
        assert_eq!(anim.loop_mode, LoopMode::PingPong);
        assert_eq!(anim.priority, 5);
        assert_eq!(anim.frames.len(), 8);
    }
}
