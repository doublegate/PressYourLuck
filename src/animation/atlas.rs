//! # Sprite Atlas Module
//!
//! Handles loading and management of sprite sheets for animations.
//! Supports both grid-based atlases and packed atlases with JSON metadata.

use ggez::graphics::{Color, Image, ImageFormat, Rect};
use ggez::{Context, GameResult};
use std::collections::HashMap;
use std::path::Path;

// =============================================================================
// CONSTANTS - WHAMMY COLOR PALETTE
// =============================================================================

/// Primary Whammy red color
pub const WHAMMY_RED: Color = Color::new(0.831, 0.125, 0.125, 1.0); // #D42020

/// Shadow red for Whammy
pub const WHAMMY_SHADOW: Color = Color::new(0.545, 0.082, 0.082, 1.0); // #8B1515

/// Highlight red for Whammy
pub const WHAMMY_HIGHLIGHT: Color = Color::new(1.0, 0.251, 0.251, 1.0); // #FF4040

/// Outline color for Whammy (near-black)
pub const WHAMMY_OUTLINE: Color = Color::new(0.102, 0.02, 0.02, 1.0); // #1A0505

// =============================================================================
// FRAME INFO
// =============================================================================

/// Information about a single frame in a packed atlas
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FrameInfo {
    /// Frame name/identifier
    pub name: String,

    /// Source rectangle in the atlas (pixels)
    pub source_rect: Rect,

    /// Original frame size before packing
    pub original_size: (u32, u32),

    /// Offset from original position (for trimmed sprites)
    pub offset: (i32, i32),

    /// Whether the frame was rotated 90 degrees in packing
    pub rotated: bool,

    /// Duration hint (optional, from metadata)
    pub duration: Option<f32>,
}

impl FrameInfo {
    /// Create frame info for a grid cell
    pub fn from_grid(name: &str, col: u32, row: u32, cell_width: u32, cell_height: u32) -> Self {
        Self {
            name: name.to_string(),
            source_rect: Rect::new(
                (col * cell_width) as f32,
                (row * cell_height) as f32,
                cell_width as f32,
                cell_height as f32,
            ),
            original_size: (cell_width, cell_height),
            offset: (0, 0),
            rotated: false,
            duration: None,
        }
    }

    /// Get UV coordinates for this frame given atlas dimensions
    pub fn to_uv(&self, atlas_width: f32, atlas_height: f32) -> Rect {
        Rect::new(
            self.source_rect.x / atlas_width,
            self.source_rect.y / atlas_height,
            self.source_rect.w / atlas_width,
            self.source_rect.h / atlas_height,
        )
    }
}

// =============================================================================
// ATLAS METADATA
// =============================================================================

/// Metadata for a sprite atlas
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AtlasMetadata {
    /// Atlas identifier
    pub id: String,

    /// Image dimensions
    pub width: u32,
    pub height: u32,

    /// Frame information by name
    pub frames: HashMap<String, FrameInfo>,

    /// Grid info (if grid-based)
    pub grid_info: Option<GridInfo>,
}

/// Grid-based atlas layout info
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GridInfo {
    pub columns: u32,
    pub rows: u32,
    pub cell_width: u32,
    pub cell_height: u32,
    pub padding: u32,
}

impl AtlasMetadata {
    /// Create metadata for a grid-based atlas
    pub fn from_grid(id: &str, width: u32, height: u32, columns: u32, rows: u32) -> Self {
        let cell_width = width / columns;
        let cell_height = height / rows;

        let mut frames = HashMap::new();

        for row in 0..rows {
            for col in 0..columns {
                let frame_idx = row * columns + col;
                let name = format!("frame_{:04}", frame_idx);
                frames.insert(
                    name.clone(),
                    FrameInfo::from_grid(&name, col, row, cell_width, cell_height),
                );
            }
        }

        Self {
            id: id.to_string(),
            width,
            height,
            frames,
            grid_info: Some(GridInfo {
                columns,
                rows,
                cell_width,
                cell_height,
                padding: 0,
            }),
        }
    }

    /// Get frame by grid index
    pub fn get_grid_frame(&self, index: u32) -> Option<&FrameInfo> {
        let name = format!("frame_{:04}", index);
        self.frames.get(&name)
    }

    /// Get frame UV by grid position
    pub fn get_grid_uv(&self, col: u32, row: u32) -> Rect {
        if let Some(grid) = &self.grid_info {
            let frame_w = 1.0 / grid.columns as f32;
            let frame_h = 1.0 / grid.rows as f32;
            Rect::new(col as f32 * frame_w, row as f32 * frame_h, frame_w, frame_h)
        } else {
            Rect::new(0.0, 0.0, 1.0, 1.0)
        }
    }
}

// =============================================================================
// SPRITE ATLAS
// =============================================================================

/// A sprite atlas containing an image and frame metadata
pub struct SpriteAtlas {
    /// Unique identifier
    pub id: String,

    /// The actual image data
    pub image: Image,

    /// Atlas metadata
    pub metadata: AtlasMetadata,
}

// Allow dead code for atlas methods that may be used for future asset loading
#[allow(dead_code)]
impl SpriteAtlas {
    /// Create a new sprite atlas from an image file (grid-based)
    pub fn from_grid_file(
        ctx: &mut Context,
        id: &str,
        path: &Path,
        columns: u32,
        rows: u32,
    ) -> GameResult<Self> {
        let image = Image::from_path(ctx, path)?;
        let width = image.width();
        let height = image.height();

        let metadata = AtlasMetadata::from_grid(id, width, height, columns, rows);

        Ok(Self {
            id: id.to_string(),
            image,
            metadata,
        })
    }

    /// Create a placeholder atlas with procedurally generated content
    pub fn create_placeholder(
        ctx: &mut Context,
        id: &str,
        width: u32,
        height: u32,
        columns: u32,
        rows: u32,
    ) -> GameResult<Self> {
        let cell_w = width / columns;
        let cell_h = height / rows;

        // Generate placeholder image data
        let mut pixels = vec![0u8; (width * height * 4) as usize];

        for row in 0..rows {
            for col in 0..columns {
                let frame_idx = row * columns + col;

                // Draw each cell with a unique pattern
                Self::draw_placeholder_frame(
                    &mut pixels,
                    width,
                    col * cell_w,
                    row * cell_h,
                    cell_w,
                    cell_h,
                    frame_idx,
                );
            }
        }

        let image = Image::from_pixels(ctx, &pixels, ImageFormat::Rgba8UnormSrgb, width, height);
        let metadata = AtlasMetadata::from_grid(id, width, height, columns, rows);

        Ok(Self {
            id: id.to_string(),
            image,
            metadata,
        })
    }

    /// Create a Whammy placeholder with characteristic colors
    pub fn create_whammy_placeholder(
        ctx: &mut Context,
        id: &str,
        frame_count: u32,
    ) -> GameResult<Self> {
        // 256x256 frames, arranged in a grid
        let frame_size = 256u32;
        let columns = 6u32;
        let rows = (frame_count + columns - 1) / columns;

        let width = columns * frame_size;
        let height = rows * frame_size;

        let mut pixels = vec![0u8; (width * height * 4) as usize];

        for frame_idx in 0..frame_count {
            let col = frame_idx % columns;
            let row = frame_idx / columns;

            Self::draw_whammy_frame(
                &mut pixels,
                width,
                col * frame_size,
                row * frame_size,
                frame_size,
                frame_size,
                frame_idx,
                frame_count,
            );
        }

        let image = Image::from_pixels(ctx, &pixels, ImageFormat::Rgba8UnormSrgb, width, height);
        let metadata = AtlasMetadata::from_grid(id, width, height, columns, rows);

        Ok(Self {
            id: id.to_string(),
            image,
            metadata,
        })
    }

    /// Draw a generic placeholder frame
    fn draw_placeholder_frame(
        pixels: &mut [u8],
        atlas_width: u32,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        frame_idx: u32,
    ) {
        // Simple colored rectangle with frame number
        let hue = (frame_idx as f32 * 0.1) % 1.0;
        let color = hsv_to_rgb(hue, 0.7, 0.9);

        for py in 0..h {
            for px in 0..w {
                let idx = ((y + py) * atlas_width + (x + px)) as usize * 4;

                // Border
                if px < 2 || px >= w - 2 || py < 2 || py >= h - 2 {
                    pixels[idx] = 40;
                    pixels[idx + 1] = 20;
                    pixels[idx + 2] = 20;
                    pixels[idx + 3] = 255;
                }
                // Fill
                else {
                    pixels[idx] = (color.0 * 255.0) as u8;
                    pixels[idx + 1] = (color.1 * 255.0) as u8;
                    pixels[idx + 2] = (color.2 * 255.0) as u8;
                    pixels[idx + 3] = 255;
                }
            }
        }
    }

    /// Draw a Whammy character placeholder frame
    #[allow(clippy::too_many_arguments)]
    fn draw_whammy_frame(
        pixels: &mut [u8],
        atlas_width: u32,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        frame_idx: u32,
        _total_frames: u32,
    ) {
        let cx = w / 2;
        let cy = h / 2;

        // Animation phase based on frame
        let phase = frame_idx as f32 * 0.2;
        let wobble = (phase.sin() * 10.0) as i32;

        for py in 0..h {
            for px in 0..w {
                let idx = ((y + py) * atlas_width + (x + px)) as usize * 4;

                // Distance from center (for body shape)
                let dx = px as i32 - cx as i32;
                let dy = py as i32 - cy as i32 + wobble;
                let dist = ((dx * dx + dy * dy) as f32).sqrt();

                // Whammy body (ellipse)
                let body_rx = (w as f32 * 0.25) as i32;
                let body_ry = (h as f32 * 0.35) as i32;
                let body_dist = ((dx * dx) as f32 / (body_rx * body_rx) as f32
                    + (dy * dy) as f32 / (body_ry * body_ry) as f32)
                    .sqrt();

                // Head
                let head_dy = dy + (h as i32 / 4);
                let head_r = (w as f32 * 0.2) as i32;
                let head_dist = ((dx * dx + head_dy * head_dy) as f32).sqrt();

                // Cell-shaded colors
                let (r, g, b, a) = if head_dist < head_r as f32 {
                    // Head
                    if head_dist < head_r as f32 * 0.8 {
                        // Inner head (primary red)
                        (212, 32, 32, 255) // #D42020
                    } else {
                        // Head outline
                        (26, 5, 5, 255) // #1A0505
                    }
                } else if body_dist < 1.0 {
                    // Body
                    if body_dist < 0.7 {
                        // Inner body with highlight
                        if dx < 0 && dy < 0 {
                            (255, 64, 64, 255) // #FF4040 highlight
                        } else {
                            (212, 32, 32, 255) // #D42020 primary
                        }
                    } else if body_dist < 0.85 {
                        // Body shadow
                        (139, 21, 21, 255) // #8B1515
                    } else {
                        // Body outline
                        (26, 5, 5, 255) // #1A0505
                    }
                } else if dist < w as f32 * 0.45 {
                    // Limbs (simplified)
                    let limb_phase = phase + (py as f32 * 0.01);
                    if limb_phase.sin().abs() > 0.7 {
                        (139, 21, 21, 255) // Shadow
                    } else {
                        (0, 0, 0, 0) // Transparent
                    }
                } else {
                    (0, 0, 0, 0) // Transparent
                };

                // Eyes (white with black pupils)
                let eye_y = cy as i32 - (h as i32 / 4) + wobble;
                let left_eye_x = cx as i32 - (w as i32 / 8);
                let right_eye_x = cx as i32 + (w as i32 / 8);
                let eye_r = w as f32 * 0.06;

                let left_dist =
                    (((px as i32 - left_eye_x).pow(2) + (py as i32 - eye_y).pow(2)) as f32).sqrt();
                let right_dist =
                    (((px as i32 - right_eye_x).pow(2) + (py as i32 - eye_y).pow(2)) as f32).sqrt();

                let (r, g, b, a) = if left_dist < eye_r || right_dist < eye_r {
                    if left_dist < eye_r * 0.4 || right_dist < eye_r * 0.4 {
                        (0, 0, 0, 255) // Pupil
                    } else {
                        (255, 255, 255, 255) // White of eye
                    }
                } else {
                    (r, g, b, a)
                };

                pixels[idx] = r;
                pixels[idx + 1] = g;
                pixels[idx + 2] = b;
                pixels[idx + 3] = a;
            }
        }
    }

    /// Get the image
    pub fn image(&self) -> &Image {
        &self.image
    }

    /// Get frame by name
    pub fn get_frame(&self, name: &str) -> Option<&FrameInfo> {
        self.metadata.frames.get(name)
    }

    /// Get frame UV by grid index
    pub fn get_frame_uv(&self, index: u32) -> Rect {
        if let Some(grid) = &self.metadata.grid_info {
            let col = index % grid.columns;
            let row = index / grid.columns;
            self.metadata.get_grid_uv(col, row)
        } else {
            Rect::new(0.0, 0.0, 1.0, 1.0)
        }
    }

    /// Get total frame count
    pub fn frame_count(&self) -> u32 {
        self.metadata.frames.len() as u32
    }
}

// =============================================================================
// ATLAS MANAGER
// =============================================================================

/// Manages multiple sprite atlases
pub struct AtlasManager {
    atlases: HashMap<String, SpriteAtlas>,
}

impl AtlasManager {
    /// Create new atlas manager
    pub fn new() -> Self {
        Self {
            atlases: HashMap::new(),
        }
    }

    /// Add an atlas
    pub fn add(&mut self, atlas: SpriteAtlas) {
        self.atlases.insert(atlas.id.clone(), atlas);
    }

    /// Get an atlas by ID
    pub fn get(&self, id: &str) -> Option<&SpriteAtlas> {
        self.atlases.get(id)
    }

    /// Remove an atlas
    pub fn remove(&mut self, id: &str) -> Option<SpriteAtlas> {
        self.atlases.remove(id)
    }

    /// Check if atlas exists
    pub fn contains(&self, id: &str) -> bool {
        self.atlases.contains_key(id)
    }

    /// Get all atlas IDs
    #[allow(dead_code)]
    pub fn ids(&self) -> Vec<&str> {
        self.atlases.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for AtlasManager {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/// Convert HSV to RGB (0-1 range)
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let h = h * 6.0;
    let i = h.floor() as i32;
    let f = h - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));

    match i % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atlas_metadata_grid() {
        let metadata = AtlasMetadata::from_grid("test", 512, 512, 4, 4);

        assert_eq!(metadata.frames.len(), 16);
        assert!(metadata.grid_info.is_some());

        let grid = metadata.grid_info.unwrap();
        assert_eq!(grid.cell_width, 128);
        assert_eq!(grid.cell_height, 128);
    }

    #[test]
    fn test_frame_info_uv() {
        let frame = FrameInfo::from_grid("test", 1, 1, 128, 128);
        let uv = frame.to_uv(512.0, 512.0);

        assert!((uv.x - 0.25).abs() < 0.001);
        assert!((uv.y - 0.25).abs() < 0.001);
        assert!((uv.w - 0.25).abs() < 0.001);
        assert!((uv.h - 0.25).abs() < 0.001);
    }
}
