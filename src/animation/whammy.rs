//! # Whammy Animation Library
//!
//! Defines all 30 Whammy character animations with their frame data,
//! timing, and effects.
//!
//! ## Animation Categories
//! - **Core (5)**: Hammer, Pogo Stick, Roller Skating, TNT, Fang (Boxing)
//! - **Extended (10)**: Boombox, UFO, Fishing, Skydiving, Cannon, Surfing,
//!   Karate, Graduation, Computer, Baseball
//! - **Holiday (10)**: Santa, Easter Bunny, Cupid, Leprechaun, Turkey,
//!   Uncle Sam, Witch, Jack-o-lantern, Pilgrim, Graduation
//! - **Special (5)**: Trap Door, Group Goodbye, Sad Walk-off, Dance, Laugh

use ggez::graphics::Color;
use ggez::mint::Vector2;
use ggez::{Context, GameResult};
use std::collections::HashMap;

use super::atlas::{
    AtlasManager, SpriteAtlas, WHAMMY_HIGHLIGHT, WHAMMY_OUTLINE, WHAMMY_RED, WHAMMY_SHADOW,
};
use super::effects::{FlashMode, ScreenEffects, ScreenFlash, ScreenShake};
use super::particles::{Particle, ParticleEmitter, ParticleSystem, ParticleType};
use super::player::{AnimationEvent, AnimationPlayer, AnimationState, PlaybackState};
use super::types::{Animation, AnimationBuilder, AnimationFrame, LoopMode};

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

#[allow(dead_code)]
impl WhammyAnimationLibrary {
    /// Create new library with placeholder sprites
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut library = Self {
            animations: HashMap::new(),
            atlas_manager: AtlasManager::new(),
        };

        // Create placeholder atlases for all animations
        for anim_id in WhammyAnimationId::all() {
            let atlas =
                SpriteAtlas::create_whammy_placeholder(ctx, anim_id.name(), anim_id.frame_count())?;
            library.atlas_manager.add(atlas);

            // Create animation definition
            let animation = library.create_animation_definition(anim_id);
            library.animations.insert(anim_id, animation);
        }

        Ok(library)
    }

    /// Get the primary Whammy color for rendering
    pub fn primary_color(&self) -> Color {
        WHAMMY_RED
    }

    /// Get the shadow color for Whammy rendering
    pub fn shadow_color(&self) -> Color {
        WHAMMY_SHADOW
    }

    /// Get the highlight color for Whammy rendering
    pub fn highlight_color(&self) -> Color {
        WHAMMY_HIGHLIGHT
    }

    /// Get the outline color for Whammy rendering
    pub fn outline_color(&self) -> Color {
        WHAMMY_OUTLINE
    }

    /// Create animation definition for given ID
    fn create_animation_definition(&self, id: WhammyAnimationId) -> Animation {
        let frame_duration = 1.0 / 24.0; // 24 FPS
        let frame_count = id.frame_count();
        let cols = 6u32;
        let rows = (frame_count + cols - 1) / cols;

        // Determine loop mode based on animation type
        let loop_mode = match id {
            WhammyAnimationId::Idle => LoopMode::Loop,
            WhammyAnimationId::Dance | WhammyAnimationId::Laugh => LoopMode::PingPong,
            WhammyAnimationId::TrapDoor | WhammyAnimationId::SadWalkOff => LoopMode::OnceAndHide,
            _ => LoopMode::Once,
        };

        let mut builder = AnimationBuilder::new(id.name())
            .display_name(id.taunt())
            .atlas(id.name())
            .loop_mode(loop_mode)
            .priority(10)
            .interruptible(false);

        // Add frames using the builder
        builder = builder.frames_grid(0, frame_count, cols, rows, frame_duration);

        // Add screen shake to appropriate animations using builder methods
        builder = match id {
            WhammyAnimationId::Hammer => builder.screen_shake(20.0, 0.3),
            WhammyAnimationId::Tnt => builder
                .screen_shake(30.0, 0.5)
                .screen_flash(1.0, 0.5, 0.0, 0.3),
            WhammyAnimationId::FangBoxing => builder.screen_shake(15.0, 0.2),
            WhammyAnimationId::Cannon => builder
                .screen_shake(25.0, 0.4)
                .screen_flash(1.0, 0.8, 0.2, 0.2),
            WhammyAnimationId::Karate => builder.screen_shake(12.0, 0.15),
            WhammyAnimationId::TrapDoor => builder.screen_shake(10.0, 0.3),
            _ => builder,
        };

        // Build the animation
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

        // Add particle effects at specific frames - using all ParticleType variants
        match id {
            WhammyAnimationId::Tnt => {
                if let Some(frame) = animation.frames.get_mut(8) {
                    frame.particle_trigger = Some("explosion".to_string());
                }
                // Add fire particles after explosion
                if let Some(frame) = animation.frames.get_mut(10) {
                    frame.particle_trigger = Some("fire".to_string());
                }
            }
            WhammyAnimationId::Cannon => {
                if let Some(frame) = animation.frames.get_mut(6) {
                    frame.particle_trigger = Some("explosion".to_string());
                }
                // Add smoke particles after cannon fire
                if let Some(frame) = animation.frames.get_mut(8) {
                    frame.particle_trigger = Some("smoke".to_string());
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
            WhammyAnimationId::Witch | WhammyAnimationId::JackOLantern => {
                // Sparkle effects for magical animations
                if let Some(frame) = animation.frames.get_mut(5) {
                    frame.particle_trigger = Some("sparkle".to_string());
                }
            }
            WhammyAnimationId::TrapDoor => {
                // Dust cloud when trap door opens
                if let Some(frame) = animation.frames.get_mut(4) {
                    frame.particle_trigger = Some("dust".to_string());
                }
            }
            WhammyAnimationId::Baseball => {
                // Money scatter particles for baseball win
                if let Some(frame) = animation.frames.get_mut(3) {
                    frame.particle_trigger = Some("money".to_string());
                }
            }
            _ => {
                // Default sparkle effect at animation midpoint for visual interest
                let mid = frame_count as usize / 2;
                if let Some(frame) = animation.frames.get_mut(mid) {
                    if frame.particle_trigger.is_none() {
                        frame.particle_trigger = Some("sparkle".to_string());
                    }
                }
            }
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(dead_code)]
pub enum WhammyState {
    /// Hidden/inactive
    #[default]
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

/// Main Whammy animation controller
#[allow(dead_code)]
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

#[allow(dead_code)]
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

        // Process all AnimationEvent variants
        let events = self.player.take_events();
        for event in events {
            match event {
                AnimationEvent::Particle(name) => {
                    self.trigger_particles(&name, particle_system);
                }
                AnimationEvent::Audio(_sound_name) => {
                    // Audio events are handled by audio system integration
                    // The sound_name is available for external handling
                }
                AnimationEvent::Started(_anim_name) => {
                    // Animation started - intro effects handled by screen_effects
                }
                AnimationEvent::Completed(_) => {
                    if self.state == WhammyState::Performing {
                        self.state = WhammyState::Exiting;
                        self.transition_timer = 0.0;
                    }
                }
                AnimationEvent::Looped(_anim_name) => {
                    // Looping animation continued - trigger subtle sparkle effect
                    particle_system.emit(ParticleType::Sparkle, self.position.x, self.position.y);
                }
                AnimationEvent::FrameChanged(_frame_index) => {
                    // Frame changed - available for external sync if needed
                }
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

                    // Trigger screen effects with appropriate FlashMode for each animation
                    if let Some(id) = self.current_animation {
                        if let Some(anim) = self.library.get_animation(id) {
                            if let Some((intensity, duration)) = anim.screen_shake {
                                screen_effects.shake(intensity, duration);
                            }
                            if let Some((r, g, b, duration)) = anim.screen_flash {
                                // Choose FlashMode based on animation type for variety
                                let flash_mode = match id {
                                    WhammyAnimationId::Tnt | WhammyAnimationId::Cannon => {
                                        FlashMode::Flash
                                    }
                                    WhammyAnimationId::Dance | WhammyAnimationId::Laugh => {
                                        FlashMode::Pulse
                                    }
                                    WhammyAnimationId::TrapDoor | WhammyAnimationId::SadWalkOff => {
                                        FlashMode::FadeOut
                                    }
                                    _ => FlashMode::FadeInOut,
                                };
                                screen_effects.flash_with_mode(
                                    Color::new(r, g, b, 0.6),
                                    duration,
                                    flash_mode,
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

    /// Trigger particle effects - handles all ParticleType variants
    fn trigger_particles(&self, effect_name: &str, particle_system: &mut ParticleSystem) {
        let particle_type = match effect_name {
            "explosion" => ParticleType::ExplosionSparks,
            "confetti" => ParticleType::Confetti,
            "stars" => ParticleType::StarBurst,
            "money" => ParticleType::MoneyScatter,
            "dust" => ParticleType::Dust,
            "fire" => ParticleType::Fire,
            "smoke" => ParticleType::Smoke,
            "sparkle" => ParticleType::Sparkle,
            "custom" => ParticleType::Custom,
            _ => ParticleType::Sparkle,
        };

        particle_system.emit(particle_type, self.position.x, self.position.y);
    }

    /// Create a custom particle emitter for special effects
    pub fn create_custom_emitter(&self, x: f32, y: f32) -> ParticleEmitter {
        let mut emitter = ParticleEmitter::new(x, y);
        emitter.particle_type = ParticleType::Custom;
        emitter.burst_count = Some(15);
        emitter.color_start = self.library.primary_color();
        emitter.color_end = Color::new(
            self.library.shadow_color().r,
            self.library.shadow_color().g,
            self.library.shadow_color().b,
            0.0,
        );
        emitter
    }

    /// Create screen shake effect directly
    pub fn create_shake(&self, intensity: f32, duration: f32) -> ScreenShake {
        ScreenShake::new(intensity, duration)
    }

    /// Create screen flash effect directly
    pub fn create_flash(&self, duration: f32) -> ScreenFlash {
        ScreenFlash::new(self.library.primary_color(), duration)
    }

    /// Check if animation is paused
    pub fn is_paused(&self) -> bool {
        self.player.playback_state() == PlaybackState::Paused
    }

    /// Pause the current animation
    pub fn pause(&mut self) {
        self.player.pause();
    }

    /// Resume a paused animation
    pub fn resume(&mut self) {
        self.player.resume();
    }

    /// Get current playback state
    pub fn playback_state(&self) -> PlaybackState {
        self.player.playback_state()
    }

    /// Get the current animation state info
    pub fn get_animation_state(&self) -> Option<AnimationState> {
        if self.player.current_animation().is_some() {
            Some(AnimationState::new(
                self.player.current_animation().unwrap_or("idle"),
            ))
        } else {
            None
        }
    }

    /// Get current frame info
    pub fn get_current_frame(&self) -> Option<&AnimationFrame> {
        self.player.get_current_frame()
    }

    /// Create a particle for manual spawning
    pub fn create_particle(&self, x: f32, y: f32, lifetime: f32) -> Particle {
        use ggez::mint::Vector2 as MintVec2;
        Particle::new(MintVec2 { x, y }, MintVec2 { x: 0.0, y: -50.0 }, lifetime)
    }

    /// Draw the Whammy
    pub fn draw(&self, canvas: &mut ggez::graphics::Canvas, _ctx: &mut Context) {
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
