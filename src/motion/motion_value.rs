//! Reactive motion primitives — ports of Framer Motion's `MotionValue`,
//! `useTransform`, `useMotionTemplate`, and `useSpring`.
//!
//! In Framer Motion a `MotionValue` is a small observable number/string that
//! animations write to and the DOM reads from. In Leptos the natural analogue
//! is a fine-grained signal, so `MotionValue<f64>` is a `RwSignal<f64>`.

use leptos::prelude::*;

use crate::motion::spring::SpringConfig;

/// A reactive scalar that animations drive. Equivalent to Framer Motion's
/// `MotionValue<number>`.
pub type MotionValue = RwSignal<f64>;

/// Create a new motion value, like `useMotionValue(initial)`.
pub fn use_motion_value(initial: f64) -> MotionValue {
    RwSignal::new(initial)
}

/// Derive a new motion value by mapping another through `f`. Equivalent to
/// `useTransform(source, f)`.
pub fn use_transform<F>(source: MotionValue, f: F) -> MotionValue
where
    F: Fn(f64) -> f64 + Copy + 'static,
{
    let derived = RwSignal::new(f(source.get()));
    Effect::new(move || {
        let v = source.get();
        derived.set(f(v));
    });
    derived
}

/// String derived from one or more motion values, for `useMotionTemplate`.
/// Implemented as a `Memo<String>` since templates produce formatted strings.
pub type MotionTemplate = Memo<String>;

/// Equivalent to `useMotionTemplate\`...${value}...\``. Accepts a closure so the
/// caller controls formatting (avoids needing a `format!`-style macro that
/// captures signals, which is awkward in Rust).
pub fn use_motion_template<F>(f: F) -> MotionTemplate
where
    F: Fn() -> String + Send + Sync + 'static,
{
    Memo::new(move |_| f())
}

/// A spring that continuously tracks a motion value and writes its smoothed
/// value into another. Equivalent to `useSpring(source, config)`.
///
/// Drives the shared rAF loop in [`crate::motion::raf_loop`].
pub fn use_spring(source: MotionValue, config: SpringConfig) -> MotionValue {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::motion::raf_loop::spawn;
    use crate::motion::spring::Spring;
    use crate::utils::raf::now_seconds;

    let initial = source.get();
    let spring_state = Rc::new(RefCell::new(Spring::new(initial, config)));
    let last_time = Rc::new(RefCell::new(now_seconds()));
    let out = RwSignal::new(initial);

    // Re-target the spring whenever the source changes.
    let spring_for_effect = spring_state.clone();
    Effect::new(move || {
        let v = source.get();
        spring_for_effect.borrow_mut().set_target(v);
    });

    let out_clone = out;
    let spring_for_tick = spring_state.clone();
    let last_for_tick = last_time.clone();
    spawn(move || {
        let now = now_seconds();
        let mut dt = now - *last_for_tick.borrow();
        *last_for_tick.borrow_mut() = now;
        // Clamp: ignore clock-jumps backwards, cap so a backgrounded tab doesn't
        // teleport the spring.
        if dt < 0.0 {
            dt = 0.0;
        } else if dt > 1.0 / 30.0 {
            dt = 1.0 / 30.0;
        }
        let settled = spring_for_tick.borrow_mut().step(dt);
        let current = spring_for_tick.borrow().current;
        out_clone.set(current);
        settled
    });

    out
}