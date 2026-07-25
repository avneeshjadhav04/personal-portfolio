//! Scroll progress bar — port of `src/components/ScrollProgress.tsx`.
//!
//! Reads scroll progress via a motion-value spring and writes it to a fixed
//! bar's `scaleX`. Uses our motion core instead of Framer Motion.

use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

use crate::motion::motion_value::{use_motion_value, use_spring};
use crate::motion::spring::SpringConfig;

/// `<ScrollProgress />` — fixed top bar showing scroll completion.
#[component]
pub fn ScrollProgress() -> impl IntoView {
    let progress = use_motion_value(0.0);
    let smoothed = use_spring(progress, SpringConfig::new(100.0, 30.0).with_rest_delta(0.001));

    let progress_for_effect = progress;
    Effect::new(move || {
        let Some(w) = window() else { return };
        let p = progress_for_effect;
        let w_for_closure = w.clone();
        let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |_: web_sys::Event| {
            let doc_el = w_for_closure.document().and_then(|d| d.document_element());
            let scroll_h = doc_el.map(|e| e.scroll_height() as f64).unwrap_or(0.0);
            let inner_h = w_for_closure.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(0.0);
            let max = (scroll_h - inner_h).max(1.0);
            let v = (w_for_closure.scroll_y().unwrap_or(0.0) / max).clamp(0.0, 1.0);
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