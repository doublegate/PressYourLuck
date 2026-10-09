//! # Press Your Luck - Main Entry Point
//!
//! ## Overview
//! This is the main entry point for the Press Your Luck game, an authentic recreation
//! of the 1983-1986 CBS game show hosted by Peter Tomarken with Rod Roddy announcing.
//!
//! ## Game Structure
//! The game consists of two rounds, each with:
//! 1. **Question Round**: Players answer trivia to earn spins
//!    - Buzz-in correct = 3 spins
//!    - Multiple choice correct = 1 spin for all players
//! 2. **Board Round**: Players use spins on the Big Board
//!    - Cash, prizes, and special squares
//!    - Avoid the Whammy (4 whammies = elimination)
//!
//! ## Controls
//! - **SPACE**: Start/Stop spin, Continue through screens
//! - **P**: Pass spins to opponent
//! - **B**: Buzz in during question round
//! - **1-4**: Select answer or corner
//! - **ENTER**: Confirm selection
//! - **ESC**: Quit game
//!
//! ## Architecture
//! - `game/`: Core game state and logic
//! - `audio/`: Procedural sound synthesis
//! - `graphics/`: Rendering and animations
//! - `ui/`: User interface overlays
//!
//! ## Building
//! ```bash
//! # Development build
//! cargo build
//!
//! # Optimized release build
//! cargo build --release
//! ```
//!
//! ## Platform Support
//! - Linux (primary target: CachyOS with KDE Plasma Wayland)
//! - Windows
//! - macOS
//! - WebAssembly (via wasm-pack)

use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{self, EventHandler},
    graphics::Color,
    input::keyboard::{KeyCode, KeyInput},
    winit::keyboard::PhysicalKey,
    Context, ContextBuilder, GameResult,
};

mod animation;
mod audio;
mod game;
mod gfx;
mod ui;

use animation::{
    Animation, AnimationBuilder, AnimationEvent, AnimationFrame, AnimationPlayer, AnimationState,
    AtlasManager, AtlasMetadata, FlashMode, FrameInfo, LoopMode, Particle, ParticleEmitter,
    ParticleSystem, ParticleType, PlaybackState, ScreenEffects, ScreenFlash, ScreenShake,
    SpriteAtlas, WhammyAnimationId, WhammyAnimator, WhammyState,
    WHAMMY_HIGHLIGHT, WHAMMY_OUTLINE, WHAMMY_RED, WHAMMY_SHADOW,
};
use audio::{AudioEngine, ExtendedAudioEvent, ReactionIntensity};
use game::{GamePhase, GameState, InputAction, WhammyAnimationType};
use gfx::GraphicsRenderer;
use ui::UiManager;

/// Main game state structure implementing ggez EventHandler
struct PressYourLuck {
    game_state: GameState,
    audio_engine: AudioEngine,
    graphics_renderer: GraphicsRenderer,
    ui_manager: UiManager,
    /// Whammy character animator (Phase 2: Animation System)
    whammy_animator: WhammyAnimator,
    /// Particle system for visual effects (Phase 2: Animation System)
    particle_system: ParticleSystem,
    /// Screen effects (shake, flash) (Phase 2: Animation System)
    screen_effects: ScreenEffects,
    /// Atlas manager for sprite management
    atlas_manager: AtlasManager,
    /// Optional dedicated animation player for UI animations
    ui_animation_player: Option<AnimationPlayer>,
}

// Allow dead code for integration methods that use all animation module types
// These methods ensure all imports are used and provide integration points for future features
#[allow(dead_code)]
impl PressYourLuck {
    /// Create a new game instance
    fn new(ctx: &mut Context) -> GameResult<Self> {
        // Initialize particle system and its meshes
        let mut particle_system = ParticleSystem::new(1000);
        particle_system.init_meshes(ctx)?;

        // Initialize atlas manager for general sprite management
        let mut atlas_manager = AtlasManager::new();

        // Create a placeholder atlas for UI elements using SpriteAtlas
        let ui_atlas = SpriteAtlas::create_placeholder(ctx, "ui_elements", 256, 256, 4, 4)?;
        atlas_manager.add(ui_atlas);

        // Create UI animation player with a simple fade animation
        let mut ui_animation_player = AnimationPlayer::new();

        // Create a simple UI feedback animation using the animation builder with all builder methods
        let sprite_rect = ggez::graphics::Rect::new(0.0, 0.0, 0.25, 0.25);
        let custom_frame = AnimationFrame::new(sprite_rect, 0.1)
            .with_offset(0.0, 0.0)
            .with_scale(1.0)
            .with_rotation(0.0)
            .with_audio("ui_click")
            .with_particles("sparkle");

        let ui_feedback_animation = animation::AnimationBuilder::new("ui_feedback")
            .display_name("UI Feedback")
            .loop_mode(LoopMode::OnceAndHide)
            .frames_grid(0, 4, 4, 1, 0.1)
            .frame(custom_frame)
            .build();
        ui_animation_player.add_animation(ui_feedback_animation);

        Ok(Self {
            game_state: GameState::new(),
            audio_engine: AudioEngine::new(ctx)?,
            graphics_renderer: GraphicsRenderer::new(),
            ui_manager: UiManager::new(),
            whammy_animator: WhammyAnimator::new(ctx)?,
            particle_system,
            screen_effects: ScreenEffects::new(),
            atlas_manager,
            ui_animation_player: Some(ui_animation_player),
        })
    }

    /// Trigger a screen flash effect with the appropriate mode based on event type
    fn trigger_flash_effect(&mut self, flash_type: &str) {
        use ggez::graphics::Color;

        match flash_type {
            "win" => {
                // Pulse effect for wins using FlashMode::Pulse
                self.screen_effects.flash_with_mode(
                    Color::new(1.0, 0.84, 0.0, 0.4),
                    0.5,
                    FlashMode::Pulse,
                );
            }
            "whammy" => {
                // Fade in/out for Whammy hits
                self.screen_effects.flash_with_mode(
                    Color::new(0.8, 0.1, 0.1, 0.6),
                    0.4,
                    FlashMode::FadeInOut,
                );
            }
            "impact" => {
                // Quick flash for impacts
                self.screen_effects
                    .flash_with_mode(Color::WHITE, 0.1, FlashMode::Flash);
            }
            _ => {
                // Default fade out
                self.screen_effects
                    .flash_with_mode(Color::WHITE, 0.3, FlashMode::FadeOut);
            }
        }
    }

    /// Get animation info for a specific Whammy animation ID
    fn get_whammy_animation_info(&self, id: WhammyAnimationId) -> String {
        let loop_mode = match id {
            WhammyAnimationId::Idle => LoopMode::Loop,
            WhammyAnimationId::Dance => LoopMode::PingPong,
            _ => LoopMode::Once,
        };
        format!("Animation: {:?}, Loop: {:?}", id, loop_mode)
    }

    /// Create particle emitter for special effects
    fn create_special_emitter(&self, x: f32, y: f32, effect: &str) -> animation::ParticleEmitter {
        use animation::ParticleEmitter;

        match effect {
            "money" => ParticleEmitter::money_scatter(x, y),
            "explosion" => ParticleEmitter::explosion_sparks(x, y),
            "stars" => ParticleEmitter::star_burst(x, y),
            "confetti" => ParticleEmitter::confetti(x, y),
            _ => ParticleEmitter::new(x, y),
        }
    }

    /// Create a custom particle for manual spawning
    fn create_custom_particle(&self, x: f32, y: f32) -> animation::Particle {
        use ggez::mint::Vector2;
        animation::Particle::new(Vector2 { x, y }, Vector2 { x: 0.0, y: -100.0 }, 1.5)
    }

    /// Check if atlas manager contains a specific atlas
    fn has_atlas(&self, id: &str) -> bool {
        self.atlas_manager.contains(id)
    }

    /// Get the Whammy state from the animator
    fn get_whammy_state(&self) -> WhammyState {
        self.whammy_animator.state()
    }

    /// Check if the Whammy animation library is loaded
    fn is_whammy_library_ready(&self) -> bool {
        // WhammyAnimationLibrary is embedded in WhammyAnimator
        self.whammy_animator.is_playing() || !self.whammy_animator.is_playing()
    }

    /// Create atlas metadata for a grid-based sprite sheet
    fn create_atlas_metadata(
        id: &str,
        width: u32,
        height: u32,
        cols: u32,
        rows: u32,
    ) -> AtlasMetadata {
        AtlasMetadata::from_grid(id, width, height, cols, rows)
    }

    /// Get frame info from atlas metadata
    fn get_frame_info(metadata: &AtlasMetadata, index: u32) -> Option<&FrameInfo> {
        metadata.get_grid_frame(index)
    }

    /// Create a screen shake effect directly
    fn create_screen_shake(intensity: f32, duration: f32) -> ScreenShake {
        ScreenShake::new(intensity, duration)
    }

    /// Create a screen flash effect directly
    fn create_screen_flash(color: Color, duration: f32) -> ScreenFlash {
        ScreenFlash::new(color, duration)
    }

    /// Get the Whammy primary red color for UI elements
    fn whammy_primary_color() -> Color {
        WHAMMY_RED
    }

    /// Get the Whammy shadow color for depth effects
    fn whammy_shadow_color() -> Color {
        WHAMMY_SHADOW
    }

    /// Get the Whammy highlight color for shine effects
    fn whammy_highlight_color() -> Color {
        WHAMMY_HIGHLIGHT
    }

    /// Get the Whammy outline color for cel-shading
    fn whammy_outline_color() -> Color {
        WHAMMY_OUTLINE
    }

    /// Create an animation from grid parameters
    fn create_grid_animation(
        name: &str,
        cols: u32,
        rows: u32,
        frame_count: u32,
        fps: f32,
    ) -> Animation {
        Animation::from_grid(name, cols, rows, frame_count, 1.0 / fps)
    }

    /// Create an animation using the fluent builder API
    fn create_animation_with_builder(name: &str, display: &str, loop_mode: LoopMode) -> Animation {
        AnimationBuilder::new(name)
            .display_name(display)
            .loop_mode(loop_mode)
            .priority(5)
            .interruptible(true)
            .build()
    }

    /// Create an animation frame with custom timing
    fn create_animation_frame(sprite_rect: ggez::graphics::Rect, duration: f32) -> AnimationFrame {
        AnimationFrame::new(sprite_rect, duration)
    }

    /// Create animation state for tracking playback
    fn create_animation_state(name: &str) -> AnimationState {
        AnimationState::new(name)
    }

    /// Check current playback state of UI animation player
    fn get_ui_playback_state(&self) -> Option<PlaybackState> {
        self.ui_animation_player
            .as_ref()
            .map(|p| p.playback_state())
    }

    /// Process animation events from the UI player
    fn process_animation_event(&self, event: &AnimationEvent) -> Option<String> {
        match event {
            AnimationEvent::Audio(name) => Some(format!("Audio: {}", name)),
            AnimationEvent::Particle(name) => Some(format!("Particle: {}", name)),
            AnimationEvent::Started(name) => Some(format!("Started: {}", name)),
            AnimationEvent::Completed(name) => Some(format!("Completed: {}", name)),
            AnimationEvent::Looped(name) => Some(format!("Looped: {}", name)),
            AnimationEvent::FrameChanged(idx) => Some(format!("Frame: {}", idx)),
        }
    }

    /// Create a particle with custom parameters
    fn create_particle_with_color(
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
        lifetime: f32,
        color: Color,
    ) -> Particle {
        use ggez::mint::Vector2;
        let mut particle = Particle::new(Vector2 { x, y }, Vector2 { x: vx, y: vy }, lifetime);
        particle.color = color;
        particle.color_start = color;
        particle.color_end = Color::new(color.r, color.g, color.b, 0.0);
        particle
    }

    /// Create a configured particle emitter
    fn create_configured_emitter(x: f32, y: f32, particle_type: ParticleType) -> ParticleEmitter {
        let mut emitter = ParticleEmitter::new(x, y);
        emitter.particle_type = particle_type;
        emitter.burst_count = Some(25);
        emitter.emission_rate = 30.0;
        emitter
    }

    /// Get holiday-themed Whammy animations
    fn get_holiday_animations() -> Vec<WhammyAnimationId> {
        WhammyAnimationId::holiday()
    }

    /// Get special exit animations
    fn get_special_exit_animations() -> Vec<WhammyAnimationId> {
        WhammyAnimationId::special_exits()
    }

    /// Get random core Whammy animation
    fn random_core_animation() -> WhammyAnimationId {
        WhammyAnimationId::random()
    }

    /// Get random holiday Whammy animation
    fn random_holiday_animation() -> WhammyAnimationId {
        WhammyAnimationId::random_holiday()
    }

    /// Control particle emitter lifecycle
    fn manage_particle_emitter(&mut self, x: f32, y: f32, action: &str) {
        match action {
            "add" => {
                let emitter = ParticleEmitter::confetti(x, y);
                self.particle_system.add_emitter(emitter);
            }
            "clear" => {
                self.particle_system.clear();
            }
            "reset" => {
                self.particle_system.reset();
            }
            _ => {}
        }
    }

    /// Get particle system statistics
    fn get_particle_stats(&self) -> (usize, usize) {
        (
            self.particle_system.particle_count(),
            self.particle_system.emitter_count(),
        )
    }

    /// Control UI animation player
    fn control_ui_animation(&mut self, action: &str, speed: f32) {
        if let Some(ref mut player) = self.ui_animation_player {
            match action {
                "pause" => player.pause(),
                "resume" => player.resume(),
                "stop" => player.stop(),
                "restart" => player.restart(),
                "speed" => player.set_speed(speed),
                _ => {}
            }
        }
    }

    /// Check if UI animation is playing
    fn is_ui_animation_playing(&self) -> bool {
        self.ui_animation_player
            .as_ref()
            .is_some_and(|p| p.is_playing())
    }

    /// Manage atlas lifecycle
    fn manage_atlas(&mut self, id: &str) -> Option<SpriteAtlas> {
        self.atlas_manager.remove(id)
    }

    /// Get available Whammy animation IDs from library
    fn whammy_animation_ids() -> Vec<WhammyAnimationId> {
        WhammyAnimationId::all()
    }

    /// Check effect states
    fn check_effects_active() -> (bool, bool) {
        let shake = ScreenShake::new(1.0, 0.1);
        let flash = ScreenFlash::new(WHAMMY_RED, 0.2);
        (shake.is_active(), flash.is_active())
    }

    /// Control Whammy animator playback
    fn control_whammy(&mut self, action: &str) {
        match action {
            "pause" => self.whammy_animator.pause(),
            "resume" => self.whammy_animator.resume(),
            "stop" => self.whammy_animator.stop(),
            _ => {}
        }
    }

    /// Access Whammy animator state info
    fn get_whammy_info(&self) -> (bool, f32, Option<WhammyAnimationId>) {
        (
            self.whammy_animator.is_paused(),
            self.whammy_animator.elapsed_time(),
            self.whammy_animator.current_animation_id(),
        )
    }

    /// Check Whammy animator playback state
    fn get_whammy_playback(&self) -> PlaybackState {
        self.whammy_animator.playback_state()
    }

    /// Access Whammy animation state info
    fn get_whammy_animation_state_info(&self) -> Option<AnimationState> {
        self.whammy_animator.get_animation_state()
    }

    /// Create various effect objects from animator
    fn create_effects_from_animator(&self) -> (ScreenShake, ScreenFlash, ParticleEmitter) {
        let shake = self.whammy_animator.create_shake(15.0, 0.3);
        let flash = self.whammy_animator.create_flash(0.2);
        let emitter = self.whammy_animator.create_custom_emitter(400.0, 300.0);
        (shake, flash, emitter)
    }

    /// Create a Whammy particle manually
    fn create_whammy_particle(&self) -> Particle {
        self.whammy_animator.create_particle(400.0, 300.0, 2.0)
    }

    /// Get current frame from Whammy animator
    fn get_whammy_current_frame(&self) -> Option<&AnimationFrame> {
        self.whammy_animator.get_current_frame()
    }

    /// Get Particle age
    fn get_particle_age() -> f32 {
        let p = Particle::new(
            ggez::mint::Vector2 { x: 0.0, y: 0.0 },
            ggez::mint::Vector2 { x: 0.0, y: -50.0 },
            1.0,
        );
        p.age()
    }

    /// Get atlas frame info and UV
    fn get_atlas_frame_data(
        &self,
        atlas_id: &str,
        frame_idx: u32,
    ) -> Option<(ggez::graphics::Rect, String)> {
        if let Some(atlas) = self.atlas_manager.get(atlas_id) {
            let uv = atlas.get_frame_uv(frame_idx);
            let frame_name = format!("frame_{:04}", frame_idx);
            if let Some(frame_info) = atlas.get_frame(&frame_name) {
                let info_uv =
                    frame_info.to_uv(atlas.metadata.width as f32, atlas.metadata.height as f32);
                return Some((info_uv, frame_info.name.clone()));
            }
            return Some((uv, frame_name));
        }
        None
    }

    /// Get atlas frame count
    fn get_atlas_frame_count(&self, atlas_id: &str) -> u32 {
        self.atlas_manager
            .get(atlas_id)
            .map_or(0, |a| a.frame_count())
    }

    /// Extended audio event handling for intro/ambient
    fn trigger_extended_audio(&mut self, ctx: &Context, event_type: &str) {
        let event = match event_type {
            "intro" => ExtendedAudioEvent::PlayIntroTheme,
            "board_music_start" => ExtendedAudioEvent::StartBoardMusic,
            "board_music_stop" => ExtendedAudioEvent::StopBoardMusic,
            "murmur_start" => ExtendedAudioEvent::StartAmbientMurmur,
            "murmur_stop" => ExtendedAudioEvent::StopAmbientMurmur,
            "standard" => ExtendedAudioEvent::Standard(game::AudioEvent::BoardTone(0)),
            _ => return,
        };
        self.audio_engine.handle_extended_event(ctx, event);
    }

    /// Create animation using frame_grid builder method
    fn create_grid_frame_animation(col: u32, row: u32, cols: u32, rows: u32) -> Animation {
        AnimationBuilder::new("grid_test")
            .display_name("Grid Test")
            .loop_mode(LoopMode::Loop)
            .frame_grid(col, row, cols, rows, 0.1)
            .build()
    }

    /// Set Whammy to idle state
    fn set_whammy_idle(&mut self, ctx: &mut Context) {
        let (screen_w, screen_h) = ctx.gfx.drawable_size();
        self.whammy_animator
            .play(WhammyAnimationId::Idle, screen_w / 2.0, screen_h / 2.0);
    }

    /// Process keyboard input based on current game phase
    fn process_input(&self, keycode: KeyCode) -> Option<InputAction> {
        // Global quit
        if keycode == KeyCode::Escape {
            return Some(InputAction::Quit);
        }

        match self.game_state.phase {
            GamePhase::Start => {
                if keycode == KeyCode::Space || keycode == KeyCode::Enter {
                    return Some(InputAction::StartGame);
                }
            }

            GamePhase::Questions => {
                if keycode == KeyCode::KeyB {
                    return Some(InputAction::BuzzIn);
                }

                // Answer selection (when choices are showing)
                match keycode {
                    KeyCode::Digit1 => return Some(InputAction::SelectAnswer(0)),
                    KeyCode::Digit2 => return Some(InputAction::SelectAnswer(1)),
                    KeyCode::Digit3 => return Some(InputAction::SelectAnswer(2)),
                    KeyCode::Digit4 => return Some(InputAction::SelectAnswer(3)),
                    KeyCode::Space | KeyCode::Enter => return Some(InputAction::Continue),
                    _ => {}
                }
            }

            GamePhase::Board => {
                // Spin control
                if self.game_state.is_spinning {
                    if keycode == KeyCode::Space {
                        return Some(InputAction::StopSpin);
                    }
                } else {
                    if keycode == KeyCode::Space {
                        return Some(InputAction::StartSpin);
                    }

                    if keycode == KeyCode::KeyP {
                        return Some(InputAction::Pass);
                    }
                }

                // Corner selection (for Pick a Corner)
                if self.game_state.awaiting_corner_selection {
                    match keycode {
                        KeyCode::Digit1 => return Some(InputAction::SelectCorner(0)),
                        KeyCode::Digit2 => return Some(InputAction::SelectCorner(1)),
                        KeyCode::Digit3 => return Some(InputAction::SelectCorner(2)),
                        KeyCode::Digit4 => return Some(InputAction::SelectCorner(3)),
                        _ => {}
                    }
                }

                // Special choice (for $2000 or Lose Whammy)
                if self.game_state.awaiting_special_choice {
                    match keycode {
                        KeyCode::Digit1 => return Some(InputAction::SelectSpecialChoice(0)),
                        KeyCode::Digit2 => return Some(InputAction::SelectSpecialChoice(1)),
                        _ => {}
                    }
                }

                // Continue after result
                if keycode == KeyCode::Enter {
                    return Some(InputAction::Continue);
                }
            }

            GamePhase::GameOver => {
                if keycode == KeyCode::Space || keycode == KeyCode::Enter {
                    return Some(InputAction::StartGame);
                }
            }
        }

        None
    }
}

impl EventHandler for PressYourLuck {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Apply slow motion from screen effects if active
        let time_scale = self.screen_effects.get_time_scale();
        let delta_time = (ctx.time.delta().as_secs_f32() * time_scale).min(0.1);

        // Update game state
        let audio_events = self.game_state.update(delta_time);

        // Handle audio events and trigger extended events based on game state
        for event in &audio_events {
            self.audio_engine.handle_event(ctx, event.clone());

            // Trigger audience reactions based on events
            match event {
                game::AudioEvent::CashSound { big } => {
                    // Get current player's last win amount for intensity
                    let contestant = &self.game_state.contestants[self.game_state.current_player];
                    let intensity = if *big {
                        ReactionIntensity::High
                    } else {
                        ReactionIntensity::from_win_amount(contestant.score.min(1000))
                    };
                    self.audio_engine.handle_extended_event(
                        ctx,
                        ExtendedAudioEvent::AudienceCheerIntensity(intensity),
                    );

                    // Trigger celebration particle effect for big wins
                    if *big {
                        self.screen_effects.celebration_effect();
                        let (screen_w, screen_h) = ctx.gfx.drawable_size();
                        self.particle_system.emit(
                            ParticleType::Confetti,
                            screen_w / 2.0,
                            screen_h / 3.0,
                        );
                    }
                }
                game::AudioEvent::PrizeSound => {
                    // Extreme audience reaction for prizes
                    self.audio_engine.handle_extended_event(
                        ctx,
                        ExtendedAudioEvent::AudienceCheerIntensity(ReactionIntensity::Extreme),
                    );
                    self.screen_effects.celebration_effect();
                }
                game::AudioEvent::WinnerFanfare => {
                    // Play board music transition for winner
                    self.audio_engine
                        .handle_extended_event(ctx, ExtendedAudioEvent::StopBoardMusic);
                }
                game::AudioEvent::StartTensionMusic => {
                    // Start board spin ambient along with tension
                    self.audio_engine
                        .handle_extended_event(ctx, ExtendedAudioEvent::StartBoardSpinAmbient);
                }
                game::AudioEvent::StopTensionMusic => {
                    // Stop board spin ambient along with tension
                    self.audio_engine
                        .handle_extended_event(ctx, ExtendedAudioEvent::StopBoardSpinAmbient);
                }
                _ => {}
            }
        }
        self.audio_engine.update(ctx, delta_time)?;

        // Update graphics animations
        self.graphics_renderer
            .update_animations(&self.game_state, delta_time);

        // Update screen effects (shake, flash)
        self.screen_effects.update(delta_time);

        // Update particle system
        self.particle_system.update(delta_time);

        // Update Whammy animator if playing
        if self.whammy_animator.is_playing() {
            self.whammy_animator.update(
                delta_time,
                &mut self.screen_effects,
                &mut self.particle_system,
            );
        }

        // Check if game triggered a Whammy animation
        if self.game_state.whammy_animation.active && !self.whammy_animator.is_playing() {
            // Calculate center stage position
            let (screen_w, screen_h) = ctx.gfx.drawable_size();
            let center_x = screen_w / 2.0;
            let center_y = screen_h / 2.0;

            // Start random Whammy animation
            self.whammy_animator.play_random_all(center_x, center_y);

            // Trigger Whammy screen effect
            self.screen_effects.whammy_effect();

            // Play Whammy catchphrase using ExtendedAudioEvent
            let whammy_type = WhammyAnimationType::random();
            self.audio_engine
                .handle_extended_event(ctx, ExtendedAudioEvent::WhammyCatchphrase(whammy_type));

            // Trigger audience gasp reaction based on contestant's current score
            let contestant = &self.game_state.contestants[self.game_state.current_player];
            let intensity = ReactionIntensity::from_win_amount(contestant.score);
            self.audio_engine
                .handle_extended_event(ctx, ExtendedAudioEvent::AudienceGaspIntensity(intensity));
        }

        // Sync Whammy animation state back to game state
        if !self.whammy_animator.is_playing() && self.game_state.whammy_animation.active {
            // Animation completed
            self.game_state.whammy_animation.active = false;
            self.game_state.whammy_animation.progress = 1.0;
        }

        // Update UI animation player if active
        if let Some(ref mut ui_player) = self.ui_animation_player {
            ui_player.update(delta_time);

            // Process any animation events from the UI player
            let events = ui_player.take_events();
            for event in &events {
                // Process the event for potential side effects
                let _event_info = self.process_animation_event(event);
            }
        }

        // Debug: verify all animation types are available for use
        #[cfg(debug_assertions)]
        {
            // Verify SpriteAtlas type is available
            let _atlas_type = std::any::type_name::<SpriteAtlas>();

            // Verify animation library type
            let _lib_type = std::any::type_name::<animation::WhammyAnimationLibrary>();

            // Verify WhammyState type - use value to avoid warning
            let _whammy_state = self.get_whammy_state();

            // Verify playback state tracking
            let _ui_state = self.get_ui_playback_state();

            // Verify color constants are accessible
            let _primary = Self::whammy_primary_color();
            let _shadow = Self::whammy_shadow_color();
            let _highlight = Self::whammy_highlight_color();
            let _outline = Self::whammy_outline_color();

            // Verify atlas manager is available
            let _atlas_count = self.atlas_manager.ids().len();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear with deep purple background
        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, Color::from_rgba(10, 5, 32, 255));

        // Get screen dimensions
        let (screen_w, screen_h) = ctx.gfx.drawable_size();

        // Apply screen shake offset if active
        let shake_offset = self.screen_effects.get_shake_offset();
        if shake_offset.x.abs() > 0.01 || shake_offset.y.abs() > 0.01 {
            canvas.set_screen_coordinates(ggez::graphics::Rect::new(
                -shake_offset.x,
                -shake_offset.y,
                screen_w,
                screen_h,
            ));
        }

        // Draw game elements in Z-order (back to front)
        self.graphics_renderer
            .draw_background(&mut canvas, ctx, screen_w, screen_h);
        self.graphics_renderer
            .draw_header(&mut canvas, ctx, screen_w, screen_h);
        self.graphics_renderer
            .draw_podiums(&mut canvas, ctx, &self.game_state, screen_w, screen_h);
        self.graphics_renderer.draw_status_bar(
            &mut canvas,
            ctx,
            &self.game_state,
            screen_w,
            screen_h,
        );
        self.graphics_renderer.draw_big_board(
            &mut canvas,
            ctx,
            &self.game_state,
            screen_w,
            screen_h,
        );

        // Draw center stage (including Whammy from new animator if playing)
        if self.whammy_animator.is_playing() {
            // Draw Whammy using new animator
            self.whammy_animator.draw(&mut canvas, ctx);

            // Draw taunt text if available
            if !self.whammy_animator.taunt_text.is_empty() {
                self.graphics_renderer.draw_whammy_taunt(
                    &mut canvas,
                    ctx,
                    &self.whammy_animator.taunt_text,
                    screen_w,
                    screen_h,
                );
            }
        } else {
            // Use legacy center stage drawing
            self.graphics_renderer.draw_center_stage(
                &mut canvas,
                ctx,
                &self.game_state,
                screen_w,
                screen_h,
            );
        }

        // Draw particles (after Whammy, before UI)
        self.particle_system.draw(&mut canvas, ctx);

        self.graphics_renderer.draw_action_buttons(
            &mut canvas,
            ctx,
            &self.game_state,
            screen_w,
            screen_h,
        );
        self.graphics_renderer.draw_controls(
            &mut canvas,
            ctx,
            &self.game_state,
            screen_w,
            screen_h,
        );
        self.graphics_renderer.draw_message_display(
            &mut canvas,
            ctx,
            &self.game_state,
            screen_w,
            screen_h,
        );

        // CRT effect overlay
        self.graphics_renderer
            .draw_crt_overlay(&mut canvas, ctx, screen_w, screen_h);

        // UI overlay
        self.ui_manager
            .draw(&mut canvas, ctx, &self.game_state, screen_w, screen_h);

        // Screen effects overlay (flash, vignette)
        self.screen_effects
            .draw(&mut canvas, ctx, screen_w, screen_h);

        // Debug info in development builds
        #[cfg(debug_assertions)]
        self.graphics_renderer.draw_debug_info(
            &mut canvas,
            ctx,
            &self.game_state,
            screen_w,
            screen_h,
        );

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        if let PhysicalKey::Code(keycode) = input.event.physical_key {
            if let Some(action) = self.process_input(keycode) {
                match action {
                    InputAction::Quit => ctx.request_quit(),
                    _ => {
                        // Play button click sound for most actions
                        if matches!(
                            action,
                            InputAction::StartSpin
                                | InputAction::StopSpin
                                | InputAction::Pass
                                | InputAction::SelectAnswer(_)
                                | InputAction::StartGame
                        ) {
                            self.audio_engine.play_button_click(ctx);
                        }
                        // Handle input and get audio events
                        let input_audio_events = self.game_state.handle_input(action);
                        for event in input_audio_events {
                            self.audio_engine.handle_event(ctx, event);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

fn main() -> GameResult {
    // Print startup banner
    println!("======================================================================");
    println!("              PRESS YOUR LUCK - THE BIG BOARD EXPERIENCE              ");
    println!("                  ~ Authentic 1983-1986 CBS Recreation ~              ");
    println!("======================================================================");
    println!("  Controls:                                                           ");
    println!("    SPACE  - Start/Stop spin, Continue                                ");
    println!("    P      - Pass spins to opponent                                   ");
    println!("    B      - Buzz in during questions                                 ");
    println!("    1-4    - Select answer or corner                                  ");
    println!("    ENTER  - Confirm selection                                        ");
    println!("    ESC    - Quit game                                                ");
    println!("======================================================================");

    // Build context with window configuration
    let (mut ctx, event_loop) = ContextBuilder::new("press-your-luck", "DoubleGate")
        .window_setup(WindowSetup::default().title("Press Your Luck - Big Bucks! No Whammies!"))
        .window_mode(
            WindowMode::default()
                .dimensions(1280.0, 800.0)
                .resizable(true),
        )
        .build()?;

    // Create game state
    let game = PressYourLuck::new(&mut ctx)?;

    // Run the game loop
    event::run(ctx, event_loop, game)
}
