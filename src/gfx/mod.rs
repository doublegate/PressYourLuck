//! # Graphics Renderer Module
//!
//! ## Overview
//! This module handles all visual rendering for Press Your Luck,
//! recreating the authentic 1983-1986 CBS game show aesthetic.
//!
//! ## Visual Design
//! The original show featured:
//! - **Big Board**: 18 perimeter squares with cycling prizes
//! - **Center Stage**: Whammy animation area
//! - **Rainbow Borders**: Animated gradient outlines
//! - **Neon Glow Effects**: Bright, flashy 80s aesthetics
//!
//! ## Color Palette (Authentic CBS)
//! - Gold: #FFD700 (primary highlight)
//! - Hot Pink: #FF1493 (secondary highlight)
//! - Electric Blue: #00BFFF (accent)
//! - Deep Purple: #0A0520 (background)
//!
//! ## Rendering Layers (Back to Front)
//! 1. Background gradient
//! 2. Header/logo
//! 3. Big Board squares
//! 4. Center stage / Whammy
//! 5. Contestant podiums
//! 6. Message display
//! 7. UI overlays

use ggez::{
    glam::Vec2,
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text, TextFragment},
    Context,
};

use crate::game::{Contestant, GamePhase, GameState, PrizeType, WhammyAnimationType};

// ===============================================================================
// COLOR CONSTANTS (Authentic 1983-1986 CRT Television Palette)
// ===============================================================================
// CRT televisions of the era had distinctive characteristics:
// - Warm color temperature (slight red/orange tint)
// - Phosphor bloom (colors slightly bleed outward)
// - Slightly elevated black levels (deep purple rather than true black)
// - Saturated, vibrant primaries with soft falloff
// ===============================================================================

/// Deep purple background (CRT black level - never truly black)
const BG_COLOR: Color = Color::new(0.04, 0.02, 0.13, 1.0);

/// Gold highlight (warm CRT gold with slight red push)
const GOLD: Color = Color::new(1.0, 0.82, 0.08, 1.0);

/// Hot pink highlight (authentic 80s neon with CRT warmth)
#[allow(dead_code)]
const HOT_PINK: Color = Color::new(1.0, 0.12, 0.55, 1.0);

/// Electric blue accent (slightly cyan-shifted for CRT phosphor)
const ELECTRIC_BLUE: Color = Color::new(0.05, 0.78, 1.0, 1.0);

/// Cash square green (CRT green - slightly yellow-shifted)
const CASH_GREEN: Color = Color::new(0.08, 0.65, 0.22, 1.0);

/// Cash square green (secondary, CRT shadow)
const CASH_GREEN_DARK: Color = Color::new(0.04, 0.42, 0.14, 1.0);

/// Prize square blue (CRT blue phosphor - slightly purple)
const PRIZE_BLUE: Color = Color::new(0.06, 0.35, 0.72, 1.0);

/// Prize square blue (secondary, CRT shadow)
const PRIZE_BLUE_DARK: Color = Color::new(0.03, 0.22, 0.45, 1.0);

/// Special square gold (CRT warm gold)
const SPECIAL_GOLD: Color = Color::new(0.85, 0.62, 0.05, 1.0);

/// Special square gold (secondary, CRT shadow)
const SPECIAL_GOLD_DARK: Color = Color::new(0.62, 0.42, 0.03, 1.0);

/// Whammy square red (CRT red phosphor - vivid)
const WHAMMY_RED: Color = Color::new(0.88, 0.08, 0.08, 1.0);

/// Whammy square red (secondary, CRT shadow)
const WHAMMY_RED_DARK: Color = Color::new(0.58, 0.04, 0.04, 1.0);

/// White color
const WHITE: Color = Color::WHITE;

/// Black color
const BLACK: Color = Color::BLACK;

/// Contestant colors (CRT-warm player indicators)
const PLAYER_COLORS: [Color; 3] = [
    Color::new(1.0, 0.35, 0.28, 1.0),  // Red (slightly orange CRT red)
    Color::new(0.28, 0.95, 0.35, 1.0), // Green (CRT green phosphor)
    Color::new(0.35, 0.38, 1.0, 1.0),  // Blue (CRT blue with slight purple)
];

/// CRT scanline overlay color (subtle horizontal lines)
const CRT_SCANLINE: Color = Color::new(0.0, 0.0, 0.0, 0.08);

// ===============================================================================
// LAYOUT CONSTANTS
// ===============================================================================

/// Board layout: 6 columns x 5 rows (but only perimeter squares are active)
const BOARD_COLS: usize = 6;
const BOARD_ROWS: usize = 5;

/// Board square positions (clockwise from top-left)
/// Format: (column, row) for each of the 18 squares
const SQUARE_POSITIONS: [(usize, usize); 18] = [
    (0, 0),
    (1, 0),
    (2, 0),
    (3, 0),
    (4, 0),
    (5, 0), // Top row (0-5)
    (5, 1),
    (5, 2),
    (5, 3), // Right column (6-8)
    (5, 4),
    (4, 4),
    (3, 4),
    (2, 4),
    (1, 4),
    (0, 4), // Bottom row (9-14)
    (0, 3),
    (0, 2),
    (0, 1), // Left column (15-17)
];

// ===============================================================================
// CHASE LIGHT CONSTANTS (Authentic 1983-1986 CBS Show)
// ===============================================================================

/// Number of chase lights around the board perimeter
const CHASE_LIGHT_COUNT: usize = 48;

/// Chase light bulb color (warm incandescent yellow-white)
const CHASE_LIGHT_ON: Color = Color::new(1.0, 0.95, 0.7, 1.0);

/// Chase light bulb off color (dim gray)
const CHASE_LIGHT_OFF: Color = Color::new(0.2, 0.18, 0.15, 0.6);

/// Chase light bulb glow color
const CHASE_LIGHT_GLOW: Color = Color::new(1.0, 0.9, 0.5, 0.4);

// ===============================================================================
// GRAPHICS RENDERER
// ===============================================================================

/// Main graphics renderer for the game
///
/// # Responsibilities
/// - Background rendering with animated effects
/// - Big Board display with cycling prizes
/// - Contestant podiums with scores
/// - Whammy character animations
/// - Message and notification display
/// - Control hints
/// - Authentic chase lights around board perimeter
/// - LED-style score displays
pub struct GraphicsRenderer {
    /// Animation timer for effects
    time: f32,

    /// Rainbow border animation phase
    rainbow_phase: f32,

    /// Header shimmer effect phase
    shimmer_phase: f32,

    /// Cached board layout
    board_rect: Option<Rect>,

    /// Chase light animation phase (0.0 - 1.0, cycles through all lights)
    chase_light_phase: f32,

    /// Current score display values (for rolling animation)
    displayed_scores: [f32; 3],

    /// Previous lit square for trail effect
    prev_lit_square: usize,

    /// Square flash intensities for spin effects (18 squares)
    square_flash: [f32; 18],
}

impl GraphicsRenderer {
    /// Create a new graphics renderer
    pub fn new() -> Self {
        Self {
            time: 0.0,
            rainbow_phase: 0.0,
            shimmer_phase: 0.0,
            board_rect: None,
            chase_light_phase: 0.0,
            displayed_scores: [0.0; 3],
            prev_lit_square: 0,
            square_flash: [0.0; 18],
        }
    }

    /// Calculate board layout based on screen size
    fn calculate_board_rect(&mut self, screen_w: f32, screen_h: f32) -> Rect {
        if let Some(rect) = self.board_rect {
            return rect;
        }

        // Board occupies center of screen
        let board_w = screen_w * 0.65;
        let board_h = screen_h * 0.55;
        let board_x = (screen_w - board_w) / 2.0;
        let board_y = screen_h * 0.20;

        let rect = Rect::new(board_x, board_y, board_w, board_h);
        self.board_rect = Some(rect);
        rect
    }

    /// Update animation states based on game state
    /// Call this before drawing to update flash effects
    pub fn update_animations(&mut self, game_state: &GameState, delta: f32) {
        self.time += delta;
        self.rainbow_phase += delta * 0.5;
        self.shimmer_phase += delta * 2.0;

        // Update chase light animation
        // Speed up during spins, slow down otherwise
        let chase_speed = if game_state.is_spinning {
            // Chase lights run faster during spin for excitement
            8.0 + game_state.spin_speed * 0.1
        } else {
            // Slow ambient chase when idle
            2.0
        };
        self.chase_light_phase += delta * chase_speed;
        if self.chase_light_phase >= CHASE_LIGHT_COUNT as f32 {
            self.chase_light_phase -= CHASE_LIGHT_COUNT as f32;
        }

        // Update square flash effects
        for i in 0..18 {
            // Decay flash over time
            self.square_flash[i] = (self.square_flash[i] - delta * 4.0).max(0.0);
        }

        // Add flash to lit square during spin
        if game_state.is_spinning {
            let lit = game_state.lit_square;
            self.square_flash[lit] = 1.0;
            // Add diminishing trail flash
            let trail_len = 3;
            for t in 1..=trail_len {
                let trail_idx = if lit >= t { lit - t } else { 18 - (t - lit) };
                let trail_intensity = 1.0 - (t as f32 / trail_len as f32) * 0.7;
                self.square_flash[trail_idx] =
                    self.square_flash[trail_idx].max(trail_intensity * 0.5);
            }
            self.prev_lit_square = lit;
        }

        // Update displayed scores with rolling animation
        for (i, contestant) in game_state.contestants.iter().enumerate() {
            let target = contestant.score as f32;
            let current = self.displayed_scores[i];
            if current < target {
                // Roll up quickly at first, then slow down
                let diff = target - current;
                let speed = (diff * 0.15).max(50.0).min(diff);
                self.displayed_scores[i] = (current + speed * delta * 60.0).min(target);
            } else if current > target {
                // Instant reset when score goes down (Whammy)
                self.displayed_scores[i] = target;
            }
        }
    }

    /// Draw the animated background
    pub fn draw_background(
        &mut self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        screen_w: f32,
        screen_h: f32,
    ) {
        // Draw gradient background using BG_COLOR as base with CRT warmth
        // For simplicity in ggez, we draw horizontal lines for gradient
        for y in 0..(screen_h as i32) {
            let t = y as f32 / screen_h;
            // Add slight CRT phosphor warmth
            let warmth = (1.0 - (t - 0.5).abs() * 2.0).max(0.0) * 0.02;
            let color = Color::new(
                (BG_COLOR.r + 0.02 * t + warmth).min(1.0),
                BG_COLOR.g + 0.01 * t,
                (BG_COLOR.b + 0.05 * t).min(1.0),
                1.0,
            );

            if let Ok(line) =
                Mesh::new_line(ctx, &[[0.0, y as f32], [screen_w, y as f32]], 1.0, color)
            {
                canvas.draw(&line, DrawParam::default());
            }
        }

        // Pulsing glow in center (enhanced for CRT bloom effect)
        let pulse = (self.time * 2.0).sin() * 0.5 + 0.5;
        let glow_radius = screen_w.min(screen_h) * 0.45;
        let center_x = screen_w / 2.0;
        let center_y = screen_h / 2.0;

        // Outer purple haze (CRT bloom simulation)
        for r in (0..(glow_radius as i32)).step_by(8) {
            let t = r as f32 / glow_radius;
            let alpha = (1.0 - t) * 0.06 * pulse;
            if let Ok(circle) = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                [center_x, center_y],
                r as f32,
                0.5,
                Color::new(0.6, 0.1, 0.7, alpha),
            ) {
                canvas.draw(&circle, DrawParam::default());
            }
        }
    }

    /// Draw the game header with rainbow shimmer effect
    pub fn draw_header(
        &self,
        canvas: &mut Canvas,
        _ctx: &mut Context,
        screen_w: f32,
        screen_h: f32,
    ) {
        let title = "PRESS YOUR LUCK";

        // Calculate font size based on screen width
        let font_size = (screen_w * 0.05).min(60.0);
        let y = screen_h * 0.08;

        // Draw each character with rainbow shimmer
        let chars: Vec<char> = title.chars().collect();
        let char_width = font_size * 0.6; // Approximate character width
        let total_width = chars.len() as f32 * char_width;
        let mut current_x = (screen_w - total_width) / 2.0;

        for (i, c) in chars.iter().enumerate() {
            let char_str = c.to_string();

            // Calculate rainbow color based on position and time
            let hue = (self.shimmer_phase + i as f32 * 0.15) % 1.0;
            let color = hsv_to_rgb(hue, 0.8, 1.0);

            // Draw shadow
            let shadow_text = Text::new(TextFragment::new(&char_str).scale(font_size));
            canvas.draw(
                &shadow_text,
                DrawParam::default()
                    .dest([current_x + 3.0, y + 3.0])
                    .color(Color::new(0.0, 0.0, 0.0, 0.5)),
            );

            // Draw character
            let text = Text::new(TextFragment::new(&char_str).scale(font_size));
            canvas.draw(
                &text,
                DrawParam::default().dest([current_x, y]).color(color),
            );

            current_x += char_width;
        }

        // Subtitle
        let subtitle = "~ BIG BUCKS! NO WHAMMIES! ~";
        let sub_size = font_size * 0.4;
        let sub_text = Text::new(TextFragment::new(subtitle).scale(sub_size));
        let sub_width = subtitle.len() as f32 * sub_size * 0.5;
        canvas.draw(
            &sub_text,
            DrawParam::default()
                .dest([(screen_w - sub_width) / 2.0, y + font_size * 0.6])
                .color(GOLD),
        );
    }

    /// Draw the contestant podiums
    pub fn draw_podiums(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        game_state: &GameState,
        screen_w: f32,
        screen_h: f32,
    ) {
        let podium_width = screen_w * 0.22;
        let podium_height = screen_h * 0.12;
        let podium_y = screen_h * 0.85;
        let spacing = (screen_w - podium_width * 3.0) / 4.0;

        for (i, contestant) in game_state.contestants.iter().enumerate() {
            let x = spacing + (podium_width + spacing) * i as f32;

            self.draw_single_podium(
                canvas,
                ctx,
                x,
                podium_y,
                podium_width,
                podium_height,
                contestant,
                i == game_state.current_player,
                i,
            );
        }
    }

    /// Draw a single contestant podium
    #[allow(clippy::too_many_arguments)]
    fn draw_single_podium(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        contestant: &Contestant,
        is_active: bool,
        player_index: usize,
    ) {
        let player_color = PLAYER_COLORS[player_index];

        // Podium background
        let bg_color = if is_active {
            Color::new(
                player_color.r * 0.25,
                player_color.g * 0.25,
                player_color.b * 0.25,
                0.95,
            )
        } else {
            Color::new(0.12, 0.10, 0.18, 0.95)
        };

        if let Ok(rect) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(x, y, width, height),
            bg_color,
        ) {
            canvas.draw(&rect, DrawParam::default());
        }

        // Metallic trim border
        let trim_color = if is_active {
            GOLD
        } else {
            Color::new(0.6, 0.55, 0.5, 1.0)
        };

        if let Ok(border) = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(4.0),
            Rect::new(x, y, width, height),
            trim_color,
        ) {
            canvas.draw(&border, DrawParam::default());
        }

        // Active indicator glow
        if is_active {
            let pulse = (self.time * 4.0).sin() * 0.4 + 0.6;
            if let Ok(glow) = Mesh::new_rectangle(
                ctx,
                DrawMode::stroke(3.0),
                Rect::new(x - 4.0, y - 4.0, width + 8.0, height + 8.0),
                Color::new(1.0, 0.84, 0.0, pulse * 0.6),
            ) {
                canvas.draw(&glow, DrawParam::default());
            }
        }

        // Eliminated overlay
        if contestant.eliminated {
            if let Ok(overlay) = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(x, y, width, height),
                Color::new(0.0, 0.0, 0.0, 0.8),
            ) {
                canvas.draw(&overlay, DrawParam::default());
            }

            // Red X pattern
            if let Ok(line1) = Mesh::new_line(
                ctx,
                &[[x, y], [x + width, y + height]],
                4.0,
                Color::new(0.8, 0.0, 0.0, 0.6),
            ) {
                canvas.draw(&line1, DrawParam::default());
            }
            if let Ok(line2) = Mesh::new_line(
                ctx,
                &[[x + width, y], [x, y + height]],
                4.0,
                Color::new(0.8, 0.0, 0.0, 0.6),
            ) {
                canvas.draw(&line2, DrawParam::default());
            }

            // "WHAMMIED OUT!" text
            let elim_text = "WHAMMIED OUT!";
            let elim_size = width * 0.11;
            let pulse = (self.time * 3.0).sin() * 0.3 + 0.7;
            let text = Text::new(TextFragment::new(elim_text).scale(elim_size));
            canvas.draw(
                &text,
                DrawParam::default()
                    .dest([x + width * 0.1, y + height / 2.0 - elim_size / 2.0])
                    .color(Color::new(1.0, 0.1, 0.1, pulse)),
            );
            return;
        }

        // Name plate
        let name_plate_h = height * 0.18;
        let name_plate_y = y + height * 0.05;
        if let Ok(plate) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(x + 5.0, name_plate_y, width - 10.0, name_plate_h),
            Color::new(0.25, 0.22, 0.3, 1.0),
        ) {
            canvas.draw(&plate, DrawParam::default());
        }

        let name_size = width * 0.11;
        let name_text = Text::new(TextFragment::new(&contestant.name).scale(name_size));
        canvas.draw(
            &name_text,
            DrawParam::default()
                .dest([x + width * 0.15, name_plate_y + name_plate_h * 0.2])
                .color(player_color),
        );

        // LED-style score display
        let displayed_score = self.displayed_scores[player_index] as u32;
        let score_text = format!("${}", format_money(displayed_score));
        self.draw_led_score(
            canvas,
            ctx,
            x + width * 0.08,
            y + height * 0.32,
            width * 0.84,
            height * 0.25,
            &score_text,
            is_active,
        );

        // Spins display
        let total_spins = contestant.earned_spins + contestant.passed_spins;
        let spins_text = format!("{} SPINS", total_spins);
        let spins_size = width * 0.085;
        let text = Text::new(TextFragment::new(&spins_text).scale(spins_size));
        canvas.draw(
            &text,
            DrawParam::default()
                .dest([x + width * 0.25, y + height * 0.62])
                .color(ELECTRIC_BLUE),
        );

        // Earned/Passed detail
        let detail_text = format!(
            "({}E / {}P)",
            contestant.earned_spins, contestant.passed_spins
        );
        let detail_size = width * 0.055;
        let detail = Text::new(TextFragment::new(&detail_text).scale(detail_size));
        canvas.draw(
            &detail,
            DrawParam::default()
                .dest([x + width * 0.25, y + height * 0.72])
                .color(Color::new(0.6, 0.6, 0.7, 0.8)),
        );

        // Whammy indicators (4 LED-style lights)
        let whammy_y = y + height * 0.88;
        let whammy_spacing = width * 0.16;
        let whammy_start = x + (width - whammy_spacing * 3.0) / 2.0;
        let whammy_radius = width * 0.045;

        for w in 0..4 {
            let wx = whammy_start + whammy_spacing * w as f32;
            let filled = w < contestant.whammies as usize;

            let color = if filled {
                WHAMMY_RED
            } else {
                Color::new(0.25, 0.05, 0.05, 0.9)
            };

            if let Ok(circle) = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                [wx, whammy_y],
                whammy_radius,
                0.5,
                color,
            ) {
                canvas.draw(&circle, DrawParam::default());
            }

            // Glow for filled
            if filled {
                if let Ok(glow) = Mesh::new_circle(
                    ctx,
                    DrawMode::fill(),
                    [wx, whammy_y],
                    whammy_radius * 1.5,
                    0.5,
                    Color::new(1.0, 0.0, 0.0, 0.3),
                ) {
                    canvas.draw(&glow, DrawParam::default());
                }
            }
        }
    }

    /// Draw LED-style score display
    #[allow(clippy::too_many_arguments)]
    fn draw_led_score(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        text: &str,
        is_active: bool,
    ) {
        // LED display background
        if let Ok(bg) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(x, y, width, height),
            Color::new(0.02, 0.02, 0.05, 1.0),
        ) {
            canvas.draw(&bg, DrawParam::default());
        }

        if let Ok(border) = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(2.0),
            Rect::new(x, y, width, height),
            Color::new(0.3, 0.3, 0.3, 0.8),
        ) {
            canvas.draw(&border, DrawParam::default());
        }

        // LED text (green for authentic 1980s LED look)
        let text_size = height * 0.7;
        let led_brightness = if is_active {
            (self.time * 60.0).sin() * 0.03 + 0.97
        } else {
            0.85
        };
        let led_color = Color::new(0.1, led_brightness, 0.15, 1.0);

        let display_text = Text::new(TextFragment::new(text).scale(text_size));
        canvas.draw(
            &display_text,
            DrawParam::default()
                .dest([x + width * 0.1, y + height * 0.15])
                .color(led_color),
        );
    }

    /// Draw the status bar showing round info
    pub fn draw_status_bar(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        game_state: &GameState,
        screen_w: f32,
        screen_h: f32,
    ) {
        let bar_y = screen_h * 0.125;

        // Round indicator panel
        let panel_w = screen_w * 0.18;
        let panel_h = screen_h * 0.045;
        let panel_x = (screen_w - panel_w) / 2.0;
        let panel_y = bar_y - panel_h * 0.7;

        // Panel background
        let panel_color = if game_state.round == 1 {
            Color::new(0.0, 0.25, 0.5, 0.95)
        } else {
            Color::new(0.5, 0.35, 0.0, 0.95)
        };

        if let Ok(panel) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(panel_x, panel_y, panel_w, panel_h),
            panel_color,
        ) {
            canvas.draw(&panel, DrawParam::default());
        }

        if let Ok(border) = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(2.0),
            Rect::new(panel_x, panel_y, panel_w, panel_h),
            GOLD,
        ) {
            canvas.draw(&border, DrawParam::default());
        }

        // Round text
        let round_text = format!("ROUND {}", game_state.round);
        let round_size = panel_h * 0.65;
        let text = Text::new(TextFragment::new(&round_text).scale(round_size));
        canvas.draw(
            &text,
            DrawParam::default()
                .dest([panel_x + panel_w * 0.25, panel_y + panel_h * 0.15])
                .color(WHITE),
        );

        // "BIG MONEY!" for Round 2
        if game_state.round == 2 {
            let bonus_text = "BIG MONEY!";
            let bonus_size = screen_w * 0.012;
            let pulse = (self.time * 3.0).sin() * 0.3 + 0.7;
            let bonus = Text::new(TextFragment::new(bonus_text).scale(bonus_size));
            canvas.draw(
                &bonus,
                DrawParam::default()
                    .dest([panel_x + panel_w * 0.25, panel_y + panel_h + 5.0])
                    .color(Color::new(1.0, 0.84, 0.0, pulse)),
            );
        }
    }

    /// Draw PLAY/PASS button indicators
    pub fn draw_action_buttons(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        game_state: &GameState,
        screen_w: f32,
        screen_h: f32,
    ) {
        if game_state.phase != GamePhase::Board || game_state.is_spinning {
            return;
        }

        let button_y = screen_h * 0.76;
        let button_w = screen_w * 0.12;
        let button_h = screen_h * 0.04;
        let button_spacing = screen_w * 0.05;
        let center_x = screen_w / 2.0;

        // PLAY button
        let play_x = center_x - button_spacing - button_w;
        self.draw_action_button(
            canvas,
            ctx,
            play_x,
            button_y,
            button_w,
            button_h,
            "PLAY",
            true,
            Color::new(0.0, 0.6, 0.2, 1.0),
        );

        // PASS button
        let contestant = &game_state.contestants[game_state.current_player];
        let can_pass = contestant.earned_spins > 0 && !contestant.eliminated;
        let pass_x = center_x + button_spacing;
        self.draw_action_button(
            canvas,
            ctx,
            pass_x,
            button_y,
            button_w,
            button_h,
            "PASS",
            can_pass,
            Color::new(0.8, 0.5, 0.0, 1.0),
        );
    }

    /// Draw a single action button
    #[allow(clippy::too_many_arguments)]
    fn draw_action_button(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        label: &str,
        is_active: bool,
        active_color: Color,
    ) {
        let bg_color = if is_active {
            let pulse = (self.time * 3.0).sin() * 0.15 + 0.85;
            Color::new(
                active_color.r * pulse,
                active_color.g * pulse,
                active_color.b * pulse,
                0.95,
            )
        } else {
            Color::new(0.15, 0.15, 0.2, 0.8)
        };

        if let Ok(button) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(x, y, width, height),
            bg_color,
        ) {
            canvas.draw(&button, DrawParam::default());
        }

        // Border
        let border_color = if is_active {
            WHITE
        } else {
            Color::new(0.4, 0.4, 0.4, 0.6)
        };
        if let Ok(border) = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(2.0),
            Rect::new(x, y, width, height),
            border_color,
        ) {
            canvas.draw(&border, DrawParam::default());
        }

        // Label
        let text_size = height * 0.55;
        let text_color = if is_active {
            WHITE
        } else {
            Color::new(0.5, 0.5, 0.5, 0.7)
        };
        let text = Text::new(TextFragment::new(label).scale(text_size));
        canvas.draw(
            &text,
            DrawParam::default()
                .dest([x + width * 0.25, y + height * 0.2])
                .color(text_color),
        );
    }

    /// Draw the Big Board with all 18 squares
    pub fn draw_big_board(
        &mut self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        game_state: &GameState,
        screen_w: f32,
        screen_h: f32,
    ) {
        let board_rect = self.calculate_board_rect(screen_w, screen_h);

        // Draw outer frame
        let frame_margin = 25.0;
        if let Ok(frame) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(
                board_rect.x - frame_margin,
                board_rect.y - frame_margin,
                board_rect.w + frame_margin * 2.0,
                board_rect.h + frame_margin * 2.0,
            ),
            Color::new(0.15, 0.12, 0.2, 1.0),
        ) {
            canvas.draw(&frame, DrawParam::default());
        }

        // Draw board background
        if let Ok(bg) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(
                board_rect.x - 10.0,
                board_rect.y - 10.0,
                board_rect.w + 20.0,
                board_rect.h + 20.0,
            ),
            Color::new(0.05, 0.02, 0.1, 0.95),
        ) {
            canvas.draw(&bg, DrawParam::default());
        }

        // Draw chase lights
        self.draw_chase_lights(
            canvas,
            ctx,
            board_rect.x - frame_margin + 5.0,
            board_rect.y - frame_margin + 5.0,
            board_rect.w + frame_margin * 2.0 - 10.0,
            board_rect.h + frame_margin * 2.0 - 10.0,
            game_state.is_spinning,
        );

        // Draw rainbow border
        self.draw_rainbow_border(
            canvas,
            ctx,
            board_rect.x - 12.0,
            board_rect.y - 12.0,
            board_rect.w + 24.0,
            board_rect.h + 24.0,
        );

        // Calculate square dimensions
        let square_w = board_rect.w / BOARD_COLS as f32;
        let square_h = board_rect.h / BOARD_ROWS as f32;

        // Draw each square
        for (i, (col, row)) in SQUARE_POSITIONS.iter().enumerate() {
            let x = board_rect.x + *col as f32 * square_w;
            let y = board_rect.y + *row as f32 * square_h;

            let is_lit = i == game_state.lit_square;
            let square = &game_state.board[i];
            let flash_intensity = self.square_flash[i];

            self.draw_board_square(
                canvas,
                ctx,
                x,
                y,
                square_w - 4.0,
                square_h - 4.0,
                &square.prizes[square.current_index],
                is_lit,
                flash_intensity,
                game_state.is_spinning,
            );
        }
    }

    /// Draw chase lights around the board perimeter
    #[allow(clippy::too_many_arguments)]
    fn draw_chase_lights(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        is_spinning: bool,
    ) {
        let perimeter = 2.0 * (width + height);
        let light_spacing = perimeter / CHASE_LIGHT_COUNT as f32;
        let bulb_radius = 4.0;

        let on_count = if is_spinning { 8 } else { 4 };

        for i in 0..CHASE_LIGHT_COUNT {
            let dist = i as f32 * light_spacing;
            let (lx, ly) = perimeter_point(x, y, width, height, dist);

            let phase_offset =
                (self.chase_light_phase - i as f32).rem_euclid(CHASE_LIGHT_COUNT as f32);
            let is_on = phase_offset < on_count as f32;

            let brightness = if is_on {
                let falloff = 1.0 - (phase_offset / on_count as f32);
                falloff.powf(0.5)
            } else {
                0.0
            };

            // Draw glow
            if brightness > 0.3 {
                let glow_size = bulb_radius * 2.5 * brightness;
                if let Ok(glow) = Mesh::new_circle(
                    ctx,
                    DrawMode::fill(),
                    [lx, ly],
                    glow_size,
                    0.5,
                    Color::new(
                        CHASE_LIGHT_GLOW.r,
                        CHASE_LIGHT_GLOW.g,
                        CHASE_LIGHT_GLOW.b,
                        CHASE_LIGHT_GLOW.a * brightness,
                    ),
                ) {
                    canvas.draw(&glow, DrawParam::default());
                }
            }

            // Draw bulb
            let bulb_color = if brightness > 0.1 {
                Color::new(
                    CHASE_LIGHT_OFF.r + (CHASE_LIGHT_ON.r - CHASE_LIGHT_OFF.r) * brightness,
                    CHASE_LIGHT_OFF.g + (CHASE_LIGHT_ON.g - CHASE_LIGHT_OFF.g) * brightness,
                    CHASE_LIGHT_OFF.b + (CHASE_LIGHT_ON.b - CHASE_LIGHT_OFF.b) * brightness,
                    1.0,
                )
            } else {
                CHASE_LIGHT_OFF
            };

            if let Ok(bulb) = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                [lx, ly],
                bulb_radius,
                0.5,
                bulb_color,
            ) {
                canvas.draw(&bulb, DrawParam::default());
            }
        }
    }

    /// Draw rainbow border
    fn draw_rainbow_border(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) {
        let segments = 60;
        let perimeter = 2.0 * (width + height);
        let segment_length = perimeter / segments as f32;

        for i in 0..segments {
            let hue = (self.rainbow_phase + i as f32 / segments as f32) % 1.0;
            let color = hsv_to_rgb(hue, 0.9, 1.0);

            let start_dist = i as f32 * segment_length;
            let end_dist = (i + 1) as f32 * segment_length;

            let (x1, y1) = perimeter_point(x, y, width, height, start_dist);
            let (x2, y2) = perimeter_point(x, y, width, height, end_dist);

            if let Ok(line) = Mesh::new_line(ctx, &[[x1, y1], [x2, y2]], 4.0, color) {
                canvas.draw(&line, DrawParam::default());
            }
        }
    }

    /// Draw a single board square
    #[allow(clippy::too_many_arguments)]
    fn draw_board_square(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        prize: &crate::game::Prize,
        is_lit: bool,
        flash_intensity: f32,
        _is_spinning: bool,
    ) {
        // Determine colors based on prize type
        let (primary, _secondary) = match &prize.prize_type {
            PrizeType::Cash { bonus_spin, .. } => {
                if *bonus_spin {
                    (SPECIAL_GOLD, SPECIAL_GOLD_DARK)
                } else {
                    (CASH_GREEN, CASH_GREEN_DARK)
                }
            }
            PrizeType::Prize { .. } => (PRIZE_BLUE, PRIZE_BLUE_DARK),
            PrizeType::Whammy => (WHAMMY_RED, WHAMMY_RED_DARK),
            PrizeType::Special(_) => (SPECIAL_GOLD, SPECIAL_GOLD_DARK),
        };

        // Apply flash brightening
        let flash_boost = flash_intensity * 0.4;
        let primary_boosted = Color::new(
            (primary.r + flash_boost).min(1.0),
            (primary.g + flash_boost).min(1.0),
            (primary.b + flash_boost).min(1.0),
            1.0,
        );

        // Draw square background
        if let Ok(bg) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(x, y, width, height),
            primary_boosted,
        ) {
            canvas.draw(&bg, DrawParam::default());
        }

        // Active glow effect
        if is_lit {
            let pulse = (self.time * 10.0).sin() * 0.2 + 0.8;
            if let Ok(glow) = Mesh::new_rectangle(
                ctx,
                DrawMode::stroke(6.0),
                Rect::new(x - 6.0, y - 6.0, width + 12.0, height + 12.0),
                Color::new(1.0, 1.0, 0.8, pulse * 0.8),
            ) {
                canvas.draw(&glow, DrawParam::default());
            }
        }

        // Border
        let border_color = if is_lit {
            WHITE
        } else {
            Color::new(0.3, 0.3, 0.3, 0.8)
        };
        if let Ok(border) = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(2.0),
            Rect::new(x, y, width, height),
            border_color,
        ) {
            canvas.draw(&border, DrawParam::default());
        }

        // Prize text
        let main_size = width * 0.18;
        let text_color = if is_lit {
            Color::new(1.0, 1.0, 0.9, 1.0)
        } else {
            WHITE
        };

        let main_y = if prize.display_sub.is_some() {
            y + height * 0.3
        } else {
            y + height * 0.4
        };

        let main_text = Text::new(TextFragment::new(&prize.display_main).scale(main_size));
        canvas.draw(
            &main_text,
            DrawParam::default()
                .dest([x + width * 0.1, main_y])
                .color(text_color),
        );

        // Sub text
        if let Some(sub) = &prize.display_sub {
            let sub_size = width * 0.12;
            let sub_text = Text::new(TextFragment::new(sub).scale(sub_size));
            canvas.draw(
                &sub_text,
                DrawParam::default()
                    .dest([x + width * 0.2, y + height * 0.6])
                    .color(if is_lit {
                        Color::new(1.0, 0.9, 0.3, 1.0)
                    } else {
                        GOLD
                    }),
            );
        }

        // Mini Whammy icons
        if matches!(prize.prize_type, PrizeType::Whammy) {
            self.draw_mini_whammy(
                canvas,
                ctx,
                x + width * 0.1,
                y + height * 0.15,
                width * 0.25,
            );
            self.draw_mini_whammy(
                canvas,
                ctx,
                x + width * 0.65,
                y + height * 0.15,
                width * 0.25,
            );
        }
    }

    /// Draw a small Whammy icon
    fn draw_mini_whammy(&self, canvas: &mut Canvas, ctx: &mut Context, x: f32, y: f32, size: f32) {
        // Body (red circle)
        if let Ok(body) = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [x + size / 2.0, y + size / 2.0],
            size / 2.0,
            0.5,
            WHAMMY_RED,
        ) {
            canvas.draw(&body, DrawParam::default());
        }

        // Eyes
        let eye_size = size * 0.2;
        let eye_y = y + size * 0.35;

        // Left eye white
        if let Ok(eye) = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [x + size * 0.3, eye_y],
            eye_size,
            0.5,
            WHITE,
        ) {
            canvas.draw(&eye, DrawParam::default());
        }
        // Left pupil
        if let Ok(pupil) = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [x + size * 0.3, eye_y],
            eye_size * 0.5,
            0.5,
            BLACK,
        ) {
            canvas.draw(&pupil, DrawParam::default());
        }

        // Right eye white
        if let Ok(eye) = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [x + size * 0.7, eye_y],
            eye_size,
            0.5,
            WHITE,
        ) {
            canvas.draw(&eye, DrawParam::default());
        }
        // Right pupil
        if let Ok(pupil) = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [x + size * 0.7, eye_y],
            eye_size * 0.5,
            0.5,
            BLACK,
        ) {
            canvas.draw(&pupil, DrawParam::default());
        }
    }

    /// Draw the center stage area
    pub fn draw_center_stage(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        game_state: &GameState,
        _screen_w: f32,
        _screen_h: f32,
    ) {
        let board_rect = self.board_rect.unwrap_or(Rect::new(0.0, 0.0, 100.0, 100.0));

        let square_w = board_rect.w / BOARD_COLS as f32;
        let square_h = board_rect.h / BOARD_ROWS as f32;

        let center_x = board_rect.x + square_w;
        let center_y = board_rect.y + square_h;
        let center_w = board_rect.w - square_w * 2.0;
        let center_h = board_rect.h - square_h * 2.0;

        // Center background
        if let Ok(bg) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(center_x, center_y, center_w, center_h),
            Color::new(0.02, 0.01, 0.05, 0.9),
        ) {
            canvas.draw(&bg, DrawParam::default());
        }

        // Draw based on game state
        if game_state.whammy_animation.active {
            self.draw_whammy_animation(
                canvas,
                ctx,
                center_x,
                center_y,
                center_w,
                center_h,
                &game_state.whammy_animation,
            );
        } else if game_state.result_state.showing {
            self.draw_result_display(
                canvas,
                ctx,
                center_x,
                center_y,
                center_w,
                center_h,
                &game_state.result_state.main_text,
                &game_state.result_state.sub_text,
            );
        } else {
            // Draw "THE BIG BOARD" text when idle
            let text = "THE BIG BOARD";
            let text_size = center_w * 0.1;
            let pulse = (self.time * 1.5).sin() * 0.3 + 0.7;
            let display_text = Text::new(TextFragment::new(text).scale(text_size));
            canvas.draw(
                &display_text,
                DrawParam::default()
                    .dest([
                        center_x + center_w * 0.2,
                        center_y + center_h / 2.0 - text_size / 2.0,
                    ])
                    .color(Color::new(0.5, 0.3, 0.6, pulse)),
            );
        }
    }

    /// Draw Whammy character animation
    #[allow(clippy::too_many_arguments)]
    fn draw_whammy_animation(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        animation: &crate::game::WhammyAnimation,
    ) {
        let progress = animation.progress;
        let center_x = x + width / 2.0;
        let center_y = y + height / 2.0;
        let whammy_size = width.min(height) * 0.4;

        // Calculate position based on animation type
        let (wx, wy, _rotation, scale) = self.calculate_whammy_position(
            animation.animation_type,
            progress,
            center_x,
            center_y,
            x,
            width,
            height,
        );

        // Draw the Whammy character
        self.draw_whammy_character(
            canvas,
            ctx,
            wx,
            wy,
            whammy_size * scale,
            animation.animation_type,
        );

        // Draw catchphrase
        if !animation.catchphrase.is_empty() {
            let text_size = width * 0.06;

            // Speech bubble background
            let bubble_x = x + width * 0.1;
            let bubble_y = y + height * 0.75;
            let bubble_w = width * 0.8;
            let bubble_h = text_size + 10.0;

            if let Ok(bubble) = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(bubble_x, bubble_y - text_size, bubble_w, bubble_h),
                WHITE,
            ) {
                canvas.draw(&bubble, DrawParam::default());
            }

            let text = Text::new(TextFragment::new(&animation.catchphrase).scale(text_size));
            canvas.draw(
                &text,
                DrawParam::default()
                    .dest([bubble_x + 10.0, bubble_y - text_size + 5.0])
                    .color(BLACK),
            );
        }
    }

    /// Calculate Whammy position based on animation type
    #[allow(clippy::too_many_arguments)]
    fn calculate_whammy_position(
        &self,
        animation_type: WhammyAnimationType,
        progress: f32,
        center_x: f32,
        center_y: f32,
        x: f32,
        width: f32,
        height: f32,
    ) -> (f32, f32, f32, f32) {
        match animation_type {
            WhammyAnimationType::Pogo => {
                let bounce = (progress * 10.0 * std::f32::consts::PI).sin().abs();
                (
                    center_x,
                    center_y - bounce * height * 0.3,
                    0.0,
                    1.0 + bounce * 0.2,
                )
            }
            WhammyAnimationType::Dance => {
                let sway = (progress * 8.0 * std::f32::consts::PI).sin();
                (center_x + sway * width * 0.2, center_y, sway * 0.3, 1.0)
            }
            WhammyAnimationType::Hammer => {
                let swing = (progress * 6.0 * std::f32::consts::PI).sin();
                (center_x, center_y, swing * 0.5, 1.0)
            }
            WhammyAnimationType::Jumping => {
                let jump = (progress * 8.0 * std::f32::consts::PI).sin().max(0.0);
                (center_x, center_y - jump * height * 0.4, 0.0, 1.0)
            }
            WhammyAnimationType::LawnMower => {
                let x_pos = x + width * 0.1 + progress * width * 0.8;
                let bounce = (progress * 20.0 * std::f32::consts::PI).sin() * 0.1;
                (x_pos, center_y + bounce * height, 0.0, 1.0)
            }
            WhammyAnimationType::TNT => {
                if progress > 0.7 {
                    let exp_progress = (progress - 0.7) / 0.3;
                    (center_x, center_y, 0.0, 1.0 + exp_progress * 2.0)
                } else {
                    (center_x, center_y, progress * 0.5, 1.0)
                }
            }
            _ => {
                // Default subtle movement
                let sway = (self.time * 2.0).sin() * width * 0.05;
                (center_x + sway, center_y, 0.0, 1.0)
            }
        }
    }

    /// Draw the Whammy character
    fn draw_whammy_character(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        x: f32,
        y: f32,
        size: f32,
        animation_type: WhammyAnimationType,
    ) {
        // Cape (yellow)
        self.draw_whammy_cape(canvas, ctx, x, y, size, animation_type);

        // Body (red)
        self.draw_whammy_body(canvas, ctx, x, y, size);

        // Arms
        self.draw_whammy_arms(canvas, ctx, x, y, size, animation_type);

        // Legs
        self.draw_whammy_legs(canvas, ctx, x, y, size, animation_type);

        // Emblem (dollar sign)
        self.draw_whammy_emblem(canvas, ctx, x, y, size);

        // Face
        self.draw_whammy_face(canvas, ctx, x, y, size, animation_type);
    }

    /// Draw the Whammy's yellow cape
    fn draw_whammy_cape(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        x: f32,
        y: f32,
        size: f32,
        animation_type: WhammyAnimationType,
    ) {
        let cape_yellow = Color::new(1.0, 0.85, 0.0, 1.0);

        let wave = match animation_type {
            WhammyAnimationType::Dance | WhammyAnimationType::Hula => {
                (self.time * 6.0).sin() * size * 0.08
            }
            WhammyAnimationType::Pogo | WhammyAnimationType::Jumping => {
                (self.time * 10.0).sin() * size * 0.1
            }
            _ => (self.time * 3.0).sin() * size * 0.04,
        };

        let cape_attach_y = y - size * 0.15;
        let cape_width = size * 0.5;
        let cape_length = size * 0.6;

        // Left cape side
        if let Ok(cape) = Mesh::new_polygon(
            ctx,
            DrawMode::fill(),
            &[
                Vec2::new(x - size * 0.2, cape_attach_y),
                Vec2::new(x - cape_width + wave * 0.5, cape_attach_y + cape_length),
                Vec2::new(x - size * 0.05, cape_attach_y + cape_length * 0.5),
            ],
            cape_yellow,
        ) {
            canvas.draw(&cape, DrawParam::default());
        }

        // Right cape side
        if let Ok(cape) = Mesh::new_polygon(
            ctx,
            DrawMode::fill(),
            &[
                Vec2::new(x + size * 0.2, cape_attach_y),
                Vec2::new(x + cape_width - wave * 0.5, cape_attach_y + cape_length),
                Vec2::new(x + size * 0.05, cape_attach_y + cape_length * 0.5),
            ],
            cape_yellow,
        ) {
            canvas.draw(&cape, DrawParam::default());
        }

        // Cape collar
        if let Ok(collar) = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [x, cape_attach_y],
            size * 0.12,
            0.5,
            cape_yellow,
        ) {
            canvas.draw(&collar, DrawParam::default());
        }
    }

    /// Draw the Whammy's red body
    fn draw_whammy_body(&self, canvas: &mut Canvas, ctx: &mut Context, x: f32, y: f32, size: f32) {
        let breath = 1.0 + (self.time * 4.0).sin() * 0.02;
        let body_width = size * 0.35 * breath;
        let body_height = size * 0.4 * breath;

        // Body shadow
        if let Ok(shadow) = Mesh::new_ellipse(
            ctx,
            DrawMode::fill(),
            [x + size * 0.02, y + size * 0.02],
            body_width,
            body_height,
            0.5,
            Color::new(0.3, 0.0, 0.0, 0.5),
        ) {
            canvas.draw(&shadow, DrawParam::default());
        }

        // Main body
        if let Ok(body) = Mesh::new_ellipse(
            ctx,
            DrawMode::fill(),
            [x, y],
            body_width,
            body_height,
            0.5,
            WHAMMY_RED,
        ) {
            canvas.draw(&body, DrawParam::default());
        }

        // Body highlight
        if let Ok(highlight) = Mesh::new_ellipse(
            ctx,
            DrawMode::fill(),
            [x - size * 0.08, y - size * 0.1],
            body_width * 0.5,
            body_height * 0.4,
            0.5,
            Color::new(1.0, 0.35, 0.3, 0.4),
        ) {
            canvas.draw(&highlight, DrawParam::default());
        }
    }

    /// Draw the Whammy's arms
    fn draw_whammy_arms(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        x: f32,
        y: f32,
        size: f32,
        animation_type: WhammyAnimationType,
    ) {
        let arm_thickness = size * 0.06;
        let hand_size = size * 0.08;

        let (left_wave, right_wave) = match animation_type {
            WhammyAnimationType::Dance => {
                let wave = (self.time * 6.0).sin();
                (wave * 0.1, -wave * 0.1)
            }
            _ => (0.0, 0.0),
        };

        let arm_length = size * 0.35;

        // Left arm
        let left_shoulder = (x - size * 0.28, y - size * 0.05);
        let left_hand_x = left_shoulder.0 - arm_length + left_wave * size;
        let left_hand_y = left_shoulder.1 + arm_length * 0.3;

        if let Ok(arm) = Mesh::new_line(
            ctx,
            &[
                [left_shoulder.0, left_shoulder.1],
                [left_hand_x, left_hand_y],
            ],
            arm_thickness,
            WHAMMY_RED,
        ) {
            canvas.draw(&arm, DrawParam::default());
        }

        if let Ok(hand) = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [left_hand_x, left_hand_y],
            hand_size,
            0.5,
            WHAMMY_RED,
        ) {
            canvas.draw(&hand, DrawParam::default());
        }

        // Right arm
        let right_shoulder = (x + size * 0.28, y - size * 0.05);
        let right_hand_x = right_shoulder.0 + arm_length + right_wave * size;
        let right_hand_y = right_shoulder.1 + arm_length * 0.3;

        if let Ok(arm) = Mesh::new_line(
            ctx,
            &[
                [right_shoulder.0, right_shoulder.1],
                [right_hand_x, right_hand_y],
            ],
            arm_thickness,
            WHAMMY_RED,
        ) {
            canvas.draw(&arm, DrawParam::default());
        }

        if let Ok(hand) = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [right_hand_x, right_hand_y],
            hand_size,
            0.5,
            WHAMMY_RED,
        ) {
            canvas.draw(&hand, DrawParam::default());
        }
    }

    /// Draw the Whammy's legs
    fn draw_whammy_legs(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        x: f32,
        y: f32,
        size: f32,
        animation_type: WhammyAnimationType,
    ) {
        let leg_thickness = size * 0.055;
        let leg_spread = 0.15;

        let (left_offset, right_offset) = match animation_type {
            WhammyAnimationType::Dance => {
                let dance = (self.time * 5.0).sin();
                (dance * size * 0.08, -dance * size * 0.08)
            }
            _ => (0.0, 0.0),
        };

        let leg_top_y = y + size * 0.25;
        let leg_length = size * 0.4;

        // Left leg
        let left_hip_x = x - size * leg_spread;
        let left_foot_y = leg_top_y + leg_length;

        if let Ok(leg) = Mesh::new_line(
            ctx,
            &[
                [left_hip_x, leg_top_y],
                [left_hip_x + left_offset, left_foot_y],
            ],
            leg_thickness,
            WHAMMY_RED,
        ) {
            canvas.draw(&leg, DrawParam::default());
        }

        // Left foot
        let foot_width = size * 0.12;
        let foot_height = size * 0.06;
        if let Ok(foot) = Mesh::new_polygon(
            ctx,
            DrawMode::fill(),
            &[
                Vec2::new(left_hip_x + left_offset - foot_width * 0.3, left_foot_y),
                Vec2::new(
                    left_hip_x + left_offset - foot_width,
                    left_foot_y + foot_height,
                ),
                Vec2::new(
                    left_hip_x + left_offset + foot_width * 0.5,
                    left_foot_y + foot_height,
                ),
            ],
            WHAMMY_RED,
        ) {
            canvas.draw(&foot, DrawParam::default());
        }

        // Right leg
        let right_hip_x = x + size * leg_spread;
        let right_foot_y = leg_top_y + leg_length;

        if let Ok(leg) = Mesh::new_line(
            ctx,
            &[
                [right_hip_x, leg_top_y],
                [right_hip_x + right_offset, right_foot_y],
            ],
            leg_thickness,
            WHAMMY_RED,
        ) {
            canvas.draw(&leg, DrawParam::default());
        }

        // Right foot
        if let Ok(foot) = Mesh::new_polygon(
            ctx,
            DrawMode::fill(),
            &[
                Vec2::new(right_hip_x + right_offset + foot_width * 0.3, right_foot_y),
                Vec2::new(
                    right_hip_x + right_offset + foot_width,
                    right_foot_y + foot_height,
                ),
                Vec2::new(
                    right_hip_x + right_offset - foot_width * 0.5,
                    right_foot_y + foot_height,
                ),
            ],
            WHAMMY_RED,
        ) {
            canvas.draw(&foot, DrawParam::default());
        }
    }

    /// Draw the Whammy's dollar sign chest emblem
    fn draw_whammy_emblem(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        x: f32,
        y: f32,
        size: f32,
    ) {
        let emblem_y = y + size * 0.05;
        let emblem_width = size * 0.18;
        let emblem_height = size * 0.2;

        let shield_yellow = Color::new(1.0, 0.85, 0.0, 1.0);

        // Shield shape
        if let Ok(shield) = Mesh::new_polygon(
            ctx,
            DrawMode::fill(),
            &[
                Vec2::new(x - emblem_width, emblem_y - emblem_height * 0.4),
                Vec2::new(x + emblem_width, emblem_y - emblem_height * 0.4),
                Vec2::new(x, emblem_y + emblem_height),
            ],
            shield_yellow,
        ) {
            canvas.draw(&shield, DrawParam::default());
        }

        // Top of shield
        if let Ok(top) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(
                x - emblem_width,
                emblem_y - emblem_height * 0.5,
                emblem_width * 2.0,
                emblem_height * 0.5,
            ),
            shield_yellow,
        ) {
            canvas.draw(&top, DrawParam::default());
        }

        // Dollar sign
        let dollar_size = emblem_height * 0.5;
        let dollar_y = emblem_y - emblem_height * 0.15;

        // Vertical line
        if let Ok(line) = Mesh::new_line(
            ctx,
            &[
                [x, dollar_y - dollar_size * 0.7],
                [x, dollar_y + dollar_size * 0.7],
            ],
            2.5,
            BLACK,
        ) {
            canvas.draw(&line, DrawParam::default());
        }

        // S shape (simplified)
        let text = Text::new(TextFragment::new("$").scale(dollar_size * 1.5));
        canvas.draw(
            &text,
            DrawParam::default()
                .dest([x - dollar_size * 0.3, dollar_y - dollar_size * 0.6])
                .color(BLACK),
        );
    }

    /// Draw the Whammy's face
    fn draw_whammy_face(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        x: f32,
        y: f32,
        size: f32,
        animation_type: WhammyAnimationType,
    ) {
        // Head
        let head_y = y - size * 0.25;
        let head_radius = size * 0.32;

        // Head shadow
        if let Ok(shadow) = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [x + size * 0.015, head_y + size * 0.015],
            head_radius,
            0.5,
            Color::new(0.3, 0.0, 0.0, 0.4),
        ) {
            canvas.draw(&shadow, DrawParam::default());
        }

        // Main head
        if let Ok(head) = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [x, head_y],
            head_radius,
            0.5,
            WHAMMY_RED,
        ) {
            canvas.draw(&head, DrawParam::default());
        }

        // Yellow eye mask
        let mask_yellow = Color::new(1.0, 0.85, 0.0, 1.0);
        let mask_width = head_radius * 1.3;
        let mask_height = head_radius * 0.45;
        let mask_y = head_y - head_radius * 0.1;

        if let Ok(mask) = Mesh::new_ellipse(
            ctx,
            DrawMode::fill(),
            [x, mask_y],
            mask_width,
            mask_height,
            0.5,
            mask_yellow,
        ) {
            canvas.draw(&mask, DrawParam::default());
        }

        // Mask points
        if let Ok(left_point) = Mesh::new_polygon(
            ctx,
            DrawMode::fill(),
            &[
                Vec2::new(x - mask_width, mask_y),
                Vec2::new(x - mask_width - size * 0.12, mask_y - size * 0.08),
                Vec2::new(x - mask_width + size * 0.05, mask_y - mask_height * 0.5),
            ],
            mask_yellow,
        ) {
            canvas.draw(&left_point, DrawParam::default());
        }

        if let Ok(right_point) = Mesh::new_polygon(
            ctx,
            DrawMode::fill(),
            &[
                Vec2::new(x + mask_width, mask_y),
                Vec2::new(x + mask_width + size * 0.12, mask_y - size * 0.08),
                Vec2::new(x + mask_width - size * 0.05, mask_y - mask_height * 0.5),
            ],
            mask_yellow,
        ) {
            canvas.draw(&right_point, DrawParam::default());
        }

        // Eyes
        let eye_offset_x = size * 0.18;
        let eye_offset_y = head_y - size * 0.02;
        let eye_size = size * 0.14;

        // Left eye
        if let Ok(eye) = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [x - eye_offset_x, eye_offset_y],
            eye_size,
            0.5,
            WHITE,
        ) {
            canvas.draw(&eye, DrawParam::default());
        }

        // Animated pupil
        let pupil_offset = match animation_type {
            WhammyAnimationType::Dance => (self.time * 5.0).sin() * eye_size * 0.3,
            _ => 0.0,
        };

        if let Ok(pupil) = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [x - eye_offset_x + pupil_offset, eye_offset_y],
            eye_size * 0.5,
            0.5,
            BLACK,
        ) {
            canvas.draw(&pupil, DrawParam::default());
        }

        // Right eye
        if let Ok(eye) = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [x + eye_offset_x, eye_offset_y],
            eye_size,
            0.5,
            WHITE,
        ) {
            canvas.draw(&eye, DrawParam::default());
        }

        if let Ok(pupil) = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [x + eye_offset_x + pupil_offset, eye_offset_y],
            eye_size * 0.5,
            0.5,
            BLACK,
        ) {
            canvas.draw(&pupil, DrawParam::default());
        }

        // Hair tuft
        let hair_base_y = head_y - head_radius;

        if let Ok(tuft) = Mesh::new_polygon(
            ctx,
            DrawMode::fill(),
            &[
                Vec2::new(x - size * 0.04, hair_base_y + size * 0.02),
                Vec2::new(x, hair_base_y - size * 0.12),
                Vec2::new(x + size * 0.04, hair_base_y + size * 0.02),
            ],
            WHAMMY_RED,
        ) {
            canvas.draw(&tuft, DrawParam::default());
        }

        // Mischievous grin
        let mouth_y = head_y + head_radius * 0.5;
        let grin_width = size * 0.22;

        // Simple curved grin
        if let Ok(mouth) = Mesh::new_line(
            ctx,
            &[
                [x - grin_width, mouth_y],
                [x - grin_width * 0.5, mouth_y + size * 0.06],
                [x, mouth_y + size * 0.1],
                [x + grin_width * 0.5, mouth_y + size * 0.06],
                [x + grin_width, mouth_y],
            ],
            3.0,
            BLACK,
        ) {
            canvas.draw(&mouth, DrawParam::default());
        }

        // Nose
        let nose_y = head_y + head_radius * 0.15;
        if let Ok(nose) = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [x, nose_y],
            size * 0.035,
            0.5,
            Color::new(0.75, 0.05, 0.05, 1.0),
        ) {
            canvas.draw(&nose, DrawParam::default());
        }
    }

    /// Draw result display in center
    #[allow(clippy::too_many_arguments)]
    fn draw_result_display(
        &self,
        canvas: &mut Canvas,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        main: &str,
        sub: &str,
    ) {
        let pulse = (self.time * 3.0).sin() * 0.1 + 0.9;
        let main_size = width * 0.15;

        // Shadow
        let shadow_text = Text::new(TextFragment::new(main).scale(main_size));
        canvas.draw(
            &shadow_text,
            DrawParam::default()
                .dest([x + width * 0.2 + 3.0, y + height * 0.35 + 3.0])
                .color(Color::new(0.0, 0.0, 0.0, 0.5)),
        );

        // Main text
        let main_text = Text::new(TextFragment::new(main).scale(main_size * pulse));
        canvas.draw(
            &main_text,
            DrawParam::default()
                .dest([x + width * 0.2, y + height * 0.35])
                .color(GOLD),
        );

        // Sub text
        if !sub.is_empty() {
            let sub_size = width * 0.08;
            let sub_text = Text::new(TextFragment::new(sub).scale(sub_size));
            canvas.draw(
                &sub_text,
                DrawParam::default()
                    .dest([x + width * 0.3, y + height * 0.55])
                    .color(WHITE),
            );
        }
    }

    /// Draw Whammy taunt text (for new animation system integration)
    ///
    /// Called when WhammyAnimator is playing to display the taunt/catchphrase
    /// in a speech bubble style below the Whammy character.
    pub fn draw_whammy_taunt(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        taunt_text: &str,
        screen_w: f32,
        screen_h: f32,
    ) {
        if taunt_text.is_empty() {
            return;
        }

        // Position the speech bubble in the lower center area
        let bubble_x = screen_w * 0.2;
        let bubble_y = screen_h * 0.7;
        let bubble_w = screen_w * 0.6;
        let text_size = screen_w * 0.025;
        let bubble_h = text_size + 20.0;

        // Speech bubble background
        if let Ok(bubble) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(bubble_x, bubble_y, bubble_w, bubble_h),
            WHITE,
        ) {
            canvas.draw(&bubble, DrawParam::default());
        }

        // Speech bubble border
        if let Ok(border) = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(3.0),
            Rect::new(bubble_x, bubble_y, bubble_w, bubble_h),
            BLACK,
        ) {
            canvas.draw(&border, DrawParam::default());
        }

        // Taunt text
        let text = Text::new(TextFragment::new(taunt_text).scale(text_size));
        canvas.draw(
            &text,
            DrawParam::default()
                .dest([bubble_x + 15.0, bubble_y + 10.0])
                .color(BLACK),
        );
    }

    /// Draw control hints
    pub fn draw_controls(
        &self,
        canvas: &mut Canvas,
        _ctx: &mut Context,
        game_state: &GameState,
        screen_w: f32,
        screen_h: f32,
    ) {
        let controls_y = screen_h * 0.02;
        let font_size = screen_w * 0.012;

        let controls = match game_state.phase {
            GamePhase::Start => "[SPACE/ENTER] Start Game  |  [ESC] Quit",
            GamePhase::Questions => {
                if game_state.question_state.waiting_for_buzz {
                    "[B] Buzz In  |  [ESC] Quit"
                } else if game_state.question_state.showing_choices {
                    "[1-4] Select Answer  |  [ESC] Quit"
                } else {
                    "[SPACE] Continue  |  [ESC] Quit"
                }
            }
            GamePhase::Board => {
                if game_state.awaiting_corner_selection {
                    "[1-4] Pick Corner  |  [ESC] Quit"
                } else if game_state.awaiting_special_choice {
                    "[1] $2,000  |  [2] Lose Whammy  |  [ESC] Quit"
                } else if game_state.is_spinning {
                    "[SPACE] STOP!  |  [ESC] Quit"
                } else {
                    "[SPACE] Spin  |  [P] Pass  |  [ESC] Quit"
                }
            }
            GamePhase::GameOver => "[SPACE/ENTER] Play Again  |  [ESC] Quit",
        };

        let text = Text::new(TextFragment::new(controls).scale(font_size));
        canvas.draw(
            &text,
            DrawParam::default()
                .dest([screen_w * 0.25, controls_y])
                .color(Color::new(0.7, 0.7, 0.7, 0.8)),
        );
    }

    /// Draw message display area
    pub fn draw_message_display(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        game_state: &GameState,
        screen_w: f32,
        screen_h: f32,
    ) {
        if game_state.message.is_empty() {
            return;
        }

        let msg_y = screen_h * 0.78;
        let font_size = screen_w * 0.022;

        // Background bar
        if let Ok(bar) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, msg_y - font_size * 0.8, screen_w, font_size * 1.6),
            Color::new(0.0, 0.0, 0.0, 0.7),
        ) {
            canvas.draw(&bar, DrawParam::default());
        }

        // Message text
        let color = if game_state.message_excited {
            let pulse = (self.time * 5.0).sin() * 0.5 + 0.5;
            Color::new(1.0, pulse, 0.0, 1.0)
        } else {
            WHITE
        };

        let text = Text::new(TextFragment::new(&game_state.message).scale(font_size));
        canvas.draw(
            &text,
            DrawParam::default()
                .dest([screen_w * 0.3, msg_y - font_size * 0.5])
                .color(color),
        );
    }

    /// Draw CRT effect overlay
    pub fn draw_crt_overlay(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        screen_w: f32,
        screen_h: f32,
    ) {
        // Scanlines
        let scanline_spacing = 3.0;
        let mut y = 0.0;
        while y < screen_h {
            if let Ok(line) = Mesh::new_line(ctx, &[[0.0, y], [screen_w, y]], 1.0, CRT_SCANLINE) {
                canvas.draw(&line, DrawParam::default());
            }
            y += scanline_spacing;
        }

        // Vignette (top and bottom)
        let vignette_strength = 0.15;
        for i in 0..30 {
            let alpha = vignette_strength * (1.0 - i as f32 / 30.0);
            if let Ok(rect) = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(0.0, i as f32, screen_w, 1.0),
                Color::new(0.0, 0.0, 0.0, alpha),
            ) {
                canvas.draw(&rect, DrawParam::default());
            }
            if let Ok(rect) = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(0.0, screen_h - i as f32 - 1.0, screen_w, 1.0),
                Color::new(0.0, 0.0, 0.0, alpha),
            ) {
                canvas.draw(&rect, DrawParam::default());
            }
        }
    }

    /// Draw debug info (only in debug builds)
    #[cfg(debug_assertions)]
    pub fn draw_debug_info(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        game_state: &GameState,
        _screen_w: f32,
        screen_h: f32,
    ) {
        let fps = ctx.time.fps();
        let debug_text = format!("FPS: {:.0} | Phase: {:?}", fps, game_state.phase);
        let text = Text::new(TextFragment::new(&debug_text).scale(14.0));
        canvas.draw(
            &text,
            DrawParam::default()
                .dest([10.0, screen_h - 20.0])
                .color(Color::new(1.0, 1.0, 0.0, 0.7)),
        );
    }
}

impl Default for GraphicsRenderer {
    fn default() -> Self {
        Self::new()
    }
}

// ===============================================================================
// HELPER FUNCTIONS
// ===============================================================================

/// Convert HSV to RGB color
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let h = h * 6.0;
    let i = h.floor() as i32;
    let f = h - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));

    let (r, g, b) = match i % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };

    Color::new(r, g, b, 1.0)
}

/// Get point on rectangle perimeter at given distance from top-left
fn perimeter_point(x: f32, y: f32, width: f32, height: f32, distance: f32) -> (f32, f32) {
    let perimeter = 2.0 * (width + height);
    let d = distance % perimeter;

    if d < width {
        // Top edge
        (x + d, y)
    } else if d < width + height {
        // Right edge
        (x + width, y + (d - width))
    } else if d < 2.0 * width + height {
        // Bottom edge
        (x + width - (d - width - height), y + height)
    } else {
        // Left edge
        (x, y + height - (d - 2.0 * width - height))
    }
}

/// Format money value with commas
fn format_money(value: u32) -> String {
    if value >= 1_000_000 {
        format!(
            "{},{:03},{:03}",
            value / 1_000_000,
            (value / 1000) % 1000,
            value % 1000
        )
    } else if value >= 1000 {
        format!("{},{:03}", value / 1000, value % 1000)
    } else {
        value.to_string()
    }
}
