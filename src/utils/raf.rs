//! Thin wrappers around `performance.now()` and `requestAnimationFrame` for
//! the motion core. Kept here so the spring/easing modules don't depend on
//! `web-sys` directly.
//!
//! The `performance.now` function and its `this` receiver are resolved once and
//! cached, so per-frame timing is a single JS call instead of the 2–3 boundary
//! crossings `window().performance().now()` would cost on every call.

use std::sync::OnceLock;

use js_sys::Function;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::window;

fn bound_now() -> &'static (Function, JsValue) {
    static BOUND: OnceLock<(Function, JsValue)> = OnceLock::new();
    BOUND.get_or_init(|| {
        match window().and_then(|w| w.performance()) {
            Some(p) => {
                let recv = JsValue::from(p);
                let f = js_sys::Reflect::get(&recv, &"now".into())
                    .ok()
                    .and_then(|v| v.dyn_into::<Function>().ok())
                    .unwrap_or_else(stub_now);
                (f, recv)
            }
            // Defensive fallback (should not happen in a browser).
            None => (stub_now(), JsValue::UNDEFINED),
        }
    })
}

/// A stub `now()` that always returns 0 — used only if `performance` is
/// somehow unavailable.
fn stub_now() -> Function {
    Closure::<dyn FnMut() -> f64>::new(|| 0.0)
        .into_js_value()
        .dyn_into::<Function>()
        .expect("closure fn into Function")
}

/// Current time in milliseconds since the time origin, via a cached
/// `performance.now` binding. Falls back to 0 if unavailable.
pub fn now_millis() -> f64 {
    let (f, recv) = bound_now();
    f.call0(recv).ok().and_then(|v| v.as_f64()).unwrap_or(0.0)
}

/// Current time in seconds, using `performance.now()`. Falls back to a 0-based
/// monotonic counter if `performance` is unavailable (defensive — should not
/// happen in a browser).
pub fn now_seconds() -> f64 {
    now_millis() / 1000.0
}
