//! # Whammy Animation Library
//!
//! Defines all 30 Whammy character animations with their frame data,
//! timing, and effects.
//!
//! ## Animation Categories
//! - **Core (5)**: Hammer, Pogo Stick, Roller Skating, TNT, Fang (Boxing)
//! - **Extended (10)**: Boombox, UFO, Fishing, Skydiving, Cannon, Surfing,
//!                      Karate, Graduation, Computer, Baseball
//! - **Holiday (10)**: Santa, Easter Bunny, Cupid, Leprechaun, Turkey,
//!                     Uncle Sam, Witch, Jack-o-lantern, Pilgrim, Graduation
//! - **Special (5)**: Trap Door, Group Goodbye, Sad Walk-off, Dance, Laugh

use ggez::graphics::Color;
use ggez::mint::Vector2;
use ggez::{Context, GameResult};
use std::collections::HashMap;

use super::atlas::{AtlasManager, SpriteAtlas};
use super::effects::{FlashMode, ScreenEffects};
use super::particles::{ParticleSystem, ParticleType};
use super::player::AnimationPlayer;
use super::types::{Animation, AnimationBuilder, LoopMode};

// =============================================================================
// WHAMMY ANIMATION IDS
// =============================================================================

/// All available Whammy animations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WhammyAnimationId {
    // Core 5
    Hammer,
    PogoStick,
    RollerSkating,
    Tnt,
    FangBoxing,

    // Extended 10
    Boombox,
    UfoAbduction,
    Fishing,
    Skydiving,
    Cannon,
    Surfing,
    Karate,
    GraduationThrow,
    Computer,
    Baseball,

    // Holiday 10
    Santa,
    EasterBunny,
    Cupid,
    Leprechaun,
    Turkey,
    UncleSam,
    Witch,
    JackOLantern,
    Pilgrim,
    GraduationCap,

    // Special Exit Animations
    TrapDoor,
    GroupGoodbye,
    SadWalkOff,

    // Misc
    Dance,
    Laugh,
    Idle,
}

impl WhammyAnimationId {
    /// Get all animation IDs
    pub fn all() -> Vec<Self> {
        vec![
            Self::Hammer,
            Self::PogoStick,
            Self::RollerSkating,
            Self::Tnt,
            Self::FangBoxing,
            Self::Boombox,
            Self::UfoAbduction,
            Self::Fishing,
            Self::Skydiving,
            Self::Cannon,
            Self::Surfing,
            Self::Karate,
            Self::GraduationThrow,
            Self::Computer,
            Self::Baseball,
            Self::Santa,
            Self::EasterBunny,
            Self::Cupid,
            Self::Leprechaun,
            Self::Turkey,
            Self::UncleSam,
            Self::Witch,
            Self::JackOLantern,
            Self::Pilgrim,
            Self::GraduationCap,
            Self::TrapDoor,
            Self::GroupGoodbye,
            Self::SadWalkOff,
            Self::Dance,
            Self::Laugh,
            Self::Idle,
        ]
    }

    /// Get core 5 animations
    pub fn core() -> Vec<Self> {
        vec![
            Self::Hammer,
            Self::PogoStick,
            Self::RollerSkating,
            Self::Tnt,
            Self::FangBoxing,
        ]
    }

    /// Get extended 10 animations
    pub fn extended() -> Vec<Self> {
        vec![
            Self::Boombox,
            Self::UfoAbduction,
            Self::Fishing,
            Self::Skydiving,
            Self::Cannon,
            Self::Surfing,
            Self::Karate,
            Self::GraduationThrow,
            Self::Computer,
            Self::Baseball,
        ]
    }

    /// Get holiday animations
    pub fn holiday() -> Vec<Self> {
        vec![
            Self::Santa,
            Self::EasterBunny,
            Self::Cupid,
            Self::Leprechaun,
            Self::Turkey,
            Self::UncleSam,
            Self::Witch,
            Self::JackOLantern,
            Self::Pilgrim,
            Self::GraduationCap,
        ]
    }

    /// Get special exit animations
    pub fn special_exits() -> Vec<Self> {
        vec![Self::TrapDoor, Self::GroupGoodbye, Self::SadWalkOff]
    }

    /// Get string name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Hammer => "hammer",
            Self::PogoStick => "pogo_stick",
            Self::RollerSkating => "roller_skating",
            Self::Tnt => "tnt",
            Self::FangBoxing => "fang_boxing",
            Self::Boombox => "boombox",
            Self::UfoAbduction => "ufo_abduction",
            Self::Fishing => "fishing",
            Self::Skydiving => "skydiving",
            Self::Cannon => "cannon",
            Self::Surfing => "surfing",
            Self::Karate => "karate",
            Self::GraduationThrow => "graduation_throw",
            Self::Computer => "computer",
            Self::Baseball => "baseball",
            Self::Santa => "santa",
            Self::EasterBunny => "easter_bunny",
            Self::Cupid => "cupid",
            Self::Leprechaun => "leprechaun",
            Self::Turkey => "turkey",
            Self::UncleSam => "uncle_sam",
            Self::Witch => "witch",
            Self::JackOLantern => "jack_o_lantern",
            Self::Pilgrim => "pilgrim",
            Self::GraduationCap => "graduation_cap",
            Self::TrapDoor => "trap_door",
            Self::GroupGoodbye => "group_goodbye",
            Self::SadWalkOff => "sad_walk_off",
            Self::Dance => "dance",
            Self::Laugh => "laugh",
            Self::Idle => "idle",
        }
    }

    /// Get display text for Whammy
    pub fn taunt(&self) -> &'static str {
        match self {
            Self::Hammer => "The Big Whammy smashes your score!",
            Self::PogoStick => "Boing! Boing! There goes your money!",
            Self::RollerSkating => "Wheee! Your cash rolled away!",
            Self::Tnt => "BOOM! Your score just exploded!",
            Self::FangBoxing => "Knockout! Your money's down for the count!",
            Self::Boombox => "Your cash is dancing away!",
            Self::UfoAbduction => "Your money's been abducted!",
            Self::Fishing => "Hook, line, and sinker! Your cash is gone!",
            Self::Skydiving => "Your money just jumped ship!",
            Self::Cannon => "Fire! Your cash is blasted away!",
            Self::Surfing => "Cowabunga! Your money caught a wave!",
            Self::Karate => "Hi-YA! Your cash is chopped!",
            Self::GraduationThrow => "Congratulations! You graduated to $0!",
            Self::Computer => "ERROR 404: Money Not Found!",
            Self::Baseball => "Strike three! Your money's out!",
            Self::Santa => "Ho ho NO! No presents for you!",
            Self::EasterBunny => "Hoppy to take your cash!",
            Self::Cupid => "Cupid shot down your money!",
            Self::Leprechaun => "No luck of the Irish today!",
            Self::Turkey => "Your money got gobbled up!",
            Self::UncleSam => "Uncle Sam wants your money!",
            Self::Witch => "Your cash vanished like magic!",
            Self::JackOLantern => "Trick! Your money got treated away!",
            Self::Pilgrim => "Thanks for giving us your money!",
            Self::GraduationCap => "Class dismissed... with $0!",
            Self::TrapDoor => "Down the hatch goes your cash!",
            Self::GroupGoodbye => "Bye bye money! Bye bye!",
            Self::SadWalkOff => "So long, farewell to your cash!",
            Self::Dance => "The Whammy dances on your dreams!",
            Self::Laugh => "Ha ha ha! Better luck next time!",
            Self::Idle => "Whammy!",
        }
    }

    /// Get frame count for this animation
    pub fn frame_count(&self) -> u32 {
        match self {
            Self::Hammer => 18,
            Self::PogoStick => 24,
            Self::RollerSkating => 20,
            Self::Tnt => 15,
            Self::FangBoxing => 22,
            Self::Boombox => 20,
            Self::UfoAbduction => 24,
            Self::Fishing => 18,
            Self::Skydiving => 20,
            Self::Cannon => 16,
            Self::Surfing => 22,
            Self::Karate => 18,
            Self::GraduationThrow => 20,
            Self::Computer => 18,
            Self::Baseball => 20,
            Self::Santa => 20,
            Self::EasterBunny => 18,
            Self::Cupid => 16,
            Self::Leprechaun => 18,
            Self::Turkey => 16,
            Self::UncleSam => 18,
            Self::Witch => 20,
            Self::JackOLantern => 16,
            Self::Pilgrim => 16,
            Self::GraduationCap => 18,
            Self::TrapDoor => 24,
            Self::GroupGoodbye => 30,
            Self::SadWalkOff => 32,
            Self::Dance => 24,
            Self::Laugh => 16,
            Self::Idle => 8,
        }
    }

    /// Get random standard animation
    pub fn random() -> Self {
        let animations = Self::core();
        animations[fastrand::usize(..animations.len())]
    }

    /// Get random from all main animations
    pub fn random_all() -> Self {
        let mut all = Self::core();
        all.extend(Self::extended());
        all[fastrand::usize(..all.len())]
    }

    /// Get random holiday animation
    pub fn random_holiday() -> Self {
        let animations = Self::holiday();
        animations[fastrand::usize(..animations.len())]
    }
}

// =============================================================================
// WHAMMY ANIMATION LIBRARY
// =============================================================================

/// Library of all Whammy animations
pub struct WhammyAnimationLibrary {
    /// All animations by ID
    animations: HashMap<WhammyAnimationId, Animation>,

    /// Atlas manager for sprites
    atlas_manager: AtlasManager,
}

impl WhammyAnimationLibrary {
    /// Create new library with placeholder sprites
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut library = Self {
            animations: HashMap::new(),
            atlas_manager: AtlasManager::new(),
        };

        // Create placeholder atlases for all animations
        for anim_id in WhammyAnimationId::all() {
            let atlas = SpriteAtlas::create_whammy_placeholder(
                ctx,
                anim_id.name(),
                anim_id.frame_count(),
            )?;
            library.atlas_manager.add(atlas);

            // Create animation definition
            let animation = library.create_animation_definition(anim_id);
            library.animations.insert(anim_id, animation);
        }

        Ok(library)
    }

    /// Create animation definition for given ID
    fn create_animation_definition(&self, id: WhammyAnimationId) -> Animation {
        let frame_duration = 1.0 / 24.0; // 24 FPS
        let frame_count = id.frame_count();
        let cols = 6u32;
        let rows = (frame_count + cols - 1) / cols;

        let mut builder = AnimationBuilder::new(id.name())
            .display_name(id.taunt())
            .atlas(id.name())
            .loop_mode(LoopMode::Once)
            .priority(10)
            .interruptible(false);

        // Add frames
        builder = builder.frames_grid(0, frame_count, cols, rows, frame_duration);

        // Add effects based on animation type
        let mut animation = builder.build();

        match id {
            WhammyAnimationId::Hammer => {
                animation.screen_shake = Some((20.0, 0.3));
            }
            WhammyAnimationId::Tnt => {
                animation.screen_shake = Some((30.0, 0.5));
                animation.screen_flash = Some((1.0, 0.5, 0.0, 0.3));
            }
            WhammyAnimationId::FangBoxing => {
                animation.screen_shake = Some((15.0, 0.2));
            }
            WhammyAnimationId::Cannon => {
                animation.screen_shake = Some((25.0, 0.4));
                animation.screen_flash = Some((1.0, 0.8, 0.2, 0.2));
            }
            WhammyAnimationId::Karate => {
                animation.screen_shake = Some((12.0, 0.15));
            }
            WhammyAnimationId::TrapDoor => {
                animation.screen_shake = Some((10.0, 0.3));
            }
            _ => {}
        }

        // Add audio trigger to first frame
        if let Some(frame) = animation.frames.get_mut(0) {
            frame.audio_trigger = Some("whammy".to_string());
        }

        // Add particle effects at specific frames
        match id {
            WhammyAnimationId::Tnt => {
                if let Some(frame) = animation.frames.get_mut(8) {
                    frame.particle_trigger = Some("explosion".to_string());
                }
            }
            WhammyAnimationId::Cannon => {
                if let Some(frame) = animation.frames.get_mut(6) {
                    frame.particle_trigger = Some("explosion".to_string());
                }
            }
            WhammyAnimationId::GraduationThrow | WhammyAnimationId::GraduationCap => {
                if let Some(frame) = animation.frames.get_mut(10) {
                    frame.particle_trigger = Some("confetti".to_string());
                }
            }
            WhammyAnimationId::Santa => {
                if let Some(frame) = animation.frames.get_mut(frame_count as usize / 2) {
                    frame.particle_trigger = Some("stars".to_string());
                }
            }
            _ => {}
        }

        animation
    }

    /// Get animation by ID
    pub fn get_animation(&self, id: WhammyAnimationId) -> Option<&Animation> {
        self.animations.get(&id)
    }

    /// Get atlas by animation ID
    pub fn get_atlas(&self, id: WhammyAnimationId) -> Option<&SpriteAtlas> {
        self.atlas_manager.get(id.name())
    }

    /// Get all animation IDs
    pub fn animation_ids(&self) -> Vec<WhammyAnimationId> {
        WhammyAnimationId::all()
    }
}

// =============================================================================
// WHAMMY ANIMATOR
// =============================================================================

/// State for the Whammy animation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhammyState {
    /// Hidden/inactive
    Hidden,

    /// Entering the screen
    Entering,

    /// Performing animation
    Performing,

    /// Exiting the screen
    Exiting,

    /// Idle (looping)
    Idle,
}

impl Default for WhammyState {
    fn default() -> Self {
        Self::Hidden
    }
}

/// Main Whammy animation controller
pub struct WhammyAnimator {
    /// Animation player
    player: AnimationPlayer,

    /// Animation library
    library: WhammyAnimationLibrary,

    /// Current state
    state: WhammyState,

    /// Current animation ID
    current_animation: Option<WhammyAnimationId>,

    /// Position on screen
    pub position: Vector2<f32>,

    /// Target position (for movement)
    target_position: Vector2<f32>,

    /// Scale
    pub scale: f32,

    /// Movement speed
    movement_speed: f32,

    /// Enter/exit timer
    transition_timer: f32,

    /// Total animation time
    animation_time: f32,

    /// Callback for animation complete
    on_complete: Option<Box<dyn Fn() + Send + Sync>>,

    /// Current taunt text
    pub taunt_text: String,
}

impl WhammyAnimator {
    /// Create new Whammy animator
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let library = WhammyAnimationLibrary::new(ctx)?;

        Ok(Self {
            player: AnimationPlayer::new(),
            library,
            state: WhammyState::Hidden,
            current_animation: None,
            position: Vector2 { x: 0.0, y: 0.0 },
            target_position: Vector2 { x: 0.0, y: 0.0 },
            scale: 1.0,
            movement_speed: 500.0,
            transition_timer: 0.0,
            animation_time: 0.0,
            on_complete: None,
            taunt_text: String::new(),
        })
    }

    /// Play a Whammy animation
    pub fn play(&mut self, id: WhammyAnimationId, center_x: f32, center_y: f32) {
        // Get animation and load into player
        if let Some(animation) = self.library.get_animation(id) {
            self.player.add_animation(animation.clone());
            self.player.play(id.name());

            self.current_animation = Some(id);
            self.state = WhammyState::Entering;
            self.transition_timer = 0.0;
            self.animation_time = 0.0;

            // Set positions
            self.target_position = Vector2 {
                x: center_x,
                y: center_y,
            };

            // Start off-screen (bottom)
            self.position = Vector2 {
                x: center_x,
                y: center_y + 400.0,
            };

            // Set taunt text
            self.taunt_text = id.taunt().to_string();

            // Make visible
            self.player.visible = true;
        }
    }

    /// Play random core animation
    pub fn play_random(&mut self, center_x: f32, center_y: f32) {
        self.play(WhammyAnimationId::random(), center_x, center_y);
    }

    /// Play random from all animations
    pub fn play_random_all(&mut self, center_x: f32, center_y: f32) {
        self.play(WhammyAnimationId::random_all(), center_x, center_y);
    }

    /// Check if currently playing
    pub fn is_playing(&self) -> bool {
        self.state != WhammyState::Hidden
    }

    /// Get current state
    pub fn state(&self) -> WhammyState {
        self.state
    }

    /// Get current animation ID
    pub fn current_animation_id(&self) -> Option<WhammyAnimationId> {
        self.current_animation
    }

    /// Update animation
    pub fn update(
        &mut self,
        delta_time: f32,
        screen_effects: &mut ScreenEffects,
        particle_system: &mut ParticleSystem,
    ) {
        if self.state == WhammyState::Hidden {
            return;
        }

        self.animation_time += delta_time;

        // Update player
        self.player.update(delta_time);

        // Process events
        let events = self.player.take_events();
        for event in events {
            match event {
                super::player::AnimationEvent::Particle(name) => {
                    self.trigger_particles(&name, particle_system);
                }
                super::player::AnimationEvent::Completed(_) => {
                    if self.state == WhammyState::Performing {
                        self.state = WhammyState::Exiting;
                        self.transition_timer = 0.0;
                    }
                }
                _ => {}
            }
        }

        // Update state machine
        match self.state {
            WhammyState::Entering => {
                self.transition_timer += delta_time;
                let t = (self.transition_timer / 0.3).min(1.0);
                let ease_t = 1.0 - (1.0 - t).powi(3); // Ease out cubic

                self.position.x =
                    self.position.x + (self.target_position.x - self.position.x) * ease_t;
                self.position.y =
                    self.position.y + (self.target_position.y - self.position.y) * ease_t;

                if t >= 1.0 {
                    self.state = WhammyState::Performing;
                    self.position = self.target_position;

                    // Trigger screen effects
                    if let Some(id) = self.current_animation {
                        if let Some(anim) = self.library.get_animation(id) {
                            if let Some((intensity, duration)) = anim.screen_shake {
                                screen_effects.shake(intensity, duration);
                            }
                            if let Some((r, g, b, duration)) = anim.screen_flash {
                                screen_effects.flash_with_mode(
                                    Color::new(r, g, b, 0.6),
                                    duration,
                                    FlashMode::FadeInOut,
                                );
                            }
                        }
                    }
                }
            }
            WhammyState::Performing => {
                if self.player.is_completed() {
                    self.state = WhammyState::Exiting;
                    self.transition_timer = 0.0;
                }
            }
            WhammyState::Exiting => {
                self.transition_timer += delta_time;
                let t = (self.transition_timer / 0.4).min(1.0);

                // Move down and fade out
                self.position.y = self.target_position.y + 400.0 * t;
                self.player.color = Color::new(1.0, 1.0, 1.0, 1.0 - t);

                if t >= 1.0 {
                    self.state = WhammyState::Hidden;
                    self.player.visible = false;
                    self.taunt_text.clear();

                    if let Some(callback) = self.on_complete.take() {
                        callback();
                    }
                }
            }
            WhammyState::Idle => {
                // Idle looping animation
            }
            WhammyState::Hidden => {}
        }

        // Update player position
        self.player.position = self.position;
        self.player.scale = self.scale;
    }

    /// Trigger particle effects
    fn trigger_particles(&self, effect_name: &str, particle_system: &mut ParticleSystem) {
        let particle_type = match effect_name {
            "explosion" => ParticleType::ExplosionSparks,
            "confetti" => ParticleType::Confetti,
            "stars" => ParticleType::StarBurst,
            "money" => ParticleType::MoneyScatter,
            _ => ParticleType::Sparkle,
        };

        particle_system.emit(particle_type, self.position.x, self.position.y);
    }

    /// Draw the Whammy
    pub fn draw(
        &self,
        canvas: &mut ggez::graphics::Canvas,
        _ctx: &mut Context,
    ) {
        if self.state == WhammyState::Hidden {
            return;
        }

        // Get current atlas
        if let Some(id) = self.current_animation {
            if let Some(atlas) = self.library.get_atlas(id) {
                self.player.draw(canvas, atlas);
            }
        }
    }

    /// Set callback for animation complete
    pub fn on_complete<F: Fn() + Send + Sync + 'static>(&mut self, callback: F) {
        self.on_complete = Some(Box::new(callback));
    }

    /// Force stop animation
    pub fn stop(&mut self) {
        self.state = WhammyState::Hidden;
        self.player.stop();
        self.player.visible = false;
        self.taunt_text.clear();
        self.current_animation = None;
    }

    /// Get animation duration
    pub fn animation_duration(&self) -> f32 {
        if let Some(id) = self.current_animation {
            if let Some(anim) = self.library.get_animation(id) {
                return anim.total_duration;
            }
        }
        0.0
    }

    /// Get elapsed animation time
    pub fn elapsed_time(&self) -> f32 {
        self.animation_time
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whammy_animation_id() {
        assert_eq!(WhammyAnimationId::Hammer.name(), "hammer");
        assert_eq!(WhammyAnimationId::Hammer.frame_count(), 18);

        let core = WhammyAnimationId::core();
        assert_eq!(core.len(), 5);

        let all = WhammyAnimationId::all();
        assert!(all.len() >= 30);
    }

    #[test]
    fn test_whammy_animation_id_random() {
        // Just ensure it doesn't panic
        let _ = WhammyAnimationId::random();
        let _ = WhammyAnimationId::random_all();
        let _ = WhammyAnimationId::random_holiday();
    }

    #[test]
    fn test_whammy_taunt() {
        assert!(WhammyAnimationId::Hammer.taunt().contains("smash"));
        assert!(WhammyAnimationId::Tnt.taunt().contains("BOOM"));
    }
}
