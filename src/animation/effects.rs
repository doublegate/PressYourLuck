//! # Screen Effects Module
//!
//! Provides screen shake, flash, and other global visual effects.

use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::mint::Vector2;
use ggez::Context;

// =============================================================================
// SCREEN SHAKE
// =============================================================================

/// Configuration for screen shake effect
#[derive(Debug, Clone)]
pub struct ScreenShake {
    /// Current shake intensity
    pub intensity: f32,

    /// Remaining duration
    pub duration: f32,

    /// Maximum duration
    pub max_duration: f32,

    /// Current offset
    pub offset: Vector2<f32>,

    /// Shake frequency (oscillations per second)
    pub frequency: f32,

    /// Whether shake is active
    pub active: bool,

    /// Time accumulator
    time: f32,

    /// Decay rate (how quickly shake diminishes)
    pub decay_rate: f32,
}

impl Default for ScreenShake {
    fn default() -> Self {
        Self {
            intensity: 0.0,
            duration: 0.0,
            max_duration: 0.0,
            offset: Vector2 { x: 0.0, y: 0.0 },
            frequency: 30.0,
            active: false,
            time: 0.0,
            decay_rate: 3.0,
        }
    }
}

impl ScreenShake {
    /// Create a new screen shake
    pub fn new(intensity: f32, duration: f32) -> Self {
        Self {
            intensity,
            duration,
            max_duration: duration,
            active: true,
            ..Default::default()
        }
    }

    /// Start shake with parameters
    pub fn start(&mut self, intensity: f32, duration: f32) {
        self.intensity = intensity;
        self.duration = duration;
        self.max_duration = duration;
        self.time = 0.0;
        self.active = true;
    }

    /// Stop shake immediately
    pub fn stop(&mut self) {
        self.active = false;
        self.offset = Vector2 { x: 0.0, y: 0.0 };
    }

    /// Update shake
    pub fn update(&mut self, delta_time: f32) {
        if !self.active {
            return;
        }

        self.time += delta_time;
        self.duration -= delta_time;

        if self.duration <= 0.0 {
            self.stop();
            return;
        }

        // Calculate current intensity with decay
        let progress = 1.0 - (self.duration / self.max_duration);
        let current_intensity = self.intensity * (1.0 - progress.powf(self.decay_rate));

        // Perlin-like shake using sine waves at different frequencies
        let t = self.time;
        self.offset.x = current_intensity
            * ((t * self.frequency).sin() * 0.5
                + (t * self.frequency * 1.7).sin() * 0.3
                + (t * self.frequency * 2.3).sin() * 0.2);

        self.offset.y = current_intensity
            * ((t * self.frequency * 0.9).sin() * 0.5
                + (t * self.frequency * 1.5).sin() * 0.3
                + (t * self.frequency * 2.1).sin() * 0.2);
    }

    /// Get current offset for transforming draw calls
    pub fn get_offset(&self) -> Vector2<f32> {
        self.offset
    }

    /// Check if shake is active
    pub fn is_active(&self) -> bool {
        self.active
    }
}

// =============================================================================
// SCREEN FLASH
// =============================================================================

/// Screen flash effect
#[derive(Debug, Clone)]
pub struct ScreenFlash {
    /// Flash color
    pub color: Color,

    /// Current alpha
    pub alpha: f32,

    /// Flash duration
    pub duration: f32,

    /// Remaining time
    pub remaining: f32,

    /// Whether flash is active
    pub active: bool,

    /// Flash mode
    pub mode: FlashMode,
}

/// How flash behaves over time
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlashMode {
    /// Fade out linearly
    FadeOut,

    /// Fade in then out
    FadeInOut,

    /// Flash on then immediate fade
    Flash,

    /// Pulse multiple times
    Pulse,
}

impl Default for ScreenFlash {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            alpha: 0.0,
            duration: 0.0,
            remaining: 0.0,
            active: false,
            mode: FlashMode::FadeOut,
        }
    }
}

impl ScreenFlash {
    /// Create a new flash
    pub fn new(color: Color, duration: f32) -> Self {
        Self {
            color,
            duration,
            remaining: duration,
            alpha: 1.0,
            active: true,
            mode: FlashMode::FadeOut,
        }
    }

    /// Start flash with parameters
    pub fn start(&mut self, color: Color, duration: f32, mode: FlashMode) {
        self.color = color;
        self.duration = duration;
        self.remaining = duration;
        self.active = true;
        self.mode = mode;
        self.alpha = match mode {
            FlashMode::FadeOut | FlashMode::Flash => 1.0,
            FlashMode::FadeInOut | FlashMode::Pulse => 0.0,
        };
    }

    /// Stop flash
    pub fn stop(&mut self) {
        self.active = false;
        self.alpha = 0.0;
    }

    /// Update flash
    pub fn update(&mut self, delta_time: f32) {
        if !self.active {
            return;
        }

        self.remaining -= delta_time;

        if self.remaining <= 0.0 {
            self.stop();
            return;
        }

        let progress = 1.0 - (self.remaining / self.duration);

        self.alpha = match self.mode {
            FlashMode::FadeOut => 1.0 - progress,
            FlashMode::FadeInOut => {
                if progress < 0.5 {
                    progress * 2.0
                } else {
                    (1.0 - progress) * 2.0
                }
            }
            FlashMode::Flash => {
                if progress < 0.1 {
                    1.0
                } else {
                    (1.0 - progress) / 0.9
                }
            }
            FlashMode::Pulse => {
                let cycles = 3.0;
                (progress * cycles * std::f32::consts::TAU).sin().abs()
            }
        };
    }

    /// Draw the flash overlay
    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context, screen_w: f32, screen_h: f32) {
        if !self.active || self.alpha < 0.01 {
            return;
        }

        if let Ok(rect) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, screen_w, screen_h),
            Color::new(
                self.color.r,
                self.color.g,
                self.color.b,
                self.alpha * self.color.a,
            ),
        ) {
            canvas.draw(&rect, DrawParam::default());
        }
    }

    /// Check if flash is active
    pub fn is_active(&self) -> bool {
        self.active
    }
}

// =============================================================================
// SCREEN EFFECTS MANAGER
// =============================================================================

/// Manages all screen effects
#[allow(dead_code)]
pub struct ScreenEffects {
    /// Current shake
    pub shake: ScreenShake,

    /// Current flash
    pub flash: ScreenFlash,

    /// Vignette intensity (0-1)
    pub vignette_intensity: f32,

    /// Chromatic aberration intensity
    pub chromatic_aberration: f32,

    /// Slow motion factor (1.0 = normal)
    pub slow_motion: f32,

    /// Target slow motion (for smooth transitions)
    slow_motion_target: f32,

    /// Slow motion transition speed
    slow_motion_speed: f32,
}

#[allow(dead_code)]
impl ScreenEffects {
    /// Create new screen effects manager
    pub fn new() -> Self {
        Self {
            shake: ScreenShake::default(),
            flash: ScreenFlash::default(),
            vignette_intensity: 0.0,
            chromatic_aberration: 0.0,
            slow_motion: 1.0,
            slow_motion_target: 1.0,
            slow_motion_speed: 5.0,
        }
    }

    /// Start screen shake
    pub fn shake(&mut self, intensity: f32, duration: f32) {
        self.shake.start(intensity, duration);
    }

    /// Start screen flash
    pub fn flash(&mut self, color: Color, duration: f32) {
        self.flash.start(color, duration, FlashMode::FadeOut);
    }

    /// Start screen flash with mode
    pub fn flash_with_mode(&mut self, color: Color, duration: f32, mode: FlashMode) {
        self.flash.start(color, duration, mode);
    }

    /// Quick white flash (for impacts)
    pub fn impact_flash(&mut self) {
        self.flash.start(Color::WHITE, 0.1, FlashMode::Flash);
    }

    /// Red damage flash
    pub fn damage_flash(&mut self) {
        self.flash
            .start(Color::new(1.0, 0.0, 0.0, 0.5), 0.3, FlashMode::FadeOut);
    }

    /// Whammy flash (red with shake)
    pub fn whammy_effect(&mut self) {
        self.shake.start(15.0, 0.5);
        self.flash
            .start(Color::new(0.8, 0.1, 0.1, 0.6), 0.4, FlashMode::FadeInOut);
    }

    /// Win celebration effect
    pub fn celebration_effect(&mut self) {
        self.flash
            .start(Color::new(1.0, 0.84, 0.0, 0.4), 0.5, FlashMode::Pulse);
    }

    /// Set slow motion
    pub fn set_slow_motion(&mut self, factor: f32) {
        self.slow_motion_target = factor.clamp(0.1, 2.0);
    }

    /// Reset slow motion to normal
    pub fn reset_slow_motion(&mut self) {
        self.slow_motion_target = 1.0;
    }

    /// Update all effects
    pub fn update(&mut self, delta_time: f32) {
        self.shake.update(delta_time);
        self.flash.update(delta_time);

        // Update slow motion
        if (self.slow_motion - self.slow_motion_target).abs() > 0.01 {
            let dir = if self.slow_motion_target > self.slow_motion {
                1.0
            } else {
                -1.0
            };
            self.slow_motion += dir * self.slow_motion_speed * delta_time;
            self.slow_motion = self.slow_motion.clamp(0.1, 2.0);
        } else {
            self.slow_motion = self.slow_motion_target;
        }
    }

    /// Draw all screen effects
    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context, screen_w: f32, screen_h: f32) {
        self.flash.draw(canvas, ctx, screen_w, screen_h);
        self.draw_vignette(canvas, ctx, screen_w, screen_h);
    }

    /// Draw vignette effect
    fn draw_vignette(&self, canvas: &mut Canvas, ctx: &mut Context, screen_w: f32, screen_h: f32) {
        if self.vignette_intensity < 0.01 {
            return;
        }

        // Draw four gradient rectangles around edges
        let edge_width = screen_w * 0.15 * self.vignette_intensity;
        let alpha = 0.7 * self.vignette_intensity;

        // Top edge
        if let Ok(rect) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, screen_w, edge_width),
            Color::new(0.0, 0.0, 0.0, alpha),
        ) {
            canvas.draw(&rect, DrawParam::default());
        }

        // Bottom edge
        if let Ok(rect) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, screen_h - edge_width, screen_w, edge_width),
            Color::new(0.0, 0.0, 0.0, alpha),
        ) {
            canvas.draw(&rect, DrawParam::default());
        }

        // Left edge
        if let Ok(rect) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, edge_width, screen_h),
            Color::new(0.0, 0.0, 0.0, alpha),
        ) {
            canvas.draw(&rect, DrawParam::default());
        }

        // Right edge
        if let Ok(rect) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(screen_w - edge_width, 0.0, edge_width, screen_h),
            Color::new(0.0, 0.0, 0.0, alpha),
        ) {
            canvas.draw(&rect, DrawParam::default());
        }
    }

    /// Get draw offset from shake
    pub fn get_shake_offset(&self) -> Vector2<f32> {
        self.shake.get_offset()
    }

    /// Get time scale from slow motion
    pub fn get_time_scale(&self) -> f32 {
        self.slow_motion
    }

    /// Check if any effects are active
    pub fn has_active_effects(&self) -> bool {
        self.shake.is_active() || self.flash.is_active()
    }

    /// Clear all effects
    pub fn clear(&mut self) {
        self.shake.stop();
        self.flash.stop();
        self.vignette_intensity = 0.0;
        self.chromatic_aberration = 0.0;
        self.slow_motion = 1.0;
        self.slow_motion_target = 1.0;
    }
}

impl Default for ScreenEffects {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_shake() {
        let mut shake = ScreenShake::new(10.0, 1.0);

        assert!(shake.is_active());

        shake.update(0.5);
        assert!(shake.is_active());
        assert!(shake.offset.x.abs() > 0.0 || shake.offset.y.abs() > 0.0);

        shake.update(0.6);
        assert!(!shake.is_active());
    }

    #[test]
    fn test_screen_flash() {
        let mut flash = ScreenFlash::new(Color::WHITE, 1.0);

        assert!(flash.is_active());
        assert!((flash.alpha - 1.0).abs() < 0.01);

        flash.update(0.5);
        assert!(flash.is_active());
        assert!(flash.alpha < 1.0);

        flash.update(0.6);
        assert!(!flash.is_active());
    }

    #[test]
    fn test_screen_effects_manager() {
        let mut effects = ScreenEffects::new();

        effects.shake(10.0, 1.0);
        effects.flash(Color::WHITE, 0.5);

        assert!(effects.has_active_effects());

        effects.update(2.0);
        assert!(!effects.has_active_effects());
    }
}
