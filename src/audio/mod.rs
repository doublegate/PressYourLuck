//! # Audio Engine Module
//!
//! ## Overview
//! This module provides procedural audio synthesis for Press Your Luck,
//! generating all sounds at runtime without external audio files.
//!
//! ## Sound Design
//! The authentic 1983-1986 CBS game show featured distinctive audio cues:
//! - **Board Tones**: Musical sequence as light moves around the board
//! - **Whammy Sound**: Foghorn-style warning blast
//! - **Cash Register**: "Cha-ching" for cash wins
//! - **Prize Fanfare**: Celebratory horn flourish
//! - **Special Sound**: Magical sweep for special squares
//!
//! ## Technical Implementation
//! - Sample rate: 44,100 Hz (CD quality)
//! - Format: 16-bit PCM mono
//! - All sounds generated mathematically using waveform synthesis
//! - Attack/Decay/Sustain/Release (ADSR) envelopes for natural sound
//!
//! ## Board Tone Frequencies (Hz)
//! The authentic board tones follow a specific musical pattern:
//! ```text
//! D4, E4, G4, Bb4, D4, Ab4, F4, C4, Eb4, D4, B4, A4, C#4, E4, F#4, A4, D4, F4
//! ```

use ggez::audio::{SoundData, SoundSource, Source};
use ggez::{Context, GameResult};

use crate::game::AudioEvent;

// ===============================================================================
// CONSTANTS
// ===============================================================================

/// Audio sample rate (CD quality)
const SAMPLE_RATE: u32 = 44100;

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
// AUDIO ENGINE
// ===============================================================================

/// Main audio engine managing all game sounds
///
/// # Architecture
/// The engine pre-generates all sound effects on initialization,
/// storing them as ggez Source objects for instant playback.
///
/// # Sound Categories
/// 1. **Board Tones (18)**: One for each square position
/// 2. **Game Events**: Whammy, cash, prize, special, correct, wrong
/// 3. **UI Sounds**: Button clicks, transitions
#[derive(Default)]
pub struct AudioEngine {
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

    /// Audience cheer sound
    audience_cheer_sound: Option<Source>,

    /// Audience gasp sound
    audience_gasp_sound: Option<Source>,

    /// Tension music loop for spinning
    tension_music: Option<Source>,

    /// Whether tension music is currently playing
    tension_playing: bool,

    /// Whether audio has been initialized
    initialized: bool,
}

impl AudioEngine {
    /// Create a new audio engine
    ///
    /// # Note
    /// Sounds are generated and loaded synchronously.
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut engine = Self {
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
            audience_cheer_sound: None,
            audience_gasp_sound: None,
            tension_music: None,
            tension_playing: false,
            initialized: false,
        };

        // Generate all sounds
        engine.generate_sounds(ctx)?;
        engine.initialized = true;
        println!(
            "Audio engine initialized with {} board tones",
            engine.board_tones.len()
        );

        Ok(engine)
    }

    /// Generate all sound effects procedurally
    fn generate_sounds(&mut self, ctx: &mut Context) -> GameResult {
        // Helper to create a Source from WAV data
        let make_source = |ctx: &mut Context, wav_data: Vec<u8>| -> Option<Source> {
            let sound_data: SoundData = wav_data.into();
            Source::from_data(ctx, sound_data).ok()
        };

        // Generate board tones (18 unique frequencies)
        for &freq in BOARD_TONES.iter() {
            let wav_data = generate_board_tone(freq);
            self.board_tones.push(make_source(ctx, wav_data));
        }

        // Generate game event sounds
        self.whammy_sound = make_source(ctx, generate_whammy_sound());
        self.cash_sound = make_source(ctx, generate_cash_sound(false));
        self.big_cash_sound = make_source(ctx, generate_cash_sound(true));
        self.prize_sound = make_source(ctx, generate_prize_sound());
        self.special_sound = make_source(ctx, generate_special_sound());
        self.correct_sound = make_source(ctx, generate_correct_sound());
        self.wrong_sound = make_source(ctx, generate_wrong_sound());
        self.sad_trombone = make_source(ctx, generate_sad_trombone());
        self.spin_added_sound = make_source(ctx, generate_spin_added_sound());
        self.buzz_in_sound = make_source(ctx, generate_buzz_in_sound());
        self.winner_sound = make_source(ctx, generate_winner_fanfare());
        self.click_sound = make_source(ctx, generate_click_sound());
        self.board_stop_sound = make_source(ctx, generate_board_stop_sound());
        self.audience_cheer_sound = make_source(ctx, generate_audience_cheer());
        self.audience_gasp_sound = make_source(ctx, generate_audience_gasp());

        // Tension music with repeat
        if let Some(mut source) = make_source(ctx, generate_tension_music()) {
            source.set_repeat(true);
            self.tension_music = Some(source);
        }

        Ok(())
    }

    /// Update the audio engine
    ///
    /// # Arguments
    /// * `_ctx` - ggez context
    /// * `_delta_time` - Frame delta time (unused, for future features)
    pub fn update(&mut self, _ctx: &mut Context, _delta_time: f32) -> GameResult {
        Ok(())
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
            AudioEvent::AudienceCheer => self.play_audience_cheer(ctx),
            AudioEvent::AudienceGasp => self.play_audience_gasp(ctx),
            AudioEvent::StartTensionMusic => self.start_tension_music(ctx),
            AudioEvent::StopTensionMusic => self.stop_tension_music(ctx),
        }
    }

    /// Play a board tone for the given square index
    fn play_board_tone(&mut self, ctx: &Context, index: usize) {
        if index < self.board_tones.len() {
            if let Some(sound) = &mut self.board_tones[index] {
                sound.set_volume(0.6);
                let _ = sound.play_detached(ctx);
            }
        }
    }

    /// Play the Whammy foghorn sound
    fn play_whammy(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.whammy_sound {
            sound.set_volume(0.8);
            let _ = sound.play_detached(ctx);
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
            s.set_volume(0.7);
            let _ = s.play_detached(ctx);
        }
    }

    /// Play prize fanfare
    fn play_prize(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.prize_sound {
            sound.set_volume(0.7);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play special square sound
    fn play_special(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.special_sound {
            sound.set_volume(0.7);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play correct answer chime
    fn play_correct(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.correct_sound {
            sound.set_volume(0.6);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play wrong answer buzzer
    fn play_wrong(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.wrong_sound {
            sound.set_volume(0.6);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play sad trombone for elimination
    fn play_sad_trombone(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.sad_trombone {
            sound.set_volume(0.7);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play spin added bell
    fn play_spin_added(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.spin_added_sound {
            sound.set_volume(0.5);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play buzz-in sound
    fn play_buzz_in(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.buzz_in_sound {
            sound.set_volume(0.6);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play winner fanfare
    fn play_winner(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.winner_sound {
            sound.set_volume(0.8);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play button click sound
    pub fn play_button_click(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.click_sound {
            sound.set_volume(0.3);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play board stop mechanical "chunk" sound
    fn play_board_stop(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.board_stop_sound {
            sound.set_volume(0.7);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play audience cheer sound
    fn play_audience_cheer(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.audience_cheer_sound {
            sound.set_volume(0.4);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Play audience gasp sound
    fn play_audience_gasp(&mut self, ctx: &Context) {
        if let Some(sound) = &mut self.audience_gasp_sound {
            sound.set_volume(0.35);
            let _ = sound.play_detached(ctx);
        }
    }

    /// Start playing tension music loop (for spinning)
    pub fn start_tension_music(&mut self, ctx: &Context) {
        if !self.tension_playing {
            if let Some(sound) = &mut self.tension_music {
                sound.set_volume(0.25);
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
///
/// Creates a rich tone with harmonics for the distinctive board sound.
///
/// # Arguments
/// * `freq` - Base frequency in Hz
///
/// # Returns
/// WAV file bytes
fn generate_board_tone(freq: f32) -> Vec<u8> {
    let duration = 0.08; // 80ms per tone
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;

        // ADSR envelope: quick attack, short decay
        let attack_time = 0.005;
        let decay_time = 0.075;
        let envelope = if t < attack_time {
            t / attack_time
        } else {
            let decay_progress = (t - attack_time) / decay_time;
            (1.0 - decay_progress).max(0.0)
        };

        // Main tone with harmonics for richness
        let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
        let harmonic2 = 0.3 * (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin();
        let harmonic3 = 0.15 * (2.0 * std::f32::consts::PI * freq * 3.0 * t).sin();

        let sample = envelope * (fundamental + harmonic2 + harmonic3) * 0.5;
        samples.push(sample);
    }

    samples_to_wav(&samples)
}

/// Generate the Whammy foghorn sound
///
/// A descending sweep with distortion for that classic warning sound.
///
/// # Returns
/// WAV file bytes
fn generate_whammy_sound() -> Vec<u8> {
    let duration = 0.8;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let progress = t / duration;

        // Frequency sweep: 300 Hz down to 150 Hz, then back up briefly
        let freq = if progress < 0.6 {
            300.0 - (150.0 * progress / 0.6)
        } else {
            150.0 + (50.0 * (progress - 0.6) / 0.4)
        };

        // Envelope: attack, sustain, decay
        let envelope = if t < 0.05 {
            t / 0.05
        } else if t < 0.6 {
            1.0
        } else {
            1.0 - ((t - 0.6) / 0.2).min(1.0)
        };

        // Square wave with harmonics for that brassy quality
        let phase = 2.0 * std::f32::consts::PI * freq * t;
        let square = if phase.sin() > 0.0 { 0.8 } else { -0.8 };
        let sine = 0.3 * phase.sin();

        // Add some "wobble" for comedy effect
        let wobble = (20.0 * t).sin() * 0.1;

        let sample = envelope * (square + sine + wobble) * 0.6;
        samples.push(sample);
    }

    samples_to_wav(&samples)
}

/// Generate cash register sound
///
/// # Arguments
/// * `big` - If true, generate a more elaborate sound for big wins
///
/// # Returns
/// WAV file bytes
fn generate_cash_sound(big: bool) -> Vec<u8> {
    let duration = if big { 0.6 } else { 0.3 };
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    // C major arpeggio frequencies
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

        // Quick attack, fast decay for "ping" quality
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

    // Add a "ching" metallic sound at the end
    let ching_start = num_samples - (SAMPLE_RATE as f32 * 0.1) as usize;
    #[allow(clippy::needless_range_loop)]
    for i in ching_start..num_samples {
        let t = (i - ching_start) as f32 / SAMPLE_RATE as f32;
        let envelope = (1.0 - t / 0.1).max(0.0).powf(3.0);

        // High frequency metallic ping
        let ching = envelope
            * 0.3
            * ((2.0 * std::f32::consts::PI * 2500.0 * t).sin()
                + 0.5 * (2.0 * std::f32::consts::PI * 3500.0 * t).sin());

        samples[i] = (samples[i] + ching).clamp(-1.0, 1.0);
    }

    samples_to_wav(&samples)
}

/// Generate prize fanfare
///
/// Triumphant horn flourish for winning prizes.
///
/// # Returns
/// WAV file bytes
fn generate_prize_sound() -> Vec<u8> {
    let duration = 0.6;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    // Fanfare notes: G4, C5, E5, G5
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

                // Brass-like envelope
                let attack = 0.02;
                let envelope = if note_t < attack {
                    note_t / attack
                } else {
                    (1.0 - (note_t - attack) / (dur - attack))
                        .max(0.0)
                        .powf(0.5)
                };

                // Brass timbre: fundamental + odd harmonics
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
///
/// Rising frequency sweep with shimmer.
///
/// # Returns
/// WAV file bytes
fn generate_special_sound() -> Vec<u8> {
    let duration = 0.5;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let progress = t / duration;

        // Rising frequency sweep
        let freq = 440.0 + 800.0 * progress * progress;

        // Fade in and out
        let envelope = if progress < 0.1 {
            progress / 0.1
        } else if progress > 0.8 {
            (1.0 - progress) / 0.2
        } else {
            1.0
        };

        // Main tone with vibrato
        let vibrato = (30.0 * t).sin() * 20.0;
        let main_tone = (2.0 * std::f32::consts::PI * (freq + vibrato) * t).sin();

        // Shimmer effect (high frequency modulation)
        let shimmer = 0.3 * (2.0 * std::f32::consts::PI * (freq * 2.5) * t).sin();

        let sample = envelope * (main_tone + shimmer) * 0.5;
        samples.push(sample);
    }

    samples_to_wav(&samples)
}

/// Generate correct answer chime
///
/// Two-note ascending chime.
///
/// # Returns
/// WAV file bytes
fn generate_correct_sound() -> Vec<u8> {
    let duration = 0.4;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    // C5 to G5 (ascending perfect fifth)
    let notes = [(0.0, 523.25), (0.15, 783.99)];

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let mut sample = 0.0;

        for &(start, freq) in &notes {
            if t >= start {
                let note_t = t - start;
                let note_dur = 0.25;

                if note_t < note_dur {
                    // Bell-like envelope
                    let envelope = (1.0 - note_t / note_dur).powf(2.0);

                    // Bell timbre
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
///
/// Low frequency buzz.
///
/// # Returns
/// WAV file bytes
fn generate_wrong_sound() -> Vec<u8> {
    let duration = 0.3;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;

        // Envelope: quick attack, sustain, quick release
        let envelope = if t < 0.02 {
            t / 0.02
        } else if t < 0.25 {
            1.0
        } else {
            (0.3 - t) / 0.05
        };

        // Low buzz (150 Hz square wave)
        let freq = 150.0;
        let phase = 2.0 * std::f32::consts::PI * freq * t;
        let square = if phase.sin() > 0.0 { 1.0 } else { -1.0 };

        // Add some grit
        let noise = (fastrand::f32() - 0.5) * 0.1;

        let sample = envelope * (square * 0.4 + noise);
        samples.push(sample.clamp(-1.0, 1.0));
    }

    samples_to_wav(&samples)
}

/// Generate sad trombone sound
///
/// Descending "wah wah wah wahhh" for elimination.
///
/// # Returns
/// WAV file bytes
fn generate_sad_trombone() -> Vec<u8> {
    let duration = 1.5;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    // Four descending notes
    let notes = [
        (0.0, 0.3, 311.13),   // Eb4
        (0.35, 0.3, 277.18),  // C#4
        (0.7, 0.3, 246.94),   // B3
        (1.05, 0.45, 207.65), // G#3 (held longer)
    ];

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let mut sample = 0.0;

        for &(start, dur, freq) in &notes {
            if t >= start && t < start + dur {
                let note_t = t - start;

                // Trombone envelope with slight vibrato
                let envelope = if note_t < 0.05 {
                    note_t / 0.05
                } else if note_t < dur - 0.1 {
                    0.9 + 0.1 * (note_t * 8.0).sin()
                } else {
                    ((dur - note_t) / 0.1).max(0.0)
                };

                // Slight pitch bend down at end
                let pitch_bend = if note_t > dur - 0.1 {
                    1.0 - 0.05 * ((note_t - (dur - 0.1)) / 0.1)
                } else {
                    1.0
                };

                // Brass timbre
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
///
/// Quick celebratory ding.
///
/// # Returns
/// WAV file bytes
fn generate_spin_added_sound() -> Vec<u8> {
    let duration = 0.2;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;

        // Quick decay
        let envelope = (1.0 - t / duration).powf(3.0);

        // High bell tone
        let freq = 1200.0;
        let fundamental = (2.0 * std::f32::consts::PI * freq * t).sin();
        let partial = 0.4 * (2.0 * std::f32::consts::PI * freq * 2.3 * t).sin();

        let sample = envelope * (fundamental + partial) * 0.4;
        samples.push(sample);
    }

    samples_to_wav(&samples)
}

/// Generate buzz-in sound
///
/// Quick attention-getting beep.
///
/// # Returns
/// WAV file bytes
fn generate_buzz_in_sound() -> Vec<u8> {
    let duration = 0.15;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;

        // Sharp attack, quick decay
        let envelope = if t < 0.01 {
            t / 0.01
        } else {
            (1.0 - (t - 0.01) / (duration - 0.01)).max(0.0)
        };

        // Two-tone buzz
        let tone1 = (2.0 * std::f32::consts::PI * 800.0 * t).sin();
        let tone2 = 0.5 * (2.0 * std::f32::consts::PI * 1000.0 * t).sin();

        let sample = envelope * (tone1 + tone2) * 0.5;
        samples.push(sample);
    }

    samples_to_wav(&samples)
}

/// Generate winner fanfare
///
/// Triumphant celebration music.
///
/// # Returns
/// WAV file bytes
fn generate_winner_fanfare() -> Vec<u8> {
    let duration = 2.0;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    // Victory melody
    let notes = [
        (0.0, 0.2, 523.25),    // C5
        (0.2, 0.2, 659.25),    // E5
        (0.4, 0.2, 783.99),    // G5
        (0.6, 0.4, 1046.50),   // C6 (held)
        (1.0, 0.15, 783.99),   // G5
        (1.15, 0.15, 1046.50), // C6
        (1.3, 0.7, 1318.51),   // E6 (finale)
    ];

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let mut sample = 0.0;

        for &(start, dur, freq) in &notes {
            if t >= start && t < start + dur {
                let note_t = t - start;

                // Triumphant brass envelope
                let envelope = if note_t < 0.03 {
                    note_t / 0.03
                } else {
                    (1.0 - (note_t - 0.03) / (dur - 0.03)).max(0.0).powf(0.3)
                };

                // Rich brass timbre
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
///
/// Subtle UI feedback.
///
/// # Returns
/// WAV file bytes
fn generate_click_sound() -> Vec<u8> {
    let duration = 0.05;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;

        // Very quick decay
        let envelope = (1.0 - t / duration).powf(4.0);

        // Click is mostly noise with a slight tone
        let noise = (fastrand::f32() - 0.5) * 0.6;
        let tone = 0.4 * (2.0 * std::f32::consts::PI * 1500.0 * t).sin();

        let sample = envelope * (noise + tone);
        samples.push(sample);
    }

    samples_to_wav(&samples)
}

/// Generate board stop "chunk" sound
///
/// Authentic mechanical sound when the board stops spinning.
/// Combines a low thump with metallic click for that 1980s game show feel.
///
/// # Returns
/// WAV file bytes
fn generate_board_stop_sound() -> Vec<u8> {
    let duration = 0.15;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;

        // Quick attack, fast decay for impact
        let envelope = if t < 0.005 {
            t / 0.005
        } else {
            (1.0 - (t - 0.005) / (duration - 0.005)).powf(2.0).max(0.0)
        };

        // Low thump component (mechanical relay sound)
        let thump_freq = 80.0;
        let thump = (2.0 * std::f32::consts::PI * thump_freq * t).sin();

        // Mid-range impact
        let mid_freq = 200.0;
        let mid = 0.5 * (2.0 * std::f32::consts::PI * mid_freq * t).sin();

        // High metallic click
        let click_envelope = if t < 0.02 {
            (1.0 - t / 0.02).powf(4.0)
        } else {
            0.0
        };
        let click = click_envelope * 0.4 * (2.0 * std::f32::consts::PI * 1200.0 * t).sin();

        // Add subtle mechanical noise
        let noise = (fastrand::f32() - 0.5) * 0.15 * envelope;

        let sample = envelope * (thump + mid) * 0.5 + click + noise;
        samples.push(sample.clamp(-1.0, 1.0));
    }

    samples_to_wav(&samples)
}

/// Generate synthesized audience cheer
///
/// Creates a crowd cheering effect using layered noise and pitched tones.
/// Used for big wins and exciting moments.
///
/// # Returns
/// WAV file bytes
fn generate_audience_cheer() -> Vec<u8> {
    let duration = 1.2;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let progress = t / duration;

        // Envelope: quick rise, sustain, gradual fade
        let envelope = if progress < 0.1 {
            progress / 0.1
        } else if progress < 0.6 {
            1.0
        } else {
            (1.0 - (progress - 0.6) / 0.4).max(0.0)
        };

        // Multi-layer crowd simulation
        // Layer 1: Filtered noise (crowd murmur)
        let noise1 = fastrand::f32() - 0.5;
        let noise2 = fastrand::f32() - 0.5;

        // Layer 2: Pitched "yeah!" components
        let yeah_freq = 400.0 + 100.0 * (progress * 2.0 * std::f32::consts::PI).sin();
        let yeah = 0.2 * (2.0 * std::f32::consts::PI * yeah_freq * t).sin();

        // Layer 3: Higher frequency excitement
        let excitement =
            0.15 * (2.0 * std::f32::consts::PI * 800.0 * t).sin() * (1.0 + 0.3 * (t * 30.0).sin());

        // Layer 4: Clapping rhythm simulation
        let clap_phase = (t * 4.0) % 1.0; // ~4 claps per second
        let clap = if clap_phase < 0.05 {
            (fastrand::f32() - 0.5) * 0.3 * (1.0 - clap_phase / 0.05)
        } else {
            0.0
        };

        // Mix layers with volume modulation for realism
        let volume_mod = 0.8 + 0.2 * (t * 3.0).sin();
        let sample =
            envelope * volume_mod * (noise1 * 0.3 + noise2 * 0.2 + yeah + excitement + clap);

        samples.push(sample.clamp(-1.0, 1.0) * 0.6);
    }

    samples_to_wav(&samples)
}

/// Generate synthesized audience gasp
///
/// Creates a collective gasp sound for Whammy hits and close calls.
///
/// # Returns
/// WAV file bytes
fn generate_audience_gasp() -> Vec<u8> {
    let duration = 0.8;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let progress = t / duration;

        // Gasp envelope: quick inhale, sustain, fade
        let envelope = if progress < 0.15 {
            progress / 0.15
        } else if progress < 0.4 {
            1.0
        } else {
            (1.0 - (progress - 0.4) / 0.6).max(0.0).powf(0.5)
        };

        // Inward breath sound (aspirated noise)
        let breath_noise = fastrand::f32() - 0.5;

        // Pitched "ohhh" component (descending)
        let oh_freq = 350.0 - 100.0 * progress;
        let oh = 0.3 * (2.0 * std::f32::consts::PI * oh_freq * t).sin();

        // Secondary "ahhh" component
        let ah_freq = 280.0 - 60.0 * progress;
        let ah = 0.2 * (2.0 * std::f32::consts::PI * ah_freq * t).sin();

        // Subtle resonance
        let resonance = 0.1 * (2.0 * std::f32::consts::PI * 600.0 * t).sin() * (1.0 - progress);

        let sample = envelope * (breath_noise * 0.25 + oh + ah + resonance);

        samples.push(sample.clamp(-1.0, 1.0) * 0.5);
    }

    samples_to_wav(&samples)
}

/// Generate tension music loop for spinning
///
/// Creates an authentic tension building loop similar to the show's
/// suspenseful backing music during board spins.
///
/// # Returns
/// WAV file bytes (loops seamlessly)
fn generate_tension_music() -> Vec<u8> {
    // 2-second loop for seamless playback
    let duration = 2.0;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    // Musical parameters for authentic 80s game show feel
    // Base: driving bass pulse + synth pad + rhythmic elements
    let bpm = 140.0;
    let beat_duration = 60.0 / bpm;

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;

        // --- DRIVING BASS PULSE (D minor foundation) ---
        // D2 = 73.42 Hz, pulsing on quarter notes
        let beat_phase = (t / beat_duration) % 1.0;
        let bass_envelope = if beat_phase < 0.3 {
            1.0 - beat_phase / 0.3
        } else {
            0.0
        };
        let bass_freq = 73.42;
        let bass = bass_envelope * 0.35 * (2.0 * std::f32::consts::PI * bass_freq * t).sin();

        // Octave bass for fullness (D3)
        let bass_oct =
            bass_envelope * 0.15 * (2.0 * std::f32::consts::PI * bass_freq * 2.0 * t).sin();

        // --- SYNTH PAD (Suspenseful chord) ---
        // Dm chord: D4 (293.66), F4 (349.23), A4 (440.00)
        let pad_volume = 0.12;
        let d4 = (2.0 * std::f32::consts::PI * 293.66 * t).sin();
        let f4 = (2.0 * std::f32::consts::PI * 349.23 * t).sin();
        let a4 = (2.0 * std::f32::consts::PI * 440.00 * t).sin();
        // Add slight detuning for thickness
        let d4_det = (2.0 * std::f32::consts::PI * 294.5 * t).sin();
        let pad = pad_volume * (d4 * 0.4 + f4 * 0.3 + a4 * 0.2 + d4_det * 0.1);

        // Slow LFO modulation on pad for movement
        let lfo = 0.5 + 0.5 * (2.0 * std::f32::consts::PI * 0.5 * t).sin();
        let pad_modulated = pad * (0.7 + 0.3 * lfo);

        // --- RHYTHMIC HIGH HAT (Eighth notes) ---
        let eighth_phase = (t / (beat_duration / 2.0)) % 1.0;
        let hat_envelope = if eighth_phase < 0.05 {
            1.0 - eighth_phase / 0.05
        } else {
            0.0
        };
        // White noise for hi-hat
        let hat_noise = fastrand::f32() - 0.5;
        let hat = hat_envelope * 0.08 * hat_noise;

        // --- TENSION RISER (Rising tone) ---
        // Subtle rising sweep throughout the loop
        let sweep_freq = 200.0 + 300.0 * (t / duration);
        let sweep = 0.04
            * (2.0 * std::f32::consts::PI * sweep_freq * t).sin()
            * (0.5 + 0.5 * (t / duration));

        // --- SYNTH STAB (On beat 2 and 4) ---
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
        // Bb4 (466.16 Hz) for tension
        let stab = stab_envelope * 0.15 * (2.0 * std::f32::consts::PI * 466.16 * t).sin();

        // Combine all elements
        let sample = bass + bass_oct + pad_modulated + hat + sweep + stab;

        // Soft clip for warmth
        let clipped = (sample * 1.2).tanh() * 0.8;

        samples.push(clipped.clamp(-1.0, 1.0));
    }

    samples_to_wav(&samples)
}
