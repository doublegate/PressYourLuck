//! # Animation Player Module
//!
//! Plays animations with state machine support and blending capabilities.

use ggez::graphics::{Canvas, Color, DrawParam, Rect};
use ggez::mint::Vector2;
use std::collections::HashMap;

use super::atlas::SpriteAtlas;
use super::types::{Animation, AnimationFrame, LoopMode};

// =============================================================================
// PLAYBACK STATE
// =============================================================================

/// Current state of animation playback
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlaybackState {
    /// Not playing
    #[default]
    Stopped,

    /// Currently playing
    Playing,

    /// Paused at current frame
    Paused,

    /// Completed (for non-looping animations)
    Completed,
}

// =============================================================================
// ANIMATION STATE
// =============================================================================

/// State for a single playing animation
#[derive(Debug, Clone)]
pub struct AnimationState {
    /// Current animation name
    pub animation_name: String,

    /// Current playback time
    pub time: f32,

    /// Playback speed multiplier (1.0 = normal)
    pub speed: f32,

    /// Current playback state
    pub state: PlaybackState,

    /// Current frame index
    pub current_frame: usize,

    /// Previous frame index (for event detection)
    pub previous_frame: usize,

    /// Blend weight for transitions (0.0-1.0)
    pub blend_weight: f32,

    /// Whether to reverse playback
    pub reversed: bool,

    /// Events triggered this update
    pub triggered_events: Vec<AnimationEvent>,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            animation_name: String::new(),
            time: 0.0,
            speed: 1.0,
            state: PlaybackState::Stopped,
            current_frame: 0,
            previous_frame: 0,
            blend_weight: 1.0,
            reversed: false,
            triggered_events: Vec::new(),
        }
    }
}

impl AnimationState {
    /// Create new animation state
    pub fn new(animation_name: &str) -> Self {
        Self {
            animation_name: animation_name.to_string(),
            ..Default::default()
        }
    }

    /// Start playing
    pub fn play(&mut self) {
        self.state = PlaybackState::Playing;
    }

    /// Pause playback
    pub fn pause(&mut self) {
        if self.state == PlaybackState::Playing {
            self.state = PlaybackState::Paused;
        }
    }

    /// Resume from pause
    pub fn resume(&mut self) {
        if self.state == PlaybackState::Paused {
            self.state = PlaybackState::Playing;
        }
    }

    /// Stop and reset
    pub fn stop(&mut self) {
        self.state = PlaybackState::Stopped;
        self.time = 0.0;
        self.current_frame = 0;
        self.previous_frame = 0;
    }

    /// Restart from beginning
    pub fn restart(&mut self) {
        self.time = 0.0;
        self.current_frame = 0;
        self.previous_frame = 0;
        self.state = PlaybackState::Playing;
    }

    /// Set playback speed
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.max(0.0);
    }

    /// Is currently playing
    pub fn is_playing(&self) -> bool {
        self.state == PlaybackState::Playing
    }

    /// Is completed
    pub fn is_completed(&self) -> bool {
        self.state == PlaybackState::Completed
    }
}

// =============================================================================
// ANIMATION EVENT
// =============================================================================

/// Events triggered during animation playback
#[derive(Debug, Clone)]
pub enum AnimationEvent {
    /// Audio trigger
    Audio(String),

    /// Particle effect trigger
    Particle(String),

    /// Animation started
    Started(String),

    /// Animation completed
    Completed(String),

    /// Animation looped
    Looped(String),

    /// Frame changed
    FrameChanged(usize),
}

// =============================================================================
// ANIMATION PLAYER
// =============================================================================

/// Plays and manages animation state
pub struct AnimationPlayer {
    /// Available animations
    animations: HashMap<String, Animation>,

    /// Current animation state
    state: AnimationState,

    /// Previous animation (for blending)
    previous_state: Option<AnimationState>,

    /// Blend duration
    blend_duration: f32,

    /// Current blend time
    blend_time: f32,

    /// Position on screen
    pub position: Vector2<f32>,

    /// Base scale
    pub scale: f32,

    /// Rotation in radians
    pub rotation: f32,

    /// Tint color
    pub color: Color,

    /// Origin point (0-1 normalized)
    pub origin: Vector2<f32>,

    /// Whether player is visible
    pub visible: bool,

    /// Flip horizontally
    pub flip_x: bool,

    /// Flip vertically
    pub flip_y: bool,
}

#[allow(dead_code)]
impl AnimationPlayer {
    /// Create a new animation player
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            state: AnimationState::default(),
            previous_state: None,
            blend_duration: 0.0,
            blend_time: 0.0,
            position: Vector2 { x: 0.0, y: 0.0 },
            scale: 1.0,
            rotation: 0.0,
            color: Color::WHITE,
            origin: Vector2 { x: 0.5, y: 0.5 }, // Center origin
            visible: true,
            flip_x: false,
            flip_y: false,
        }
    }

    /// Add an animation
    pub fn add_animation(&mut self, animation: Animation) {
        self.animations.insert(animation.name.clone(), animation);
    }

    /// Remove an animation
    pub fn remove_animation(&mut self, name: &str) -> Option<Animation> {
        self.animations.remove(name)
    }

    /// Get animation by name
    pub fn get_animation(&self, name: &str) -> Option<&Animation> {
        self.animations.get(name)
    }

    /// List all animation names
    pub fn animation_names(&self) -> Vec<&str> {
        self.animations.keys().map(|s| s.as_str()).collect()
    }

    /// Play animation by name
    pub fn play(&mut self, name: &str) -> bool {
        if !self.animations.contains_key(name) {
            return false;
        }

        self.state = AnimationState::new(name);
        self.state.play();
        self.state
            .triggered_events
            .push(AnimationEvent::Started(name.to_string()));

        // Clear blend state
        self.previous_state = None;
        self.blend_time = 0.0;

        true
    }

    /// Play animation with blending from current
    pub fn play_with_blend(&mut self, name: &str, blend_duration: f32) -> bool {
        if !self.animations.contains_key(name) {
            return false;
        }

        // Store current state for blending
        if self.state.is_playing() {
            self.previous_state = Some(self.state.clone());
            self.blend_duration = blend_duration;
            self.blend_time = 0.0;
        }

        self.state = AnimationState::new(name);
        self.state.play();
        self.state.blend_weight = 0.0; // Start at 0, blend to 1
        self.state
            .triggered_events
            .push(AnimationEvent::Started(name.to_string()));

        true
    }

    /// Stop playback
    pub fn stop(&mut self) {
        self.state.stop();
        self.previous_state = None;
    }

    /// Pause playback
    pub fn pause(&mut self) {
        self.state.pause();
    }

    /// Resume playback
    pub fn resume(&mut self) {
        self.state.resume();
    }

    /// Restart playback from beginning
    pub fn restart(&mut self) {
        self.state.restart();
    }

    /// Get current animation name
    pub fn current_animation(&self) -> Option<&str> {
        if self.state.animation_name.is_empty() {
            None
        } else {
            Some(&self.state.animation_name)
        }
    }

    /// Get playback state
    pub fn playback_state(&self) -> PlaybackState {
        self.state.state
    }

    /// Is currently playing
    pub fn is_playing(&self) -> bool {
        self.state.is_playing()
    }

    /// Is completed
    pub fn is_completed(&self) -> bool {
        self.state.is_completed()
    }

    /// Get current frame index
    pub fn current_frame(&self) -> usize {
        self.state.current_frame
    }

    /// Get current time
    pub fn current_time(&self) -> f32 {
        self.state.time
    }

    /// Set playback speed
    pub fn set_speed(&mut self, speed: f32) {
        self.state.set_speed(speed);
    }

    /// Get playback speed
    pub fn speed(&self) -> f32 {
        self.state.speed
    }

    /// Get triggered events (and clear)
    pub fn take_events(&mut self) -> Vec<AnimationEvent> {
        std::mem::take(&mut self.state.triggered_events)
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) {
        if self.state.state != PlaybackState::Playing {
            return;
        }

        // Clear previous events
        self.state.triggered_events.clear();

        // Get animation
        let animation = match self.animations.get(&self.state.animation_name) {
            Some(a) => a,
            None => return,
        };

        // Update time
        let time_delta = delta_time * self.state.speed;
        let new_time = if self.state.reversed {
            self.state.time - time_delta
        } else {
            self.state.time + time_delta
        };

        self.state.previous_frame = self.state.current_frame;
        self.state.time = new_time;

        // Handle loop modes
        match animation.loop_mode {
            LoopMode::Once => {
                if self.state.time >= animation.total_duration {
                    self.state.time = animation.total_duration;
                    self.state.state = PlaybackState::Completed;
                    self.state
                        .triggered_events
                        .push(AnimationEvent::Completed(self.state.animation_name.clone()));
                }
            }
            LoopMode::OnceAndHide => {
                if self.state.time >= animation.total_duration {
                    self.state.time = animation.total_duration;
                    self.state.state = PlaybackState::Completed;
                    self.visible = false;
                    self.state
                        .triggered_events
                        .push(AnimationEvent::Completed(self.state.animation_name.clone()));
                }
            }
            LoopMode::Loop => {
                while self.state.time >= animation.total_duration {
                    self.state.time -= animation.total_duration;
                    self.state
                        .triggered_events
                        .push(AnimationEvent::Looped(self.state.animation_name.clone()));
                }
                while self.state.time < 0.0 {
                    self.state.time += animation.total_duration;
                    self.state
                        .triggered_events
                        .push(AnimationEvent::Looped(self.state.animation_name.clone()));
                }
            }
            LoopMode::PingPong => {
                let cycle = animation.total_duration * 2.0;
                while self.state.time >= cycle {
                    self.state.time -= cycle;
                    self.state
                        .triggered_events
                        .push(AnimationEvent::Looped(self.state.animation_name.clone()));
                }
                while self.state.time < 0.0 {
                    self.state.time += cycle;
                    self.state
                        .triggered_events
                        .push(AnimationEvent::Looped(self.state.animation_name.clone()));
                }
            }
        }

        // Update current frame
        self.state.current_frame = animation.get_frame_index_at_time(self.state.time);

        // Frame change event
        if self.state.current_frame != self.state.previous_frame {
            self.state
                .triggered_events
                .push(AnimationEvent::FrameChanged(self.state.current_frame));

            // Check for audio/particle triggers
            if let Some(frame) = animation.frames.get(self.state.current_frame) {
                if let Some(audio) = &frame.audio_trigger {
                    self.state
                        .triggered_events
                        .push(AnimationEvent::Audio(audio.clone()));
                }
                if let Some(particle) = &frame.particle_trigger {
                    self.state
                        .triggered_events
                        .push(AnimationEvent::Particle(particle.clone()));
                }
            }
        }

        // Update blending
        if self.previous_state.is_some() && self.blend_duration > 0.0 {
            self.blend_time += delta_time;
            let t = (self.blend_time / self.blend_duration).min(1.0);
            self.state.blend_weight = t;

            if let Some(prev) = &mut self.previous_state {
                prev.blend_weight = 1.0 - t;

                // Update previous animation time too
                if let Some(prev_anim) = self.animations.get(&prev.animation_name) {
                    prev.time += delta_time * prev.speed;
                    prev.current_frame = prev_anim.get_frame_index_at_time(prev.time);
                }
            }

            // Blend complete
            if t >= 1.0 {
                self.previous_state = None;
            }
        } else {
            self.state.blend_weight = 1.0;
        }
    }

    /// Get current frame data
    pub fn get_current_frame(&self) -> Option<&AnimationFrame> {
        let animation = self.animations.get(&self.state.animation_name)?;
        animation.get_frame_at_time(self.state.time)
    }

    /// Draw the current animation frame
    pub fn draw(&self, canvas: &mut Canvas, atlas: &SpriteAtlas) {
        if !self.visible {
            return;
        }

        let animation = match self.animations.get(&self.state.animation_name) {
            Some(a) => a,
            None => return,
        };

        let frame = match animation.get_frame_at_time(self.state.time) {
            Some(f) => f,
            None => return,
        };

        self.draw_frame(canvas, atlas, frame, self.state.blend_weight);

        // Draw previous animation (for blending)
        if let Some(prev) = &self.previous_state {
            if let Some(prev_anim) = self.animations.get(&prev.animation_name) {
                if let Some(prev_frame) = prev_anim.get_frame_at_time(prev.time) {
                    self.draw_frame(canvas, atlas, prev_frame, prev.blend_weight);
                }
            }
        }
    }

    /// Draw a single frame
    fn draw_frame(
        &self,
        canvas: &mut Canvas,
        atlas: &SpriteAtlas,
        frame: &AnimationFrame,
        blend_weight: f32,
    ) {
        // Calculate draw parameters
        let scale_x = self.scale * frame.scale * if self.flip_x { -1.0 } else { 1.0 };
        let scale_y = self.scale * frame.scale * if self.flip_y { -1.0 } else { 1.0 };

        let rotation = self.rotation + frame.rotation;

        let color = Color::new(
            self.color.r,
            self.color.g,
            self.color.b,
            self.color.a * blend_weight,
        );

        // Get sprite rect from atlas
        let src_rect = frame.sprite_rect;

        // Calculate destination position
        let dest_x = self.position.x + frame.offset.x * self.scale;
        let dest_y = self.position.y + frame.offset.y * self.scale;

        let draw_param = DrawParam::default()
            .src(src_rect)
            .dest([dest_x, dest_y])
            .scale([scale_x, scale_y])
            .rotation(rotation)
            .offset([self.origin.x, self.origin.y])
            .color(color);

        canvas.draw(atlas.image(), draw_param);
    }

    /// Draw with a custom draw parameter (for batching)
    pub fn get_draw_params(&self, _atlas: &SpriteAtlas) -> Option<(Rect, DrawParam)> {
        if !self.visible {
            return None;
        }

        let animation = self.animations.get(&self.state.animation_name)?;
        let frame = animation.get_frame_at_time(self.state.time)?;

        let scale_x = self.scale * frame.scale * if self.flip_x { -1.0 } else { 1.0 };
        let scale_y = self.scale * frame.scale * if self.flip_y { -1.0 } else { 1.0 };
        let rotation = self.rotation + frame.rotation;

        let dest_x = self.position.x + frame.offset.x * self.scale;
        let dest_y = self.position.y + frame.offset.y * self.scale;

        let draw_param = DrawParam::default()
            .src(frame.sprite_rect)
            .dest([dest_x, dest_y])
            .scale([scale_x, scale_y])
            .rotation(rotation)
            .offset([self.origin.x, self.origin.y])
            .color(self.color);

        Some((frame.sprite_rect, draw_param))
    }
}

impl Default for AnimationPlayer {
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
    use crate::animation::types::Animation;

    #[test]
    fn test_animation_player_basic() {
        let mut player = AnimationPlayer::new();

        let anim = Animation::from_grid("test", 4, 1, 4, 0.25);
        player.add_animation(anim);

        assert!(player.play("test"));
        assert!(player.is_playing());
        assert_eq!(player.current_animation(), Some("test"));
    }

    #[test]
    fn test_animation_player_update() {
        let mut player = AnimationPlayer::new();

        let mut anim = Animation::from_grid("test", 4, 1, 4, 0.25);
        anim.loop_mode = LoopMode::Once;
        player.add_animation(anim);

        player.play("test");

        // Update through animation
        player.update(0.5);
        assert_eq!(player.current_frame(), 2);

        // Complete animation
        player.update(0.6);
        assert!(player.is_completed());
    }

    #[test]
    fn test_animation_player_looping() {
        let mut player = AnimationPlayer::new();

        let mut anim = Animation::from_grid("test", 4, 1, 4, 0.25);
        anim.loop_mode = LoopMode::Loop;
        player.add_animation(anim);

        player.play("test");
        player.update(1.5); // 1.5 loops

        assert!(player.is_playing());
        assert!(!player.is_completed());
    }

    #[test]
    fn test_animation_events() {
        let mut player = AnimationPlayer::new();

        let anim = Animation::from_grid("test", 4, 1, 4, 0.25);
        player.add_animation(anim);

        player.play("test");

        // Should have started event
        let events = player.take_events();
        assert!(events
            .iter()
            .any(|e| matches!(e, AnimationEvent::Started(_))));

        // Update to next frame
        player.update(0.3);
        let events = player.take_events();
        assert!(events
            .iter()
            .any(|e| matches!(e, AnimationEvent::FrameChanged(_))));
    }
}
