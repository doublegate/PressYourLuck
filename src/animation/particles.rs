//! # Particle System Module
//!
//! Provides visual particle effects for money scatter, explosions, stars, and confetti.

use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::mint::Vector2;
use ggez::{Context, GameResult};
use std::collections::HashMap;

// =============================================================================
// PARTICLE TYPE
// =============================================================================

/// Types of particle effects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParticleType {
    /// Money bills flying away
    MoneyScatter,

    /// Explosion sparks
    ExplosionSparks,

    /// Star burst effect
    StarBurst,

    /// Confetti celebration
    Confetti,

    /// Dust cloud
    Dust,

    /// Fire particles
    Fire,

    /// Smoke particles
    Smoke,

    /// Sparkle/shimmer
    Sparkle,

    /// Custom particle
    Custom,
}

impl Default for ParticleType {
    fn default() -> Self {
        Self::Sparkle
    }
}

// =============================================================================
// PARTICLE
// =============================================================================

/// A single particle
#[derive(Debug, Clone)]
pub struct Particle {
    /// Current position
    pub position: Vector2<f32>,

    /// Current velocity
    pub velocity: Vector2<f32>,

    /// Acceleration (e.g., gravity)
    pub acceleration: Vector2<f32>,

    /// Current rotation
    pub rotation: f32,

    /// Angular velocity
    pub angular_velocity: f32,

    /// Current scale
    pub scale: f32,

    /// Scale velocity
    pub scale_velocity: f32,

    /// Current color
    pub color: Color,

    /// Start color
    pub color_start: Color,

    /// End color
    pub color_end: Color,

    /// Current lifetime
    pub lifetime: f32,

    /// Maximum lifetime
    pub max_lifetime: f32,

    /// Whether particle is alive
    pub alive: bool,

    /// Particle type
    pub particle_type: ParticleType,

    /// Size at start
    pub size_start: f32,

    /// Size at end
    pub size_end: f32,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            position: Vector2 { x: 0.0, y: 0.0 },
            velocity: Vector2 { x: 0.0, y: 0.0 },
            acceleration: Vector2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
            angular_velocity: 0.0,
            scale: 1.0,
            scale_velocity: 0.0,
            color: Color::WHITE,
            color_start: Color::WHITE,
            color_end: Color::new(1.0, 1.0, 1.0, 0.0),
            lifetime: 0.0,
            max_lifetime: 1.0,
            alive: true,
            particle_type: ParticleType::Sparkle,
            size_start: 10.0,
            size_end: 2.0,
        }
    }
}

impl Particle {
    /// Create a new particle
    pub fn new(position: Vector2<f32>, velocity: Vector2<f32>, lifetime: f32) -> Self {
        Self {
            position,
            velocity,
            max_lifetime: lifetime,
            ..Default::default()
        }
    }

    /// Update particle
    pub fn update(&mut self, delta_time: f32) {
        if !self.alive {
            return;
        }

        // Update lifetime
        self.lifetime += delta_time;
        if self.lifetime >= self.max_lifetime {
            self.alive = false;
            return;
        }

        // Calculate normalized age (0-1)
        let t = self.lifetime / self.max_lifetime;

        // Update physics
        self.velocity.x += self.acceleration.x * delta_time;
        self.velocity.y += self.acceleration.y * delta_time;
        self.position.x += self.velocity.x * delta_time;
        self.position.y += self.velocity.y * delta_time;

        // Update rotation
        self.rotation += self.angular_velocity * delta_time;

        // Update scale
        self.scale += self.scale_velocity * delta_time;
        self.scale = (self.size_start + (self.size_end - self.size_start) * t).max(0.0);

        // Interpolate color
        self.color = Color::new(
            self.color_start.r + (self.color_end.r - self.color_start.r) * t,
            self.color_start.g + (self.color_end.g - self.color_start.g) * t,
            self.color_start.b + (self.color_end.b - self.color_start.b) * t,
            self.color_start.a + (self.color_end.a - self.color_start.a) * t,
        );
    }

    /// Get normalized age (0-1)
    pub fn age(&self) -> f32 {
        (self.lifetime / self.max_lifetime).clamp(0.0, 1.0)
    }

    /// Check if particle is visible
    pub fn is_visible(&self) -> bool {
        self.alive && self.color.a > 0.01 && self.scale > 0.1
    }
}

// =============================================================================
// PARTICLE EMITTER
// =============================================================================

/// Configuration for a particle emitter
#[derive(Debug, Clone)]
pub struct ParticleEmitter {
    /// Emitter position
    pub position: Vector2<f32>,

    /// Emission rate (particles per second)
    pub emission_rate: f32,

    /// Particle lifetime range (min, max)
    pub particle_lifetime: (f32, f32),

    /// Initial velocity range
    pub velocity_range: (Vector2<f32>, Vector2<f32>),

    /// Acceleration applied to particles
    pub acceleration: Vector2<f32>,

    /// Start color
    pub color_start: Color,

    /// End color
    pub color_end: Color,

    /// Size at start
    pub size_start: f32,

    /// Size at end
    pub size_end: f32,

    /// Angular velocity range
    pub angular_velocity_range: (f32, f32),

    /// Particle type
    pub particle_type: ParticleType,

    /// Whether emitter is active
    pub active: bool,

    /// Duration (None = infinite)
    pub duration: Option<f32>,

    /// Burst count (emit all at once)
    pub burst_count: Option<u32>,

    /// Accumulated emission time
    emission_accumulator: f32,

    /// Elapsed time
    elapsed: f32,

    /// Whether burst has fired
    burst_fired: bool,
}

impl Default for ParticleEmitter {
    fn default() -> Self {
        Self {
            position: Vector2 { x: 0.0, y: 0.0 },
            emission_rate: 10.0,
            particle_lifetime: (0.5, 1.5),
            velocity_range: (
                Vector2 { x: -50.0, y: -100.0 },
                Vector2 { x: 50.0, y: -50.0 },
            ),
            acceleration: Vector2 { x: 0.0, y: 200.0 }, // Gravity
            color_start: Color::WHITE,
            color_end: Color::new(1.0, 1.0, 1.0, 0.0),
            size_start: 10.0,
            size_end: 2.0,
            angular_velocity_range: (-3.0, 3.0),
            particle_type: ParticleType::Sparkle,
            active: true,
            duration: None,
            burst_count: None,
            emission_accumulator: 0.0,
            elapsed: 0.0,
            burst_fired: false,
        }
    }
}

impl ParticleEmitter {
    /// Create a new emitter at position
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vector2 { x, y },
            ..Default::default()
        }
    }

    /// Create money scatter emitter
    pub fn money_scatter(x: f32, y: f32) -> Self {
        Self {
            position: Vector2 { x, y },
            emission_rate: 0.0,
            burst_count: Some(30),
            particle_lifetime: (1.5, 2.5),
            velocity_range: (
                Vector2 { x: -200.0, y: -300.0 },
                Vector2 { x: 200.0, y: -100.0 },
            ),
            acceleration: Vector2 { x: 0.0, y: 400.0 },
            color_start: Color::new(0.2, 0.8, 0.2, 1.0), // Green money
            color_end: Color::new(0.2, 0.8, 0.2, 0.0),
            size_start: 20.0,
            size_end: 15.0,
            angular_velocity_range: (-5.0, 5.0),
            particle_type: ParticleType::MoneyScatter,
            active: true,
            ..Default::default()
        }
    }

    /// Create explosion sparks emitter
    pub fn explosion_sparks(x: f32, y: f32) -> Self {
        Self {
            position: Vector2 { x, y },
            emission_rate: 0.0,
            burst_count: Some(50),
            particle_lifetime: (0.3, 0.8),
            velocity_range: (
                Vector2 { x: -300.0, y: -300.0 },
                Vector2 { x: 300.0, y: 300.0 },
            ),
            acceleration: Vector2 { x: 0.0, y: 200.0 },
            color_start: Color::new(1.0, 0.8, 0.2, 1.0), // Orange
            color_end: Color::new(1.0, 0.2, 0.0, 0.0), // Red fade
            size_start: 8.0,
            size_end: 2.0,
            angular_velocity_range: (0.0, 0.0),
            particle_type: ParticleType::ExplosionSparks,
            active: true,
            ..Default::default()
        }
    }

    /// Create star burst emitter
    pub fn star_burst(x: f32, y: f32) -> Self {
        Self {
            position: Vector2 { x, y },
            emission_rate: 0.0,
            burst_count: Some(20),
            particle_lifetime: (0.5, 1.0),
            velocity_range: (
                Vector2 { x: -150.0, y: -150.0 },
                Vector2 { x: 150.0, y: 150.0 },
            ),
            acceleration: Vector2 { x: 0.0, y: 50.0 },
            color_start: Color::new(1.0, 1.0, 0.5, 1.0), // Yellow
            color_end: Color::new(1.0, 1.0, 1.0, 0.0),
            size_start: 15.0,
            size_end: 5.0,
            angular_velocity_range: (-2.0, 2.0),
            particle_type: ParticleType::StarBurst,
            active: true,
            ..Default::default()
        }
    }

    /// Create confetti emitter
    pub fn confetti(x: f32, y: f32) -> Self {
        Self {
            position: Vector2 { x, y },
            emission_rate: 50.0,
            burst_count: None,
            duration: Some(2.0),
            particle_lifetime: (2.0, 4.0),
            velocity_range: (
                Vector2 { x: -100.0, y: -200.0 },
                Vector2 { x: 100.0, y: -50.0 },
            ),
            acceleration: Vector2 { x: 0.0, y: 150.0 },
            color_start: Color::WHITE, // Random colors applied on spawn
            color_end: Color::new(1.0, 1.0, 1.0, 0.0),
            size_start: 12.0,
            size_end: 8.0,
            angular_velocity_range: (-8.0, 8.0),
            particle_type: ParticleType::Confetti,
            active: true,
            ..Default::default()
        }
    }

    /// Update emitter and spawn new particles
    pub fn update(&mut self, delta_time: f32, particles: &mut Vec<Particle>) {
        if !self.active {
            return;
        }

        self.elapsed += delta_time;

        // Check duration
        if let Some(dur) = self.duration {
            if self.elapsed >= dur {
                self.active = false;
                return;
            }
        }

        // Handle burst emission
        if let Some(count) = self.burst_count {
            if !self.burst_fired {
                for _ in 0..count {
                    particles.push(self.spawn_particle());
                }
                self.burst_fired = true;
            }
            return;
        }

        // Handle continuous emission
        self.emission_accumulator += delta_time;
        let emit_interval = 1.0 / self.emission_rate;

        while self.emission_accumulator >= emit_interval {
            self.emission_accumulator -= emit_interval;
            particles.push(self.spawn_particle());
        }
    }

    /// Spawn a single particle
    fn spawn_particle(&self) -> Particle {
        let lifetime = rand_range(self.particle_lifetime.0, self.particle_lifetime.1);

        let velocity = Vector2 {
            x: rand_range(self.velocity_range.0.x, self.velocity_range.1.x),
            y: rand_range(self.velocity_range.0.y, self.velocity_range.1.y),
        };

        let angular_velocity = rand_range(
            self.angular_velocity_range.0,
            self.angular_velocity_range.1,
        );

        // Random color for confetti
        let (color_start, color_end) = if self.particle_type == ParticleType::Confetti {
            let hue = fastrand::f32();
            let color = hsv_to_rgb(hue, 0.9, 1.0);
            (
                Color::new(color.0, color.1, color.2, 1.0),
                Color::new(color.0, color.1, color.2, 0.0),
            )
        } else {
            (self.color_start, self.color_end)
        };

        Particle {
            position: self.position,
            velocity,
            acceleration: self.acceleration,
            rotation: fastrand::f32() * std::f32::consts::TAU,
            angular_velocity,
            scale: self.size_start,
            scale_velocity: 0.0,
            color: color_start,
            color_start,
            color_end,
            lifetime: 0.0,
            max_lifetime: lifetime,
            alive: true,
            particle_type: self.particle_type,
            size_start: self.size_start,
            size_end: self.size_end,
        }
    }

    /// Check if emitter is finished
    pub fn is_finished(&self) -> bool {
        if !self.active {
            return true;
        }
        if self.burst_count.is_some() {
            return self.burst_fired;
        }
        if let Some(dur) = self.duration {
            return self.elapsed >= dur;
        }
        false
    }

    /// Reset emitter
    pub fn reset(&mut self) {
        self.emission_accumulator = 0.0;
        self.elapsed = 0.0;
        self.burst_fired = false;
        self.active = true;
    }
}

// =============================================================================
// PARTICLE SYSTEM
// =============================================================================

/// Manages particle emitters and rendering
pub struct ParticleSystem {
    /// All active particles
    particles: Vec<Particle>,

    /// Active emitters
    emitters: Vec<ParticleEmitter>,

    /// Particle pool max size
    max_particles: usize,

    /// Pre-built meshes for particle shapes
    particle_meshes: HashMap<ParticleType, Option<Mesh>>,
}

impl ParticleSystem {
    /// Create a new particle system
    pub fn new(max_particles: usize) -> Self {
        Self {
            particles: Vec::with_capacity(max_particles),
            emitters: Vec::new(),
            max_particles,
            particle_meshes: HashMap::new(),
        }
    }

    /// Initialize particle meshes
    pub fn init_meshes(&mut self, ctx: &mut Context) -> GameResult {
        // Sparkle (diamond shape)
        self.particle_meshes.insert(
            ParticleType::Sparkle,
            Mesh::new_polygon(
                ctx,
                DrawMode::fill(),
                &[
                    [0.0, -1.0],
                    [0.7, 0.0],
                    [0.0, 1.0],
                    [-0.7, 0.0],
                ],
                Color::WHITE,
            )
            .ok(),
        );

        // Star burst (star shape)
        let star_points = create_star_points(5, 1.0, 0.5);
        self.particle_meshes.insert(
            ParticleType::StarBurst,
            Mesh::new_polygon(ctx, DrawMode::fill(), &star_points, Color::WHITE).ok(),
        );

        // Money (rectangle)
        self.particle_meshes.insert(
            ParticleType::MoneyScatter,
            Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(-0.5, -0.25, 1.0, 0.5),
                Color::WHITE,
            )
            .ok(),
        );

        // Explosion sparks (circle)
        self.particle_meshes.insert(
            ParticleType::ExplosionSparks,
            Mesh::new_circle(ctx, DrawMode::fill(), [0.0, 0.0], 1.0, 0.1, Color::WHITE).ok(),
        );

        // Confetti (small rectangle)
        self.particle_meshes.insert(
            ParticleType::Confetti,
            Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(-0.5, -0.3, 1.0, 0.6),
                Color::WHITE,
            )
            .ok(),
        );

        Ok(())
    }

    /// Add an emitter
    pub fn add_emitter(&mut self, emitter: ParticleEmitter) {
        self.emitters.push(emitter);
    }

    /// Emit particles at position
    pub fn emit(&mut self, particle_type: ParticleType, x: f32, y: f32) {
        let emitter = match particle_type {
            ParticleType::MoneyScatter => ParticleEmitter::money_scatter(x, y),
            ParticleType::ExplosionSparks => ParticleEmitter::explosion_sparks(x, y),
            ParticleType::StarBurst => ParticleEmitter::star_burst(x, y),
            ParticleType::Confetti => ParticleEmitter::confetti(x, y),
            _ => {
                let mut e = ParticleEmitter::new(x, y);
                e.particle_type = particle_type;
                e.burst_count = Some(20);
                e
            }
        };
        self.emitters.push(emitter);
    }

    /// Update all particles and emitters
    pub fn update(&mut self, delta_time: f32) {
        // Update emitters
        for emitter in &mut self.emitters {
            emitter.update(delta_time, &mut self.particles);
        }

        // Remove finished emitters
        self.emitters.retain(|e| !e.is_finished() || e.burst_count.is_some());

        // Update particles
        for particle in &mut self.particles {
            particle.update(delta_time);
        }

        // Remove dead particles
        self.particles.retain(|p| p.alive);

        // Limit particle count
        if self.particles.len() > self.max_particles {
            let remove_count = self.particles.len() - self.max_particles;
            self.particles.drain(0..remove_count);
        }
    }

    /// Draw all particles
    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context) {
        for particle in &self.particles {
            if !particle.is_visible() {
                continue;
            }

            // Get mesh for particle type
            if let Some(Some(mesh)) = self.particle_meshes.get(&particle.particle_type) {
                let draw_param = DrawParam::default()
                    .dest([particle.position.x, particle.position.y])
                    .scale([particle.scale, particle.scale])
                    .rotation(particle.rotation)
                    .color(particle.color);

                canvas.draw(mesh, draw_param);
            } else {
                // Fallback: draw as circle
                if let Ok(circle) = Mesh::new_circle(
                    ctx,
                    DrawMode::fill(),
                    [particle.position.x, particle.position.y],
                    particle.scale,
                    0.1,
                    particle.color,
                ) {
                    canvas.draw(&circle, DrawParam::default());
                }
            }
        }
    }

    /// Clear all particles
    pub fn clear(&mut self) {
        self.particles.clear();
        self.emitters.clear();
    }

    /// Get particle count
    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }

    /// Get emitter count
    pub fn emitter_count(&self) -> usize {
        self.emitters.len()
    }
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new(1000)
    }
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/// Random float in range
fn rand_range(min: f32, max: f32) -> f32 {
    min + fastrand::f32() * (max - min)
}

/// Convert HSV to RGB
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

/// Create star points for mesh
fn create_star_points(points: u32, outer_radius: f32, inner_radius: f32) -> Vec<[f32; 2]> {
    let mut vertices = Vec::with_capacity((points * 2) as usize);
    let angle_step = std::f32::consts::PI / points as f32;

    for i in 0..(points * 2) {
        let radius = if i % 2 == 0 {
            outer_radius
        } else {
            inner_radius
        };
        let angle = i as f32 * angle_step - std::f32::consts::FRAC_PI_2;
        vertices.push([angle.cos() * radius, angle.sin() * radius]);
    }

    vertices
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_update() {
        let mut particle = Particle::new(
            Vector2 { x: 0.0, y: 0.0 },
            Vector2 { x: 100.0, y: 0.0 },
            1.0,
        );

        particle.update(0.5);

        assert!(particle.alive);
        assert!((particle.position.x - 50.0).abs() < 0.001);
        assert!((particle.age() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_particle_death() {
        let mut particle = Particle::new(
            Vector2 { x: 0.0, y: 0.0 },
            Vector2 { x: 0.0, y: 0.0 },
            1.0,
        );

        particle.update(1.5);
        assert!(!particle.alive);
    }

    #[test]
    fn test_emitter_burst() {
        let emitter = ParticleEmitter::explosion_sparks(100.0, 100.0);
        let mut particles = Vec::new();
        let mut e = emitter.clone();

        e.update(0.1, &mut particles);

        assert!(!particles.is_empty());
        assert!(e.burst_fired);
    }

    #[test]
    fn test_particle_system() {
        let mut system = ParticleSystem::new(100);
        system.emit(ParticleType::StarBurst, 100.0, 100.0);

        system.update(0.1);

        assert!(system.particle_count() > 0);
    }
}
