//! `prefers-reduced-motion` detection for the JS-driven motion core.
//!
//! The CSS layer already shortens animations, but the hand-rolled rAF loops
//! would still run every frame unless the motion core opts out too. This caches
//! the media-query result so gating is one `RefCell` read per check.

use std::cell::Cell;

use web_sys::window;

thread_local! {
    static CACHED: Cell<Option<bool>> = const { Cell::new(None) };
}

/// Whether the user has requested reduced motion. Cached after the first call.
pub fn reduced_motion() -> bool {
    CACHED.with(|c| {
        if let Some(v) = c.get() {
            return v;
        }
        let v = window()
            .and_then(|w| w.match_media("(prefers-reduced-motion: reduce)").ok().flatten())
            .map(|mq| mq.matches())
            .unwrap_or(false);
        c.set(Some(v));
        v
    })
}
