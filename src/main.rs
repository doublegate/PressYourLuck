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

use macroquad::prelude::*;

mod game;
mod audio;
mod graphics;
mod ui;

use game::{GameState, GamePhase, InputAction};
use audio::AudioEngine;
use graphics::GraphicsRenderer;
use ui::UiManager;

/// Window configuration for the game
/// 
/// # Display Settings
/// - Resolution: 1280x800 (16:10 aspect ratio, ideal for modern displays)
/// - MSAA: 4x anti-aliasing for smooth edges
/// - VSync: Enabled for tear-free rendering
fn window_conf() -> Conf {
    Conf {
        window_title: "Press Your Luck - Big Bucks! No Whammies!".to_string(),
        window_width: 1280,
        window_height: 800,
        high_dpi: true,
        fullscreen: false,
        sample_count: 4, // 4x MSAA
        window_resizable: true,
        ..Default::default()
    }
}

/// Main entry point using macroquad's async runtime
/// 
/// # Game Loop
/// The main loop runs at the display's refresh rate (typically 60 Hz) and consists of:
/// 1. **Input Processing**: Keyboard and mouse state polling
/// 2. **State Update**: Game logic, timers, animations
/// 3. **Audio Update**: Sound playback and synthesis
/// 4. **Rendering**: Graphics drawing in correct Z-order
/// 5. **UI Overlay**: Questions, messages, controls
#[macroquad::main(window_conf)]
async fn main() {
    // Print startup banner
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║              PRESS YOUR LUCK - THE BIG BOARD EXPERIENCE              ║");
    println!("║                  ~ Authentic 1983-1986 CBS Recreation ~              ║");
    println!("╠══════════════════════════════════════════════════════════════════════╣");
    println!("║  Controls:                                                           ║");
    println!("║    SPACE  - Start/Stop spin, Continue                                ║");
    println!("║    P      - Pass spins to opponent                                   ║");
    println!("║    B      - Buzz in during questions                                 ║");
    println!("║    1-4    - Select answer or corner                                  ║");
    println!("║    ENTER  - Confirm selection                                        ║");
    println!("║    ESC    - Quit game                                                ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");
    
    // Initialize game subsystems
    let mut game_state = GameState::new();
    let mut audio_engine = AudioEngine::new();
    let mut graphics_renderer = GraphicsRenderer::new();
    let mut ui_manager = UiManager::new();
    
    // Track frame timing for smooth animations
    let mut last_frame_time = get_time();
    
    // Main game loop
    loop {
        // Calculate delta time for frame-rate independent updates
        let current_time = get_time();
        let delta_time = (current_time - last_frame_time) as f32;
        last_frame_time = current_time;
        
        // Cap delta time to prevent physics explosions on lag spikes
        let delta_time = delta_time.min(0.1);
        
        // ─────────────────────────────────────────────────────────────────────
        // INPUT PROCESSING
        // ─────────────────────────────────────────────────────────────────────
        let mut input_audio_events = Vec::new();
        if let Some(action) = process_input(&game_state) {
            match action {
                InputAction::Quit => break,
                _ => {
                    // Play button click sound for most actions
                    if matches!(action,
                        InputAction::StartSpin |
                        InputAction::StopSpin |
                        InputAction::Pass |
                        InputAction::SelectAnswer(_) |
                        InputAction::StartGame
                    ) {
                        audio_engine.play_button_click();
                    }
                    // handle_input now returns audio events
                    input_audio_events = game_state.handle_input(action);
                }
            }
        }

        // Play audio events from input handling
        for event in input_audio_events {
            audio_engine.handle_event(event);
        }
        
        // ─────────────────────────────────────────────────────────────────────
        // GAME STATE UPDATE
        // ─────────────────────────────────────────────────────────────────────
        let audio_events = game_state.update(delta_time);
        
        // ─────────────────────────────────────────────────────────────────────
        // AUDIO UPDATE
        // ─────────────────────────────────────────────────────────────────────
        for event in audio_events {
            audio_engine.handle_event(event);
        }
        audio_engine.update(delta_time);
        
        // ─────────────────────────────────────────────────────────────────────
        // GRAPHICS ANIMATION UPDATE
        // ─────────────────────────────────────────────────────────────────────
        graphics_renderer.update_animations(&game_state, delta_time);

        // ─────────────────────────────────────────────────────────────────────
        // RENDERING
        // ─────────────────────────────────────────────────────────────────────

        // Clear with deep purple background
        clear_background(Color::from_rgba(10, 5, 32, 255));

        // Draw game elements in Z-order (back to front)
        graphics_renderer.draw_background();
        graphics_renderer.draw_header();
        graphics_renderer.draw_podiums(&game_state);
        graphics_renderer.draw_status_bar(&game_state);
        graphics_renderer.draw_big_board(&game_state);
        graphics_renderer.draw_center_stage(&game_state);
        graphics_renderer.draw_action_buttons(&game_state);
        graphics_renderer.draw_controls(&game_state);
        graphics_renderer.draw_message_display(&game_state);

        // ─────────────────────────────────────────────────────────────────────
        // CRT EFFECT OVERLAY (authentic 1980s television look)
        // ─────────────────────────────────────────────────────────────────────
        graphics_renderer.draw_crt_overlay();

        // ─────────────────────────────────────────────────────────────────────
        // UI OVERLAY
        // ─────────────────────────────────────────────────────────────────────
        ui_manager.draw(&game_state);
        
        // Draw debug info in development builds
        #[cfg(debug_assertions)]
        draw_debug_info(&game_state, delta_time);
        
        // Present frame
        next_frame().await;
    }
    
    println!("\n★ Thanks for playing Press Your Luck! ★\n");
}

/// Process keyboard and mouse input based on current game phase
/// 
/// # Input Mapping
/// Different game phases respond to different inputs:
/// - **Start**: SPACE/ENTER to begin
/// - **Questions**: B to buzz in, 1-4 for answers
/// - **Board**: SPACE to spin/stop, P to pass
/// - **GameOver**: SPACE/ENTER to restart
/// 
/// # Arguments
/// * `game_state` - Current game state for context-sensitive input
/// 
/// # Returns
/// * `Some(InputAction)` - Action to process
/// * `None` - No action this frame
fn process_input(game_state: &GameState) -> Option<InputAction> {
    // Global quit
    if is_key_pressed(KeyCode::Escape) {
        return Some(InputAction::Quit);
    }
    
    match game_state.phase {
        GamePhase::Start => {
            // Start game
            if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                return Some(InputAction::StartGame);
            }
        }
        
        GamePhase::Questions => {
            // Buzz in
            if is_key_pressed(KeyCode::B) {
                return Some(InputAction::BuzzIn);
            }
            
            // Answer selection (when choices are showing)
            if is_key_pressed(KeyCode::Key1) {
                return Some(InputAction::SelectAnswer(0));
            }
            if is_key_pressed(KeyCode::Key2) {
                return Some(InputAction::SelectAnswer(1));
            }
            if is_key_pressed(KeyCode::Key3) {
                return Some(InputAction::SelectAnswer(2));
            }
            if is_key_pressed(KeyCode::Key4) {
                return Some(InputAction::SelectAnswer(3));
            }
            
            // Continue/skip
            if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                return Some(InputAction::Continue);
            }
        }
        
        GamePhase::Board => {
            // Spin control
            if game_state.is_spinning {
                if is_key_pressed(KeyCode::Space) {
                    return Some(InputAction::StopSpin);
                }
            } else {
                if is_key_pressed(KeyCode::Space) {
                    return Some(InputAction::StartSpin);
                }
                
                // Pass spins
                if is_key_pressed(KeyCode::P) {
                    return Some(InputAction::Pass);
                }
            }
            
            // Corner selection (for Pick a Corner)
            if game_state.awaiting_corner_selection {
                if is_key_pressed(KeyCode::Key1) {
                    return Some(InputAction::SelectCorner(0));
                }
                if is_key_pressed(KeyCode::Key2) {
                    return Some(InputAction::SelectCorner(1));
                }
                if is_key_pressed(KeyCode::Key3) {
                    return Some(InputAction::SelectCorner(2));
                }
                if is_key_pressed(KeyCode::Key4) {
                    return Some(InputAction::SelectCorner(3));
                }
            }
            
            // Special choice (for $2000 or Lose Whammy)
            if game_state.awaiting_special_choice {
                if is_key_pressed(KeyCode::Key1) {
                    return Some(InputAction::SelectSpecialChoice(0));
                }
                if is_key_pressed(KeyCode::Key2) {
                    return Some(InputAction::SelectSpecialChoice(1));
                }
            }
            
            // Continue after result
            if is_key_pressed(KeyCode::Enter) {
                return Some(InputAction::Continue);
            }
        }
        
        GamePhase::GameOver => {
            // Restart game
            if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                return Some(InputAction::StartGame);
            }
        }
    }
    
    None
}

/// Draw debug information overlay (development builds only)
/// 
/// # Displayed Information
/// - FPS counter
/// - Frame time
/// - Current game phase
/// - Active player
/// - Spin state
#[cfg(debug_assertions)]
fn draw_debug_info(game_state: &GameState, delta_time: f32) {
    let fps = get_fps();
    let debug_text = format!(
        "FPS: {} | Frame: {:.2}ms | Phase: {:?} | Player: {} | Spinning: {}",
        fps,
        delta_time * 1000.0,
        game_state.phase,
        game_state.current_player,
        game_state.is_spinning
    );
    
    draw_rectangle(0.0, screen_height() - 25.0, screen_width(), 25.0, Color::from_rgba(0, 0, 0, 200));
    draw_text(&debug_text, 10.0, screen_height() - 8.0, 16.0, WHITE);
}
