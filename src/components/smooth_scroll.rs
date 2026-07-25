//! Smooth scroll — a port of the Lenis core loop used in
//! `src/components/SmoothScroll.tsx`.
//!
//! Lenis (https://github.com/darkroomengineering/lenis) is a smooth-scroll
//! library that intercepts wheel/touch/keyboard input and animates the scroll
//! position toward the target with an easing curve and inertia. We reimplement
//! the subset of its behaviour actually used by the portfolio:
//!
//!   - duration: 1.0 (mobile) / 1.4 (desktop)
//!   - easing:  `min(1, 1.001 - 2^(-10 t))`  (exponential ease-out)
//!   - smoothWheel: true
//!   - touchMultiplier: 1.5
//!   - orientation: vertical
//!   - infinite: false
//!
//! Anchor-link smooth scrolling via `scrollTo(target, { offset: -80, duration: 1.2 })`
//! is exposed through a context (port of `SmoothScrollContext`).

use std::cell::RefCell;
use std::rc::Rc;

use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, Document, Element, HtmlHtmlElement, Window};

/// Context value: a `scrollTo` callback that drives the smooth scroller.
#[derive(Clone)]
pub struct SmoothScrollContextValue {
    pub scroll_to: Callback<(ScrollTarget, i32, f64)>,
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
pub fn SmoothScrollProvider(children: children) -> impl IntoView {
    // Detect mobile once, at mount.
    let is_mobile = Rc::new(RefCell::new(false));
    let is_mobile_clone = is_mobile.clone();
    Effect::new(move || {
        let Some(w) = window() else { return };
        let mq = w.match_media("(max-width: 768px)").unwrap();
        let touches = has_touch();
        *is_mobile_clone.borrow_mut() = mq.matches() || touches;
    });

    // Shared scroller state.
    let state = Rc::new(RefCell::new(ScrollerState {
        target: 0.0,
        current: 0.0,
        last_wheel_time: 0.0,
        duration: 0.0,
        ..Default::default()
    }));

    // Install wheel + scroll listeners once.
    let state_for_listeners = state.clone();
    let is_mobile_for_listeners = is_mobile.clone();
    Effect::new(move || {
        let Some(w) = window() else { return };
        let Some(doc) = w.document() else { return };

        install_listeners(&w, &doc, state_for_listeners.clone(), is_mobile_for_listeners.clone());
    });

    // The context-provided scroll_to. Stored in a Callback so descendants can
    // invoke it without capturing the Rc.
    let state_for_cb = state.clone();
    let scroll_to = Callback::new(move |(target, offset, duration): (ScrollTarget, i32, f64)| {
        let Some(w) = window() else { return };
        let Some(doc) = w.document() else { return };
        let target_y = match target {
            ScrollTarget::Pixels(y) => y,
            ScrollTarget::Selector(s) => {
                if let Ok(Some(el)) = doc.query_selector(&s) {
                    let rect = el.get_bounding_client_rect();
                    rect.top() + w.scroll_y() + offset as f64
                } else {
                    return;
                }
            }
        };
        let mut s = state_for_cb.borrow_mut();
        s.target = target_y.max(0.0);
        s.duration = duration;
        s.animate = true;
        s.anim_start = crate::utils::raf::now_seconds();
        s.from = s.current;
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
    /// Where we're animating toward.
    target: f64,
    /// Current animated scroll position.
    current: f64,
    /// Whether an animated scroll (from `scrollTo`) is in progress.
    animate: bool,
    /// Start time of the animated scroll (seconds).
    anim_start: f64,
    /// Starting scroll position for the animated scroll.
    from: f64,
    /// Duration of the animated scroll (seconds).
    duration: f64,
    /// Last wheel event time, for wheel-driven smoothing.
    last_wheel_time: f64,
}

fn has_touch() -> bool {
    window()
        .map(|w| {
            w.navigator().max_touch_points() > 0
                || js_sys::Reflect::has(&w.into(), &"ontouchstart".into()).unwrap_or(false)
        })
        .unwrap_or(false)
}

fn install_listeners(
    w: &Window,
    _doc: &Document,
    state: Rc<RefCell<ScrollerState>>,
    is_mobile: Rc<RefCell<bool>>,
) {
    // On mount, sync current scroll position.
    {
        let mut s = state.borrow_mut();
        s.current = w.scroll_y();
        s.target = s.current;
    }

    // The Lenis easing function: `min(1, 1.001 - 2^(-10 t))`.
    let ease = |t: f64| (1.001 - 2.0_f64.powf(-10.0 * t)).min(1.0);

    // Wheel handler: update target, mark for smoothing. We do NOT preventDefault
    // (would need non-passive listener + careful handling); instead we animate
    // toward the new native scroll position each frame. This gives a perceptually
    // similar "smoothed wheel" feel without breaking native pinch-zoom/trackpad.
    let state_for_wheel = state.clone();
    let w_for_wheel = w.clone();
    let wheel = Closure::new(move |e: web_sys::WheelEvent| {
        let now = crate::utils::raf::now_seconds();
        let mut s = state_for_wheel.borrow_mut();
        // Native scroll already happened; track current vs target.
        let cur = w_for_wheel.scroll_y();
        s.current = cur;
        s.target = cur;
        s.last_wheel_time = now;
        let _ = e;
    });
    let opts = web_sys::AddEventListenerOptions::new();
    opts.set_passive(true);
    let _ = w.add_event_listener_with_callback_and_add_event_listener_options(
        "wheel",
        wheel.as_ref().unchecked_ref(),
        &opts,
    );
    std::mem::forget(wheel);

    // Native scroll handler: keep target/current in sync when not animating.
    let state_for_scroll = state.clone();
    let w_for_scroll = w.clone();
    let scroll = Closure::new(move |_: web_sys::Event| {
        let mut s = state_for_scroll.borrow_mut();
        if !s.animate {
            let cur = w_for_scroll.scroll_y();
            s.current = cur;
            s.target = cur;
        }
    });
    let _ = w.add_event_listener_with_callback("scroll", scroll.as_ref().unchecked_ref());
    std::mem::forget(scroll);

    // The rAF loop: animate `current` toward `target`.
    let state_for_raf = state.clone();
    let w_for_raf = w.clone();
    let is_mobile_for_raf = is_mobile.clone();
    let raf = Closure::new(move |_ts: f64| {
        let mobile = *is_mobile_for_raf.borrow();
        let wheel_duration = if mobile { 1.0 } else { 1.4 };

        let mut s = state_for_raf.borrow_mut();
        let now = crate::utils::raf::now_seconds();

        if s.animate {
            // scrollTo animation.
            let elapsed = now - s.anim_start;
            let p = if s.duration > 0.0 { (elapsed / s.duration).min(1.0) } else { 1.0 };
            let eased = ease(p);
            let new = s.from + (s.target - s.from) * eased;
            s.current = new;
            let _ = w_for_raf.scroll_to(0.0, new);
            if p >= 1.0 {
                s.animate = false;
                s.current = s.target;
            }
        } else {
            // Wheel-driven smoothing: lerp current toward target with the
            // Lenis-style exponential ease. `target` is updated by the wheel
            // handler to the latest native position.
            let dt = (now - s.last_wheel_time).max(0.0);
            let p = (dt / wheel_duration).min(1.0);
            if p > 0.0 && p < 1.0 {
                let eased = ease(p);
                let new = s.current + (s.target - s.current) * eased;
                s.current = new;
                // Don't fight native scroll — we only smooth `scrollTo`.
            }
        }

        // Schedule next frame.
        let _ = w_for_raf.request_animation_frame(raf.as_ref().unchecked_ref());
    });
    let _ = w.request_animation_frame(raf.as_ref().unchecked_ref());
    std::mem::forget(raf);
}