//! # Audio Engine Module
//!
//! ## Overview
//! This module provides a comprehensive audio system for Press Your Luck,
//! supporting both file-based audio assets and procedural fallback generation.
//!
//! ## Phase 1: Audio Integration (Complete)
//!
//! ### Sprint 1: Audio Research & Infrastructure
//! - File-based sound loading with OGG/WAV support
//! - Fallback to procedural generation if files missing
//! - Audio asset caching for performance
//! - Comprehensive AudioConfig system with per-category volumes
//!
//! ### Sprint 2: Core Sound Replacement
//! - Whammy audio system with catchphrase mapping
//! - Board audio (spin ambient, stop sound)
//! - Theme music player with looping and transitions
//! - Music volume ducking during voice clips
//!
//! ### Sprint 3: Polish & Audience Immersion
//! - Audience reaction system (cheers, gasps)
//! - Intensity-based reactions with randomization
//! - Ambient audience murmur (toggleable)
//! - Comprehensive audio mixing with clipping prevention
//!
//! ## Technical Specifications
//! - File Format: OGG Vorbis (primary), WAV (fallback)
//! - Sample Rate: 44,100 Hz (CD quality)
//! - Channels: Mono for effects, Stereo for music
//! - Bit Depth: 16-bit PCM
//!
//! ## Board Tone Frequencies (Hz)
//! The authentic board tones follow a specific musical pattern:
//! ```text
//! D4, E4, G4, Bb4, D4, Ab4, F4, C4, Eb4, D4, B4, A4, C#4, E4, F#4, A4, D4, F4
//! ```

use ggez::audio::{SoundData, SoundSource, Source};
use ggez::{Context, GameResult};
use std::collections::HashMap;

use crate::game::{AudioEvent, WhammyAnimationType};

// ===============================================================================
// CONSTANTS
// ===============================================================================

/// Audio sample rate (CD quality)
const SAMPLE_RATE: u32 = 44100;

/// Base path for audio assets
const AUDIO_ASSET_PATH: &str = "/assets/audio";

/// Board tone frequencies in Hz (authentic sequence)
/// These create the distinctive musical pattern as the light moves around the board
const BOARD_TONES: [f32; 18] = [
    293.66, // D4  - Square 0 (top-left)
    329.63, // E4  - Square 1
    392.00, // G4  - Square 2
    466.16, // Bb4 - Square 3
    293.66, // D4  - Square 4
    415.30, // Ab4 - Square 5 (top-right)
    349.23, // F4  - Square 6
    261.63, // C4  - Square 7
    311.13, // Eb4 - Square 8
    293.66, // D4  - Square 9 (bottom-right)
    493.88, // B4  - Square 10
    440.00, // A4  - Square 11
    277.18, // C#4 - Square 12
    329.63, // E4  - Square 13
    369.99, // F#4 - Square 14 (bottom-left)
    440.00, // A4  - Square 15
    293.66, // D4  - Square 16
    349.23, // F4  - Square 17
];

// ===============================================================================
// AUDIO CONFIGURATION
// ===============================================================================

/// Configuration for the audio engine with per-category volume controls
///
/// This structure allows fine-grained control over all audio aspects,
/// including the ability to toggle between authentic file-based sounds
/// and procedurally generated fallbacks.
#[derive(Debug, Clone)]
pub struct AudioConfig {
    /// Master volume (0.0 - 1.0) - affects all audio
    pub master_volume: f32,
    /// Music volume (0.0 - 1.0) - theme, board music, tension
    pub music_volume: f32,
    /// Effects volume (0.0 - 1.0) - board tones, cash, prize, etc.
    pub effects_volume: f32,
    /// Voice volume (0.0 - 1.0) - Whammy catchphrases
    pub voice_volume: f32,
    /// Audience volume (0.0 - 1.0) - cheers, gasps, murmur
    pub audience_volume: f32,
    /// Whether to attempt loading authentic audio files
    pub use_authentic_sounds: bool,
    /// Whether ambient audience murmur is enabled
    pub ambient_audience_enabled: bool,
    /// Whether to duck music volume during voice clips
    pub ducking_enabled: bool,
    /// Amount to reduce music volume during ducking (0.0 - 1.0)
    pub ducking_ratio: f32,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 0.7,
            effects_volume: 0.8,
            voice_volume: 0.85,
            audience_volume: 0.6,
            use_authentic_sounds: true,
            ambient_audience_enabled: false,
            ducking_enabled: true,
            ducking_ratio: 0.3,
        }
    }
}

impl AudioConfig {
    /// Calculate effective volume for a category
    pub fn effective_volume(&self, category_volume: f32, base_volume: f32) -> f32 {
        (self.master_volume * category_volume * base_volume).min(1.0)
    }

    /// Get effective music volume (considering ducking)
    pub fn effective_music_volume(&self, base_volume: f32, is_ducked: bool) -> f32 {
        let category_volume = if is_ducked {
            self.music_volume * self.ducking_ratio
        } else {
            self.music_volume
        };
        self.effective_volume(category_volume, base_volume)
    }
}

// ===============================================================================
// AUDIO CACHE
// ===============================================================================

/// Cache for loaded audio data to avoid repeated file I/O
#[derive(Default)]
struct AudioCache {
    /// Cached sound data by identifier
    sounds: HashMap<String, SoundData>,
}

impl AudioCache {
    fn new() -> Self {
        Self {
            sounds: HashMap::new(),
        }
    }

    /// Get or load sound data for a given identifier
    fn get_or_load(&mut self, id: &str, data: SoundData) -> SoundData {
        self.sounds.entry(id.to_string()).or_insert(data).clone()
    }

    /// Check if a sound is cached
    #[allow(dead_code)]
    fn contains(&self, id: &str) -> bool {
        self.sounds.contains_key(id)
    }

    /// Clear the cache
    #[allow(dead_code)]
    fn clear(&mut self) {
        self.sounds.clear();
    }
}

// ===============================================================================
// AUDIENCE REACTION SYSTEM
// ===============================================================================

/// Intensity levels for audience reactions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReactionIntensity {
    /// Small wins (< $500)
    Low,
    /// Medium wins ($500 - $1000)
    Medium,
    /// Large wins (> $1000)
    High,
    /// Massive wins (> $5000) or eliminations
    Extreme,
}

impl ReactionIntensity {
    /// Determine cheer intensity from win amount
    pub fn from_win_amount(amount: u32) -> Self {
        if amount >= 5000 {
            Self::Extreme
        } else if amount >= 1000 {
            Self::High
        } else if amount >= 500 {
            Self::Medium
        } else {
            Self::Low
        }
    }

    /// Get volume multiplier for this intensity
    fn volume_multiplier(&self) -> f32 {
        match self {
            Self::Low => 0.6,
            Self::Medium => 0.75,
            Self::High => 0.9,
            Self::Extreme => 1.0,
        }
    }
}

/// Extended audio event types for the enhanced audio system
#[derive(Debug, Clone)]
pub enum ExtendedAudioEvent {
    /// Standard game audio event
    Standard(AudioEvent),
    /// Play a Whammy catchphrase for the given animation type
    WhammyCatchphrase(WhammyAnimationType),
    /// Play audience cheer with intensity
    AudienceCheerIntensity(ReactionIntensity),
    /// Play audience gasp with intensity
    AudienceGaspIntensity(ReactionIntensity),
    /// Start playing intro theme
    PlayIntroTheme,
    /// Start board music loop
    StartBoardMusic,
    /// Stop board music
    StopBoardMusic,
    /// Start ambient audience murmur
    StartAmbientMurmur,
    /// Stop ambient audience murmur
    StopAmbientMurmur,
    /// Start board spin ambient sound
    StartBoardSpinAmbient,
    /// Stop board spin ambient sound
    StopBoardSpinAmbient,
}

// ===============================================================================
// AUDIO ENGINE
// ===============================================================================

/// Main audio engine managing all game sounds
///
/// # Architecture
/// The engine supports both file-based audio loading and procedural generation.
/// When `use_authentic_sounds` is true, it attempts to load audio files first,
/// falling back to procedural generation if files are not found.
///
/// # Sound Categories
/// 1. **Board Tones (18)**: One for each square position
/// 2. **Game Events**: Whammy, cash, prize, special, correct, wrong
/// 3. **Music**: Intro theme, board music, tension loop
/// 4. **Audience**: Cheers, gasps, murmur
/// 5. **Voice**: Whammy catchphrases
pub struct AudioEngine {
    /// Audio configuration
    config: AudioConfig,

    /// Audio cache for loaded sounds
    cache: AudioCache,

    /// Pre-generated board tones (one per square)
    board_tones: Vec<Option<Source>>,

    /// Whammy foghorn sound
    whammy_sound: Option<Source>,

    /// Cash register sound (small wins)
    cash_sound: Option<Source>,

    /// Big cash sound (large wins)
    big_cash_sound: Option<Source>,

    /// Prize fanfare
    prize_sound: Option<Source>,

    /// Special square magical sweep
    special_sound: Option<Source>,

    /// Correct answer chime
    correct_sound: Option<Source>,

    /// Wrong answer buzzer
    wrong_sound: Option<Source>,

    /// Sad trombone (elimination)
    sad_trombone: Option<Source>,

    /// Spin added bell
    spin_added_sound: Option<Source>,

    /// Buzz-in sound
    buzz_in_sound: Option<Source>,

    /// Winner fanfare
    winner_sound: Option<Source>,

    /// Button click
    click_sound: Option<Source>,

    /// Board stop "chunk" sound (mechanical)
    board_stop_sound: Option<Source>,

    /// Audience cheer variations
    audience_cheer_sounds: Vec<Option<Source>>,

    /// Audience gasp variations
    audience_gasp_sounds: Vec<Option<Source>>,

    /// Ambient audience murmur (looping)
    audience_murmur: Option<Source>,

    /// Tension music loop for spinning
    tension_music: Option<Source>,

    /// Intro theme music
    intro_theme: Option<Source>,

    /// Board music loop
    board_music: Option<Source>,

    /// Board spin ambient sound (looping)
    board_spin_ambient: Option<Source>,

    /// Whammy catchphrase sounds mapped by animation type
    whammy_catchphrases: HashMap<String, Source>,

    /// Generic whammy catchphrases for fallback
    generic_whammy_catchphrases: Vec<Option<Source>>,

    /// Whether tension music is currently playing
    tension_playing: bool,

    /// Whether board music is currently playing
    board_music_playing: bool,

    /// Whether ambient audience murmur is playing
    murmur_playing: bool,

    /// Whether board spin ambient is playing
    spin_ambient_playing: bool,

    /// Whether music is currently ducked (voice playing)
    music_ducked: bool,

    /// Timer for music ducking recovery
    ducking_timer: f32,

    /// Whether audio has been initialized
    initialized: bool,
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self {
            config: AudioConfig::default(),
            cache: AudioCache::new(),
            board_tones: Vec::new(),
            whammy_sound: None,
            cash_sound: None,
            big_cash_sound: None,
            prize_sound: None,
            special_sound: None,
            correct_sound: None,
            wrong_sound: None,
            sad_trombone: None,
            spin_added_sound: None,
            buzz_in_sound: None,
            winner_sound: None,
            click_sound: None,
            board_stop_sound: None,
            audience_cheer_sounds: Vec::new(),
            audience_gasp_sounds: Vec::new(),
            audience_murmur: None,
            tension_music: None,
            intro_theme: None,
            board_music: None,
            board_spin_ambient: None,
            whammy_catchphrases: HashMap::new(),
            generic_whammy_catchphrases: Vec::new(),
            tension_playing: false,
            board_music_playing: false,
            murmur_playing: false,
            spin_ambient_playing: false,
            music_ducked: false,
            ducking_timer: 0.0,
            initialized: false,
        }
    }
}

#[allow(dead_code)]
impl AudioEngine {
    /// Create a new audio engine with default configuration
    ///
    /// # Note
    /// Sounds are generated and loaded synchronously.
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Self::with_config(ctx, AudioConfig::default())
    }

    /// Create a new audio engine with custom configuration
    pub fn with_config(ctx: &mut Context, config: AudioConfig) -> GameResult<Self> {
        let mut engine = Self {
            config,
            cache: AudioCache::new(),
            board_tones: Vec::with_capacity(18),
            whammy_sound: None,
            cash_sound: None,
            big_cash_sound: None,
            prize_sound: None,
            special_sound: None,
            correct_sound: None,
            wrong_sound: None,
            sad_trombone: None,
            spin_added_sound: None,
            buzz_in_sound: None,
            winner_sound: None,
            click_sound: None,
            board_stop_sound: None,
            audience_cheer_sounds: Vec::with_capacity(6),
            audience_gasp_sounds: Vec::with_capacity(4),
            audience_murmur: None,
            tension_music: None,
            intro_theme: None,
            board_music: None,
            board_spin_ambient: None,
            whammy_catchphrases: HashMap::new(),
            generic_whammy_catchphrases: Vec::with_capacity(3),
            tension_playing: false,
            board_music_playing: false,
            murmur_playing: false,
            spin_ambient_playing: false,
            music_ducked: false,
            ducking_timer: 0.0,
            initialized: false,
        };

        // Generate/load all sounds
        engine.initialize_sounds(ctx)?;
        engine.initialized = true;

        println!(
            "Audio engine initialized: {} board tones, {} cheer variations, {} gasp variations",
            engine.board_tones.len(),
            engine.audience_cheer_sounds.len(),
            engine.audience_gasp_sounds.len(),
        );

        Ok(engine)
    }

    /// Initialize all sounds - attempts file loading first, falls back to procedural
    fn initialize_sounds(&mut self, ctx: &mut Context) -> GameResult {
        let use_authentic = self.config.use_authentic_sounds;

        // Generate board tones (18 unique frequencies)
        for (i, &freq) in BOARD_TONES.iter().enumerate() {
            let id = format!("tone_{}", i);
            let ogg_path = format!("{}/board_tones/tone_{}.ogg", AUDIO_ASSET_PATH, i);
            let wav_path = format!("{}/board_tones/tone_{}.wav", AUDIO_ASSET_PATH, i);

            let source = if use_authentic {
                if let Ok(source) = Source::new(ctx, &ogg_path) {
                    println!("Loaded: {}", ogg_path);
                    Some(source)
                } else if let Ok(source) = Source::new(ctx, &wav_path) {
                    println!("Loaded: {}", wav_path);
                    Some(source)
                } else {
                    self.make_source_procedural(ctx, &id, generate_board_tone(freq))
                }
            } else {
                self.make_source_procedural(ctx, &id, generate_board_tone(freq))
            };
            self.board_tones.push(source);
        }

        // Generate game event sounds with file fallback
        self.whammy_sound = self.load_or_generate(
            ctx,
            "whammy",
            &format!("{}/effects/whammy.ogg", AUDIO_ASSET_PATH),
            &format!("{}/effects/whammy.wav", AUDIO_ASSET_PATH),
            generate_whammy_sound,
        );

        self.cash_sound = self.load_or_generate(
            ctx,
            "cash_small",
            &format!("{}/effects/cash_small.ogg", AUDIO_ASSET_PATH),
            &format!("{}/effects/cash_small.wav", AUDIO_ASSET_PATH),
            || generate_cash_sound(false),
        );

        self.big_cash_sound = self.load_or_generate(
            ctx,
            "cash_big",
            &format!("{}/effects/cash_big.ogg", AUDIO_ASSET_PATH),
            &format!("{}/effects/cash_big.wav", AUDIO_ASSET_PATH),
            || generate_cash_sound(true),
        );

        self.prize_sound = self.load_or_generate(
            ctx,
            "prize",
            &format!("{}/effects/prize.ogg", AUDIO_ASSET_PATH),
            &format!("{}/effects/prize.wav", AUDIO_ASSET_PATH),
            generate_prize_sound,
        );

        self.special_sound = self.load_or_generate(
            ctx,
            "special",
            &format!("{}/effects/special.ogg", AUDIO_ASSET_PATH),
            &format!("{}/effects/special.wav", AUDIO_ASSET_PATH),
            generate_special_sound,
        );

        self.correct_sound = self.load_or_generate(
            ctx,
            "correct",
            &format!("{}/effects/correct.ogg", AUDIO_ASSET_PATH),
            &format!("{}/effects/correct.wav", AUDIO_ASSET_PATH),
            generate_correct_sound,
        );

        self.wrong_sound = self.load_or_generate(
            ctx,
            "wrong",
            &format!("{}/effects/wrong.ogg", AUDIO_ASSET_PATH),
            &format!("{}/effects/wrong.wav", AUDIO_ASSET_PATH),
            generate_wrong_sound,
        );

        self.sad_trombone = self.load_or_generate(
            ctx,
            "sad_trombone",
            &format!("{}/effects/sad_trombone.ogg", AUDIO_ASSET_PATH),
            &format!("{}/effects/sad_trombone.wav", AUDIO_ASSET_PATH),
            generate_sad_trombone,
        );

        self.spin_added_sound = self.load_or_generate(
            ctx,
            "spin_added",
            &format!("{}/effects/spin_added.ogg", AUDIO_ASSET_PATH),
            &format!("{}/effects/spin_added.wav", AUDIO_ASSET_PATH),
            generate_spin_added_sound,
        );

        self.buzz_in_sound = self.load_or_generate(
            ctx,
            "buzz_in",
            &format!("{}/effects/buzz_in.ogg", AUDIO_ASSET_PATH),
            &format!("{}/effects/buzz_in.wav", AUDIO_ASSET_PATH),
            generate_buzz_in_sound,
        );

        self.winner_sound = self.load_or_generate(
            ctx,
            "winner",
            &format!("{}/theme/winner.ogg", AUDIO_ASSET_PATH),
            &format!("{}/theme/winner.wav", AUDIO_ASSET_PATH),
            generate_winner_fanfare,
        );

        self.click_sound = self.load_or_generate(
            ctx,
            "click",
            &format!("{}/effects/click.ogg", AUDIO_ASSET_PATH),
            &format!("{}/effects/click.wav", AUDIO_ASSET_PATH),
            generate_click_sound,
        );

        self.board_stop_sound = self.load_or_generate(
            ctx,
            "board_stop",
            &format!("{}/effects/board_stop.ogg", AUDIO_ASSET_PATH),
            &format!("{}/effects/board_stop.wav", AUDIO_ASSET_PATH),
            generate_board_stop_sound,
        );

        // Initialize audience reaction sounds with variations
        self.initialize_audience_sounds(ctx)?;

        // Initialize music tracks with looping
        self.initialize_music(ctx)?;

        // Initialize Whammy catchphrases
        self.initialize_whammy_catchphrases(ctx)?;

        Ok(())
    }

    /// Helper to create a Source from WAV data with caching
    fn make_source_procedural(
        &mut self,
        ctx: &mut Context,
        id: &str,
        wav_data: Vec<u8>,
    ) -> Option<Source> {
        let sound_data: SoundData = wav_data.into();
        let cached_data = self.cache.get_or_load(id, sound_data);
        Source::from_data(ctx, cached_data).ok()
    }

    /// Helper to try loading from file, fall back to procedural
    fn load_or_generate<F>(
        &mut self,
        ctx: &mut Context,
        id: &str,
        ogg_path: &str,
        wav_path: &str,
        procedural_fn: F,
    ) -> Option<Source>
    where
        F: FnOnce() -> Vec<u8>,
    {
        if self.config.use_authentic_sounds {
            // Try OGG first
            if let Ok(source) = Source::new(ctx, ogg_path) {
                println!("Loaded audio: {}", ogg_path);
                return Some(source);
            }
            // Try WAV fallback
            if let Ok(source) = Source::new(ctx, wav_path) {
                println!("Loaded audio: {}", wav_path);
                return Some(source);
            }
        }
        // Fall back to procedural generation
        let wav_data = procedural_fn();
        self.make_source_procedural(ctx, id, wav_data)
    }

    /// Initialize audience reaction sounds with multiple variations
    fn initialize_audience_sounds(&mut self, ctx: &mut Context) -> GameResult {
        let use_authentic = self.config.use_authentic_sounds;

        // Cheer variations (small, medium, large, huge, + variations)
        let cheer_ids = [
            "cheer_small",
            "cheer_medium",
            "cheer_large",
            "cheer_huge",
            "cheer_var_1",
            "cheer_var_2",
        ];

        for id in &cheer_ids {
            let ogg_path = format!("{}/audience/{}.ogg", AUDIO_ASSET_PATH, id);
            let wav_path = format!("{}/audience/{}.wav", AUDIO_ASSET_PATH, id);

            let source = if use_authentic {
                if let Ok(source) = Source::new(ctx, &ogg_path) {
                    Some(source)
                } else if let Ok(source) = Source::new(ctx, &wav_path) {
                    Some(source)
                } else {
                    // Procedural fallback
                    self.make_source_procedural(ctx, id, generate_audience_cheer())
                }
            } else {
                self.make_source_procedural(ctx, id, generate_audience_cheer())
            };
            self.audience_cheer_sounds.push(source);
        }

        // Gasp variations (small, large, elimination, variation)
        let gasp_ids = ["gasp_small", "gasp_large", "gasp_elimination", "gasp_var_1"];

        for id in &gasp_ids {
            let ogg_path = format!("{}/audience/{}.ogg", AUDIO_ASSET_PATH, id);
            let wav_path = format!("{}/audience/{}.wav", AUDIO_ASSET_PATH, id);

            let source = if use_authentic {
                if let Ok(source) = Source::new(ctx, &ogg_path) {
                    Some(source)
                } else if let Ok(source) = Source::new(ctx, &wav_path) {
                    Some(source)
                } else {
                    self.make_source_procedural(ctx, id, generate_audience_gasp())
                }
            } else {
                self.make_source_procedural(ctx, id, generate_audience_gasp())
            };
            self.audience_gasp_sounds.push(source);
        }

        // Ambient murmur (looping)
        let murmur_ogg = format!("{}/audience/murmur.ogg", AUDIO_ASSET_PATH);
        let murmur_wav = format!("{}/audience/murmur.wav", AUDIO_ASSET_PATH);

        self.audience_murmur = if use_authentic {
            if let Ok(mut source) = Source::new(ctx, &murmur_ogg) {
                source.set_repeat(true);
                Some(source)
            } else if let Ok(mut source) = Source::new(ctx, &murmur_wav) {
                source.set_repeat(true);
                Some(source)
            } else {
                // Procedural fallback
                if let Some(mut source) =
                    self.make_source_procedural(ctx, "murmur", generate_audience_murmur())
                {
                    source.set_repeat(true);
                    Some(source)
                } else {
                    None
                }
            }
        } else if let Some(mut source) =
            self.make_source_procedural(ctx, "murmur", generate_audience_murmur())
        {
            source.set_repeat(true);
            Some(source)
        } else {
            None
        };

        Ok(())
    }

    /// Initialize music tracks
    fn initialize_music(&mut self, ctx: &mut Context) -> GameResult {
        let use_authentic = self.config.use_authentic_sounds;

        // Intro theme
        let intro_ogg = format!("{}/theme/intro.ogg", AUDIO_ASSET_PATH);
        let intro_wav = format!("{}/theme/intro.wav", AUDIO_ASSET_PATH);

        self.intro_theme = if use_authentic {
            if let Ok(source) = Source::new(ctx, &intro_ogg) {
                println!("Loaded intro theme: {}", intro_ogg);
                Some(source)
            } else if let Ok(source) = Source::new(ctx, &intro_wav) {
                println!("Loaded intro theme: {}", intro_wav);
                Some(source)
            } else {
                // No procedural intro theme (too complex)
                None
            }
        } else {
            None
        };

        // Board music (looping)
        let board_ogg = format!("{}/theme/board_music.ogg", AUDIO_ASSET_PATH);
        let board_wav = format!("{}/theme/board_music.wav", AUDIO_ASSET_PATH);

        self.board_music = if use_authentic {
            if let Ok(mut source) = Source::new(ctx, &board_ogg) {
                source.set_repeat(true);
                Some(source)
            } else if let Ok(mut source) = Source::new(ctx, &board_wav) {
                source.set_repeat(true);
                Some(source)
            } else {
                None
            }
        } else {
            None
        };

        // Tension music (looping)
        let tension_ogg = format!("{}/theme/tension.ogg", AUDIO_ASSET_PATH);
        let tension_wav = format!("{}/theme/tension.wav", AUDIO_ASSET_PATH);

        self.tension_music = if use_authentic {
            if let Ok(mut source) = Source::new(ctx, &tension_ogg) {
                source.set_repeat(true);
                Some(source)
            } else if let Ok(mut source) = Source::new(ctx, &tension_wav) {
                source.set_repeat(true);
                Some(source)
            } else {
                // Procedural fallback
                if let Some(mut source) =
                    self.make_source_procedural(ctx, "tension", generate_tension_music())
                {
                    source.set_repeat(true);
                    Some(source)
                } else {
                    None
                }
            }
        } else if let Some(mut source) =
            self.make_source_procedural(ctx, "tension", generate_tension_music())
        {
            source.set_repeat(true);
            Some(source)
        } else {
            None
        };

        // Board spin ambient (looping)
        let spin_ogg = format!("{}/effects/board_spin_ambient.ogg", AUDIO_ASSET_PATH);
        let spin_wav = format!("{}/effects/board_spin_ambient.wav", AUDIO_ASSET_PATH);

        self.board_spin_ambient = if use_authentic {
            if let Ok(mut source) = Source::new(ctx, &spin_ogg) {
                source.set_repeat(true);
                Some(source)
            } else if let Ok(mut source) = Source::new(ctx, &spin_wav) {
                source.set_repeat(true);
                Some(source)
            } else {
                if let Some(mut source) =
                    self.make_source_procedural(ctx, "spin_ambient", generate_board_spin_ambient())
                {
                    source.set_repeat(true);
                    Some(source)
                } else {
                    None
                }
            }
        } else if let Some(mut source) =
            self.make_source_procedural(ctx, "spin_ambient", generate_board_spin_ambient())
        {
            source.set_repeat(true);
            Some(source)
        } else {
            None
        };

        Ok(())
    }

    /// Initialize Whammy catchphrase sounds
    fn initialize_whammy_catchphrases(&mut self, ctx: &mut Context) -> GameResult {
        let use_authentic = self.config.use_authentic_sounds;

        // Map of animation types to their catchphrase file names
        let catchphrase_mappings = [
            ("Hammer", "catchphrase_hammer"),
            ("Pogo", "catchphrase_pogo"),
            ("TNT", "catchphrase_tnt"),
            ("LawnMower", "catchphrase_lawnmower"),
            ("Fang", "catchphrase_fang"),
            ("Dance", "catchphrase_dance"),
            ("Jumping", "catchphrase_jumping"),
            ("RocketShip", "catchphrase_rocketship"),
            ("GrimReaper", "catchphrase_grimreaper"),
            ("FiringSquad", "catchphrase_firingsquad"),
            ("AngelWhammy", "catchphrase_angelwhammy"),
        ];

        for (anim_type, file_id) in &catchphrase_mappings {
            if use_authentic {
                let ogg_path = format!("{}/whammy/{}.ogg", AUDIO_ASSET_PATH, file_id);
                let wav_path = format!("{}/whammy/{}.wav", AUDIO_ASSET_PATH, file_id);

                if let Ok(source) = Source::new(ctx, &ogg_path) {
                    self.whammy_catchphrases
                        .insert(anim_type.to_string(), source);
                } else if let Ok(source) = Source::new(ctx, &wav_path) {
                    self.whammy_catchphrases
                        .insert(anim_type.to_string(), source);
                }
            }
        }

        // Load generic catchphrases as fallback
        let generic_ids = [
            "catchphrase_generic_1",
            "catchphrase_generic_2",
            "catchphrase_generic_3",
        ];

        for id in &generic_ids {
            if use_authentic {
                let ogg_path = format!("{}/whammy/{}.ogg", AUDIO_ASSET_PATH, id);
                let wav_path = format!("{}/whammy/{}.wav", AUDIO_ASSET_PATH, id);

                if let Ok(source) = Source::new(ctx, &ogg_path) {
                    self.generic_whammy_catchphrases.push(Some(source));
                } else if let Ok(source) = Source::new(ctx, &wav_path) {
                    self.generic_whammy_catchphrases.push(Some(source));
                } else {
                    self.generic_whammy_catchphrases.push(None);
                }
            } else {
                self.generic_whammy_catchphrases.push(None);
            }
        }

        Ok(())
    }

    /// Update the audio engine
    ///
    /// # Arguments
    /// * `_ctx` - ggez context
    /// * `delta_time` - Frame delta time for ducking recovery
    pub fn update(&mut self, _ctx: &mut Context, delta_time: f32) -> GameResult {
        // Handle ducking recovery
        if self.music_ducked && self.ducking_timer > 0.0 {
            self.ducking_timer -= delta_time;
            if self.ducking_timer <= 0.0 {
                self.music_ducked = false;
                self.ducking_timer = 0.0;
            }
        }

        Ok(())
    }

    /// Get the current audio configuration
    pub fn config(&self) -> &AudioConfig {
        &self.config
    }

    /// Get mutable access to the audio configuration
    pub fn config_mut(&mut self) -> &mut AudioConfig {
        &mut self.config
    }

    /// Set a new audio configuration
    pub fn set_config(&mut self, config: AudioConfig) {
        self.config = config;
    }

    /// Handle an audio event from the game
    ///
    /// # Arguments
    /// * `ctx` - ggez context for audio playback
    /// * `event` - The audio event to process
    pub fn handle_event(&mut self, ctx: &Context, event: AudioEvent) {
        match event {
            AudioEvent::BoardTone(index) => self.play_board_tone(ctx, index),
            AudioEvent::WhammySound => self.play_whammy(ctx),
            AudioEvent::CashSound { big } => self.play_cash(ctx, big),
            AudioEvent::PrizeSound => self.play_prize(ctx),
            AudioEvent::SpecialSound => self.play_special(ctx),
            AudioEvent::CorrectSound => self.play_correct(ctx),
            AudioEvent::WrongSound => self.play_wrong(ctx),
            AudioEvent::SadTrombone => self.play_sad_trombone(ctx),
            AudioEvent::SpinAddedBell => self.play_spin_added(ctx),
            AudioEvent::BuzzInSound => self.play_buzz_in(ctx),
            AudioEvent::WinnerFanfare => self.play_winner(ctx),
            AudioEvent::BoardStopSound => self.play_board_stop(ctx),
            AudioEvent::AudienceCheer => self.play_audience_cheer(ctx, ReactionIntensity::High),
            AudioEvent::AudienceGasp => self.play_audience_gasp(ctx, ReactionIntensity::High),
            AudioEvent::StartTensionMusic => self.start_tension_music(ctx),
            AudioEvent::StopTensionMusic => self.stop_tension_music(ctx),
        }
    }

    /// Handle an extended audio event
    pub fn handle_extended_event(&mut self, ctx: &Context, event: ExtendedAudioEvent) {
        match event {
            ExtendedAudioEvent::Standard(e) => self.handle_event(ctx, e),
            ExtendedAudioEvent::WhammyCatchphrase(anim_type) => {
                self.play_whammy_catchphrase(ctx, anim_type)
            }
            ExtendedAudioEvent::AudienceCheerIntensity(intensity) => {
                self.play_audience_cheer(ctx, intensity)
            }
            ExtendedAudioEvent::AudienceGaspIntensity(intensity) => {
                self.play_audience_gasp(ctx, intensity)
            }
            ExtendedAudioEvent::PlayIntroTheme => self.play_intro_theme(ctx),
            ExtendedAudioEvent::StartBoardMusic => self.start_board_music(ctx),
            ExtendedAudioEvent::StopBoardMusic => self.stop_board_music(ctx),
            ExtendedAudioEvent::StartAmbientMurmur => self.start_ambient_murmur(ctx),
            ExtendedAudioEvent::StopAmbientMurmur => self.stop_ambient_murmur(ctx),
            ExtendedAudioEvent::StartBoardSpinAmbient => self.start_board_spin_ambient(ctx),
            ExtendedAudioEvent::StopBoardSpinAmbient => self.stop_board_spin_ambient(ctx),
        }
    }

    /// Play a board tone for the given square index
    fn play_board_tone(&mut self, ctx: &Context, index: usize) {
        if index < self.board_tones.len() {
            if let Some(sound) = &mut self.board_tones[index] {
                let volume = self.config.effective_volume(self.config.effects_volume, 0.6);
                sound.set_volume(volume);
                let _ = sound.play_detached(ctx);
            }
        }
    }

    /// Play the Whammy foghorn sound
    fn play_whammy(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.whammy_sound {
            let volume = self.config.effective_volume(self.config.effects_volume, 0.8);
            sound.set_volume(volume);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play a Whammy catchphrase for the given animation type
    fn play_whammy_catchphrase(&mut self, ctx: &Context, anim_type: WhammyAnimationType) {
        // Enable music ducking
        if self.config.ducking_enabled {
            self.music_ducked = true;
            self.ducking_timer = 2.0; // Duck for 2 seconds
        }

        let anim_name = format!("{:?}", anim_type);

        // Try to find specific catchphrase for this animation
        if let Some(sound) = self.whammy_catchphrases.get_mut(&anim_name) {
            let volume = self.config.effective_volume(self.config.voice_volume, 0.75);
            sound.set_volume(volume);
            let _ = sound.play_detached(ctx);
            return;
        }

        // Fall back to generic catchphrase
        if !self.generic_whammy_catchphrases.is_empty() {
            let idx = fastrand::usize(..self.generic_whammy_catchphrases.len());
            if let Some(Some(sound)) = self.generic_whammy_catchphrases.get_mut(idx) {
                let volume = self.config.effective_volume(self.config.voice_volume, 0.75);
                sound.set_volume(volume);
                let _ = sound.play_detached(ctx);
            }
        }
    }

    /// Play cash register sound
    fn play_cash(&mut self, ctx: &Context, big: bool) {
        let sound = if big {
            &mut self.big_cash_sound
        } else {
            &mut self.cash_sound
        };
        if let Some(s) = sound {
            let volume = self
                .config
                .effective_volume(self.config.effects_volume, if big { 0.75 } else { 0.7 });
            s.set_volume(volume);
            let _ = s.play_detached(ctx);
        }
    }

    /// Play prize fanfare
    fn play_prize(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.prize_sound {
            let volume = self.config.effective_volume(self.config.effects_volume, 0.7);
            sound.set_volume(volume);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play special square sound
    fn play_special(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.special_sound {
            let volume = self.config.effective_volume(self.config.effects_volume, 0.7);
            sound.set_volume(volume);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play correct answer chime
    fn play_correct(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.correct_sound {
            let volume = self.config.effective_volume(self.config.effects_volume, 0.6);
            sound.set_volume(volume);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play wrong answer buzzer
    fn play_wrong(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.wrong_sound {
            let volume = self.config.effective_volume(self.config.effects_volume, 0.6);
            sound.set_volume(volume);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play sad trombone for elimination
    fn play_sad_trombone(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.sad_trombone {
            let volume = self.config.effective_volume(self.config.effects_volume, 0.7);
            sound.set_volume(volume);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play spin added bell
    fn play_spin_added(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.spin_added_sound {
            let volume = self.config.effective_volume(self.config.effects_volume, 0.5);
            sound.set_volume(volume);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play buzz-in sound
    fn play_buzz_in(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.buzz_in_sound {
            let volume = self.config.effective_volume(self.config.effects_volume, 0.6);
            sound.set_volume(volume);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play winner fanfare
    fn play_winner(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.winner_sound {
            let volume = self
                .config
                .effective_music_volume(0.8, self.music_ducked);
            sound.set_volume(volume);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play button click sound
    pub fn play_button_click(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.click_sound {
            let volume = self.config.effective_volume(self.config.effects_volume, 0.3);
            sound.set_volume(volume);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play board stop mechanical "chunk" sound
    fn play_board_stop(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.board_stop_sound {
            let volume = self.config.effective_volume(self.config.effects_volume, 0.7);
            sound.set_volume(volume);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play audience cheer with intensity-based variation selection
    fn play_audience_cheer(&mut self, ctx: &Context, intensity: ReactionIntensity) {
        if self.audience_cheer_sounds.is_empty() {
            return;
        }

        // Select sound index based on intensity with randomization
        let base_idx = match intensity {
            ReactionIntensity::Low => 0,    // cheer_small
            ReactionIntensity::Medium => 1, // cheer_medium
            ReactionIntensity::High => {
                // Randomly pick between large and variations
                if fastrand::bool() {
                    2
                } else {
                    fastrand::usize(4..self.audience_cheer_sounds.len().max(5))
                }
            }
            ReactionIntensity::Extreme => 3, // cheer_huge
        };

        let idx = base_idx.min(self.audience_cheer_sounds.len() - 1);

        if let Some(Some(sound)) = self.audience_cheer_sounds.get_mut(idx) {
            let base_volume = 0.4 * intensity.volume_multiplier();
            let volume = self
                .config
                .effective_volume(self.config.audience_volume, base_volume);
            sound.set_volume(volume);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play audience gasp with intensity-based variation selection
    fn play_audience_gasp(&mut self, ctx: &Context, intensity: ReactionIntensity) {
        if self.audience_gasp_sounds.is_empty() {
            return;
        }

        // Select sound index based on intensity
        let base_idx = match intensity {
            ReactionIntensity::Low => 0,    // gasp_small
            ReactionIntensity::Medium => 1, // gasp_large
            ReactionIntensity::High => {
                // Randomly pick between large and variation
                if fastrand::bool() {
                    1
                } else {
                    3
                }
            }
            ReactionIntensity::Extreme => 2, // gasp_elimination
        };

        let idx = base_idx.min(self.audience_gasp_sounds.len() - 1);

        if let Some(Some(sound)) = self.audience_gasp_sounds.get_mut(idx) {
            let base_volume = 0.35 * intensity.volume_multiplier();
            let volume = self
                .config
                .effective_volume(self.config.audience_volume, base_volume);
            sound.set_volume(volume);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play the intro theme music
    fn play_intro_theme(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.intro_theme {
            let volume = self
                .config
                .effective_music_volume(0.7, self.music_ducked);
            sound.set_volume(volume);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Start playing board music loop
    fn start_board_music(&mut self, ctx: &Context) {
        if !self.board_music_playing {
            if let Some(sound) = &mut self.board_music {
                let volume = self
                    .config
                    .effective_music_volume(0.5, self.music_ducked);
                sound.set_volume(volume);
                let _ = sound.play_detached(ctx);
                self.board_music_playing = true;
            }
        }
    }

    /// Stop playing board music
    fn stop_board_music(&mut self, ctx: &Context) {
        if self.board_music_playing {
            if let Some(sound) = &mut self.board_music {
                let _ = sound.stop(ctx);
            }
            self.board_music_playing = false;
        }
    }

    /// Start playing tension music loop (for spinning)
    pub fn start_tension_music(&mut self, ctx: &Context) {
        if !self.tension_playing {
            if let Some(sound) = &mut self.tension_music {
                let volume = self
                    .config
                    .effective_music_volume(0.25, self.music_ducked);
                sound.set_volume(volume);
                let _ = sound.play_detached(ctx);
                self.tension_playing = true;
            }
        }
    }

    /// Stop playing tension music
    pub fn stop_tension_music(&mut self, ctx: &Context) {
        if self.tension_playing {
            if let Some(sound) = &mut self.tension_music {
                let _ = sound.stop(ctx);
            }
            self.tension_playing = false;
        }
    }

    /// Start ambient audience murmur
    fn start_ambient_murmur(&mut self, ctx: &Context) {
        if !self.murmur_playing && self.config.ambient_audience_enabled {
            if let Some(sound) = &mut self.audience_murmur {
                let volume = self
                    .config
                    .effective_volume(self.config.audience_volume, 0.15);
                sound.set_volume(volume);
                let _ = sound.play_detached(ctx);
                self.murmur_playing = true;
            }
        }
    }

    /// Stop ambient audience murmur
    fn stop_ambient_murmur(&mut self, ctx: &Context) {
        if self.murmur_playing {
            if let Some(sound) = &mut self.audience_murmur {
                let _ = sound.stop(ctx);
            }
            self.murmur_playing = false;
        }
    }

    /// Start board spin ambient sound
    fn start_board_spin_ambient(&mut self, ctx: &Context) {
        if !self.spin_ambient_playing {
            if let Some(sound) = &mut self.board_spin_ambient {
                let volume = self.config.effective_volume(self.config.effects_volume, 0.4);
                sound.set_volume(volume);
                let _ = sound.play_detached(ctx);
                self.spin_ambient_playing = true;
            }
        }
    }

    /// Stop board spin ambient sound
    fn stop_board_spin_ambient(&mut self, ctx: &Context) {
        if self.spin_ambient_playing {
            if let Some(sound) = &mut self.board_spin_ambient {
                let _ = sound.stop(ctx);
            }
            self.spin_ambient_playing = false;
        }
    }

    /// Check if tension music is playing
    pub fn is_tension_playing(&self) -> bool {
        self.tension_playing
    }

    /// Check if board music is playing
    pub fn is_board_music_playing(&self) -> bool {
        self.board_music_playing
    }

    /// Check if ambient murmur is playing
    pub fn is_murmur_playing(&self) -> bool {
        self.murmur_playing
    }

    /// Toggle ambient audience murmur
    pub fn toggle_ambient_murmur(&mut self, ctx: &Context) {
        if self.murmur_playing {
            self.stop_ambient_murmur(ctx);
        } else {
            self.config.ambient_audience_enabled = true;
            self.start_ambient_murmur(ctx);
        }
    }
}

// ===============================================================================
// WAV FILE GENERATION
// ===============================================================================

/// Create a WAV file header
///
/// # Arguments
/// * `data_size` - Size of the audio data in bytes
///
/// # Returns
/// A 44-byte WAV header
fn create_wav_header(data_size: u32) -> Vec<u8> {
    let file_size = data_size + 36;
    let byte_rate = SAMPLE_RATE * 2; // 16-bit mono
    let block_align: u16 = 2; // 16-bit mono

    let mut header = Vec::with_capacity(44);

    // RIFF header
    header.extend_from_slice(b"RIFF");
    header.extend_from_slice(&file_size.to_le_bytes());
    header.extend_from_slice(b"WAVE");

    // fmt chunk
    header.extend_from_slice(b"fmt ");
    header.extend_from_slice(&16u32.to_le_bytes()); // Chunk size
    header.extend_from_slice(&1u16.to_le_bytes()); // PCM format
    header.extend_from_slice(&1u16.to_le_bytes()); // Mono
    header.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    header.extend_from_slice(&byte_rate.to_le_bytes());
    header.extend_from_slice(&block_align.to_le_bytes());
    header.extend_from_slice(&16u16.to_le_bytes()); // Bits per sample

    // data chunk
    header.extend_from_slice(b"data");
    header.extend_from_slice(&data_size.to_le_bytes());

    header
}

/// Convert floating-point samples to WAV bytes
///
/// # Arguments
/// * `samples` - Audio samples in range [-1.0, 1.0]
///
/// # Returns
/// Complete WAV file as byte vector
fn samples_to_wav(samples: &[f32]) -> Vec<u8> {
    let data_size = (samples.len() * 2) as u32;
    let mut wav = create_wav_header(data_size);

    for &sample in samples {
        let clamped = sample.clamp(-1.0, 1.0);
        let value = (clamped * 32767.0) as i16;
        wav.extend_from_slice(&value.to_le_bytes());
    }

    wav
}

// ===============================================================================
// SOUND GENERATION FUNCTIONS
// ===============================================================================

/// Generate a board tone with attack/decay envelope
fn generate_board_tone(freq: f32) -> Vec<u8> {
    let duration = 0.08;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let attack_time = 0.005;
        let decay_time = 0.075;
        let envelope = if t < attack_time {
            t / attack_time
        } else {
            let decay_progress = (t - attack_time) / decay_time;
            (1.0 - decay_progress).max(0.0)
        };

        let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
        let harmonic2 = 0.3 * (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin();
        let harmonic3 = 0.15 * (2.0 * std::f32::consts::PI * freq * 3.0 * t).sin();

        let sample = envelope * (fundamental + harmonic2 + harmonic3) * 0.5;
        samples.push(sample);
    }

    samples_to_wav(&samples)
}

/// Generate the Whammy foghorn sound
fn generate_whammy_sound() -> Vec<u8> {
    let duration = 0.8;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let progress = t / duration;

        let freq = if progress < 0.6 {
            300.0 - (150.0 * progress / 0.6)
        } else {
            150.0 + (50.0 * (progress - 0.6) / 0.4)
        };

        let envelope = if t < 0.05 {
            t / 0.05
        } else if t < 0.6 {
            1.0
        } else {
            1.0 - ((t - 0.6) / 0.2).min(1.0)
        };

        let phase = 2.0 * std::f32::consts::PI * freq * t;
        let square = if phase.sin() > 0.0 { 0.8 } else { -0.8 };
        let sine = 0.3 * phase.sin();
        let wobble = (20.0 * t).sin() * 0.1;

        let sample = envelope * (square + sine + wobble) * 0.6;
        samples.push(sample);
    }

    samples_to_wav(&samples)
}

/// Generate cash register sound
fn generate_cash_sound(big: bool) -> Vec<u8> {
    let duration = if big { 0.6 } else { 0.3 };
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    let notes = if big {
        vec![523.25, 659.25, 783.99, 1046.50, 1318.51]
    } else {
        vec![523.25, 659.25, 783.99]
    };

    let note_duration = duration / notes.len() as f32;

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let note_index = ((t / note_duration) as usize).min(notes.len() - 1);
        let note_t = t - (note_index as f32 * note_duration);

        let envelope = if note_t < 0.01 {
            note_t / 0.01
        } else {
            (1.0 - (note_t - 0.01) / (note_duration - 0.01))
                .max(0.0)
                .powf(2.0)
        };

        let freq = notes[note_index];
        let sample = envelope * (2.0 * std::f32::consts::PI * freq * t).sin() * 0.6;
        samples.push(sample);
    }

    let ching_start = num_samples - (SAMPLE_RATE as f32 * 0.1) as usize;
    #[allow(clippy::needless_range_loop)]
    for i in ching_start..num_samples {
        let t = (i - ching_start) as f32 / SAMPLE_RATE as f32;
        let envelope = (1.0 - t / 0.1).max(0.0).powf(3.0);
        let ching = envelope
            * 0.3
            * ((2.0 * std::f32::consts::PI * 2500.0 * t).sin()
                + 0.5 * (2.0 * std::f32::consts::PI * 3500.0 * t).sin());
        samples[i] = (samples[i] + ching).clamp(-1.0, 1.0);
    }

    samples_to_wav(&samples)
}

/// Generate prize fanfare
fn generate_prize_sound() -> Vec<u8> {
    let duration = 0.6;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    let notes = [392.0, 523.25, 659.25, 783.99];
    let timings = [0.0, 0.15, 0.3, 0.4];
    let durations = [0.2, 0.2, 0.2, 0.25];

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let mut sample = 0.0;

        for (note_idx, &note_freq) in notes.iter().enumerate() {
            let start = timings[note_idx];
            let dur = durations[note_idx];

            if t >= start && t < start + dur {
                let note_t = t - start;
                let attack = 0.02;
                let envelope = if note_t < attack {
                    note_t / attack
                } else {
                    (1.0 - (note_t - attack) / (dur - attack))
                        .max(0.0)
                        .powf(0.5)
                };

                let fundamental = (2.0 * std::f32::consts::PI * note_freq * t).sin();
                let h3 = 0.25 * (2.0 * std::f32::consts::PI * note_freq * 3.0 * t).sin();
                let h5 = 0.1 * (2.0 * std::f32::consts::PI * note_freq * 5.0 * t).sin();

                sample += envelope * (fundamental + h3 + h5) * 0.4;
            }
        }

        samples.push(sample.clamp(-1.0, 1.0));
    }

    samples_to_wav(&samples)
}

/// Generate special square magical sweep sound
fn generate_special_sound() -> Vec<u8> {
    let duration = 0.5;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let progress = t / duration;

        let freq = 440.0 + 800.0 * progress * progress;

        let envelope = if progress < 0.1 {
            progress / 0.1
        } else if progress > 0.8 {
            (1.0 - progress) / 0.2
        } else {
            1.0
        };

        let vibrato = (30.0 * t).sin() * 20.0;
        let main_tone = (2.0 * std::f32::consts::PI * (freq + vibrato) * t).sin();
        let shimmer = 0.3 * (2.0 * std::f32::consts::PI * (freq * 2.5) * t).sin();

        let sample = envelope * (main_tone + shimmer) * 0.5;
        samples.push(sample);
    }

    samples_to_wav(&samples)
}

/// Generate correct answer chime
fn generate_correct_sound() -> Vec<u8> {
    let duration = 0.4;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    let notes = [(0.0, 523.25), (0.15, 783.99)];

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let mut sample = 0.0;

        for &(start, freq) in &notes {
            if t >= start {
                let note_t = t - start;
                let note_dur = 0.25;

                if note_t < note_dur {
                    let envelope = (1.0 - note_t / note_dur).powf(2.0);
                    let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
                    let partial = 0.5 * (2.0 * std::f32::consts::PI * freq * 2.4 * t).sin();
                    sample += envelope * (fundamental + partial) * 0.4;
                }
            }
        }

        samples.push(sample.clamp(-1.0, 1.0));
    }

    samples_to_wav(&samples)
}

/// Generate wrong answer buzzer
fn generate_wrong_sound() -> Vec<u8> {
    let duration = 0.3;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;

        let envelope = if t < 0.02 {
            t / 0.02
        } else if t < 0.25 {
            1.0
        } else {
            (0.3 - t) / 0.05
        };

        let freq = 150.0;
        let phase = 2.0 * std::f32::consts::PI * freq * t;
        let square = if phase.sin() > 0.0 { 1.0 } else { -1.0 };
        let noise = (fastrand::f32() - 0.5) * 0.1;

        let sample = envelope * (square * 0.4 + noise);
        samples.push(sample.clamp(-1.0, 1.0));
    }

    samples_to_wav(&samples)
}

/// Generate sad trombone sound
fn generate_sad_trombone() -> Vec<u8> {
    let duration = 1.5;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    let notes = [
        (0.0, 0.3, 311.13),
        (0.35, 0.3, 277.18),
        (0.7, 0.3, 246.94),
        (1.05, 0.45, 207.65),
    ];

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let mut sample = 0.0;

        for &(start, dur, freq) in &notes {
            if t >= start && t < start + dur {
                let note_t = t - start;

                let envelope = if note_t < 0.05 {
                    note_t / 0.05
                } else if note_t < dur - 0.1 {
                    0.9 + 0.1 * (note_t * 8.0).sin()
                } else {
                    ((dur - note_t) / 0.1).max(0.0)
                };

                let pitch_bend = if note_t > dur - 0.1 {
                    1.0 - 0.05 * ((note_t - (dur - 0.1)) / 0.1)
                } else {
                    1.0
                };

                let f = freq * pitch_bend;
                let fundamental = (2.0 * std::f32::consts::PI * f * t).sin();
                let h2 = 0.4 * (2.0 * std::f32::consts::PI * f * 2.0 * t).sin();
                let h3 = 0.2 * (2.0 * std::f32::consts::PI * f * 3.0 * t).sin();

                sample += envelope * (fundamental + h2 + h3) * 0.35;
            }
        }

        samples.push(sample.clamp(-1.0, 1.0));
    }

    samples_to_wav(&samples)
}

/// Generate spin added bell sound
fn generate_spin_added_sound() -> Vec<u8> {
    let duration = 0.2;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let envelope = (1.0 - t / duration).powf(3.0);

        let freq = 1200.0;
        let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
        let partial = 0.4 * (2.0 * std::f32::consts::PI * freq * 2.3 * t).sin();

        let sample = envelope * (fundamental + partial) * 0.4;
        samples.push(sample);
    }

    samples_to_wav(&samples)
}

/// Generate buzz-in sound
fn generate_buzz_in_sound() -> Vec<u8> {
    let duration = 0.15;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;

        let envelope = if t < 0.01 {
            t / 0.01
        } else {
            (1.0 - (t - 0.01) / (duration - 0.01)).max(0.0)
        };

        let tone1 = (2.0 * std::f32::consts::PI * 800.0 * t).sin();
        let tone2 = 0.5 * (2.0 * std::f32::consts::PI * 1000.0 * t).sin();

        let sample = envelope * (tone1 + tone2) * 0.5;
        samples.push(sample);
    }

    samples_to_wav(&samples)
}

/// Generate winner fanfare
fn generate_winner_fanfare() -> Vec<u8> {
    let duration = 2.0;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    let notes = [
        (0.0, 0.2, 523.25),
        (0.2, 0.2, 659.25),
        (0.4, 0.2, 783.99),
        (0.6, 0.4, 1046.50),
        (1.0, 0.15, 783.99),
        (1.15, 0.15, 1046.50),
        (1.3, 0.7, 1318.51),
    ];

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let mut sample = 0.0;

        for &(start, dur, freq) in &notes {
            if t >= start && t < start + dur {
                let note_t = t - start;

                let envelope = if note_t < 0.03 {
                    note_t / 0.03
                } else {
                    (1.0 - (note_t - 0.03) / (dur - 0.03)).max(0.0).powf(0.3)
                };

                let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
                let h2 = 0.35 * (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin();
                let h3 = 0.2 * (2.0 * std::f32::consts::PI * freq * 3.0 * t).sin();
                let h4 = 0.1 * (2.0 * std::f32::consts::PI * freq * 4.0 * t).sin();

                sample += envelope * (fundamental + h2 + h3 + h4) * 0.3;
            }
        }

        samples.push(sample.clamp(-1.0, 1.0));
    }

    samples_to_wav(&samples)
}

/// Generate button click sound
fn generate_click_sound() -> Vec<u8> {
    let duration = 0.05;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let envelope = (1.0 - t / duration).powf(4.0);

        let noise = (fastrand::f32() - 0.5) * 0.6;
        let tone = 0.4 * (2.0 * std::f32::consts::PI * 1500.0 * t).sin();

        let sample = envelope * (noise + tone);
        samples.push(sample);
    }

    samples_to_wav(&samples)
}

/// Generate board stop "chunk" sound
fn generate_board_stop_sound() -> Vec<u8> {
    let duration = 0.15;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;

        let envelope = if t < 0.005 {
            t / 0.005
        } else {
            (1.0 - (t - 0.005) / (duration - 0.005)).powf(2.0).max(0.0)
        };

        let thump_freq = 80.0;
        let thump = (2.0 * std::f32::consts::PI * thump_freq * t).sin();

        let mid_freq = 200.0;
        let mid = 0.5 * (2.0 * std::f32::consts::PI * mid_freq * t).sin();

        let click_envelope = if t < 0.02 {
            (1.0 - t / 0.02).powf(4.0)
        } else {
            0.0
        };
        let click = click_envelope * 0.4 * (2.0 * std::f32::consts::PI * 1200.0 * t).sin();

        let noise = (fastrand::f32() - 0.5) * 0.15 * envelope;

        let sample = envelope * (thump + mid) * 0.5 + click + noise;
        samples.push(sample.clamp(-1.0, 1.0));
    }

    samples_to_wav(&samples)
}

/// Generate synthesized audience cheer
fn generate_audience_cheer() -> Vec<u8> {
    let duration = 1.2;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let progress = t / duration;

        let envelope = if progress < 0.1 {
            progress / 0.1
        } else if progress < 0.6 {
            1.0
        } else {
            (1.0 - (progress - 0.6) / 0.4).max(0.0)
        };

        let noise1 = fastrand::f32() - 0.5;
        let noise2 = fastrand::f32() - 0.5;

        let yeah_freq = 400.0 + 100.0 * (progress * 2.0 * std::f32::consts::PI).sin();
        let yeah = 0.2 * (2.0 * std::f32::consts::PI * yeah_freq * t).sin();

        let excitement =
            0.15 * (2.0 * std::f32::consts::PI * 800.0 * t).sin() * (1.0 + 0.3 * (t * 30.0).sin());

        let clap_phase = (t * 4.0) % 1.0;
        let clap = if clap_phase < 0.05 {
            (fastrand::f32() - 0.5) * 0.3 * (1.0 - clap_phase / 0.05)
        } else {
            0.0
        };

        let volume_mod = 0.8 + 0.2 * (t * 3.0).sin();
        let sample =
            envelope * volume_mod * (noise1 * 0.3 + noise2 * 0.2 + yeah + excitement + clap);

        samples.push(sample.clamp(-1.0, 1.0) * 0.6);
    }

    samples_to_wav(&samples)
}

/// Generate synthesized audience gasp
fn generate_audience_gasp() -> Vec<u8> {
    let duration = 0.8;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let progress = t / duration;

        let envelope = if progress < 0.15 {
            progress / 0.15
        } else if progress < 0.4 {
            1.0
        } else {
            (1.0 - (progress - 0.4) / 0.6).max(0.0).powf(0.5)
        };

        let breath_noise = fastrand::f32() - 0.5;

        let oh_freq = 350.0 - 100.0 * progress;
        let oh = 0.3 * (2.0 * std::f32::consts::PI * oh_freq * t).sin();

        let ah_freq = 280.0 - 60.0 * progress;
        let ah = 0.2 * (2.0 * std::f32::consts::PI * ah_freq * t).sin();

        let resonance = 0.1 * (2.0 * std::f32::consts::PI * 600.0 * t).sin() * (1.0 - progress);

        let sample = envelope * (breath_noise * 0.25 + oh + ah + resonance);

        samples.push(sample.clamp(-1.0, 1.0) * 0.5);
    }

    samples_to_wav(&samples)
}

/// Generate ambient audience murmur
fn generate_audience_murmur() -> Vec<u8> {
    let duration = 5.0;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;

        let noise1 = fastrand::f32() - 0.5;
        let noise2 = fastrand::f32() - 0.5;
        let noise3 = fastrand::f32() - 0.5;

        let voice1 = 0.05 * (2.0 * std::f32::consts::PI * 200.0 * t).sin();
        let voice2 = 0.04 * (2.0 * std::f32::consts::PI * 250.0 * t).sin();
        let voice3 = 0.03 * (2.0 * std::f32::consts::PI * 180.0 * t).sin();

        let mod1 = 0.5 + 0.5 * (2.0 * std::f32::consts::PI * 0.1 * t).sin();
        let mod2 = 0.5 + 0.5 * (2.0 * std::f32::consts::PI * 0.15 * t).sin();

        let crossfade = if t < 0.1 {
            t / 0.1
        } else if t > duration - 0.1 {
            (duration - t) / 0.1
        } else {
            1.0
        };

        let sample = crossfade
            * (noise1 * 0.15 * mod1
                + noise2 * 0.12 * mod2
                + noise3 * 0.08
                + voice1
                + voice2
                + voice3);

        samples.push(sample.clamp(-1.0, 1.0) * 0.3);
    }

    samples_to_wav(&samples)
}

/// Generate board spin ambient sound
fn generate_board_spin_ambient() -> Vec<u8> {
    let duration = 0.5;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;

        let motor_freq = 60.0;
        let motor = 0.3 * (2.0 * std::f32::consts::PI * motor_freq * t).sin();

        let h2 = 0.15 * (2.0 * std::f32::consts::PI * motor_freq * 2.0 * t).sin();
        let h3 = 0.08 * (2.0 * std::f32::consts::PI * motor_freq * 3.0 * t).sin();

        let whir_freq = 180.0;
        let whir = 0.1 * (2.0 * std::f32::consts::PI * whir_freq * t).sin();

        let noise = (fastrand::f32() - 0.5) * 0.05;

        let crossfade = if t < 0.02 {
            t / 0.02
        } else if t > duration - 0.02 {
            (duration - t) / 0.02
        } else {
            1.0
        };

        let sample = crossfade * (motor + h2 + h3 + whir + noise);
        samples.push(sample.clamp(-1.0, 1.0) * 0.5);
    }

    samples_to_wav(&samples)
}

/// Generate tension music loop for spinning
fn generate_tension_music() -> Vec<u8> {
    let duration = 2.0;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    let bpm = 140.0;
    let beat_duration = 60.0 / bpm;

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;

        let beat_phase = (t / beat_duration) % 1.0;
        let bass_envelope = if beat_phase < 0.3 {
            1.0 - beat_phase / 0.3
        } else {
            0.0
        };
        let bass_freq = 73.42;
        let bass = bass_envelope * 0.35 * (2.0 * std::f32::consts::PI * bass_freq * t).sin();

        let bass_oct =
            bass_envelope * 0.15 * (2.0 * std::f32::consts::PI * bass_freq * 2.0 * t).sin();

        let pad_volume = 0.12;
        let d4 = (2.0 * std::f32::consts::PI * 293.66 * t).sin();
        let f4 = (2.0 * std::f32::consts::PI * 349.23 * t).sin();
        let a4 = (2.0 * std::f32::consts::PI * 440.00 * t).sin();
        let d4_det = (2.0 * std::f32::consts::PI * 294.5 * t).sin();
        let pad = pad_volume * (d4 * 0.4 + f4 * 0.3 + a4 * 0.2 + d4_det * 0.1);

        let lfo = 0.5 + 0.5 * (2.0 * std::f32::consts::PI * 0.5 * t).sin();
        let pad_modulated = pad * (0.7 + 0.3 * lfo);

        let eighth_phase = (t / (beat_duration / 2.0)) % 1.0;
        let hat_envelope = if eighth_phase < 0.05 {
            1.0 - eighth_phase / 0.05
        } else {
            0.0
        };
        let hat_noise = fastrand::f32() - 0.5;
        let hat = hat_envelope * 0.08 * hat_noise;

        let sweep_freq = 200.0 + 300.0 * (t / duration);
        let sweep = 0.04
            * (2.0 * std::f32::consts::PI * sweep_freq * t).sin()
            * (0.5 + 0.5 * (t / duration));

        let bar_phase = (t / (beat_duration * 4.0)) % 1.0;
        let stab_envelope =
            if (bar_phase > 0.24 && bar_phase < 0.27) || (bar_phase > 0.74 && bar_phase < 0.77) {
                let local = if bar_phase > 0.5 {
                    bar_phase - 0.74
                } else {
                    bar_phase - 0.24
                };
                (1.0 - local / 0.03).max(0.0)
            } else {
                0.0
            };
        let stab = stab_envelope * 0.15 * (2.0 * std::f32::consts::PI * 466.16 * t).sin();

        let sample = bass + bass_oct + pad_modulated + hat + sweep + stab;

        let clipped = (sample * 1.2).tanh() * 0.8;

        samples.push(clipped.clamp(-1.0, 1.0));
    }

    samples_to_wav(&samples)
}
