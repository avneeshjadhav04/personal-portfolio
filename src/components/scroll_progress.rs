//! Scroll progress bar — port of `src/components/ScrollProgress.tsx`.
//!
//! Reads scroll progress via a motion-value spring and writes it to a fixed
//! bar's `scaleX`. Uses our motion core instead of Framer Motion.

use leptos::prelude::*;
use web_sys::window;

use crate::motion::motion_value::{use_motion_value, use_spring};
use crate::motion::spring::SpringConfig;

/// `<ScrollProgress />` — fixed top bar showing scroll completion.
#[component]
pub fn ScrollProgress() -> impl IntoView {
    // 0..1 progress through the page.
    let progress = use_motion_value(0.0);
    // Spring smoothing matching Framer Motion's `useSpring(scrollYProgress, {
    // stiffness: 100, damping: 30, restDelta: 0.001 })`.
    let smoothed = use_spring(progress, SpringConfig::new(100.0, 30.0).with_rest_delta(0.001));

    // Update `progress` on scroll.
    let progress_for_effect = progress;
    Effect::new(move || {
        let Some(w) = window() else { return };
        let p = progress_for_effect;
        let closure = Closure::new(move |_: web_sys::Event| {
            let max = (w.scroll_height() - w.inner_height()).max(1.0);
            let v = (w.scroll_y() / max).clamp(0.0, 1.0);
            p.set(v);
        });
        let _ = w.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
        std::mem::forget(closure);
    });

    let scale_x = Memo::new(move |_| smoothed.get());

    view! {
        <div
            class="fixed top-0 left-0 right-0 h-[4px] z-[60] origin-left"
            style:transform=move || format!("scaleX({})", scale_x.get())
            style:background="linear-gradient(90deg, #7B61FF, #A78BFA)"
        />
    }
}