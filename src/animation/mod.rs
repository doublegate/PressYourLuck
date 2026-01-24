//! # Animation System Module
//!
//! ## Overview
//! This module provides a comprehensive cell-shaded animation system for Press Your Luck,
//! with a focus on the iconic Whammy character animations.
//!
//! ## Architecture
//! - **SpriteAtlas**: Loads and manages sprite sheets (grid-based or packed)
//! - **Animation**: Defines animation sequences with timing and effects
//! - **AnimationPlayer**: Plays animations with state machine support
//! - **ParticleSystem**: Visual effects for money scatter, explosions, etc.
//! - **ScreenEffects**: Screen shake, flash, and other global effects
//!
//! ## Whammy Animation System
//! The Whammy character features 30 unique animations across categories:
//! - Core 5: Hammer, Pogo Stick, Roller Skating, TNT, Fang (Boxing)
//! - Extended 10: Boombox, UFO, Fishing, Skydiving, Cannon, Surfing,
//!                Karate, Graduation, Computer, Baseball
//! - Holiday 10: Santa, Easter Bunny, Cupid, Leprechaun, Turkey, Uncle Sam,
//!               Witch, Jack-o-lantern, Pilgrim, Graduation
//! - Special 3: Trap Door, Group Goodbye, Sad Walk-off

mod atlas;
mod effects;
mod particles;
mod player;
mod types;
mod whammy;

// Re-export public types for external use
pub use atlas::{AtlasManager, AtlasMetadata, FrameInfo, SpriteAtlas};
pub use atlas::{WHAMMY_HIGHLIGHT, WHAMMY_OUTLINE, WHAMMY_RED, WHAMMY_SHADOW};
pub use effects::{FlashMode, ScreenEffects, ScreenFlash, ScreenShake};
pub use particles::{Particle, ParticleEmitter, ParticleSystem, ParticleType};
pub use player::{AnimationEvent, AnimationPlayer, AnimationState, PlaybackState};
pub use types::{Animation, AnimationBuilder, AnimationFrame, LoopMode};
pub use whammy::{WhammyAnimationId, WhammyAnimationLibrary, WhammyAnimator, WhammyState};
