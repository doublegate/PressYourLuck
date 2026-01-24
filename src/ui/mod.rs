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

use ggez::mint::Vector2;
use ggez::{
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text, TextFragment},
    Context,
};

use crate::game::{GamePhase, GameState};

// ===============================================================================
// HELPER FUNCTIONS
// ===============================================================================

/// Helper to get text dimensions with a fallback
fn text_dims(text: &Text, ctx: &Context) -> Vector2<f32> {
    text.measure(ctx).unwrap_or(Vector2 { x: 0.0, y: 0.0 })
}

// ===============================================================================
// CONSTANTS
// ===============================================================================

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

/// White color
const WHITE: Color = Color::WHITE;

/// Black color
const BLACK: Color = Color::BLACK;

// ===============================================================================
// UI MANAGER
// ===============================================================================

/// Manager for all UI overlay rendering
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
    pub fn draw(
        &mut self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        game_state: &GameState,
        screen_w: f32,
        screen_h: f32,
    ) {
        self.time += ctx.time.delta().as_secs_f32();

        match game_state.phase {
            GamePhase::Start => self.draw_start_screen(canvas, ctx, screen_w, screen_h),
            GamePhase::Questions => {
                self.draw_question_ui(canvas, ctx, game_state, screen_w, screen_h)
            }
            GamePhase::Board => self.draw_board_ui(canvas, ctx, game_state, screen_w, screen_h),
            GamePhase::GameOver => self.draw_game_over(canvas, ctx, game_state, screen_w, screen_h),
        }
    }

    /// Draw the start screen overlay
    fn draw_start_screen(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        screen_w: f32,
        screen_h: f32,
    ) {
        if let Ok(overlay) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, screen_w, screen_h),
            Color::new(0.0, 0.0, 0.0, 0.7),
        ) {
            canvas.draw(&overlay, DrawParam::default());
        }

        let panel_w = screen_w * 0.6;
        let panel_h = screen_h * 0.5;
        let panel_x = (screen_w - panel_w) / 2.0;
        let panel_y = (screen_h - panel_h) / 2.0;

        if let Ok(panel_bg) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(panel_x, panel_y, panel_w, panel_h),
            PANEL_BG,
        ) {
            canvas.draw(&panel_bg, DrawParam::default());
        }

        if let Ok(panel_border) = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(3.0),
            Rect::new(panel_x, panel_y, panel_w, panel_h),
            PANEL_BORDER,
        ) {
            canvas.draw(&panel_border, DrawParam::default());
        }

        // Title - animated rainbow
        let title = "PRESS YOUR LUCK";
        let title_size = panel_w * 0.08;
        let chars: Vec<char> = title.chars().collect();

        let mut total_width = 0.0;
        for c in &chars {
            let char_text = Text::new(TextFragment::new(c.to_string()).scale(title_size));
            total_width += text_dims(&char_text, ctx).x;
        }

        let mut current_x = panel_x + (panel_w - total_width) / 2.0;
        let title_y = panel_y + panel_h * 0.15;

        for (i, c) in chars.iter().enumerate() {
            let char_str = c.to_string();
            let char_text = Text::new(TextFragment::new(char_str).scale(title_size));
            let char_w = text_dims(&char_text, ctx).x;

            let hue = (self.time * 0.5 + i as f32 * 0.1) % 1.0;
            let color = hsv_to_rgb(hue, 0.8, 1.0);

            canvas.draw(
                &char_text,
                DrawParam::default().dest([current_x, title_y]).color(color),
            );
            current_x += char_w;
        }

        // Subtitle
        let subtitle = "~ The Game of Big Bucks and No Whammies ~";
        let sub_size = panel_w * 0.035;
        let sub_text = Text::new(TextFragment::new(subtitle).scale(sub_size));
        let sub_dims = text_dims(&sub_text, ctx);
        canvas.draw(
            &sub_text,
            DrawParam::default()
                .dest([
                    panel_x + (panel_w - sub_dims.x) / 2.0,
                    panel_y + panel_h * 0.28,
                ])
                .color(PANEL_BORDER),
        );

        // Instructions
        let instructions = [
            "Answer trivia questions to earn SPINS",
            "Use spins on the BIG BOARD to win cash and prizes",
            "Watch out for the WHAMMY - 4 and you're OUT!",
            "",
            "Press SPACE or ENTER to start",
        ];

        let inst_size = panel_w * 0.03;
        let inst_y_start = panel_y + panel_h * 0.42;
        let inst_spacing = panel_h * 0.08;

        for (i, line) in instructions.iter().enumerate() {
            if line.is_empty() {
                continue;
            }

            let color = if i == instructions.len() - 1 {
                let pulse = (self.time * 3.0).sin() * 0.3 + 0.7;
                Color::new(HIGHLIGHT.r, HIGHLIGHT.g, HIGHLIGHT.b, pulse)
            } else {
                WHITE
            };

            let inst_text = Text::new(TextFragment::new(*line).scale(inst_size));
            let dims = text_dims(&inst_text, ctx);
            canvas.draw(
                &inst_text,
                DrawParam::default()
                    .dest([
                        panel_x + (panel_w - dims.x) / 2.0,
                        inst_y_start + i as f32 * inst_spacing,
                    ])
                    .color(color),
            );
        }

        // Version info
        let version = "v1.0.0 - Authentic 1983-1986 CBS Recreation";
        let ver_size = panel_w * 0.022;
        let ver_text = Text::new(TextFragment::new(version).scale(ver_size));
        let ver_dims = text_dims(&ver_text, ctx);
        canvas.draw(
            &ver_text,
            DrawParam::default()
                .dest([
                    panel_x + (panel_w - ver_dims.x) / 2.0,
                    panel_y + panel_h * 0.92,
                ])
                .color(Color::new(0.5, 0.5, 0.5, 0.7)),
        );
    }

    /// Draw question round UI
    fn draw_question_ui(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        game_state: &GameState,
        screen_w: f32,
        screen_h: f32,
    ) {
        let qs = &game_state.question_state;

        let panel_w = screen_w * 0.7;
        let panel_h = screen_h * 0.45;
        let panel_x = (screen_w - panel_w) / 2.0;
        let panel_y = screen_h * 0.22;

        if let Ok(panel_bg) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(panel_x, panel_y, panel_w, panel_h),
            PANEL_BG,
        ) {
            canvas.draw(&panel_bg, DrawParam::default());
        }

        if let Ok(panel_border) = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(3.0),
            Rect::new(panel_x, panel_y, panel_w, panel_h),
            PANEL_BORDER,
        ) {
            canvas.draw(&panel_border, DrawParam::default());
        }

        let q_num = format!("Question {} of 4", qs.questions_this_round);
        let q_num_size = panel_w * 0.035;
        let q_num_text = Text::new(TextFragment::new(&q_num).scale(q_num_size));
        let q_num_dims = text_dims(&q_num_text, ctx);
        canvas.draw(
            &q_num_text,
            DrawParam::default()
                .dest([
                    panel_x + (panel_w - q_num_dims.x) / 2.0,
                    panel_y + panel_h * 0.05,
                ])
                .color(HIGHLIGHT),
        );

        if let Some(question) = &qs.current_question {
            let q_text_size = panel_w * 0.035;
            let wrapped = wrap_text(ctx, &question.question, panel_w * 0.9, q_text_size);

            let line_height = q_text_size * 1.3;
            let total_height = wrapped.len() as f32 * line_height;
            let available_space = panel_h * 0.33;
            let start_y =
                panel_y + panel_h * 0.12 + (available_space - total_height) / 2.0 + line_height;

            for (i, line) in wrapped.iter().enumerate() {
                let line_text = Text::new(TextFragment::new(line.as_str()).scale(q_text_size));
                let dims = text_dims(&line_text, ctx);
                canvas.draw(
                    &line_text,
                    DrawParam::default()
                        .dest([
                            panel_x + (panel_w - dims.x) / 2.0,
                            start_y + i as f32 * line_height,
                        ])
                        .color(WHITE),
                );
            }

            if qs.showing_choices {
                self.draw_answer_choices(
                    canvas,
                    ctx,
                    panel_x,
                    panel_y + panel_h * 0.45,
                    panel_w,
                    panel_h * 0.5,
                    &qs.choices,
                    None,
                    if qs.answer_revealed {
                        Some(qs.correct_index)
                    } else {
                        None
                    },
                );
            }
        }

        if qs.waiting_for_buzz {
            self.draw_buzz_timer(
                canvas,
                ctx,
                panel_x + panel_w * 0.1,
                panel_y + panel_h * 0.5,
                panel_w * 0.8,
                panel_h * 0.15,
                qs.buzz_timer,
                5.0,
            );

            let hint = "Press [B] to BUZZ IN for 3 spins!";
            let hint_size = panel_w * 0.035;

            let pulse = (self.time * 4.0).sin() * 0.3 + 0.7;
            let hint_text = Text::new(TextFragment::new(hint).scale(hint_size));
            let hint_dims = text_dims(&hint_text, ctx);
            canvas.draw(
                &hint_text,
                DrawParam::default()
                    .dest([
                        panel_x + (panel_w - hint_dims.x) / 2.0,
                        panel_y + panel_h * 0.72,
                    ])
                    .color(Color::new(1.0, 0.84, 0.0, pulse)),
            );
        }

        self.draw_spins_summary(canvas, ctx, game_state, screen_w, panel_y + panel_h + 10.0);
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_answer_choices(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
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

            let (bg_color, text_color, border_color) = if let Some(correct_idx) = correct {
                if i == correct_idx {
                    (CORRECT_COLOR, WHITE, WHITE)
                } else if Some(i) == selected {
                    (WRONG_COLOR, WHITE, WHITE)
                } else {
                    (
                        Color::new(0.1, 0.1, 0.15, 0.8),
                        Color::new(0.5, 0.5, 0.5, 1.0),
                        Color::new(0.3, 0.3, 0.3, 1.0),
                    )
                }
            } else if Some(i) == selected {
                (Color::new(0.0, 0.4, 0.6, 0.9), WHITE, HIGHLIGHT)
            } else {
                (
                    Color::new(0.1, 0.1, 0.15, 0.8),
                    WHITE,
                    Color::new(0.5, 0.5, 0.5, 1.0),
                )
            };

            if let Ok(choice_bg) = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(choice_x, choice_y, choice_w, choice_h),
                bg_color,
            ) {
                canvas.draw(&choice_bg, DrawParam::default());
            }

            if let Ok(choice_border) = Mesh::new_rectangle(
                ctx,
                DrawMode::stroke(2.0),
                Rect::new(choice_x, choice_y, choice_w, choice_h),
                border_color,
            ) {
                canvas.draw(&choice_border, DrawParam::default());
            }

            let num_text_str = format!("{}.", i + 1);
            let num_size = choice_h * 0.4;
            let num_text = Text::new(TextFragment::new(&num_text_str).scale(num_size));
            canvas.draw(
                &num_text,
                DrawParam::default()
                    .dest([choice_x + choice_w * 0.03, choice_y + choice_h * 0.3])
                    .color(text_color),
            );

            let text_size = choice_h * 0.35;
            let choice_text = Text::new(TextFragment::new(choice.as_str()).scale(text_size));
            let choice_text_dims = text_dims(&choice_text, ctx);

            let available_text_width = choice_w * 0.85;
            let actual_scale = if choice_text_dims.x > available_text_width {
                text_size * (available_text_width / choice_text_dims.x)
            } else {
                text_size
            };

            let actual_text = Text::new(TextFragment::new(choice.as_str()).scale(actual_scale));
            canvas.draw(
                &actual_text,
                DrawParam::default()
                    .dest([choice_x + choice_w * 0.1, choice_y + choice_h * 0.35])
                    .color(text_color),
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_buzz_timer(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        remaining: f32,
        total: f32,
    ) {
        let progress = (remaining / total).clamp(0.0, 1.0);

        if let Ok(bg) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(x, y, width, height),
            Color::new(0.2, 0.2, 0.2, 0.8),
        ) {
            canvas.draw(&bg, DrawParam::default());
        }

        let bar_color = if progress > 0.5 {
            CORRECT_COLOR
        } else if progress > 0.25 {
            PANEL_BORDER
        } else {
            WRONG_COLOR
        };

        let bar_width = if progress < 0.25 {
            let pulse = (self.time * 8.0).sin() * 0.05 + 0.95;
            width * progress * pulse
        } else {
            width * progress
        };

        if bar_width > 0.0 {
            if let Ok(bar) = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(x, y, bar_width, height),
                bar_color,
            ) {
                canvas.draw(&bar, DrawParam::default());
            }
        }

        if let Ok(border) = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(2.0),
            Rect::new(x, y, width, height),
            WHITE,
        ) {
            canvas.draw(&border, DrawParam::default());
        }

        let time_str = format!("{:.1}s", remaining);
        let time_size = height * 0.5;
        let time_text = Text::new(TextFragment::new(&time_str).scale(time_size));
        let time_dims = text_dims(&time_text, ctx);
        canvas.draw(
            &time_text,
            DrawParam::default()
                .dest([x + (width - time_dims.x) / 2.0, y + height * 0.25])
                .color(WHITE),
        );
    }

    fn draw_spins_summary(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        game_state: &GameState,
        screen_w: f32,
        y: f32,
    ) {
        let font_size = screen_w * 0.016;

        let mut positions: Vec<(usize, &crate::game::Contestant)> = game_state
            .contestants
            .iter()
            .enumerate()
            .filter(|(_, c)| !c.eliminated)
            .collect();
        positions.sort_by(|a, b| b.1.score.cmp(&a.1.score));

        let mut summary_parts = Vec::new();
        for (i, contestant) in game_state.contestants.iter().enumerate() {
            if !contestant.eliminated {
                let position = positions
                    .iter()
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
        let summary_text = Text::new(TextFragment::new(&summary).scale(font_size));
        let dims = text_dims(&summary_text, ctx);

        canvas.draw(
            &summary_text,
            DrawParam::default()
                .dest([(screen_w - dims.x) / 2.0, y])
                .color(HIGHLIGHT),
        );
    }

    fn draw_board_ui(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        game_state: &GameState,
        screen_w: f32,
        screen_h: f32,
    ) {
        if game_state.awaiting_corner_selection {
            self.draw_corner_selection(canvas, ctx, screen_w, screen_h);
        }

        if game_state.awaiting_special_choice {
            self.draw_special_choice(canvas, ctx, screen_w, screen_h);
        }
    }

    fn draw_corner_selection(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        screen_w: f32,
        screen_h: f32,
    ) {
        if let Ok(overlay) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, screen_w, screen_h),
            Color::new(0.0, 0.0, 0.0, 0.5),
        ) {
            canvas.draw(&overlay, DrawParam::default());
        }

        let text = "PICK A CORNER!";
        let text_size = screen_w * 0.04;

        let pulse = (self.time * 4.0).sin() * 0.3 + 0.7;
        let title_text = Text::new(TextFragment::new(text).scale(text_size));
        let dims = text_dims(&title_text, ctx);

        canvas.draw(
            &title_text,
            DrawParam::default()
                .dest([(screen_w - dims.x) / 2.0, screen_h * 0.1])
                .color(Color::new(1.0, 0.84, 0.0, pulse)),
        );

        let labels = [
            ("1", screen_w * 0.2, screen_h * 0.25),
            ("2", screen_w * 0.8, screen_h * 0.25),
            ("3", screen_w * 0.8, screen_h * 0.75),
            ("4", screen_w * 0.2, screen_h * 0.75),
        ];

        let label_size = screen_w * 0.035;
        for (label, cx, cy) in labels {
            if let Ok(circle) = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                [cx, cy],
                label_size,
                0.1,
                PANEL_BORDER,
            ) {
                canvas.draw(&circle, DrawParam::default());
            }

            let num_text = Text::new(TextFragment::new(label).scale(label_size * 0.8));
            let num_dims = text_dims(&num_text, ctx);
            canvas.draw(
                &num_text,
                DrawParam::default()
                    .dest([cx - num_dims.x / 2.0, cy - num_dims.y / 2.0])
                    .color(BLACK),
            );
        }
    }

    fn draw_special_choice(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        screen_w: f32,
        screen_h: f32,
    ) {
        if let Ok(overlay) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, screen_w, screen_h),
            Color::new(0.0, 0.0, 0.0, 0.6),
        ) {
            canvas.draw(&overlay, DrawParam::default());
        }

        let panel_w = screen_w * 0.5;
        let panel_h = screen_h * 0.3;
        let panel_x = (screen_w - panel_w) / 2.0;
        let panel_y = (screen_h - panel_h) / 2.0;

        if let Ok(panel_bg) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(panel_x, panel_y, panel_w, panel_h),
            PANEL_BG,
        ) {
            canvas.draw(&panel_bg, DrawParam::default());
        }

        if let Ok(panel_border) = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(3.0),
            Rect::new(panel_x, panel_y, panel_w, panel_h),
            PANEL_BORDER,
        ) {
            canvas.draw(&panel_border, DrawParam::default());
        }

        let title = "CHOOSE YOUR PRIZE!";
        let title_size = panel_w * 0.07;
        let title_text = Text::new(TextFragment::new(title).scale(title_size));
        let title_dims = text_dims(&title_text, ctx);
        canvas.draw(
            &title_text,
            DrawParam::default()
                .dest([
                    panel_x + (panel_w - title_dims.x) / 2.0,
                    panel_y + panel_h * 0.15,
                ])
                .color(PANEL_BORDER),
        );

        let opt_size = panel_w * 0.05;

        let opt1 = "[1] Take $2,000";
        let opt1_text = Text::new(TextFragment::new(opt1).scale(opt_size));
        let opt1_dims = text_dims(&opt1_text, ctx);
        canvas.draw(
            &opt1_text,
            DrawParam::default()
                .dest([
                    panel_x + (panel_w - opt1_dims.x) / 2.0,
                    panel_y + panel_h * 0.45,
                ])
                .color(CORRECT_COLOR),
        );

        let opt2 = "[2] Lose One Whammy";
        let opt2_text = Text::new(TextFragment::new(opt2).scale(opt_size));
        let opt2_dims = text_dims(&opt2_text, ctx);
        canvas.draw(
            &opt2_text,
            DrawParam::default()
                .dest([
                    panel_x + (panel_w - opt2_dims.x) / 2.0,
                    panel_y + panel_h * 0.65,
                ])
                .color(HIGHLIGHT),
        );
    }

    fn draw_game_over(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        game_state: &GameState,
        screen_w: f32,
        screen_h: f32,
    ) {
        if let Ok(overlay) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, screen_w, screen_h),
            Color::new(0.0, 0.0, 0.0, 0.7),
        ) {
            canvas.draw(&overlay, DrawParam::default());
        }

        let winner = game_state
            .contestants
            .iter()
            .filter(|c| !c.eliminated)
            .max_by_key(|c| c.score);

        let panel_w = screen_w * 0.6;
        let panel_h = screen_h * 0.5;
        let panel_x = (screen_w - panel_w) / 2.0;
        let panel_y = (screen_h - panel_h) / 2.0;

        if let Ok(panel_bg) = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(panel_x, panel_y, panel_w, panel_h),
            PANEL_BG,
        ) {
            canvas.draw(&panel_bg, DrawParam::default());
        }

        if let Ok(panel_border) = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(4.0),
            Rect::new(panel_x, panel_y, panel_w, panel_h),
            PANEL_BORDER,
        ) {
            canvas.draw(&panel_border, DrawParam::default());
        }

        if let Some(winner) = winner {
            let title = "WINNER!";
            let title_size = panel_w * 0.09;

            let pulse = (self.time * 3.0).sin() * 0.2 + 0.8;
            let title_text = Text::new(TextFragment::new(title).scale(title_size * pulse));
            let title_dims = text_dims(&title_text, ctx);

            canvas.draw(
                &title_text,
                DrawParam::default()
                    .dest([
                        panel_x + (panel_w - title_dims.x) / 2.0,
                        panel_y + panel_h * 0.15,
                    ])
                    .color(PANEL_BORDER),
            );

            let name_size = panel_w * 0.07;
            let name_text = Text::new(TextFragment::new(&winner.name).scale(name_size));
            let name_dims = text_dims(&name_text, ctx);
            canvas.draw(
                &name_text,
                DrawParam::default()
                    .dest([
                        panel_x + (panel_w - name_dims.x) / 2.0,
                        panel_y + panel_h * 0.35,
                    ])
                    .color(WHITE),
            );

            let amount = format!("${}", format_money(winner.score));
            let amount_size = panel_w * 0.1;
            let chars: Vec<char> = amount.chars().collect();

            let mut total_width = 0.0;
            for c in &chars {
                let char_text = Text::new(TextFragment::new(c.to_string()).scale(amount_size));
                total_width += text_dims(&char_text, ctx).x;
            }

            let mut current_x = panel_x + (panel_w - total_width) / 2.0;
            let amount_y = panel_y + panel_h * 0.5;

            for (i, c) in chars.iter().enumerate() {
                let char_str = c.to_string();
                let char_text = Text::new(TextFragment::new(char_str).scale(amount_size));
                let char_w = text_dims(&char_text, ctx).x;

                let hue = (self.time * 0.8 + i as f32 * 0.15) % 1.0;
                let color = hsv_to_rgb(hue, 0.9, 1.0);

                canvas.draw(
                    &char_text,
                    DrawParam::default()
                        .dest([current_x, amount_y])
                        .color(color),
                );
                current_x += char_w;
            }
        } else {
            let title = "GAME OVER";
            let title_size = panel_w * 0.09;
            let title_text = Text::new(TextFragment::new(title).scale(title_size));
            let title_dims = text_dims(&title_text, ctx);
            canvas.draw(
                &title_text,
                DrawParam::default()
                    .dest([
                        panel_x + (panel_w - title_dims.x) / 2.0,
                        panel_y + panel_h * 0.25,
                    ])
                    .color(WRONG_COLOR),
            );

            let sub = "Everyone got Whammied Out!";
            let sub_size = panel_w * 0.045;
            let sub_text = Text::new(TextFragment::new(sub).scale(sub_size));
            let sub_dims = text_dims(&sub_text, ctx);
            canvas.draw(
                &sub_text,
                DrawParam::default()
                    .dest([
                        panel_x + (panel_w - sub_dims.x) / 2.0,
                        panel_y + panel_h * 0.45,
                    ])
                    .color(WHITE),
            );
        }

        let prompt = "Press SPACE or ENTER to play again";
        let prompt_size = panel_w * 0.035;

        let pulse = (self.time * 2.0).sin() * 0.3 + 0.7;
        let prompt_text = Text::new(TextFragment::new(prompt).scale(prompt_size));
        let prompt_dims = text_dims(&prompt_text, ctx);
        canvas.draw(
            &prompt_text,
            DrawParam::default()
                .dest([
                    panel_x + (panel_w - prompt_dims.x) / 2.0,
                    panel_y + panel_h * 0.82,
                ])
                .color(Color::new(HIGHLIGHT.r, HIGHLIGHT.g, HIGHLIGHT.b, pulse)),
        );
    }
}

impl Default for UiManager {
    fn default() -> Self {
        Self::new()
    }
}

// ===============================================================================
// MODULE HELPER FUNCTIONS
// ===============================================================================

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

fn wrap_text(ctx: &Context, text: &str, max_width: f32, font_size: f32) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in words {
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };

        let test_text = Text::new(TextFragment::new(&test_line).scale(font_size));
        let dims = text_dims(&test_text, ctx);

        if dims.x > max_width && !current_line.is_empty() {
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
