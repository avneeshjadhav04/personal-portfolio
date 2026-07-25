//! Cubic-Bézier easing curves, equivalent to CSS `cubic-bezier(x1, y1, x2, y2)`.
//!
//! Matches Framer Motion's `cubic-bezier` easing and the canonical curves used
//! across the original TS portfolio in `src/lib/motion.ts`.
//!
//! Endpoints are fixed at P0 = (0, 0) and P3 = (1, 1); only the two middle control
//! points are parameterised, exactly like the CSS function.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CubicBezier {
    // Bezier coefficients for the x-curve: B(t) = a t^3 + b t^2 + c t
    ax: f64,
    bx: f64,
    cx: f64,
    // Bezier coefficients for the y-curve.
    ay: f64,
    by: f64,
    cy: f64,
}

impl CubicBezier {
    pub const fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        // For P0 = 0, P1 = p1, P2 = p2, P3 = 1:
        //   B(t) = 3 p1 t (1-t)^2 + 3 p2 t^2 (1-t) + t^3
        // Expanded into a t^3 + b t^2 + c t form:
        //   a = 1 - 3 p2 + 3 p1,  b = 3 p2 - 6 p1,  c = 3 p1
        Self {
            ax: 1.0 - 3.0 * x2 + 3.0 * x1,
            bx: 3.0 * x2 - 6.0 * x1,
            cx: 3.0 * x1,
            ay: 1.0 - 3.0 * y2 + 3.0 * y1,
            by: 3.0 * y2 - 6.0 * y1,
            cy: 3.0 * y1,
        }
    }

    #[inline]
    fn sample(a: f64, b: f64, c: f64, t: f64) -> f64 {
        ((a * t + b) * t + c) * t
    }

    #[inline]
    fn sample_derivative(a: f64, b: f64, c: f64, t: f64) -> f64 {
        (3.0 * a * t + 2.0 * b) * t + c
    }

    /// Solve for the parameter `t` such that `sample_x(t) == x`, using
    /// Newton-Raphson first (fast) then bisection as a fallback (robust).
    fn solve_t_for_x(&self, x: f64) -> f64 {
        let mut t = x;
        for _ in 0..8 {
            let x_err = Self::sample(self.ax, self.bx, self.cx, t) - x;
            if x_err.abs() < 1e-6 {
                return t;
            }
            let d = Self::sample_derivative(self.ax, self.bx, self.cx, t);
            if d.abs() < 1e-6 {
                break;
            }
            t -= x_err / d;
        }

        // Bisection fallback for the rare cases Newton diverges.
        let mut lo = 0.0_f64;
        let mut hi = 1.0_f64;
        t = x;
        for _ in 0..30 {
            let x_val = Self::sample(self.ax, self.bx, self.cx, t);
            if (x_val - x).abs() < 1e-6 {
                return t;
            }
            if x_val < x {
                lo = t;
            } else {
                hi = t;
            }
            t = (lo + hi) / 2.0;
            if hi - lo < 1e-7 {
                return t;
            }
        }
        t
    }

    /// Given a linear progress value `p` in `[0, 1]`, return the eased value
    /// in `[0, 1]`.
    pub fn ease(&self, p: f64) -> f64 {
        if p <= 0.0 {
            return 0.0;
        }
        if p >= 1.0 {
            return 1.0;
        }
        let t = self.solve_t_for_x(p);
        Self::sample(self.ay, self.by, self.cy, t)
    }
}

/// A type-erased easing curve. Framer Motion accepts either a named ease or a
/// 4-tuple cubic-bezier; the portfolio only ever uses the latter, so this is a
/// thin wrapper around [`CubicBezier`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Easing {
    Bezier(CubicBezier),
    Linear,
}

impl Easing {
    pub const fn bezier(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self::Bezier(CubicBezier::new(x1, y1, x2, y2))
    }

    pub fn ease(&self, p: f64) -> f64 {
        match self {
            Self::Bezier(b) => b.ease(p),
            Self::Linear => p.clamp(0.0, 1.0),
        }
    }
}

/// Canonical easing curves — direct ports of the constants in
/// `src/lib/motion.ts`. Keep these in one place so motion feels consistent.
pub const EASE_OUT_EXPO: Easing = Easing::bezier(0.16, 1.0, 0.3, 1.0);
pub const EASE_STANDARD: Easing = Easing::bezier(0.4, 0.0, 0.2, 1.0);
pub const EASE_SMOOTH: Easing = Easing::bezier(0.22, 1.0, 0.36, 1.0);
pub const EASE_IN_OUT: Easing = Easing::bezier(0.65, 0.0, 0.35, 1.0);

/// "easeOut" used by a couple of inline transitions (Navbar mobile menu, etc.).
pub const EASE_OUT: Easing = Easing::bezier(0.0, 0.0, 0.58, 1.0);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bezier_endpoints_are_0_and_1() {
        let e = EASE_OUT_EXPO;
        assert!((e.ease(0.0) - 0.0).abs() < 1e-9);
        assert!((e.ease(1.0) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn bezier_is_monotonic_for_portfolio_curves() {
        for &e in &[EASE_OUT_EXPO, EASE_STANDARD, EASE_SMOOTH, EASE_IN_OUT] {
            let mut last = -0.1;
            for i in 1..100 {
                let p = i as f64 / 100.0;
                let v = e.ease(p);
                assert!(v >= last, "non-monotonic at {p}");
                last = v;
            }
        }
    }

    #[test]
    fn bezier_linear_matches_identity() {
        let lin = Easing::bezier(0.0, 0.0, 1.0, 1.0);
        for i in 0..=10 {
            let p = i as f64 / 10.0;
            assert!((lin.ease(p) - p).abs() < 1e-6);
        }
    }
}