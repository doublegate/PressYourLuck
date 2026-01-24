//! # UI Manager Module
//!
//! ## Overview
//! This module handles all user interface overlays for Press Your Luck,
//! including question displays, answer choices, and game state indicators.
//!
//! ## UI Components
//! - **Start Screen**: Game title and instructions
//! - **Question Display**: Trivia questions with answer choices
//! - **Buzz Timer**: Visual countdown for buzz-in period
//! - **Answer Reveal**: Correct/incorrect feedback
//! - **Game Over Screen**: Winner announcement
//!
//! ## Design Philosophy
//! The UI overlays on top of the main game graphics, using semi-transparent
//! panels to maintain visibility of the colorful board underneath.

use macroquad::prelude::*;
use crate::game::{GameState, GamePhase};

// ═══════════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Panel background color (semi-transparent dark)
const PANEL_BG: Color = Color::new(0.05, 0.02, 0.1, 0.9);

/// Panel border color (gold)
const PANEL_BORDER: Color = Color::new(1.0, 0.84, 0.0, 1.0);

/// Highlight color for selected items
const HIGHLIGHT: Color = Color::new(0.0, 0.75, 1.0, 1.0);

/// Correct answer color
const CORRECT_COLOR: Color = Color::new(0.0, 0.8, 0.2, 1.0);

/// Wrong answer color
const WRONG_COLOR: Color = Color::new(0.9, 0.1, 0.1, 1.0);

// ═══════════════════════════════════════════════════════════════════════════════
// UI MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// Manager for all UI overlay rendering
///
/// # Responsibilities
/// - Render phase-specific UI elements
/// - Display trivia questions and answers
/// - Show buzz-in timer
/// - Present game over screen
pub struct UiManager {
    /// Animation time tracker
    time: f32,
}

impl UiManager {
    /// Create a new UI manager
    pub fn new() -> Self {
        Self { time: 0.0 }
    }

    /// Draw all UI elements for current game state
    ///
    /// # Arguments
    /// * `game_state` - Current game state
    pub fn draw(&mut self, game_state: &GameState) {
        self.time += get_frame_time();

        match game_state.phase {
            GamePhase::Start => self.draw_start_screen(),
            GamePhase::Questions => self.draw_question_ui(game_state),
            GamePhase::Board => self.draw_board_ui(game_state),
            GamePhase::GameOver => self.draw_game_over(game_state),
        }
    }

    /// Draw the start screen overlay
    fn draw_start_screen(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Semi-transparent overlay
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.7));

        // Main panel
        let panel_w = screen_w * 0.6;
        let panel_h = screen_h * 0.5;
        let panel_x = (screen_w - panel_w) / 2.0;
        let panel_y = (screen_h - panel_h) / 2.0;

        draw_rectangle(panel_x, panel_y, panel_w, panel_h, PANEL_BG);
        draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, 3.0, PANEL_BORDER);

        // Title
        let title = "PRESS YOUR LUCK";
        let title_size = panel_w * 0.12;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        
        // Animated rainbow title
        let chars: Vec<char> = title.chars().collect();
        let mut current_x = panel_x + (panel_w - title_dims.width) / 2.0;
        let title_y = panel_y + panel_h * 0.2;

        for (i, c) in chars.iter().enumerate() {
            let char_str = c.to_string();
            let char_dims = measure_text(&char_str, None, title_size as u16, 1.0);
            
            let hue = (self.time * 0.5 + i as f32 * 0.1) % 1.0;
            let color = hsv_to_rgb(hue, 0.8, 1.0);
            
            draw_text(&char_str, current_x, title_y, title_size, color);
            current_x += char_dims.width;
        }

        // Subtitle
        let subtitle = "~ The Game of Big Bucks and No Whammies ~";
        let sub_size = panel_w * 0.04;
        let sub_dims = measure_text(subtitle, None, sub_size as u16, 1.0);
        draw_text(
            subtitle,
            panel_x + (panel_w - sub_dims.width) / 2.0,
            panel_y + panel_h * 0.32,
            sub_size,
            PANEL_BORDER,
        );

        // Instructions
        let instructions = [
            "Answer trivia questions to earn SPINS",
            "Use spins on the BIG BOARD to win cash and prizes",
            "Watch out for the WHAMMY - 4 and you're OUT!",
            "",
            "Press SPACE or ENTER to start",
        ];

        let inst_size = panel_w * 0.035;
        let inst_y_start = panel_y + panel_h * 0.45;
        let inst_spacing = panel_h * 0.08;

        for (i, line) in instructions.iter().enumerate() {
            let dims = measure_text(line, None, inst_size as u16, 1.0);
            let color = if line.is_empty() {
                Color::new(0.0, 0.0, 0.0, 0.0)
            } else if i == instructions.len() - 1 {
                // Pulsing for "Press SPACE"
                let pulse = (self.time * 3.0).sin() * 0.3 + 0.7;
                Color::new(HIGHLIGHT.r, HIGHLIGHT.g, HIGHLIGHT.b, pulse)
            } else {
                WHITE
            };

            draw_text(
                line,
                panel_x + (panel_w - dims.width) / 2.0,
                inst_y_start + i as f32 * inst_spacing,
                inst_size,
                color,
            );
        }

        // Version info
        let version = "v1.0.0 - Authentic 1983-1986 CBS Recreation";
        let ver_size = panel_w * 0.025;
        let ver_dims = measure_text(version, None, ver_size as u16, 1.0);
        draw_text(
            version,
            panel_x + (panel_w - ver_dims.width) / 2.0,
            panel_y + panel_h * 0.95,
            ver_size,
            Color::new(0.5, 0.5, 0.5, 0.7),
        );
    }

    /// Draw question round UI
    fn draw_question_ui(&self, game_state: &GameState) {
        let screen_w = screen_width();
        let screen_h = screen_height();
        let qs = &game_state.question_state;

        // Question panel
        let panel_w = screen_w * 0.7;
        let panel_h = screen_h * 0.45;
        let panel_x = (screen_w - panel_w) / 2.0;
        let panel_y = screen_h * 0.22;

        draw_rectangle(panel_x, panel_y, panel_w, panel_h, PANEL_BG);
        draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, 3.0, PANEL_BORDER);

        // Question number
        let q_num = format!("Question {} of 4", qs.questions_this_round);
        let q_num_size = panel_w * 0.04;
        let q_num_dims = measure_text(&q_num, None, q_num_size as u16, 1.0);
        draw_text(
            &q_num,
            panel_x + (panel_w - q_num_dims.width) / 2.0,
            panel_y + panel_h * 0.08,
            q_num_size,
            HIGHLIGHT,
        );

        // Question text
        if let Some(question) = &qs.current_question {
            let q_text_size = panel_w * 0.04;
            let wrapped = wrap_text(&question.question, panel_w * 0.9, q_text_size);

            let line_height = q_text_size * 1.3;
            let total_height = wrapped.len() as f32 * line_height;
            // Use total_height to vertically center the question text in the available space
            // Available space is from panel_y + 0.12 to panel_y + 0.45 (before choices area)
            let available_space = panel_h * 0.33;  // 0.45 - 0.12 = 0.33 of panel height
            let start_y = panel_y + panel_h * 0.12 + (available_space - total_height) / 2.0 + line_height;

            for (i, line) in wrapped.iter().enumerate() {
                let dims = measure_text(line, None, q_text_size as u16, 1.0);
                draw_text(
                    line,
                    panel_x + (panel_w - dims.width) / 2.0,
                    start_y + i as f32 * line_height,
                    q_text_size,
                    WHITE,
                );
            }

            // Show choices if applicable
            if qs.showing_choices {
                self.draw_answer_choices(
                    panel_x,
                    panel_y + panel_h * 0.45,
                    panel_w,
                    panel_h * 0.5,
                    &qs.choices,
                    None, // No selection tracking in current design
                    if qs.answer_revealed { Some(qs.correct_index) } else { None },
                );
            }
        }

        // Buzz timer (if waiting for buzz)
        if qs.waiting_for_buzz {
            self.draw_buzz_timer(
                panel_x + panel_w * 0.1,
                panel_y + panel_h * 0.5,
                panel_w * 0.8,
                panel_h * 0.15,
                qs.buzz_timer,
                5.0,
            );

            // Buzz hint
            let hint = "Press [B] to BUZZ IN for 3 spins!";
            let hint_size = panel_w * 0.04;
            let hint_dims = measure_text(hint, None, hint_size as u16, 1.0);
            
            let pulse = (self.time * 4.0).sin() * 0.3 + 0.7;
            draw_text(
                hint,
                panel_x + (panel_w - hint_dims.width) / 2.0,
                panel_y + panel_h * 0.75,
                hint_size,
                Color::new(1.0, 0.84, 0.0, pulse),
            );
        }

        // Spins indicator (show who has what)
        self.draw_spins_summary(game_state, panel_y + panel_h + 10.0);
    }

    /// Draw answer choice buttons
    fn draw_answer_choices(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        choices: &[String],
        selected: Option<usize>,
        correct: Option<usize>,
    ) {
        let choice_h = height / 4.5;
        let choice_w = width * 0.85;
        let choice_x = x + (width - choice_w) / 2.0;
        let spacing = (height - choice_h * 4.0) / 5.0;

        for (i, choice) in choices.iter().enumerate() {
            let choice_y = y + spacing + i as f32 * (choice_h + spacing);

            // Determine color based on state
            let (bg_color, text_color, border_color) = if let Some(correct_idx) = correct {
                if i == correct_idx {
                    (CORRECT_COLOR, WHITE, WHITE)
                } else if Some(i) == selected {
                    (WRONG_COLOR, WHITE, WHITE)
                } else {
                    (Color::new(0.1, 0.1, 0.15, 0.8), Color::new(0.5, 0.5, 0.5, 1.0), Color::new(0.3, 0.3, 0.3, 1.0))
                }
            } else if Some(i) == selected {
                (Color::new(0.0, 0.4, 0.6, 0.9), WHITE, HIGHLIGHT)
            } else {
                (Color::new(0.1, 0.1, 0.15, 0.8), WHITE, Color::new(0.5, 0.5, 0.5, 1.0))
            };

            // Draw choice box
            draw_rectangle(choice_x, choice_y, choice_w, choice_h, bg_color);
            draw_rectangle_lines(choice_x, choice_y, choice_w, choice_h, 2.0, border_color);

            // Choice number
            let num_text = format!("{}.", i + 1);
            let num_size = choice_h * 0.5;
            draw_text(
                &num_text,
                choice_x + choice_w * 0.03,
                choice_y + choice_h * 0.65,
                num_size,
                text_color,
            );

            // Choice text - use text_dims to ensure proper positioning
            let text_size = choice_h * 0.4;
            let text_dims = measure_text(choice, None, text_size as u16, 1.0);
            // Calculate x position: if text fits, left-align after number; if too wide, scale font
            let available_text_width = choice_w * 0.85;  // Leave space for number and margins
            let text_x = choice_x + choice_w * 0.1;
            // Scale down text if it doesn't fit in available width
            let actual_text_size = if text_dims.width > available_text_width {
                text_size * (available_text_width / text_dims.width)
            } else {
                text_size
            };
            // Center text vertically within the button using text_dims.height
            let text_y = choice_y + (choice_h + text_dims.height) / 2.0 - text_dims.height * 0.15;
            draw_text(
                choice,
                text_x,
                text_y,
                actual_text_size,
                text_color,
            );
        }
    }

    /// Draw the buzz-in timer bar
    fn draw_buzz_timer(&self, x: f32, y: f32, width: f32, height: f32, remaining: f32, total: f32) {
        let progress = (remaining / total).clamp(0.0, 1.0);

        // Background
        draw_rectangle(x, y, width, height, Color::new(0.2, 0.2, 0.2, 0.8));

        // Progress bar with color gradient (green -> yellow -> red)
        let bar_color = if progress > 0.5 {
            CORRECT_COLOR
        } else if progress > 0.25 {
            PANEL_BORDER
        } else {
            WRONG_COLOR
        };

        // Pulsing effect when low
        let bar_width = if progress < 0.25 {
            let pulse = (self.time * 8.0).sin() * 0.05 + 0.95;
            width * progress * pulse
        } else {
            width * progress
        };

        draw_rectangle(x, y, bar_width, height, bar_color);

        // Border
        draw_rectangle_lines(x, y, width, height, 2.0, WHITE);

        // Time text
        let time_text = format!("{:.1}s", remaining);
        let time_size = height * 0.7;
        let time_dims = measure_text(&time_text, None, time_size as u16, 1.0);
        draw_text(
            &time_text,
            x + (width - time_dims.width) / 2.0,
            y + height * 0.72,
            time_size,
            WHITE,
        );
    }

    /// Draw spins summary for all players
    fn draw_spins_summary(&self, game_state: &GameState, y: f32) {
        let screen_w = screen_width();
        let font_size = screen_w * 0.018;

        // Position indicators for 1st, 2nd, 3rd place based on current scores
        let mut positions: Vec<(usize, &crate::game::Contestant)> = game_state.contestants.iter()
            .enumerate()
            .filter(|(_, c)| !c.eliminated)
            .collect();
        positions.sort_by(|a, b| b.1.score.cmp(&a.1.score));

        let mut summary_parts = Vec::new();
        for (i, contestant) in game_state.contestants.iter().enumerate() {
            if !contestant.eliminated {
                // Find position (1st, 2nd, 3rd) based on score ranking
                let position = positions.iter()
                    .position(|(idx, _)| *idx == i)
                    .map(|p| p + 1)
                    .unwrap_or(0);
                let position_indicator = match position {
                    1 => "(1st)",
                    2 => "(2nd)",
                    3 => "(3rd)",
                    _ => "",
                };
                summary_parts.push(format!(
                    "{} {}: {} spins",
                    contestant.name,
                    position_indicator,
                    contestant.earned_spins + contestant.passed_spins
                ));
            }
        }

        let summary = summary_parts.join("  |  ");
        let dims = measure_text(&summary, None, font_size as u16, 1.0);

        draw_text(
            &summary,
            (screen_w - dims.width) / 2.0,
            y,
            font_size,
            HIGHLIGHT,
        );
    }

    /// Draw board phase UI elements
    fn draw_board_ui(&self, game_state: &GameState) {
        // The board UI is mostly handled by graphics renderer
        // Here we add any overlay elements

        // Corner selection overlay
        if game_state.awaiting_corner_selection {
            self.draw_corner_selection();
        }

        // Special choice overlay
        if game_state.awaiting_special_choice {
            self.draw_special_choice();
        }
    }

    /// Draw corner selection overlay
    fn draw_corner_selection(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Overlay
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.5));

        // Instructions
        let text = "PICK A CORNER!";
        let text_size = screen_w * 0.05;
        let dims = measure_text(text, None, text_size as u16, 1.0);

        let pulse = (self.time * 4.0).sin() * 0.3 + 0.7;
        draw_text(
            text,
            (screen_w - dims.width) / 2.0,
            screen_h * 0.12,
            text_size,
            Color::new(1.0, 0.84, 0.0, pulse),
        );

        // Corner labels
        let labels = [
            ("1", screen_w * 0.2, screen_h * 0.25),
            ("2", screen_w * 0.8, screen_h * 0.25),
            ("3", screen_w * 0.8, screen_h * 0.75),
            ("4", screen_w * 0.2, screen_h * 0.75),
        ];

        let label_size = screen_w * 0.04;
        for (label, x, y) in labels {
            // Circle background
            draw_circle(x, y, label_size, PANEL_BORDER);

            // Number
            let dims = measure_text(label, None, label_size as u16, 1.0);
            draw_text(
                label,
                x - dims.width / 2.0,
                y + dims.height / 3.0,
                label_size,
                BLACK,
            );
        }
    }

    /// Draw special choice overlay ($2000 or Lose Whammy)
    fn draw_special_choice(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Overlay
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.6));

        // Panel
        let panel_w = screen_w * 0.5;
        let panel_h = screen_h * 0.3;
        let panel_x = (screen_w - panel_w) / 2.0;
        let panel_y = (screen_h - panel_h) / 2.0;

        draw_rectangle(panel_x, panel_y, panel_w, panel_h, PANEL_BG);
        draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, 3.0, PANEL_BORDER);

        // Title
        let title = "CHOOSE YOUR PRIZE!";
        let title_size = panel_w * 0.08;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            panel_x + (panel_w - title_dims.width) / 2.0,
            panel_y + panel_h * 0.2,
            title_size,
            PANEL_BORDER,
        );

        // Options
        let opt_size = panel_w * 0.06;

        let opt1 = "[1] Take $2,000";
        let opt1_dims = measure_text(opt1, None, opt_size as u16, 1.0);
        draw_text(
            opt1,
            panel_x + (panel_w - opt1_dims.width) / 2.0,
            panel_y + panel_h * 0.5,
            opt_size,
            CORRECT_COLOR,
        );

        let opt2 = "[2] Lose One Whammy";
        let opt2_dims = measure_text(opt2, None, opt_size as u16, 1.0);
        draw_text(
            opt2,
            panel_x + (panel_w - opt2_dims.width) / 2.0,
            panel_y + panel_h * 0.7,
            opt_size,
            HIGHLIGHT,
        );
    }

    /// Draw game over screen
    fn draw_game_over(&self, game_state: &GameState) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Overlay
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.7));

        // Find winner
        let winner = game_state.contestants.iter()
            .filter(|c| !c.eliminated)
            .max_by_key(|c| c.score);

        // Panel
        let panel_w = screen_w * 0.6;
        let panel_h = screen_h * 0.5;
        let panel_x = (screen_w - panel_w) / 2.0;
        let panel_y = (screen_h - panel_h) / 2.0;

        draw_rectangle(panel_x, panel_y, panel_w, panel_h, PANEL_BG);
        draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, 4.0, PANEL_BORDER);

        if let Some(winner) = winner {
            // Winner title with animation
            let title = "🎉 WINNER! 🎉";
            let title_size = panel_w * 0.1;
            let title_dims = measure_text(title, None, title_size as u16, 1.0);

            let pulse = (self.time * 3.0).sin() * 0.2 + 0.8;
            draw_text(
                title,
                panel_x + (panel_w - title_dims.width) / 2.0,
                panel_y + panel_h * 0.2,
                title_size * pulse,
                PANEL_BORDER,
            );

            // Winner name
            let name_size = panel_w * 0.08;
            let name_dims = measure_text(&winner.name, None, name_size as u16, 1.0);
            draw_text(
                &winner.name,
                panel_x + (panel_w - name_dims.width) / 2.0,
                panel_y + panel_h * 0.4,
                name_size,
                WHITE,
            );

            // Winning amount
            let amount = format!("${}", format_money(winner.score));
            let amount_size = panel_w * 0.12;
            let amount_dims = measure_text(&amount, None, amount_size as u16, 1.0);

            // Rainbow animated amount
            let chars: Vec<char> = amount.chars().collect();
            let mut current_x = panel_x + (panel_w - amount_dims.width) / 2.0;
            let amount_y = panel_y + panel_h * 0.55;

            for (i, c) in chars.iter().enumerate() {
                let char_str = c.to_string();
                let char_dims = measure_text(&char_str, None, amount_size as u16, 1.0);

                let hue = (self.time * 0.8 + i as f32 * 0.15) % 1.0;
                let color = hsv_to_rgb(hue, 0.9, 1.0);

                draw_text(&char_str, current_x, amount_y, amount_size, color);
                current_x += char_dims.width;
            }
        } else {
            // Everyone eliminated
            let title = "GAME OVER";
            let title_size = panel_w * 0.1;
            let title_dims = measure_text(title, None, title_size as u16, 1.0);
            draw_text(
                title,
                panel_x + (panel_w - title_dims.width) / 2.0,
                panel_y + panel_h * 0.3,
                title_size,
                WRONG_COLOR,
            );

            let sub = "Everyone got Whammied Out!";
            let sub_size = panel_w * 0.05;
            let sub_dims = measure_text(sub, None, sub_size as u16, 1.0);
            draw_text(
                sub,
                panel_x + (panel_w - sub_dims.width) / 2.0,
                panel_y + panel_h * 0.5,
                sub_size,
                WHITE,
            );
        }

        // Play again prompt
        let prompt = "Press SPACE or ENTER to play again";
        let prompt_size = panel_w * 0.04;
        let prompt_dims = measure_text(prompt, None, prompt_size as u16, 1.0);

        let pulse = (self.time * 2.0).sin() * 0.3 + 0.7;
        draw_text(
            prompt,
            panel_x + (panel_w - prompt_dims.width) / 2.0,
            panel_y + panel_h * 0.85,
            prompt_size,
            Color::new(HIGHLIGHT.r, HIGHLIGHT.g, HIGHLIGHT.b, pulse),
        );
    }
}

impl Default for UiManager {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

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

/// Wrap text to fit within a given width
fn wrap_text(text: &str, max_width: f32, font_size: f32) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in words {
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };

        let dims = measure_text(&test_line, None, font_size as u16, 1.0);

        if dims.width > max_width && !current_line.is_empty() {
            lines.push(current_line);
            current_line = word.to_string();
        } else {
            current_line = test_line;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
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
