//! A single shared `requestAnimationFrame` loop for all spring-driven motion
//! values, registered lazily on first subscriber.
//!
//! Rationale: Framer Motion runs one rAF per active animation; that's wasteful
//! when TiltCard + Hero + ScrollProgress + section reveals can all be active at
//! once. A single loop with a Vec of callbacks is cheaper and keeps the
//! WASM→JS bridge quiet.

use std::cell::RefCell;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

type TickFn = Box<dyn FnMut() -> bool + 'static>;

thread_local! {
    /// Callbacks registered for the current frame. Each returns `true` when
    /// its animation is done and can be dropped.
    static SUBSCRIBERS: RefCell<Vec<TickFn>> = const { RefCell::new(Vec::new()) };

    /// The installed rAF handle (0 means not running).
    static RAF_HANDLE: std::cell::Cell<i32> = const { std::cell::Cell::new(0) };

    /// The rAF callback closure, kept alive for the lifetime of the loop.
    static RAF_CLOSURE: RefCell<Option<Closure<dyn FnMut(f64)>>> = const { RefCell::new(None) };
}

/// Register a per-frame callback. The callback owns its animation state and
/// returns `true` when finished (so the loop drops it).
pub fn spawn<F>(f: F)
where
    F: FnMut() -> bool + 'static,
{
    SUBSCRIBERS.with(|s| s.borrow_mut().push(Box::new(f)));
    ensure_running();
}

fn ensure_running() {
    if RAF_HANDLE.with(|h| h.get()) != 0 {
        return;
    }

    // Install the closure once, on first use.
    if RAF_CLOSURE.with(|c| c.borrow().is_none()) {
        let closure = Closure::new(|_ts: f64| tick());
        RAF_CLOSURE.with(|c| *c.borrow_mut() = Some(closure));
    }

    schedule_next();
}

fn schedule_next() {
    if let Some(w) = window() {
        RAF_CLOSURE.with(|c| {
            if let Some(cl) = c.borrow().as_ref() {
                let cb = cl.as_ref().unchecked_ref::<js_sys::Function>();
                let h = w.request_animation_frame(cb).unwrap_or(0);
                RAF_HANDLE.with(|hcell| hcell.set(h));
            }
        });
    }
}

fn tick() {
    // Run every subscriber, drop those that report completion.
    SUBSCRIBERS.with(|s| {
        let mut subs = s.borrow_mut();
        let mut i = 0;
        while i < subs.len() {
            if subs[i]() {
                subs.swap_remove(i);
            } else {
                i += 1;
            }
        }
    });

    let empty = SUBSCRIBERS.with(|s| s.borrow().is_empty());

    if empty {
        // No more subscribers — stop the loop.
        let h = RAF_HANDLE.with(|h| h.replace(0));
        if h != 0 {
            if let Some(w) = window() {
                w.cancel_animation_frame(h);
            }
        }
        return;
    }

    // Schedule next frame.
    schedule_next();
}