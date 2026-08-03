//! Smooth scroll — a Rust reimplementation of the subset of Lenis
//! (https://github.com/darkroomengineering/lenis) used by the portfolio.
//!
//! Native wheel/touch scrolling is left to the browser (no inertia
//! interception). A single `scroll` listener keeps the scroller state in sync,
//! and anchor-link navigation via `scrollTo(target, { offset, duration })`
//! animates the scroll position on the shared rAF loop with an exponential
//! ease-out — exactly like Lenis' `scrollTo`, but without a permanently
//! running animation loop. The loop is started on demand and stops as soon as
//! the animation completes.
//!
//! Anchor-link smooth scrolling via `scrollTo(target, { offset: -80, duration: 1.2 })`
//! is exposed through a context (port of `SmoothScrollContext`).

use std::sync::{Arc, Mutex};

use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

/// Context value: a `scrollTo` callback that drives the smooth scroller.
#[derive(Clone)]
pub struct SmoothScrollContextValue {
    pub scroll_to: Arc<dyn Fn(ScrollTarget, i32, f64) + Send + Sync>,
}

/// Where to scroll to. Mirrors Lenis's `scrollTo(target)` accepting a selector,
/// a pixel offset, or an element.
#[derive(Clone, Debug)]
pub enum ScrollTarget {
    Selector(String),
    Pixels(f64),
}

/// The provider component. Wraps the app so descendants can call
/// `use_smooth_scroll().scroll_to(...)`.
#[component]
pub fn SmoothScrollProvider(children: Children) -> impl IntoView {
    // Shared scroller state.
    let state = Arc::new(Mutex::new(ScrollerState {
        target: 0.0,
        current: 0.0,
        duration: 0.0,
        loop_active: false,
        ..Default::default()
    }));

    // Install the native scroll listener once.
    let state_for_listeners = state.clone();
    Effect::new(move || {
        let Some(w) = window() else { return };
        install_listener(&w, state_for_listeners.clone());
    });

    // The context-provided scroll_to.
    let state_for_cb = state.clone();
    let scroll_to: Arc<dyn Fn(ScrollTarget, i32, f64) + Send + Sync> = Arc::new(move |target, offset, duration| {
        let Some(w) = window() else { return };
        let target_y = match target {
            ScrollTarget::Pixels(y) => y,
            ScrollTarget::Selector(s) => {
                if let Some(doc) = w.document() {
                    if let Ok(Some(el)) = doc.query_selector(&s) {
                        let rect = el.get_bounding_client_rect();
                        rect.top() + w.scroll_y().unwrap_or(0.0) + offset as f64
                    } else {
                        return;
                    }
                } else {
                    return;
                }
            }
        };
        let mut s = state_for_cb.lock().unwrap();
        s.target = target_y.max(0.0);
        s.duration = duration;
        s.animate = true;
        s.anim_start = crate::utils::raf::now_seconds();
        s.from = s.current;
        // Start the shared rAF subscriber only if one isn't already running.
        if !s.loop_active {
            s.loop_active = true;
            let state_for_loop = state_for_cb.clone();
            let w_for_loop = w.clone();
            drop(s);
            crate::motion::raf_loop::spawn(move || {
                let ease = |t: f64| (1.001 - 2.0_f64.powf(-10.0 * t)).min(1.0);
                let now = crate::utils::raf::now_seconds();
                let mut s = state_for_loop.lock().unwrap();
                if !s.animate {
                    s.loop_active = false;
                    return true;
                }
                let elapsed = now - s.anim_start;
                let p = if s.duration > 0.0 { (elapsed / s.duration).min(1.0) } else { 1.0 };
                let eased = ease(p);
                let new = s.from + (s.target - s.from) * eased;
                s.current = new;
                let _ = w_for_loop.scroll_to_with_x_and_y(0.0, new);
                if p >= 1.0 {
                    s.animate = false;
                    s.current = s.target;
                    s.loop_active = false;
                    true
                } else {
                    false
                }
            });
        }
    });

    provide_context(SmoothScrollContextValue { scroll_to });

    children()
}

/// Hook to read the smooth-scroll context (port of `useSmoothScroll`).
pub fn use_smooth_scroll() -> Option<SmoothScrollContextValue> {
    use_context::<SmoothScrollContextValue>()
}

#[derive(Default)]
struct ScrollerState {
    target: f64,
    current: f64,
    animate: bool,
    anim_start: f64,
    from: f64,
    duration: f64,
    loop_active: bool,
}

fn install_listener(w: &web_sys::Window, state: Arc<Mutex<ScrollerState>>) {
    // On mount, sync current scroll position.
    {
        let mut s = state.lock().unwrap();
        s.current = w.scroll_y().unwrap_or(0.0);
        s.target = s.current;
    }

    // Native scroll handler: keep target/current in sync when not animating.
    let state_for_scroll = state.clone();
    let w_for_scroll = w.clone();
    let scroll = Closure::<dyn FnMut(web_sys::Event)>::new(move |_: web_sys::Event| {
        let mut s = state_for_scroll.lock().unwrap();
        if !s.animate {
            let cur = w_for_scroll.scroll_y().unwrap_or(0.0);
            s.current = cur;
            s.target = cur;
        }
    });
    let _ = w.add_event_listener_with_callback("scroll", scroll.as_ref().unchecked_ref());
    std::mem::forget(scroll);
}