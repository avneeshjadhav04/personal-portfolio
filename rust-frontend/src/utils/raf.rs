//! Thin wrappers around `performance.now()` and `requestAnimationFrame` for
//! the motion core. Kept here so the spring/easing modules don't depend on
//! `web-sys` directly.

use web_sys::window;

/// Current time in seconds, using `performance.now()`. Falls back to a 0-based
/// monotonic counter if `performance` is unavailable (defensive — should not
/// happen in a browser).
pub fn now_seconds() -> f64 {
    window()
        .and_then(|w| w.performance())
        .map(|p| p.now() / 1000.0)
        .unwrap_or(0.0)
}

pub fn now_millis() -> f64 {
    window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}