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

use macroquad::prelude::*;
use crate::game::{GameState, GamePhase, Contestant, PrizeType, WhammyAnimationType};

// ═══════════════════════════════════════════════════════════════════════════════
// COLOR CONSTANTS (Authentic 1983-1986 CRT Television Palette)
// ═══════════════════════════════════════════════════════════════════════════════
// CRT televisions of the era had distinctive characteristics:
// - Warm color temperature (slight red/orange tint)
// - Phosphor bloom (colors slightly bleed outward)
// - Slightly elevated black levels (deep purple rather than true black)
// - Saturated, vibrant primaries with soft falloff
// ═══════════════════════════════════════════════════════════════════════════════

/// Deep purple background (CRT black level - never truly black)
const BG_COLOR: Color = Color::new(0.04, 0.02, 0.13, 1.0);

/// Gold highlight (warm CRT gold with slight red push)
const GOLD: Color = Color::new(1.0, 0.82, 0.08, 1.0);

/// Hot pink highlight (authentic 80s neon with CRT warmth)
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

/// Contestant colors (CRT-warm player indicators)
const PLAYER_COLORS: [Color; 3] = [
    Color::new(1.0, 0.35, 0.28, 1.0),  // Red (slightly orange CRT red)
    Color::new(0.28, 0.95, 0.35, 1.0),  // Green (CRT green phosphor)
    Color::new(0.35, 0.38, 1.0, 1.0),  // Blue (CRT blue with slight purple)
];

/// CRT scanline overlay color (subtle horizontal lines)
const CRT_SCANLINE: Color = Color::new(0.0, 0.0, 0.0, 0.08);

// ═══════════════════════════════════════════════════════════════════════════════
// LAYOUT CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Board layout: 6 columns x 5 rows (but only perimeter squares are active)
const BOARD_COLS: usize = 6;
const BOARD_ROWS: usize = 5;

/// Board square positions (clockwise from top-left)
/// Format: (column, row) for each of the 18 squares
const SQUARE_POSITIONS: [(usize, usize); 18] = [
    (0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0),  // Top row (0-5)
    (5, 1), (5, 2), (5, 3),                          // Right column (6-8)
    (5, 4), (4, 4), (3, 4), (2, 4), (1, 4), (0, 4),  // Bottom row (9-14)
    (0, 3), (0, 2), (0, 1),                          // Left column (15-17)
];

// ═══════════════════════════════════════════════════════════════════════════════
// GRAPHICS RENDERER
// ═══════════════════════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════════════════════
// CHASE LIGHT CONSTANTS (Authentic 1983-1986 CBS Show)
// ═══════════════════════════════════════════════════════════════════════════════

/// Number of chase lights around the board perimeter
const CHASE_LIGHT_COUNT: usize = 48;

/// Chase light bulb color (warm incandescent yellow-white)
const CHASE_LIGHT_ON: Color = Color::new(1.0, 0.95, 0.7, 1.0);

/// Chase light bulb off color (dim gray)
const CHASE_LIGHT_OFF: Color = Color::new(0.2, 0.18, 0.15, 0.6);

/// Chase light bulb glow color
const CHASE_LIGHT_GLOW: Color = Color::new(1.0, 0.9, 0.5, 0.4);

// ═══════════════════════════════════════════════════════════════════════════════
// GRAPHICS RENDERER
// ═══════════════════════════════════════════════════════════════════════════════

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
    fn calculate_board_rect(&mut self) -> Rect {
        if let Some(rect) = self.board_rect {
            return rect;
        }

        let screen_w = screen_width();
        let screen_h = screen_height();

        // Board occupies center of screen
        let board_w = screen_w * 0.65;
        let board_h = screen_h * 0.55;
        let board_x = (screen_w - board_w) / 2.0;
        let board_y = screen_h * 0.20;

        let rect = Rect::new(board_x, board_y, board_w, board_h);
        self.board_rect = Some(rect);
        rect
    }

    /// Draw the animated background
    ///
    /// Creates a deep purple gradient with subtle pulsing glow effects.
    /// Uses BG_COLOR as the base color for the gradient.
    /// Now includes authentic CRT-style color enhancement.
    pub fn draw_background(&mut self) {
        let delta = get_frame_time();
        self.time += delta;
        self.rainbow_phase += delta * 0.5;
        self.shimmer_phase += delta * 2.0;

        let screen_w = screen_width();
        let screen_h = screen_height();

        // Draw gradient background using BG_COLOR as base with CRT warmth
        // Authentic 1980s TV had slightly warm, saturated colors
        for y in 0..(screen_h as i32) {
            let t = y as f32 / screen_h;
            // Add slight CRT phosphor warmth (subtle red-shift at edges)
            let warmth = (1.0 - (t - 0.5).abs() * 2.0).max(0.0) * 0.02;
            let color = Color::new(
                (BG_COLOR.r + 0.02 * t + warmth).min(1.0),
                BG_COLOR.g + 0.01 * t,
                (BG_COLOR.b + 0.05 * t).min(1.0),
                1.0,
            );
            draw_line(0.0, y as f32, screen_w, y as f32, 1.0, color);
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
            draw_circle(
                center_x,
                center_y,
                r as f32,
                Color::new(0.6, 0.1, 0.7, alpha),
            );
        }
    }

    /// Update animation states based on game state
    /// Call this before drawing to update flash effects
    pub fn update_animations(&mut self, game_state: &GameState, delta: f32) {
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
                self.square_flash[trail_idx] = self.square_flash[trail_idx].max(trail_intensity * 0.5);
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

    /// Draw the game header with rainbow shimmer effect
    pub fn draw_header(&self) {
        let screen_w = screen_width();
        let title = "PRESS YOUR LUCK";

        // Calculate font size based on screen width
        let font_size = (screen_w * 0.05).min(60.0);
        let text_dims = measure_text(title, None, font_size as u16, 1.0);
        let x = (screen_w - text_dims.width) / 2.0;
        let y = screen_height() * 0.08;

        // Draw shadow
        draw_text(title, x + 3.0, y + 3.0, font_size, Color::new(0.0, 0.0, 0.0, 0.5));

        // Draw each character with rainbow shimmer
        let chars: Vec<char> = title.chars().collect();
        let mut current_x = x;

        for (i, c) in chars.iter().enumerate() {
            let char_str = c.to_string();
            let char_dims = measure_text(&char_str, None, font_size as u16, 1.0);

            // Calculate rainbow color based on position and time
            let hue = (self.shimmer_phase + i as f32 * 0.15) % 1.0;
            let color = hsv_to_rgb(hue, 0.8, 1.0);

            draw_text(&char_str, current_x, y, font_size, color);
            current_x += char_dims.width;
        }

        // Subtitle
        let subtitle = "~ BIG BUCKS! NO WHAMMIES! ~";
        let sub_size = font_size * 0.4;
        let sub_dims = measure_text(subtitle, None, sub_size as u16, 1.0);
        draw_text(
            subtitle,
            (screen_w - sub_dims.width) / 2.0,
            y + font_size * 0.6,
            sub_size,
            GOLD,
        );
    }

    /// Draw the contestant podiums
    ///
    /// Shows each contestant's name, score, spin counts, and whammy indicators.
    pub fn draw_podiums(&self, game_state: &GameState) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        let podium_width = screen_w * 0.22;
        let podium_height = screen_h * 0.12;
        let podium_y = screen_h * 0.85;
        let spacing = (screen_w - podium_width * 3.0) / 4.0;

        for (i, contestant) in game_state.contestants.iter().enumerate() {
            let x = spacing + (podium_width + spacing) * i as f32;

            self.draw_single_podium(
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
    /// Authentic 1983-1986 CBS show styling with name plates and LED scores
    fn draw_single_podium(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        contestant: &Contestant,
        is_active: bool,
        player_index: usize,
    ) {
        let player_color = PLAYER_COLORS[player_index];

        // Angled podium background (darker at bottom for 3D effect)
        let bg_top = if is_active {
            Color::new(player_color.r * 0.25, player_color.g * 0.25, player_color.b * 0.25, 0.95)
        } else {
            Color::new(0.12, 0.10, 0.18, 0.95)
        };
        let bg_bottom = Color::new(bg_top.r * 0.5, bg_top.g * 0.5, bg_top.b * 0.5, 0.95);

        // Draw gradient podium background
        for row in 0..(height as i32) {
            let t = row as f32 / height;
            let color = Color::new(
                bg_top.r * (1.0 - t) + bg_bottom.r * t,
                bg_top.g * (1.0 - t) + bg_bottom.g * t,
                bg_top.b * (1.0 - t) + bg_bottom.b * t,
                bg_top.a,
            );
            draw_line(x, y + row as f32, x + width, y + row as f32, 1.0, color);
        }

        // Metallic trim border
        let trim_color = if is_active {
            GOLD
        } else {
            Color::new(0.6, 0.55, 0.5, 1.0) // Brushed metal look
        };
        draw_rectangle_lines(x, y, width, height, 4.0, trim_color);

        // Inner border highlight
        draw_rectangle_lines(x + 2.0, y + 2.0, width - 4.0, height - 4.0, 1.0,
            Color::new(1.0, 1.0, 1.0, 0.2));

        // Active indicator glow (more dramatic pulsing)
        if is_active {
            let pulse = (self.time * 4.0).sin() * 0.4 + 0.6;
            // Outer glow
            draw_rectangle_lines(
                x - 4.0,
                y - 4.0,
                width + 8.0,
                height + 8.0,
                3.0,
                Color::new(1.0, 0.84, 0.0, pulse * 0.6),
            );
            // Inner bright edge
            draw_rectangle_lines(
                x - 1.0,
                y - 1.0,
                width + 2.0,
                height + 2.0,
                2.0,
                Color::new(1.0, 0.9, 0.5, pulse),
            );
        }

        // Eliminated overlay
        if contestant.eliminated {
            draw_rectangle(x, y, width, height, Color::new(0.0, 0.0, 0.0, 0.8));

            // Red X pattern
            draw_line(x, y, x + width, y + height, 4.0, Color::new(0.8, 0.0, 0.0, 0.6));
            draw_line(x + width, y, x, y + height, 4.0, Color::new(0.8, 0.0, 0.0, 0.6));

            let elim_text = "WHAMMIED OUT!";
            let elim_size = width * 0.11;
            let elim_dims = measure_text(elim_text, None, elim_size as u16, 1.0);

            // Pulsing red text
            let pulse = (self.time * 3.0).sin() * 0.3 + 0.7;
            draw_text(
                elim_text,
                x + (width - elim_dims.width) / 2.0,
                y + height / 2.0,
                elim_size,
                Color::new(1.0, 0.1, 0.1, pulse),
            );
            return;
        }

        // Name plate (metallic background with engraved look)
        let name_plate_h = height * 0.18;
        let name_plate_y = y + height * 0.05;
        draw_rectangle(x + 5.0, name_plate_y, width - 10.0, name_plate_h,
            Color::new(0.25, 0.22, 0.3, 1.0));
        draw_rectangle_lines(x + 5.0, name_plate_y, width - 10.0, name_plate_h, 1.0,
            player_color);

        let name_size = width * 0.11;
        let name_dims = measure_text(&contestant.name, None, name_size as u16, 1.0);
        draw_text(
            &contestant.name,
            x + (width - name_dims.width) / 2.0,
            name_plate_y + name_plate_h * 0.75,
            name_size,
            player_color,
        );

        // LED-style score display
        let displayed_score = self.displayed_scores[player_index] as u32;
        let score_text = format!("${}", format_money(displayed_score));
        self.draw_led_score(
            x + width * 0.08,
            y + height * 0.32,
            width * 0.84,
            height * 0.25,
            &score_text,
            is_active,
        );

        // Spins display with PLAY/PASS indicators
        let total_spins = contestant.earned_spins + contestant.passed_spins;
        let spins_text = format!("{} SPINS", total_spins);
        let spins_size = width * 0.085;
        let spins_dims = measure_text(&spins_text, None, spins_size as u16, 1.0);

        // Spins background panel
        draw_rectangle(x + width * 0.1, y + height * 0.6, width * 0.8, height * 0.12,
            Color::new(0.05, 0.08, 0.15, 0.9));
        draw_text(
            &spins_text,
            x + (width - spins_dims.width) / 2.0,
            y + height * 0.7,
            spins_size,
            ELECTRIC_BLUE,
        );

        // Show earned vs passed detail below
        let detail_size = width * 0.055;
        let detail_text = format!("({}E / {}P)", contestant.earned_spins, contestant.passed_spins);
        let detail_dims = measure_text(&detail_text, None, detail_size as u16, 1.0);
        draw_text(
            &detail_text,
            x + (width - detail_dims.width) / 2.0,
            y + height * 0.77,
            detail_size,
            Color::new(0.6, 0.6, 0.7, 0.8),
        );

        // Whammy indicators (4 LED-style lights)
        let whammy_y = y + height * 0.88;
        let whammy_spacing = width * 0.16;
        let whammy_start = x + (width - whammy_spacing * 3.0) / 2.0;
        let whammy_radius = width * 0.045;

        for w in 0..4 {
            let wx = whammy_start + whammy_spacing * w as f32;
            let filled = w < contestant.whammies as usize;

            if filled {
                // Lit red LED with glow
                draw_circle(wx, whammy_y, whammy_radius * 1.5, Color::new(1.0, 0.0, 0.0, 0.3));
                draw_circle(wx, whammy_y, whammy_radius, WHAMMY_RED);
                draw_circle(wx - whammy_radius * 0.3, whammy_y - whammy_radius * 0.3,
                    whammy_radius * 0.25, Color::new(1.0, 0.5, 0.5, 0.8));
            } else {
                // Unlit LED (dark red glass look)
                draw_circle(wx, whammy_y, whammy_radius, Color::new(0.25, 0.05, 0.05, 0.9));
                draw_circle_lines(wx, whammy_y, whammy_radius, 1.0, Color::new(0.4, 0.1, 0.1, 0.6));
            }
        }
    }

    /// Draw LED-style score display (seven-segment aesthetic)
    fn draw_led_score(&self, x: f32, y: f32, width: f32, height: f32, text: &str, is_active: bool) {
        // LED display background (dark with slight glow)
        draw_rectangle(x, y, width, height, Color::new(0.02, 0.02, 0.05, 1.0));
        draw_rectangle_lines(x, y, width, height, 2.0, Color::new(0.3, 0.3, 0.3, 0.8));

        // Inner bevel
        draw_rectangle_lines(x + 2.0, y + 2.0, width - 4.0, height - 4.0, 1.0,
            Color::new(0.0, 0.0, 0.0, 0.5));

        // LED glow behind text
        if is_active {
            let glow = (self.time * 2.0).sin() * 0.1 + 0.9;
            draw_rectangle(x + 4.0, y + 4.0, width - 8.0, height - 8.0,
                Color::new(0.0, 0.15 * glow, 0.0, 0.3));
        }

        // LED text (green for authentic 1980s LED look)
        let text_size = height * 0.7;
        let dims = measure_text(text, None, text_size as u16, 1.0);

        // LED color - bright green with slight flicker when active
        let led_brightness = if is_active {
            (self.time * 60.0).sin() * 0.03 + 0.97 // Subtle 60Hz flicker
        } else {
            0.85
        };

        let led_color = Color::new(0.1, led_brightness, 0.15, 1.0);

        // Shadow/ghost effect for LED segments
        draw_text(
            text,
            x + (width - dims.width) / 2.0 + 1.0,
            y + height * 0.72 + 1.0,
            text_size,
            Color::new(0.0, 0.2, 0.0, 0.3),
        );

        // Main LED text
        draw_text(
            text,
            x + (width - dims.width) / 2.0,
            y + height * 0.72,
            text_size,
            led_color,
        );

        // Add subtle scanline effect for CRT authenticity
        for line in (0..(height as i32)).step_by(3) {
            draw_line(
                x, y + line as f32,
                x + width, y + line as f32,
                1.0,
                Color::new(0.0, 0.0, 0.0, 0.15),
            );
        }
    }

    /// Draw the status bar showing round info with authentic game show styling
    pub fn draw_status_bar(&self, game_state: &GameState) {
        let screen_w = screen_width();
        let screen_h = screen_height();
        let bar_y = screen_h * 0.125;

        // Round indicator panel (authentic game show graphic style)
        let panel_w = screen_w * 0.18;
        let panel_h = screen_h * 0.045;
        let panel_x = (screen_w - panel_w) / 2.0;
        let panel_y = bar_y - panel_h * 0.7;

        // Panel background with gradient
        for row in 0..(panel_h as i32) {
            let t = row as f32 / panel_h;
            let color = if game_state.round == 1 {
                // Round 1: Blue theme
                Color::new(0.0, 0.2 + t * 0.1, 0.5 - t * 0.2, 0.95)
            } else {
                // Round 2: Gold theme (higher values available)
                Color::new(0.6 - t * 0.2, 0.4 - t * 0.15, 0.0, 0.95)
            };
            draw_line(panel_x, panel_y + row as f32, panel_x + panel_w, panel_y + row as f32, 1.0, color);
        }

        // Panel border with metallic trim
        draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, 2.0, GOLD);
        draw_rectangle_lines(panel_x + 1.0, panel_y + 1.0, panel_w - 2.0, panel_h - 2.0, 1.0,
            Color::new(1.0, 1.0, 1.0, 0.3));

        // Round text
        let round_text = format!("ROUND {}", game_state.round);
        let round_size = panel_h * 0.65;
        let round_dims = measure_text(&round_text, None, round_size as u16, 1.0);

        // Pulsing effect for Round 2 (more exciting)
        let text_color = if game_state.round == 2 {
            let pulse = (self.time * 2.0).sin() * 0.2 + 0.8;
            Color::new(1.0, pulse, 0.3, 1.0)
        } else {
            WHITE
        };

        draw_text(
            &round_text,
            panel_x + (panel_w - round_dims.width) / 2.0,
            panel_y + panel_h * 0.72,
            round_size,
            text_color,
        );

        // Show "BIG MONEY!" indicator for Round 2
        if game_state.round == 2 {
            let bonus_text = "BIG MONEY!";
            let bonus_size = screen_w * 0.012;
            let bonus_dims = measure_text(bonus_text, None, bonus_size as u16, 1.0);
            let pulse = (self.time * 3.0).sin() * 0.3 + 0.7;
            draw_text(
                bonus_text,
                panel_x + (panel_w - bonus_dims.width) / 2.0,
                panel_y + panel_h + bonus_size * 1.2,
                bonus_size,
                Color::new(1.0, 0.84, 0.0, pulse),
            );
        }
    }

    /// Draw PLAY/PASS button indicators
    /// Shows which actions are available to the current player
    pub fn draw_action_buttons(&self, game_state: &GameState) {
        if game_state.phase != GamePhase::Board || game_state.is_spinning {
            return;
        }

        let screen_w = screen_width();
        let screen_h = screen_height();

        // Position buttons below the board
        let button_y = screen_h * 0.76;
        let button_w = screen_w * 0.12;
        let button_h = screen_h * 0.04;
        let button_spacing = screen_w * 0.05;

        let center_x = screen_w / 2.0;

        // PLAY button (always available when not spinning)
        let play_x = center_x - button_spacing - button_w;
        self.draw_action_button(
            play_x,
            button_y,
            button_w,
            button_h,
            "PLAY",
            true, // Always active during board phase
            Color::new(0.0, 0.6, 0.2, 1.0), // Green
        );

        // PASS button (only available with earned spins)
        let contestant = &game_state.contestants[game_state.current_player];
        let can_pass = contestant.earned_spins > 0 && !contestant.eliminated;
        let pass_x = center_x + button_spacing;
        self.draw_action_button(
            pass_x,
            button_y,
            button_w,
            button_h,
            "PASS",
            can_pass,
            Color::new(0.8, 0.5, 0.0, 1.0), // Orange
        );
    }

    /// Draw a single action button with lit/unlit state
    fn draw_action_button(&self, x: f32, y: f32, width: f32, height: f32, label: &str, is_active: bool, active_color: Color) {
        // Button background
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

        // Draw button with rounded look (approximated with rectangles)
        draw_rectangle(x, y, width, height, bg_color);

        // Glow effect when active
        if is_active {
            let glow = (self.time * 4.0).sin() * 0.3 + 0.7;
            draw_rectangle_lines(x - 2.0, y - 2.0, width + 4.0, height + 4.0, 2.0,
                Color::new(active_color.r, active_color.g, active_color.b, glow * 0.6));
        }

        // Border
        let border_color = if is_active { WHITE } else { Color::new(0.4, 0.4, 0.4, 0.6) };
        draw_rectangle_lines(x, y, width, height, 2.0, border_color);

        // Inner highlight
        if is_active {
            draw_rectangle(x + 2.0, y + 2.0, width - 4.0, height * 0.3,
                Color::new(1.0, 1.0, 1.0, 0.2));
        }

        // Label text
        let text_size = height * 0.55;
        let dims = measure_text(label, None, text_size as u16, 1.0);
        let text_color = if is_active { WHITE } else { Color::new(0.5, 0.5, 0.5, 0.7) };

        draw_text(
            label,
            x + (width - dims.width) / 2.0,
            y + height * 0.68,
            text_size,
            text_color,
        );
    }

    /// Draw the Big Board with all 18 squares
    /// Includes authentic chase lights around the perimeter
    pub fn draw_big_board(&mut self, game_state: &GameState) {
        let board_rect = self.calculate_board_rect();

        // Draw outer frame with metallic appearance
        let frame_margin = 25.0;
        draw_rectangle(
            board_rect.x - frame_margin,
            board_rect.y - frame_margin,
            board_rect.w + frame_margin * 2.0,
            board_rect.h + frame_margin * 2.0,
            Color::new(0.15, 0.12, 0.2, 1.0),
        );

        // Draw board background
        draw_rectangle(
            board_rect.x - 10.0,
            board_rect.y - 10.0,
            board_rect.w + 20.0,
            board_rect.h + 20.0,
            Color::new(0.05, 0.02, 0.1, 0.95),
        );

        // Draw chase lights around the perimeter (authentic 1983-1986 CBS look)
        self.draw_chase_lights(
            board_rect.x - frame_margin + 5.0,
            board_rect.y - frame_margin + 5.0,
            board_rect.w + frame_margin * 2.0 - 10.0,
            board_rect.h + frame_margin * 2.0 - 10.0,
            game_state.is_spinning,
        );

        // Draw rainbow border (inside chase lights)
        self.draw_rainbow_border(
            board_rect.x - 12.0,
            board_rect.y - 12.0,
            board_rect.w + 24.0,
            board_rect.h + 24.0,
        );

        // Calculate square dimensions
        let square_w = board_rect.w / BOARD_COLS as f32;
        let square_h = board_rect.h / BOARD_ROWS as f32;

        // Draw each square with flash effects
        for (i, (col, row)) in SQUARE_POSITIONS.iter().enumerate() {
            let x = board_rect.x + *col as f32 * square_w;
            let y = board_rect.y + *row as f32 * square_h;

            let is_lit = i == game_state.lit_square;
            let square = &game_state.board[i];
            let flash_intensity = self.square_flash[i];

            self.draw_board_square_enhanced(
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

    /// Draw authentic chase lights around the board perimeter
    /// These lights chase around the border, speeding up during spins
    fn draw_chase_lights(&self, x: f32, y: f32, width: f32, height: f32, is_spinning: bool) {
        let perimeter = 2.0 * (width + height);
        let light_spacing = perimeter / CHASE_LIGHT_COUNT as f32;
        let bulb_radius = 4.0;

        // Number of lights that are "on" in the chase pattern
        let on_count = if is_spinning { 8 } else { 4 };

        for i in 0..CHASE_LIGHT_COUNT {
            let dist = i as f32 * light_spacing;
            let (lx, ly) = perimeter_point(x, y, width, height, dist);

            // Determine if this light is "on" based on chase phase
            let phase_offset = (self.chase_light_phase - i as f32).rem_euclid(CHASE_LIGHT_COUNT as f32);
            let is_on = phase_offset < on_count as f32;

            // Calculate brightness with smooth falloff
            let brightness = if is_on {
                let falloff = 1.0 - (phase_offset / on_count as f32);
                falloff.powf(0.5) // Smooth falloff curve
            } else {
                0.0
            };

            // Draw glow behind lit bulbs
            if brightness > 0.3 {
                let glow_size = bulb_radius * 2.5 * brightness;
                draw_circle(lx, ly, glow_size, Color::new(
                    CHASE_LIGHT_GLOW.r,
                    CHASE_LIGHT_GLOW.g,
                    CHASE_LIGHT_GLOW.b,
                    CHASE_LIGHT_GLOW.a * brightness,
                ));
            }

            // Draw the bulb
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

            draw_circle(lx, ly, bulb_radius, bulb_color);

            // Add highlight reflection on lit bulbs
            if brightness > 0.5 {
                draw_circle(
                    lx - bulb_radius * 0.3,
                    ly - bulb_radius * 0.3,
                    bulb_radius * 0.3,
                    Color::new(1.0, 1.0, 1.0, brightness * 0.6),
                );
            }
        }
    }

    /// Draw a single board square with enhanced flash effects
    fn draw_board_square_enhanced(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        prize: &crate::game::Prize,
        is_lit: bool,
        flash_intensity: f32,
        is_spinning: bool,
    ) {
        // Determine colors based on prize type
        let (primary, secondary) = match &prize.prize_type {
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

        // Draw gradient background with flash effect
        for row in 0..(height as i32) {
            let t = row as f32 / height;
            let color = Color::new(
                (primary_boosted.r * (1.0 - t * 0.3) + secondary.r * t * 0.3).min(1.0),
                (primary_boosted.g * (1.0 - t * 0.3) + secondary.g * t * 0.3).min(1.0),
                (primary_boosted.b * (1.0 - t * 0.3) + secondary.b * t * 0.3).min(1.0),
                1.0,
            );
            draw_line(x, y + row as f32, x + width, y + row as f32, 1.0, color);
        }

        // Enhanced active glow effect (brighter and more dramatic)
        if is_lit {
            let pulse = (self.time * 10.0).sin() * 0.2 + 0.8;

            // Outer bright glow
            draw_rectangle_lines(x - 6.0, y - 6.0, width + 12.0, height + 12.0, 6.0,
                Color::new(1.0, 1.0, 0.8, pulse * 0.8));

            // Inner white glow
            draw_rectangle_lines(x - 3.0, y - 3.0, width + 6.0, height + 6.0, 3.0,
                Color::new(1.0, 1.0, 1.0, pulse));

            // Top highlight shine
            draw_rectangle(x, y, width, height * 0.15,
                Color::new(1.0, 1.0, 1.0, 0.4));

            // Corner flares for extra drama during spin
            if is_spinning {
                let flare_size = width * 0.1 * pulse;
                draw_circle(x, y, flare_size, Color::new(1.0, 1.0, 1.0, 0.6));
                draw_circle(x + width, y, flare_size, Color::new(1.0, 1.0, 1.0, 0.6));
                draw_circle(x, y + height, flare_size, Color::new(1.0, 1.0, 1.0, 0.6));
                draw_circle(x + width, y + height, flare_size, Color::new(1.0, 1.0, 1.0, 0.6));
            }
        } else if flash_intensity > 0.1 {
            // Trail flash effect for recently passed squares
            draw_rectangle_lines(x - 2.0, y - 2.0, width + 4.0, height + 4.0, 2.0,
                Color::new(1.0, 1.0, 0.7, flash_intensity * 0.6));
        }

        // Border
        let border_color = if is_lit {
            WHITE
        } else if flash_intensity > 0.1 {
            Color::new(0.8, 0.8, 0.6, 0.9)
        } else {
            Color::new(0.3, 0.3, 0.3, 0.8)
        };
        draw_rectangle_lines(x, y, width, height, 2.0, border_color);

        // Prize text
        let main_size = width * 0.18;
        let sub_size = width * 0.12;

        // Main text
        let main_dims = measure_text(&prize.display_main, None, main_size as u16, 1.0);
        let main_y = if prize.display_sub.is_some() {
            y + height * 0.4
        } else {
            y + height * 0.55
        };

        // Shadow
        draw_text(
            &prize.display_main,
            x + (width - main_dims.width) / 2.0 + 1.0,
            main_y + 1.0,
            main_size,
            Color::new(0.0, 0.0, 0.0, 0.5),
        );

        // Main text (brighter when lit)
        let text_color = if is_lit {
            Color::new(1.0, 1.0, 0.9, 1.0)
        } else {
            WHITE
        };
        draw_text(
            &prize.display_main,
            x + (width - main_dims.width) / 2.0,
            main_y,
            main_size,
            text_color,
        );

        // Sub text (if present)
        if let Some(sub) = &prize.display_sub {
            let sub_dims = measure_text(sub, None, sub_size as u16, 1.0);
            draw_text(
                sub,
                x + (width - sub_dims.width) / 2.0,
                y + height * 0.7,
                sub_size,
                if is_lit { Color::new(1.0, 0.9, 0.3, 1.0) } else { GOLD },
            );
        }

        // Special Whammy decoration
        if matches!(prize.prize_type, PrizeType::Whammy) {
            self.draw_mini_whammy(x + width * 0.1, y + height * 0.15, width * 0.25);
            self.draw_mini_whammy(x + width * 0.65, y + height * 0.15, width * 0.25);
        }
    }

    /// Draw the center stage area (for Whammy animations and results)
    pub fn draw_center_stage(&self, game_state: &GameState) {
        let board_rect = self.board_rect.unwrap_or(Rect::new(0.0, 0.0, 100.0, 100.0));

        // Calculate center area (inside the board perimeter)
        let square_w = board_rect.w / BOARD_COLS as f32;
        let square_h = board_rect.h / BOARD_ROWS as f32;

        let center_x = board_rect.x + square_w;
        let center_y = board_rect.y + square_h;
        let center_w = board_rect.w - square_w * 2.0;
        let center_h = board_rect.h - square_h * 2.0;

        // Draw center background
        draw_rectangle(
            center_x,
            center_y,
            center_w,
            center_h,
            Color::new(0.02, 0.01, 0.05, 0.9),
        );

        // Draw based on game state
        if game_state.whammy_animation.active {
            self.draw_whammy_animation(
                center_x,
                center_y,
                center_w,
                center_h,
                &game_state.whammy_animation,
            );
        } else if game_state.result_state.showing {
            self.draw_result_display(
                center_x,
                center_y,
                center_w,
                center_h,
                &game_state.result_state.main_text,
                &game_state.result_state.sub_text,
            );
        } else {
            // Draw "BIG BOARD" text in center when idle
            let text = "THE BIG BOARD";
            let text_size = center_w * 0.1;
            let dims = measure_text(text, None, text_size as u16, 1.0);

            let pulse = (self.time * 1.5).sin() * 0.3 + 0.7;
            draw_text(
                text,
                center_x + (center_w - dims.width) / 2.0,
                center_y + center_h / 2.0,
                text_size,
                Color::new(0.5, 0.3, 0.6, pulse),
            );
        }
    }

    /// Draw a small Whammy icon
    fn draw_mini_whammy(&self, x: f32, y: f32, size: f32) {
        // Body (red circle)
        draw_circle(x + size / 2.0, y + size / 2.0, size / 2.0, WHAMMY_RED);

        // Eyes (white with black pupils and HOT_PINK glint)
        let eye_size = size * 0.2;
        let eye_y = y + size * 0.35;

        // Left eye
        draw_circle(x + size * 0.3, eye_y, eye_size, WHITE);
        draw_circle(x + size * 0.3, eye_y, eye_size * 0.5, BLACK);
        // HOT_PINK evil eye glint
        draw_circle(x + size * 0.25, eye_y - eye_size * 0.2, eye_size * 0.15, HOT_PINK);

        // Right eye
        draw_circle(x + size * 0.7, eye_y, eye_size, WHITE);
        draw_circle(x + size * 0.7, eye_y, eye_size * 0.5, BLACK);
        // HOT_PINK evil eye glint
        draw_circle(x + size * 0.65, eye_y - eye_size * 0.2, eye_size * 0.15, HOT_PINK);

        // Horns
        let horn_color = Color::new(0.6, 0.0, 0.0, 1.0);
        draw_triangle(
            Vec2::new(x + size * 0.25, y + size * 0.1),
            Vec2::new(x + size * 0.15, y - size * 0.15),
            Vec2::new(x + size * 0.35, y + size * 0.2),
            horn_color,
        );
        draw_triangle(
            Vec2::new(x + size * 0.75, y + size * 0.1),
            Vec2::new(x + size * 0.85, y - size * 0.15),
            Vec2::new(x + size * 0.65, y + size * 0.2),
            horn_color,
        );
    }

    /// Draw animated rainbow border
    fn draw_rainbow_border(&self, x: f32, y: f32, width: f32, height: f32) {
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

            draw_line(x1, y1, x2, y2, 4.0, color);
        }
    }

    /// Draw Whammy character animation
    fn draw_whammy_animation(
        &self,
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
        let (wx, wy, rotation, scale) = match animation.animation_type {
            WhammyAnimationType::Pogo => {
                let bounce = (progress * 10.0 * std::f32::consts::PI).sin().abs();
                (center_x, center_y - bounce * height * 0.3, 0.0, 1.0 + bounce * 0.2)
            }
            WhammyAnimationType::Dance => {
                let sway = (progress * 8.0 * std::f32::consts::PI).sin();
                (center_x + sway * width * 0.2, center_y, sway * 0.3, 1.0)
            }
            WhammyAnimationType::TNT => {
                if progress > 0.7 {
                    // Explosion
                    let exp_progress = (progress - 0.7) / 0.3;
                    (center_x, center_y, 0.0, 1.0 + exp_progress * 2.0)
                } else {
                    (center_x, center_y, progress * 0.5, 1.0)
                }
            }
            WhammyAnimationType::Hammer => {
                let swing = (progress * 6.0 * std::f32::consts::PI).sin();
                (center_x, center_y, swing * 0.5, 1.0)
            }
            WhammyAnimationType::LawnMower => {
                let x_pos = x + width * 0.1 + progress * width * 0.8;
                let bounce = (progress * 20.0 * std::f32::consts::PI).sin() * 0.1;
                (x_pos, center_y + bounce * height, 0.0, 1.0)
            }
            WhammyAnimationType::Fang => {
                let scale = 0.8 + (progress * 4.0 * std::f32::consts::PI).sin() * 0.2;
                (center_x, center_y, 0.0, scale)
            }
            WhammyAnimationType::Hula => {
                let sway = (progress * 6.0 * std::f32::consts::PI).sin();
                (center_x + sway * width * 0.15, center_y, 0.0, 1.0)
            }
            WhammyAnimationType::BoyGeorge => {
                let spin = progress * 4.0 * std::f32::consts::PI;
                (center_x, center_y, spin, 1.0)
            }
            WhammyAnimationType::Franklin => {
                let wobble = (progress * 8.0 * std::f32::consts::PI).sin() * 0.2;
                (center_x, center_y, wobble, 1.0)
            }
            WhammyAnimationType::Astronaut => {
                let float_y = center_y + (progress * 4.0 * std::f32::consts::PI).sin() * height * 0.2;
                (center_x, float_y, progress * 0.5, 1.0)
            }
            WhammyAnimationType::Jumping => {
                let jump = (progress * 8.0 * std::f32::consts::PI).sin().max(0.0);
                (center_x, center_y - jump * height * 0.4, 0.0, 1.0)
            }
            WhammyAnimationType::FlyingCarpet => {
                let x_pos = x + progress * width;
                let wave = (progress * 6.0 * std::f32::consts::PI).sin() * height * 0.15;
                (x_pos, center_y + wave, 0.0, 1.0)
            }
            // ═══════════════════════════════════════════════════════════════════════
            // New authentic Whammy animations (September 1983 - June 1986)
            // ═══════════════════════════════════════════════════════════════════════

            // Set 1 additions - Action animations with lateral movement
            WhammyAnimationType::Jaws => {
                // Shark attack - side-to-side swimming motion
                let x_pos = x + width * 0.1 + progress * width * 0.8;
                let swim = (progress * 8.0 * std::f32::consts::PI).sin() * 0.1;
                (x_pos, center_y + swim * height, swim * 0.2, 1.0)
            }
            WhammyAnimationType::Pilot => {
                // Plane nosedive - swooping motion
                let x_pos = x + width * 0.2 + progress * width * 0.6;
                let dive = progress * progress * height * 0.4;
                (x_pos, center_y - height * 0.3 + dive, progress * 0.8, 1.0)
            }
            WhammyAnimationType::Bulldozer => {
                // Bulldozer - steady horizontal movement
                let x_pos = x + width * 0.1 + progress * width * 0.8;
                let rumble = (progress * 20.0 * std::f32::consts::PI).sin() * 0.02;
                (x_pos, center_y + rumble * height, 0.0, 1.0)
            }
            WhammyAnimationType::Magician => {
                // Magician - flourishing movement with scale puff
                let puff = if progress > 0.7 { (progress - 0.7) / 0.3 } else { 0.0 };
                let scale = 1.0 - puff * 0.5;
                (center_x, center_y, progress * 0.3, scale)
            }

            // Set 2 (October 1983)
            WhammyAnimationType::RocketShip => {
                // Rocket launch - vertical lift-off
                let launch_y = if progress < 0.5 { center_y } else { center_y - (progress - 0.5) * 2.0 * height * 0.6 };
                let shake = (progress * 30.0 * std::f32::consts::PI).sin() * 0.03;
                (center_x + shake * width, launch_y, 0.0, 1.0 + progress * 0.3)
            }
            WhammyAnimationType::RollerSkating => {
                // Out of control skating
                let x_pos = x + width * 0.1 + progress * width * 0.8;
                let wobble = (progress * 12.0 * std::f32::consts::PI).sin() * 0.3;
                (x_pos, center_y, wobble, 1.0)
            }
            WhammyAnimationType::Boxer => {
                // Boxing stance then hit
                let punch = if progress > 0.7 { (progress - 0.7) / 0.3 } else { 0.0 };
                let recoil = punch * width * 0.3;
                (center_x - recoil, center_y, punch * 0.5, 1.0)
            }
            WhammyAnimationType::Jackhammer => {
                // Jackhammer vibration
                let shake = (progress * 40.0 * std::f32::consts::PI).sin() * 0.05;
                (center_x, center_y + shake * height, 0.0, 1.0)
            }
            WhammyAnimationType::Surfer => {
                // Surfing wave motion
                let x_pos = x + width * 0.2 + progress * width * 0.6;
                let wave = (progress * 6.0 * std::f32::consts::PI).sin() * height * 0.15;
                (x_pos, center_y + wave, (progress * 4.0 * std::f32::consts::PI).sin() * 0.15, 1.0)
            }

            // Set 3 (January 1984)
            WhammyAnimationType::Skateboarder => {
                // Skateboard crash
                let x_pos = x + width * 0.1 + progress * width * 0.8;
                let crash = if progress > 0.8 { (progress - 0.8) / 0.2 * 0.5 } else { 0.0 };
                (x_pos, center_y, crash, 1.0)
            }
            WhammyAnimationType::PaulRevere => {
                // Horse galloping
                let x_pos = x + width * 0.1 + progress * width * 0.8;
                let gallop = (progress * 16.0 * std::f32::consts::PI).sin().abs() * 0.1;
                (x_pos, center_y - gallop * height, 0.0, 1.0)
            }
            WhammyAnimationType::Skier => {
                // Skiing downhill
                let x_pos = x + width * 0.2 + progress * width * 0.6;
                let swerve = (progress * 4.0 * std::f32::consts::PI).sin() * width * 0.1;
                (x_pos + swerve, center_y + progress * height * 0.2, 0.0, 1.0)
            }
            WhammyAnimationType::DollarBill => {
                // Dollar bill floating
                let float = (progress * 5.0 * std::f32::consts::PI).sin() * 0.15;
                (center_x + float * width, center_y, float * 0.3, 1.0)
            }

            // Set 4-5 (March-June 1984)
            WhammyAnimationType::UFO => {
                // UFO hovering then zapped
                let hover = (progress * 6.0 * std::f32::consts::PI).sin() * height * 0.1;
                let zap = if progress > 0.8 { (progress - 0.8) / 0.2 } else { 0.0 };
                (center_x, center_y + hover, progress * 0.2, 1.0 - zap * 0.3)
            }
            WhammyAnimationType::MichaelJackson => {
                // Moonwalk - sliding backward
                let x_pos = x + width * 0.8 - progress * width * 0.6;
                let lean = (progress * 2.0 * std::f32::consts::PI).sin() * 0.15;
                (x_pos, center_y, lean, 1.0)
            }
            WhammyAnimationType::Breakdancing => {
                // Breakdancing spin
                let spin = progress * 8.0 * std::f32::consts::PI;
                (center_x, center_y, spin, 1.0)
            }
            WhammyAnimationType::Picnic => {
                // Picnic scene - gentle sway
                let sway = (progress * 3.0 * std::f32::consts::PI).sin() * 0.1;
                (center_x + sway * width, center_y, 0.0, 1.0)
            }
            WhammyAnimationType::WaterSkiing => {
                // Water skiing - wave motion
                let x_pos = x + width * 0.1 + progress * width * 0.8;
                let wave = (progress * 8.0 * std::f32::consts::PI).sin() * height * 0.1;
                (x_pos, center_y + wave, 0.0, 1.0)
            }
            WhammyAnimationType::DixielandBand => {
                // Band marching
                let x_pos = x + width * 0.1 + progress * width * 0.8;
                let step = (progress * 12.0 * std::f32::consts::PI).sin().abs() * 0.05;
                (x_pos, center_y - step * height, 0.0, 1.0)
            }

            // Set 6 (September 1984)
            WhammyAnimationType::Weightlifter => {
                // Lifting then falling through floor
                let lift = (progress * 2.0).min(1.0);
                let fall = if progress > 0.7 { (progress - 0.7) / 0.3 * height * 0.4 } else { 0.0 };
                (center_x, center_y - lift * height * 0.2 + fall, 0.0, 1.0)
            }
            WhammyAnimationType::PizzaGuy => {
                // Pizza spinning
                let spin = progress * 6.0 * std::f32::consts::PI;
                (center_x, center_y, spin * 0.1, 1.0)
            }
            WhammyAnimationType::Umpire => {
                // Umpire "You're out!" gesture
                let gesture = if progress > 0.5 { ((progress - 0.5) * 4.0).min(1.0) } else { 0.0 };
                (center_x, center_y, gesture * 0.3, 1.0 + gesture * 0.2)
            }
            WhammyAnimationType::Elvis => {
                // Elvis hip shake
                let shake = (progress * 10.0 * std::f32::consts::PI).sin() * 0.15;
                (center_x + shake * width, center_y, 0.0, 1.0)
            }
            WhammyAnimationType::FootballPlayer => {
                // Running for catch then explosion
                let x_pos = x + width * 0.2 + progress * width * 0.5;
                let boom = if progress > 0.7 { 1.0 + (progress - 0.7) / 0.3 } else { 1.0 };
                (x_pos, center_y, 0.0, boom)
            }

            // Set 7-8 (1985)
            WhammyAnimationType::Supremes => {
                // Singing trio sway
                let sway = (progress * 6.0 * std::f32::consts::PI).sin() * 0.1;
                (center_x, center_y, sway, 1.0)
            }
            WhammyAnimationType::Beatles => {
                // Beatles head bob
                let bob = (progress * 8.0 * std::f32::consts::PI).sin() * 0.1;
                (center_x, center_y + bob * height, 0.0, 1.0)
            }
            WhammyAnimationType::Tarzan => {
                // Vine swing
                let swing_x = (progress * 4.0 * std::f32::consts::PI).sin() * width * 0.3;
                let swing_y = (progress * 4.0 * std::f32::consts::PI).cos().abs() * height * 0.2;
                (center_x + swing_x, center_y - swing_y, 0.0, 1.0)
            }
            WhammyAnimationType::CruiseShip => {
                // Ship sailing
                let x_pos = x + progress * width;
                let rock = (progress * 4.0 * std::f32::consts::PI).sin() * 0.1;
                (x_pos, center_y, rock, 1.0)
            }
            WhammyAnimationType::CyndiLauper => {
                // Energetic bouncing
                let bounce = (progress * 12.0 * std::f32::consts::PI).sin().abs() * 0.2;
                let sway = (progress * 6.0 * std::f32::consts::PI).sin() * 0.15;
                (center_x + sway * width, center_y - bounce * height, 0.0, 1.0)
            }
            WhammyAnimationType::SherlockHolmes => {
                // Investigating - leaning motion
                let lean = (progress * 3.0 * std::f32::consts::PI).sin() * 0.2;
                (center_x + lean * width * 0.2, center_y, lean * 0.3, 1.0)
            }

            // Set 9-10 (Late 1985)
            WhammyAnimationType::Bowler => {
                // Bowling then struck
                let x_pos = x + width * 0.3 + progress * width * 0.4;
                let hit = if progress > 0.7 { (progress - 0.7) / 0.3 * 0.5 } else { 0.0 };
                (x_pos, center_y, hit, 1.0)
            }
            WhammyAnimationType::Photographer => {
                // Flash then explosion
                let flash = if progress > 0.4 && progress < 0.6 { 1.3 } else { 1.0 };
                (center_x, center_y, 0.0, flash)
            }
            WhammyAnimationType::BigBuck => {
                // Deer prancing
                let x_pos = x + width * 0.2 + progress * width * 0.6;
                let prance = (progress * 10.0 * std::f32::consts::PI).sin().abs() * 0.15;
                (x_pos, center_y - prance * height, 0.0, 1.0)
            }
            WhammyAnimationType::RockStar => {
                // Guitar shredding then electrocuted
                let shake = (progress * 16.0 * std::f32::consts::PI).sin() * 0.1;
                let zap = if progress > 0.8 { (progress - 0.8) / 0.2 * 0.3 } else { 0.0 };
                (center_x + shake * width, center_y, shake, 1.0 + zap)
            }
            WhammyAnimationType::HumanCannonball => {
                // Launch from cannon
                let launch = if progress > 0.3 { (progress - 0.3) / 0.7 } else { 0.0 };
                let x_pos = center_x + launch * width * 0.4;
                let y_pos = center_y - launch * (1.0 - launch) * height * 0.6;
                (x_pos, y_pos, launch * 2.0, 1.0)
            }
            WhammyAnimationType::AerobicsInstructor => {
                // Stretching then pretzel
                let stretch = (progress * 8.0 * std::f32::consts::PI).sin() * 0.2;
                let pretzel = if progress > 0.7 { (progress - 0.7) / 0.3 * 0.5 } else { 0.0 };
                (center_x, center_y, stretch + pretzel, 1.0 - pretzel * 0.2)
            }
            WhammyAnimationType::Liberace => {
                // Piano playing flourish
                let flourish = (progress * 6.0 * std::f32::consts::PI).sin() * 0.15;
                (center_x + flourish * width * 0.2, center_y, flourish * 0.2, 1.0)
            }
            WhammyAnimationType::BarbershopQuartet => {
                // Harmonizing sway
                let sway = (progress * 4.0 * std::f32::consts::PI).sin() * 0.1;
                (center_x, center_y, sway, 1.0)
            }

            // Set 11 (June 1986) - Final set
            WhammyAnimationType::Orchestra => {
                // Conducting then explosion
                let conduct = (progress * 6.0 * std::f32::consts::PI).sin() * 0.2;
                let boom = if progress > 0.8 { 1.0 + (progress - 0.8) / 0.2 * 0.5 } else { 1.0 };
                (center_x, center_y, conduct, boom)
            }
            WhammyAnimationType::EyeDoctor => {
                // Pointing at chart
                let point = (progress * 2.0).min(1.0) * 0.2;
                (center_x, center_y, point, 1.0)
            }
            WhammyAnimationType::Judge => {
                // Gavel slam
                let slam = (progress * 4.0 * std::f32::consts::PI).sin().abs() * 0.15;
                (center_x, center_y, slam, 1.0 + slam * 0.1)
            }
            WhammyAnimationType::ClownCar => {
                // Driving then firecracker
                let x_pos = x + width * 0.1 + progress * width * 0.6;
                let boom = if progress > 0.7 { 1.0 + (progress - 0.7) / 0.3 * 0.5 } else { 1.0 };
                (x_pos, center_y, 0.0, boom)
            }

            // ═══════════════════════════════════════════════════════════════════════
            // SPECIAL OCCASION WHAMMIES (Holiday specials)
            // ═══════════════════════════════════════════════════════════════════════

            WhammyAnimationType::ThanksgivingTurkey => {
                // Turkey hunting - musket backfire
                let aim = (progress * 2.0).min(1.0);
                let backfire = if progress > 0.6 { (progress - 0.6) / 0.4 } else { 0.0 };
                (center_x - backfire * width * 0.3, center_y, aim * 0.1, 1.0 - backfire * 0.2)
            }
            WhammyAnimationType::ScroogeWhammy => {
                // Scrooge with cane - hunched walk
                let x_pos = x + width * 0.2 + progress * width * 0.6;
                let hunch = (progress * 6.0 * std::f32::consts::PI).sin() * 0.05;
                (x_pos, center_y + hunch * height, 0.1, 0.9)
            }
            WhammyAnimationType::ChristmasTree => {
                // Decorating tree then it falls
                let sway = (progress * 4.0 * std::f32::consts::PI).sin() * 0.1;
                let fall = if progress > 0.7 { (progress - 0.7) / 0.3 * 0.8 } else { 0.0 };
                (center_x + fall * width * 0.3, center_y, sway + fall, 1.0)
            }
            WhammyAnimationType::ValentineCupid => {
                // Cupid flying and shooting arrow
                let fly_y = center_y + (progress * 4.0 * std::f32::consts::PI).sin() * height * 0.15;
                let ouch = if progress > 0.7 { (progress - 0.7) / 0.3 * 0.3 } else { 0.0 };
                (center_x, fly_y, ouch, 1.0)
            }
            WhammyAnimationType::EasterBunny => {
                // Bunny hopping with eggs falling
                let hop = (progress * 8.0 * std::f32::consts::PI).sin().abs() * 0.2;
                (center_x, center_y - hop * height, 0.0, 1.0)
            }
            WhammyAnimationType::Leprechaun => {
                // Leprechaun jig dancing
                let jig = (progress * 12.0 * std::f32::consts::PI).sin() * 0.15;
                let hop = (progress * 12.0 * std::f32::consts::PI).sin().abs() * 0.1;
                (center_x + jig * width, center_y - hop * height, 0.0, 1.0)
            }
            WhammyAnimationType::FourthOfJuly => {
                // Patriotic with sparkler mishap
                let wave = (progress * 8.0 * std::f32::consts::PI).sin() * 0.2;
                let boom = if progress > 0.7 { 1.0 + (progress - 0.7) / 0.3 * 0.5 } else { 1.0 };
                (center_x, center_y, wave, boom)
            }
            WhammyAnimationType::HalloweenVampire => {
                // Vampire cape flourish
                let cape = (progress * 4.0 * std::f32::consts::PI).sin() * 0.2;
                let scale = 1.0 + (progress * 2.0 * std::f32::consts::PI).sin().abs() * 0.1;
                (center_x, center_y, cape, scale)
            }
            WhammyAnimationType::OlympicsWhammy => {
                // Running then tripping over hurdle
                let x_pos = x + width * 0.1 + progress * width * 0.7;
                let trip = if progress > 0.6 { (progress - 0.6) / 0.4 * 1.0 } else { 0.0 };
                (x_pos, center_y + trip * height * 0.2, trip * 0.5, 1.0)
            }
            WhammyAnimationType::NewYearBaby => {
                // Baby New Year bouncing
                let bounce = (progress * 10.0 * std::f32::consts::PI).sin().abs() * 0.15;
                (center_x, center_y - bounce * height, 0.0, 0.8)
            }

            // ═══════════════════════════════════════════════════════════════════════
            // WHAMMY-OUT ANIMATIONS (4th Whammy elimination specials)
            // ═══════════════════════════════════════════════════════════════════════

            WhammyAnimationType::GrimReaper => {
                // Grim Reaper slowly approaching then escorting out
                let x_pos = x + width * 0.8 - progress * width * 0.6;
                let hover = (progress * 3.0 * std::f32::consts::PI).sin() * 0.05;
                (x_pos, center_y + hover * height, 0.0, 1.2)
            }
            WhammyAnimationType::FiringSquad => {
                // Firing squad - "Ready! Aim! FIRE!"
                let aim = (progress * 3.0).min(1.0);
                let recoil = if progress > 0.8 { (progress - 0.8) / 0.2 * 0.3 } else { 0.0 };
                (center_x, center_y, aim * 0.1 - recoil, 1.0 + recoil * 0.2)
            }
            WhammyAnimationType::AngelWhammy => {
                // Angel floating up to heaven with money
                let rise = progress * height * 0.5;
                let wings = (progress * 8.0 * std::f32::consts::PI).sin() * 0.1;
                (center_x, center_y - rise, wings, 1.0 - progress * 0.3)
            }
        };

        // Draw the Whammy character
        self.draw_whammy_character(wx, wy, whammy_size * scale, rotation, animation.animation_type);

        // Draw catchphrase
        if !animation.catchphrase.is_empty() {
            let text_size = width * 0.08;
            let dims = measure_text(&animation.catchphrase, None, text_size as u16, 1.0);

            // Speech bubble background
            let bubble_x = x + (width - dims.width - 20.0) / 2.0;
            let bubble_y = y + height * 0.75;
            draw_rectangle(
                bubble_x - 10.0,
                bubble_y - text_size,
                dims.width + 20.0,
                text_size + 10.0,
                WHITE,
            );

            draw_text(
                &animation.catchphrase,
                bubble_x,
                bubble_y,
                text_size,
                BLACK,
            );
        }

        // TNT explosion effect
        if matches!(animation.animation_type, WhammyAnimationType::TNT) && progress > 0.7 {
            let exp_progress = (progress - 0.7) / 0.3;
            for i in 0..12 {
                let angle = i as f32 * std::f32::consts::PI / 6.0;
                let dist = exp_progress * width * 0.5;
                let px = center_x + angle.cos() * dist;
                let py = center_y + angle.sin() * dist;
                let star_size = whammy_size * 0.3 * (1.0 - exp_progress);
                draw_circle(px, py, star_size, GOLD);
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // AUTHENTIC WHAMMY CHARACTER (1983-1986 CBS Press Your Luck)
    // ═══════════════════════════════════════════════════════════════════════════════
    // Research sources:
    // - Character designed by Savage Steve Holland, sketched on a napkin
    // - Red cartoon creature (NOT a devil - that was predecessor show "Second Chance")
    // - Yellow superhero-style eye mask
    // - Yellow flowing cape
    // - Dollar sign ($) on yellow chest shield/emblem
    // - Small tuft of hair on top of head
    // - Pointy feet
    // - Mischievous grin and big expressive eyes
    // - Animated by Holland on early computer graphics system
    // ═══════════════════════════════════════════════════════════════════════════════

    /// Draw the authentic Whammy character from the 1983-1986 CBS Press Your Luck
    ///
    /// The Whammy was a mischievous red cartoon creature with:
    /// - Yellow superhero-style eye mask
    /// - Yellow flowing cape
    /// - Dollar sign ($) insignia on yellow chest shield
    /// - Small tuft of hair (NOT devil horns)
    /// - Pointy feet
    /// - Big expressive eyes and wide mischievous grin
    fn draw_whammy_character(&self, x: f32, y: f32, size: f32, rotation: f32, animation_type: WhammyAnimationType) {
        // Apply rotation transform
        let cos_r = rotation.cos();
        let sin_r = rotation.sin();

        // Breathing/bounce animation for liveliness
        let breath = 1.0 + (self.time * 4.0).sin() * 0.02;

        // ═══════════════════════════════════════════════════════════════════════════
        // CAPE (Drawn first, behind body) - Yellow flowing superhero cape
        // ═══════════════════════════════════════════════════════════════════════════
        self.draw_whammy_cape(x, y, size, rotation, animation_type);

        // ═══════════════════════════════════════════════════════════════════════════
        // BODY - Red rounded imp/creature body
        // ═══════════════════════════════════════════════════════════════════════════
        self.draw_whammy_body(x, y, size, breath, animation_type);

        // ═══════════════════════════════════════════════════════════════════════════
        // ARMS - Thin cartoon arms with hands
        // ═══════════════════════════════════════════════════════════════════════════
        self.draw_whammy_arms(x, y, size, animation_type);

        // ═══════════════════════════════════════════════════════════════════════════
        // LEGS - Thin legs with pointy feet (authentic to the show)
        // ═══════════════════════════════════════════════════════════════════════════
        self.draw_whammy_legs(x, y, size, animation_type);

        // ═══════════════════════════════════════════════════════════════════════════
        // DOLLAR SIGN CHEST EMBLEM - Yellow shield with $ symbol
        // ═══════════════════════════════════════════════════════════════════════════
        self.draw_whammy_emblem(x, y, size);

        // ═══════════════════════════════════════════════════════════════════════════
        // HEAD/FACE - Eyes visible through yellow mask, hair tuft, mischievous grin
        // ═══════════════════════════════════════════════════════════════════════════
        self.draw_whammy_face(x, y, size, cos_r, sin_r, animation_type);
    }

    /// Draw the Whammy's yellow flowing cape (behind body)
    fn draw_whammy_cape(&self, x: f32, y: f32, size: f32, _rotation: f32, animation_type: WhammyAnimationType) {
        // Cape color - bright yellow like the 1983 show
        let cape_yellow = Color::new(1.0, 0.85, 0.0, 1.0);
        let cape_dark = Color::new(0.85, 0.7, 0.0, 1.0);

        // Cape wave animation
        let wave = match animation_type {
            WhammyAnimationType::Dance | WhammyAnimationType::Hula =>
                (self.time * 6.0).sin() * size * 0.08,
            WhammyAnimationType::Pogo | WhammyAnimationType::Jumping =>
                (self.time * 10.0).sin() * size * 0.1,
            WhammyAnimationType::FlyingCarpet | WhammyAnimationType::LawnMower =>
                (self.time * 8.0).sin() * size * 0.12,
            _ => (self.time * 3.0).sin() * size * 0.04,
        };

        // Cape attaches at neck/shoulder area and flows down and back
        let cape_attach_y = y - size * 0.15;
        let cape_width = size * 0.5;
        let cape_length = size * 0.6;

        // Draw flowing cape with multiple segments for wave effect
        // Left cape side
        draw_triangle(
            Vec2::new(x - size * 0.2, cape_attach_y),
            Vec2::new(x - cape_width + wave * 0.5, cape_attach_y + cape_length),
            Vec2::new(x - size * 0.05, cape_attach_y + cape_length * 0.5),
            cape_dark,
        );
        draw_triangle(
            Vec2::new(x - size * 0.2, cape_attach_y),
            Vec2::new(x - cape_width * 0.7 + wave, cape_attach_y + cape_length * 0.8),
            Vec2::new(x - cape_width + wave * 0.5, cape_attach_y + cape_length),
            cape_yellow,
        );

        // Right cape side
        draw_triangle(
            Vec2::new(x + size * 0.2, cape_attach_y),
            Vec2::new(x + cape_width - wave * 0.5, cape_attach_y + cape_length),
            Vec2::new(x + size * 0.05, cape_attach_y + cape_length * 0.5),
            cape_dark,
        );
        draw_triangle(
            Vec2::new(x + size * 0.2, cape_attach_y),
            Vec2::new(x + cape_width * 0.7 - wave, cape_attach_y + cape_length * 0.8),
            Vec2::new(x + cape_width - wave * 0.5, cape_attach_y + cape_length),
            cape_yellow,
        );

        // Cape collar - connects to body
        draw_circle(x, cape_attach_y, size * 0.12, cape_yellow);
    }

    /// Draw the Whammy's red body
    fn draw_whammy_body(&self, x: f32, y: f32, size: f32, breath: f32, _animation_type: WhammyAnimationType) {
        let body_width = size * 0.35 * breath;
        let body_height = size * 0.4 * breath;

        // Body shadow for depth
        draw_ellipse(
            x + size * 0.02,
            y + size * 0.02,
            body_width,
            body_height,
            0.0,
            Color::new(0.3, 0.0, 0.0, 0.5)
        );

        // Main body - red oval torso
        draw_ellipse(x, y, body_width, body_height, 0.0, WHAMMY_RED);

        // Body highlight for 3D cartoon effect
        draw_ellipse(
            x - size * 0.08,
            y - size * 0.1,
            body_width * 0.5,
            body_height * 0.4,
            0.0,
            Color::new(1.0, 0.35, 0.3, 0.4)
        );

        // Darker belly area (subtle shading)
        draw_ellipse(
            x,
            y + size * 0.08,
            body_width * 0.7,
            body_height * 0.5,
            0.0,
            Color::new(0.7, 0.05, 0.05, 0.3)
        );
    }

    /// Draw the Whammy's thin cartoon arms with hands
    fn draw_whammy_arms(&self, x: f32, y: f32, size: f32, animation_type: WhammyAnimationType) {
        // Arm animation based on what the Whammy is doing
        let (left_arm_angle, right_arm_angle, left_wave, right_wave) = match animation_type {
            WhammyAnimationType::Dance => {
                let wave = (self.time * 6.0).sin();
                (0.3 + wave * 0.4, -0.3 - wave * 0.4, wave * 0.1, -wave * 0.1)
            }
            WhammyAnimationType::Pogo | WhammyAnimationType::Jumping => {
                let bounce = (self.time * 10.0).sin();
                (-0.5 - bounce.abs() * 0.3, 0.5 + bounce.abs() * 0.3, 0.0, 0.0)
            }
            WhammyAnimationType::Hammer => {
                let swing = (self.time * 8.0).sin();
                (swing * 0.8, swing * 0.8, 0.0, 0.0)
            }
            WhammyAnimationType::Hula => {
                let sway = (self.time * 5.0).sin();
                (0.4 + sway * 0.2, -0.4 - sway * 0.2, sway * 0.05, -sway * 0.05)
            }
            _ => {
                // Default subtle arm movement
                let subtle = (self.time * 2.0).sin() * 0.1;
                (0.3 + subtle, -0.3 - subtle, 0.0, 0.0)
            }
        };

        let arm_length = size * 0.35;
        let arm_thickness = size * 0.06;
        let hand_size = size * 0.08;

        // Left arm
        let left_shoulder = (x - size * 0.28, y - size * 0.05);
        let left_elbow_x = left_shoulder.0 - arm_length * 0.5 * (1.0 + left_arm_angle).cos() + left_wave * size;
        let left_elbow_y = left_shoulder.1 + arm_length * 0.5 * (1.0 + left_arm_angle).sin();
        let left_hand_x = left_elbow_x - arm_length * 0.5 * left_arm_angle.cos() + left_wave * size;
        let left_hand_y = left_elbow_y + arm_length * 0.5 * left_arm_angle.sin().abs();

        // Left upper arm
        draw_line(left_shoulder.0, left_shoulder.1, left_elbow_x, left_elbow_y, arm_thickness, WHAMMY_RED);
        // Left forearm
        draw_line(left_elbow_x, left_elbow_y, left_hand_x, left_hand_y, arm_thickness * 0.8, WHAMMY_RED);
        // Left hand (circle)
        draw_circle(left_hand_x, left_hand_y, hand_size, WHAMMY_RED);
        // Hand highlight
        draw_circle(left_hand_x - hand_size * 0.3, left_hand_y - hand_size * 0.3, hand_size * 0.3, Color::new(1.0, 0.4, 0.35, 0.5));

        // Right arm
        let right_shoulder = (x + size * 0.28, y - size * 0.05);
        let right_elbow_x = right_shoulder.0 + arm_length * 0.5 * (1.0 - right_arm_angle).cos() + right_wave * size;
        let right_elbow_y = right_shoulder.1 + arm_length * 0.5 * (1.0 - right_arm_angle).sin();
        let right_hand_x = right_elbow_x + arm_length * 0.5 * (-right_arm_angle).cos() + right_wave * size;
        let right_hand_y = right_elbow_y + arm_length * 0.5 * (-right_arm_angle).sin().abs();

        // Right upper arm
        draw_line(right_shoulder.0, right_shoulder.1, right_elbow_x, right_elbow_y, arm_thickness, WHAMMY_RED);
        // Right forearm
        draw_line(right_elbow_x, right_elbow_y, right_hand_x, right_hand_y, arm_thickness * 0.8, WHAMMY_RED);
        // Right hand (circle)
        draw_circle(right_hand_x, right_hand_y, hand_size, WHAMMY_RED);
        // Hand highlight
        draw_circle(right_hand_x - hand_size * 0.3, right_hand_y - hand_size * 0.3, hand_size * 0.3, Color::new(1.0, 0.4, 0.35, 0.5));
    }

    /// Draw the Whammy's thin legs with pointy feet
    fn draw_whammy_legs(&self, x: f32, y: f32, size: f32, animation_type: WhammyAnimationType) {
        // Leg animation
        let (left_leg_offset, right_leg_offset, leg_spread) = match animation_type {
            WhammyAnimationType::Dance => {
                let dance = (self.time * 5.0).sin();
                (dance * size * 0.08, -dance * size * 0.08, 0.15)
            }
            WhammyAnimationType::Pogo | WhammyAnimationType::Jumping => {
                let jump = (self.time * 12.0).sin();
                (0.0, 0.0, 0.1 + jump.abs() * 0.1)
            }
            WhammyAnimationType::Hula => {
                let sway = (self.time * 4.0).sin();
                (sway * size * 0.05, -sway * size * 0.05, 0.18)
            }
            WhammyAnimationType::LawnMower | WhammyAnimationType::RollerSkating => {
                let run = (self.time * 10.0).sin();
                (run * size * 0.12, -run * size * 0.12, 0.12)
            }
            _ => (0.0, 0.0, 0.15),
        };

        let leg_top_y = y + size * 0.25;
        let leg_length = size * 0.4;
        let leg_thickness = size * 0.055;
        let foot_width = size * 0.12;
        let foot_height = size * 0.06;

        // Left leg
        let left_hip_x = x - size * leg_spread;
        let left_knee_y = leg_top_y + leg_length * 0.5;
        let left_foot_y = leg_top_y + leg_length;

        // Left thigh
        draw_line(
            left_hip_x, leg_top_y,
            left_hip_x + left_leg_offset * 0.5, left_knee_y,
            leg_thickness, WHAMMY_RED
        );
        // Left shin
        draw_line(
            left_hip_x + left_leg_offset * 0.5, left_knee_y,
            left_hip_x + left_leg_offset, left_foot_y,
            leg_thickness * 0.9, WHAMMY_RED
        );
        // Left pointy foot (triangle pointing outward)
        draw_triangle(
            Vec2::new(left_hip_x + left_leg_offset - foot_width * 0.3, left_foot_y),
            Vec2::new(left_hip_x + left_leg_offset - foot_width, left_foot_y + foot_height),
            Vec2::new(left_hip_x + left_leg_offset + foot_width * 0.5, left_foot_y + foot_height),
            WHAMMY_RED,
        );

        // Right leg
        let right_hip_x = x + size * leg_spread;
        let right_knee_y = leg_top_y + leg_length * 0.5;
        let right_foot_y = leg_top_y + leg_length;

        // Right thigh
        draw_line(
            right_hip_x, leg_top_y,
            right_hip_x + right_leg_offset * 0.5, right_knee_y,
            leg_thickness, WHAMMY_RED
        );
        // Right shin
        draw_line(
            right_hip_x + right_leg_offset * 0.5, right_knee_y,
            right_hip_x + right_leg_offset, right_foot_y,
            leg_thickness * 0.9, WHAMMY_RED
        );
        // Right pointy foot (triangle pointing outward)
        draw_triangle(
            Vec2::new(right_hip_x + right_leg_offset + foot_width * 0.3, right_foot_y),
            Vec2::new(right_hip_x + right_leg_offset + foot_width, right_foot_y + foot_height),
            Vec2::new(right_hip_x + right_leg_offset - foot_width * 0.5, right_foot_y + foot_height),
            WHAMMY_RED,
        );
    }

    /// Draw the Whammy's dollar sign chest emblem (yellow shield with $)
    fn draw_whammy_emblem(&self, x: f32, y: f32, size: f32) {
        let emblem_y = y + size * 0.05;
        let emblem_width = size * 0.18;
        let emblem_height = size * 0.2;

        // Yellow shield background
        let shield_yellow = Color::new(1.0, 0.85, 0.0, 1.0);
        let shield_dark = Color::new(0.9, 0.75, 0.0, 1.0);

        // Shield shape (pentagon-ish)
        draw_triangle(
            Vec2::new(x - emblem_width, emblem_y - emblem_height * 0.4),
            Vec2::new(x + emblem_width, emblem_y - emblem_height * 0.4),
            Vec2::new(x, emblem_y + emblem_height),
            shield_yellow,
        );
        draw_rectangle(
            x - emblem_width, emblem_y - emblem_height * 0.5,
            emblem_width * 2.0, emblem_height * 0.5,
            shield_yellow,
        );

        // Shield border
        draw_line(
            x - emblem_width, emblem_y - emblem_height * 0.5,
            x + emblem_width, emblem_y - emblem_height * 0.5,
            2.0, shield_dark
        );
        draw_line(
            x - emblem_width, emblem_y - emblem_height * 0.5,
            x - emblem_width, emblem_y - emblem_height * 0.4,
            2.0, shield_dark
        );
        draw_line(
            x + emblem_width, emblem_y - emblem_height * 0.5,
            x + emblem_width, emblem_y - emblem_height * 0.4,
            2.0, shield_dark
        );
        draw_line(x - emblem_width, emblem_y - emblem_height * 0.4, x, emblem_y + emblem_height, 2.0, shield_dark);
        draw_line(x + emblem_width, emblem_y - emblem_height * 0.4, x, emblem_y + emblem_height, 2.0, shield_dark);

        // Dollar sign ($) on the shield - authentic to the 1983 show
        let dollar_size = emblem_height * 0.5;
        let dollar_y = emblem_y - emblem_height * 0.15;

        // Draw $ using lines (S-curve with vertical line)
        // Vertical line through the S
        draw_line(x, dollar_y - dollar_size * 0.7, x, dollar_y + dollar_size * 0.7, 2.5, BLACK);

        // S curve (simplified as two arcs)
        // Top curve of S
        for i in 0..15 {
            let t = i as f32 / 14.0;
            let angle = std::f32::consts::PI * (0.5 + t * 0.8);
            let cx = x + angle.cos() * dollar_size * 0.35;
            let cy = dollar_y - dollar_size * 0.3 + angle.sin() * dollar_size * 0.25;
            if i > 0 {
                let prev_t = (i - 1) as f32 / 14.0;
                let prev_angle = std::f32::consts::PI * (0.5 + prev_t * 0.8);
                let prev_cx = x + prev_angle.cos() * dollar_size * 0.35;
                let prev_cy = dollar_y - dollar_size * 0.3 + prev_angle.sin() * dollar_size * 0.25;
                draw_line(prev_cx, prev_cy, cx, cy, 2.5, BLACK);
            }
        }
        // Bottom curve of S (reversed direction)
        for i in 0..15 {
            let t = i as f32 / 14.0;
            let angle = std::f32::consts::PI * (1.5 + t * 0.8);
            let cx = x + angle.cos() * dollar_size * 0.35;
            let cy = dollar_y + dollar_size * 0.2 + angle.sin() * dollar_size * 0.25;
            if i > 0 {
                let prev_t = (i - 1) as f32 / 14.0;
                let prev_angle = std::f32::consts::PI * (1.5 + prev_t * 0.8);
                let prev_cx = x + prev_angle.cos() * dollar_size * 0.35;
                let prev_cy = dollar_y + dollar_size * 0.2 + prev_angle.sin() * dollar_size * 0.25;
                draw_line(prev_cx, prev_cy, cx, cy, 2.5, BLACK);
            }
        }
    }

    /// Draw the Whammy's face with yellow mask, eyes, hair tuft, and mischievous grin
    fn draw_whammy_face(&self, x: f32, y: f32, size: f32, cos_r: f32, sin_r: f32, animation_type: WhammyAnimationType) {
        // Head is slightly above and overlapping the body
        let head_y = y - size * 0.25;
        let head_radius = size * 0.32;

        // Head shadow
        draw_circle(x + size * 0.015, head_y + size * 0.015, head_radius, Color::new(0.3, 0.0, 0.0, 0.4));

        // Main head (red circle)
        draw_circle(x, head_y, head_radius, WHAMMY_RED);

        // Head highlight
        draw_circle(x - head_radius * 0.35, head_y - head_radius * 0.35, head_radius * 0.4, Color::new(1.0, 0.4, 0.35, 0.4));

        // ─── YELLOW EYE MASK (Superhero/bandit style) ───
        let mask_yellow = Color::new(1.0, 0.85, 0.0, 1.0);
        let mask_dark = Color::new(0.85, 0.7, 0.0, 1.0);

        // Mask covers the eye area across the face
        let mask_width = head_radius * 1.3;
        let mask_height = head_radius * 0.45;
        let mask_y = head_y - head_radius * 0.1;

        // Main mask shape (elongated oval)
        draw_ellipse(x, mask_y, mask_width, mask_height, 0.0, mask_yellow);

        // Mask points (superhero style points at the sides)
        // Left point
        draw_triangle(
            Vec2::new(x - mask_width, mask_y),
            Vec2::new(x - mask_width - size * 0.12, mask_y - size * 0.08),
            Vec2::new(x - mask_width + size * 0.05, mask_y - mask_height * 0.5),
            mask_yellow,
        );
        // Right point
        draw_triangle(
            Vec2::new(x + mask_width, mask_y),
            Vec2::new(x + mask_width + size * 0.12, mask_y - size * 0.08),
            Vec2::new(x + mask_width - size * 0.05, mask_y - mask_height * 0.5),
            mask_yellow,
        );

        // Mask outline for definition
        draw_ellipse_lines(x, mask_y, mask_width, mask_height, 0.0, 2.0, mask_dark);

        // ─── EYES (Large, visible through the mask) ───
        let eye_offset_x = size * 0.18;
        let eye_offset_y = head_y - size * 0.02;
        let eye_size = size * 0.14;

        // Apply rotation to eye positions
        let left_eye_x = x + ((-eye_offset_x) * cos_r);
        let left_eye_y = eye_offset_y + ((-eye_offset_x) * sin_r);
        let right_eye_x = x + (eye_offset_x * cos_r);
        let right_eye_y = eye_offset_y + (eye_offset_x * sin_r);

        // Eye holes in mask (dark outline)
        draw_circle(left_eye_x, left_eye_y, eye_size * 1.1, mask_dark);
        draw_circle(right_eye_x, right_eye_y, eye_size * 1.1, mask_dark);

        // White of eyes
        draw_circle(left_eye_x, left_eye_y, eye_size, WHITE);
        draw_circle(right_eye_x, right_eye_y, eye_size, WHITE);

        // Animated pupils based on animation type
        let pupil_offset = match animation_type {
            WhammyAnimationType::Dance => ((self.time * 5.0).sin() * eye_size * 0.3, 0.0),
            WhammyAnimationType::Jumping | WhammyAnimationType::Pogo => (0.0, -eye_size * 0.25),
            WhammyAnimationType::Fang => (0.0, eye_size * 0.15),
            WhammyAnimationType::TNT => {
                ((self.time * 8.0).sin() * eye_size * 0.2, (self.time * 6.0).cos() * eye_size * 0.2)
            }
            WhammyAnimationType::LawnMower | WhammyAnimationType::RollerSkating => {
                ((self.time * 10.0).sin() * eye_size * 0.15, 0.0)
            }
            _ => ((self.time * 2.0).sin() * eye_size * 0.08, 0.0),
        };

        // Pupils (larger, more expressive)
        let pupil_size = eye_size * 0.5;
        draw_circle(
            left_eye_x + pupil_offset.0,
            left_eye_y + pupil_offset.1,
            pupil_size, BLACK
        );
        draw_circle(
            right_eye_x + pupil_offset.0,
            right_eye_y + pupil_offset.1,
            pupil_size, BLACK
        );

        // Eye glints for life
        let glint_size = eye_size * 0.2;
        draw_circle(left_eye_x - eye_size * 0.25, left_eye_y - eye_size * 0.2, glint_size, WHITE);
        draw_circle(right_eye_x - eye_size * 0.25, right_eye_y - eye_size * 0.2, glint_size, WHITE);

        // ─── SMALL TUFT OF HAIR (NOT horns - authentic to the show) ───
        let hair_base_y = head_y - head_radius;
        let hair_color = WHAMMY_RED;
        let hair_dark = Color::new(0.6, 0.0, 0.0, 1.0);

        // Small spiky tuft on top of head
        // Center spike
        draw_triangle(
            Vec2::new(x - size * 0.04, hair_base_y + size * 0.02),
            Vec2::new(x, hair_base_y - size * 0.12),
            Vec2::new(x + size * 0.04, hair_base_y + size * 0.02),
            hair_color,
        );
        // Left smaller spike
        draw_triangle(
            Vec2::new(x - size * 0.08, hair_base_y + size * 0.04),
            Vec2::new(x - size * 0.05, hair_base_y - size * 0.06),
            Vec2::new(x - size * 0.02, hair_base_y + size * 0.02),
            hair_dark,
        );
        // Right smaller spike
        draw_triangle(
            Vec2::new(x + size * 0.02, hair_base_y + size * 0.02),
            Vec2::new(x + size * 0.05, hair_base_y - size * 0.06),
            Vec2::new(x + size * 0.08, hair_base_y + size * 0.04),
            hair_dark,
        );

        // ─── MISCHIEVOUS GRIN ───
        let mouth_y = head_y + head_radius * 0.5;
        let mouth_color = BLACK;

        match animation_type {
            WhammyAnimationType::TNT | WhammyAnimationType::Hammer |
            WhammyAnimationType::RocketShip | WhammyAnimationType::FiringSquad => {
                // Excited/surprised open mouth (O shape)
                draw_circle(x, mouth_y, size * 0.08, mouth_color);
                // Tongue inside
                draw_circle(x, mouth_y + size * 0.02, size * 0.04, Color::new(0.8, 0.2, 0.2, 1.0));
            }
            WhammyAnimationType::Fang | WhammyAnimationType::GrimReaper |
            WhammyAnimationType::HalloweenVampire => {
                // Evil toothy grin with fangs showing
                let grin_width = size * 0.2;
                // Mouth line
                draw_line(x - grin_width, mouth_y, x + grin_width, mouth_y, 3.0, mouth_color);
                // Curved up at corners (evil smile)
                draw_line(x - grin_width, mouth_y, x - grin_width - size * 0.03, mouth_y - size * 0.04, 3.0, mouth_color);
                draw_line(x + grin_width, mouth_y, x + grin_width + size * 0.03, mouth_y - size * 0.04, 3.0, mouth_color);
                // Fangs
                draw_triangle(
                    Vec2::new(x - size * 0.08, mouth_y),
                    Vec2::new(x - size * 0.1, mouth_y + size * 0.08),
                    Vec2::new(x - size * 0.06, mouth_y),
                    WHITE,
                );
                draw_triangle(
                    Vec2::new(x + size * 0.08, mouth_y),
                    Vec2::new(x + size * 0.1, mouth_y + size * 0.08),
                    Vec2::new(x + size * 0.06, mouth_y),
                    WHITE,
                );
            }
            WhammyAnimationType::AngelWhammy => {
                // Innocent smile (small, closed)
                let smile_width = size * 0.12;
                for i in 0..20 {
                    let t = i as f32 / 19.0;
                    let angle = std::f32::consts::PI * t;
                    let sx = x - smile_width + (smile_width * 2.0) * t;
                    let sy = mouth_y + angle.sin() * size * 0.05;
                    if i > 0 {
                        let prev_t = (i - 1) as f32 / 19.0;
                        let prev_angle = std::f32::consts::PI * prev_t;
                        let prev_sx = x - smile_width + (smile_width * 2.0) * prev_t;
                        let prev_sy = mouth_y + prev_angle.sin() * size * 0.05;
                        draw_line(prev_sx, prev_sy, sx, sy, 2.5, mouth_color);
                    }
                }
            }
            _ => {
                // Default mischievous wide grin (signature Whammy expression)
                let grin_width = size * 0.22;
                let grin_height = size * 0.12;

                // Draw wide curved grin
                for i in 0..30 {
                    let t = i as f32 / 29.0;
                    let angle = std::f32::consts::PI * t;
                    let gx = x - grin_width + (grin_width * 2.0) * t;
                    let gy = mouth_y + angle.sin() * grin_height;

                    if i > 0 {
                        let prev_t = (i - 1) as f32 / 29.0;
                        let prev_angle = std::f32::consts::PI * prev_t;
                        let prev_gx = x - grin_width + (grin_width * 2.0) * prev_t;
                        let prev_gy = mouth_y + prev_angle.sin() * grin_height;
                        draw_line(prev_gx, prev_gy, gx, gy, 4.0, mouth_color);
                    }
                }

                // Upper lip line for that mischievous look
                draw_line(x - grin_width, mouth_y, x + grin_width, mouth_y, 2.0, mouth_color);

                // Optional teeth showing for extra mischief
                let tooth_y = mouth_y + 1.0;
                draw_rectangle(x - size * 0.05, tooth_y, size * 0.04, size * 0.04, WHITE);
                draw_rectangle(x + size * 0.01, tooth_y, size * 0.04, size * 0.04, WHITE);
            }
        }

        // ─── NOSE (Small, cute) ───
        let nose_y = head_y + head_radius * 0.15;
        draw_circle(x, nose_y, size * 0.035, Color::new(0.75, 0.05, 0.05, 1.0));
        draw_circle(x - size * 0.01, nose_y - size * 0.01, size * 0.015, Color::new(1.0, 0.3, 0.3, 0.5));
    }

    /// Draw result display in center
    fn draw_result_display(&self, x: f32, y: f32, width: f32, height: f32, main: &str, sub: &str) {
        let pulse = (self.time * 3.0).sin() * 0.1 + 0.9;

        // Main text
        let main_size = width * 0.15;
        let main_dims = measure_text(main, None, main_size as u16, 1.0);

        // Shadow
        draw_text(
            main,
            x + (width - main_dims.width) / 2.0 + 3.0,
            y + height * 0.45 + 3.0,
            main_size,
            Color::new(0.0, 0.0, 0.0, 0.5),
        );

        // Main text with pulse
        draw_text(
            main,
            x + (width - main_dims.width) / 2.0,
            y + height * 0.45,
            main_size * pulse,
            GOLD,
        );

        // Sub text
        if !sub.is_empty() {
            let sub_size = width * 0.08;
            let sub_dims = measure_text(sub, None, sub_size as u16, 1.0);
            draw_text(
                sub,
                x + (width - sub_dims.width) / 2.0,
                y + height * 0.65,
                sub_size,
                WHITE,
            );
        }
    }

    /// Draw control hints
    pub fn draw_controls(&self, game_state: &GameState) {
        let screen_w = screen_width();
        let controls_y = screen_height() * 0.02;
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

        let dims = measure_text(controls, None, font_size as u16, 1.0);
        draw_text(
            controls,
            (screen_w - dims.width) / 2.0,
            controls_y + font_size,
            font_size,
            Color::new(0.7, 0.7, 0.7, 0.8),
        );
    }

    /// Draw message display area
    pub fn draw_message_display(&self, game_state: &GameState) {
        if game_state.message.is_empty() {
            return;
        }

        let screen_w = screen_width();
        let screen_h = screen_height();

        let msg_y = screen_h * 0.78;
        let font_size = screen_w * 0.022;

        let dims = measure_text(&game_state.message, None, font_size as u16, 1.0);

        // Background bar
        draw_rectangle(
            0.0,
            msg_y - font_size * 0.8,
            screen_w,
            font_size * 1.6,
            Color::new(0.0, 0.0, 0.0, 0.7),
        );

        // Message text
        let color = if game_state.message_excited {
            let pulse = (self.time * 5.0).sin() * 0.5 + 0.5;
            Color::new(1.0, pulse, 0.0, 1.0)
        } else {
            WHITE
        };

        draw_text(
            &game_state.message,
            (screen_w - dims.width) / 2.0,
            msg_y,
            font_size,
            color,
        );
    }

    /// Draw CRT effect overlay (authentic 1980s television look)
    ///
    /// This adds subtle effects that simulate CRT display characteristics:
    /// - Horizontal scanlines (phosphor rows)
    /// - Slight vignette (screen curvature darkening)
    /// - Warm color cast at edges (phosphor decay)
    pub fn draw_crt_overlay(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // ─── SCANLINES (subtle horizontal lines) ───
        // Real CRTs had visible phosphor rows, especially on close inspection
        let scanline_spacing = 3.0;  // Every 3 pixels
        let mut y = 0.0;
        while y < screen_h {
            draw_line(0.0, y, screen_w, y, 1.0, CRT_SCANLINE);
            y += scanline_spacing;
        }

        // ─── VIGNETTE (darker edges from screen curvature) ───
        // CRT screens curved slightly, causing light falloff at edges
        let vignette_strength = 0.15;

        // Top edge darkening
        for i in 0..30 {
            let alpha = vignette_strength * (1.0 - i as f32 / 30.0);
            draw_rectangle(0.0, i as f32, screen_w, 1.0, Color::new(0.0, 0.0, 0.0, alpha));
        }

        // Bottom edge darkening
        for i in 0..30 {
            let alpha = vignette_strength * (1.0 - i as f32 / 30.0);
            draw_rectangle(0.0, screen_h - i as f32 - 1.0, screen_w, 1.0, Color::new(0.0, 0.0, 0.0, alpha));
        }

        // Left edge darkening
        for i in 0..20 {
            let alpha = vignette_strength * 0.7 * (1.0 - i as f32 / 20.0);
            draw_rectangle(i as f32, 0.0, 1.0, screen_h, Color::new(0.0, 0.0, 0.0, alpha));
        }

        // Right edge darkening
        for i in 0..20 {
            let alpha = vignette_strength * 0.7 * (1.0 - i as f32 / 20.0);
            draw_rectangle(screen_w - i as f32 - 1.0, 0.0, 1.0, screen_h, Color::new(0.0, 0.0, 0.0, alpha));
        }

        // ─── PHOSPHOR GLOW (subtle warm bloom) ───
        // CRT phosphors had a subtle warm glow, especially in bright areas
        // This is simulated with a very subtle warm overlay in the center
        let center_x = screen_w / 2.0;
        let center_y = screen_h / 2.0;
        let glow_radius = screen_w.min(screen_h) * 0.7;

        // Very subtle central warm glow
        for i in 0..5 {
            let radius = glow_radius * (1.0 - i as f32 * 0.15);
            let alpha = 0.008 * (i as f32 + 1.0);
            draw_circle(center_x, center_y, radius, Color::new(1.0, 0.95, 0.85, alpha));
        }

        // ─── SUBTLE COLOR FRINGING (chromatic aberration at edges) ───
        // Real CRTs had slight color separation at screen edges
        // This is a very subtle effect - just slight red/blue shift at corners
        let fringe_alpha = 0.02;
        let fringe_size = 40.0;

        // Top-left: slight red
        draw_rectangle(0.0, 0.0, fringe_size, fringe_size * 0.5, Color::new(1.0, 0.0, 0.0, fringe_alpha));
        // Top-right: slight blue
        draw_rectangle(screen_w - fringe_size, 0.0, fringe_size, fringe_size * 0.5, Color::new(0.0, 0.0, 1.0, fringe_alpha));
        // Bottom-left: slight blue
        draw_rectangle(0.0, screen_h - fringe_size * 0.5, fringe_size, fringe_size * 0.5, Color::new(0.0, 0.0, 1.0, fringe_alpha));
        // Bottom-right: slight red
        draw_rectangle(screen_w - fringe_size, screen_h - fringe_size * 0.5, fringe_size, fringe_size * 0.5, Color::new(1.0, 0.0, 0.0, fringe_alpha));
    }
}

impl Default for GraphicsRenderer {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Convert HSV to RGB color
///
/// # Arguments
/// * `h` - Hue (0.0-1.0)
/// * `s` - Saturation (0.0-1.0)
/// * `v` - Value (0.0-1.0)
///
/// # Returns
/// RGB Color
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
///
/// # Arguments
/// * `x`, `y` - Rectangle position
/// * `width`, `height` - Rectangle dimensions
/// * `distance` - Distance along perimeter from top-left corner
///
/// # Returns
/// (x, y) coordinates of the point
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
        format!("{},{:03},{:03}", value / 1_000_000, (value / 1000) % 1000, value % 1000)
    } else if value >= 1000 {
        format!("{},{:03}", value / 1000, value % 1000)
    } else {
        value.to_string()
    }
}
