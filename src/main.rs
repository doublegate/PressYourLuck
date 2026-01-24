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
    Context, ContextBuilder, GameResult,
};

mod audio;
mod game;
mod gfx;
mod ui;

use audio::AudioEngine;
use game::{GamePhase, GameState, InputAction};
use gfx::GraphicsRenderer;
use ui::UiManager;

/// Main game state structure implementing ggez EventHandler
struct PressYourLuck {
    game_state: GameState,
    audio_engine: AudioEngine,
    graphics_renderer: GraphicsRenderer,
    ui_manager: UiManager,
}

impl PressYourLuck {
    /// Create a new game instance
    fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            game_state: GameState::new(),
            audio_engine: AudioEngine::new(ctx)?,
            graphics_renderer: GraphicsRenderer::new(),
            ui_manager: UiManager::new(),
        })
    }

    /// Process keyboard input based on current game phase
    fn process_input(&self, keycode: KeyCode) -> Option<InputAction> {
        // Global quit
        if keycode == KeyCode::Escape {
            return Some(InputAction::Quit);
        }

        match self.game_state.phase {
            GamePhase::Start => {
                if keycode == KeyCode::Space || keycode == KeyCode::Return {
                    return Some(InputAction::StartGame);
                }
            }

            GamePhase::Questions => {
                if keycode == KeyCode::B {
                    return Some(InputAction::BuzzIn);
                }

                // Answer selection (when choices are showing)
                match keycode {
                    KeyCode::Key1 => return Some(InputAction::SelectAnswer(0)),
                    KeyCode::Key2 => return Some(InputAction::SelectAnswer(1)),
                    KeyCode::Key3 => return Some(InputAction::SelectAnswer(2)),
                    KeyCode::Key4 => return Some(InputAction::SelectAnswer(3)),
                    KeyCode::Space | KeyCode::Return => return Some(InputAction::Continue),
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

                    if keycode == KeyCode::P {
                        return Some(InputAction::Pass);
                    }
                }

                // Corner selection (for Pick a Corner)
                if self.game_state.awaiting_corner_selection {
                    match keycode {
                        KeyCode::Key1 => return Some(InputAction::SelectCorner(0)),
                        KeyCode::Key2 => return Some(InputAction::SelectCorner(1)),
                        KeyCode::Key3 => return Some(InputAction::SelectCorner(2)),
                        KeyCode::Key4 => return Some(InputAction::SelectCorner(3)),
                        _ => {}
                    }
                }

                // Special choice (for $2000 or Lose Whammy)
                if self.game_state.awaiting_special_choice {
                    match keycode {
                        KeyCode::Key1 => return Some(InputAction::SelectSpecialChoice(0)),
                        KeyCode::Key2 => return Some(InputAction::SelectSpecialChoice(1)),
                        _ => {}
                    }
                }

                // Continue after result
                if keycode == KeyCode::Return {
                    return Some(InputAction::Continue);
                }
            }

            GamePhase::GameOver => {
                if keycode == KeyCode::Space || keycode == KeyCode::Return {
                    return Some(InputAction::StartGame);
                }
            }
        }

        None
    }
}

impl EventHandler for PressYourLuck {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta_time = ctx.time.delta().as_secs_f32().min(0.1);

        // Update game state
        let audio_events = self.game_state.update(delta_time);

        // Handle audio events
        for event in audio_events {
            self.audio_engine.handle_event(ctx, event);
        }
        self.audio_engine.update(ctx, delta_time)?;

        // Update graphics animations
        self.graphics_renderer
            .update_animations(&self.game_state, delta_time);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear with deep purple background
        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, Color::from_rgba(10, 5, 32, 255));

        // Get screen dimensions
        let (screen_w, screen_h) = ctx.gfx.drawable_size();

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
        self.graphics_renderer.draw_center_stage(
            &mut canvas,
            ctx,
            &self.game_state,
            screen_w,
            screen_h,
        );
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
        if let Some(keycode) = input.keycode {
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
    let (mut ctx, event_loop) = ContextBuilder::new("press-your-luck", "Luke Parobek")
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
