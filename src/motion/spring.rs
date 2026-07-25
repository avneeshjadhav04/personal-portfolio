//! Critically-damped / underdamped spring physics.
//!
//! Ports Framer Motion's spring model (mass = 1) used by `useSpring` in the TS
//! portfolio: a simple semi-implicit Euler integrator over a linear spring-damper.
//!
//! `springSmooth = { stiffness: 300, damping: 40, restDelta: 0.001 }`
//! `springSnappy = { stiffness: 200, damping: 25, restDelta: 0.001 }`

use crate::utils::raf::now_seconds;

/// Spring configuration matching Framer Motion's `Spring` options.
#[derive(Clone, Copy, Debug)]
pub struct SpringConfig {
    pub stiffness: f64,
    pub damping: f64,
    pub mass: f64,
    pub rest_delta: f64,
}

impl SpringConfig {
    pub const fn new(stiffness: f64, damping: f64) -> Self {
        Self { stiffness, damping, mass: 1.0, rest_delta: 0.001 }
    }

    pub const fn with_rest_delta(mut self, rest_delta: f64) -> Self {
        self.rest_delta = rest_delta;
        self
    }
}

/// Direct ports of `src/lib/motion.ts`.
pub const SPRING_SMOOTH: SpringConfig = SpringConfig::new(300.0, 40.0).with_rest_delta(0.001);
pub const SPRING_SNAPPY: SpringConfig = SpringConfig::new(200.0, 25.0).with_rest_delta(0.001);

/// A single spring instance tracking a scalar target.
#[derive(Clone, Debug)]
pub struct Spring {
    pub current: f64,
    pub velocity: f64,
    pub target: f64,
    pub config: SpringConfig,
}

impl Spring {
    pub fn new(initial: f64, config: SpringConfig) -> Self {
        Self { current: initial, velocity: 0.0, target: initial, config }
    }

    pub fn set_target(&mut self, target: f64) {
        self.target = target;
    }

    /// Advance the spring by `dt` seconds. Returns `true` if the spring has
    /// settled (within `rest_delta` of the target with low velocity).
    pub fn step(&mut self, dt: f64) -> bool {
        let Spring { current, velocity, target, config } = *self;
        let dx = target - current;
        // F = -k * (x - target) - c * v   (mass = 1)
        let force = config.stiffness * dx - config.damping * velocity;
        // Semi-implicit Euler: update velocity first, then position.
        let new_velocity = velocity + force * dt;
        let new_current = current + new_velocity * dt;
        self.velocity = new_velocity;
        self.current = new_current;

        if (new_current - target).abs() < config.rest_delta
            && new_velocity.abs() < config.rest_delta
        {
            self.current = target;
            self.velocity = 0.0;
            true
        } else {
            false
        }
    }
}

/// Advance a spring against wall-clock time, clamping `dt` to a sensible range
/// so a stalled tab doesn't cause a giant jump on resume.
pub fn step_wallclock(spring: &mut Spring, last_time: &mut f64) -> bool {
    let now = now_seconds();
    let mut dt = now - *last_time;
    *last_time = now;
    // Clamp: ignore negative (clock jumped backwards) and cap at ~1/30s so a
    // backgrounded tab doesn't teleport the spring.
    if dt < 0.0 {
        dt = 0.0;
    } else if dt > 1.0 / 30.0 {
        dt = 1.0 / 30.0;
    }
    spring.step(dt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spring_settles_at_target() {
        let mut s = Spring::new(0.0, SPRING_SMOOTH);
        s.set_target(100.0);
        for _ in 0..1000 {
            if s.step(1.0 / 60.0) {
                break;
            }
        }
        assert!((s.current - 100.0).abs() < 0.01, "current={}", s.current);
        assert!(s.velocity.abs() < 0.01);
    }

    #[test]
    fn spring_stays_put_when_already_at_target() {
        let mut s = Spring::new(50.0, SPRING_SMOOTH);
        let done = s.step(1.0 / 60.0);
        assert!(done, "should settle immediately");
        assert_eq!(s.current, 50.0);
    }
}