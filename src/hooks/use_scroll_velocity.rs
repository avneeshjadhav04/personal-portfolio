//! `useScrollVelocity` — port of `src/hooks/useScrollVelocity.ts`.
//!
//! Returns a motion value tracking scroll velocity (pixels/ms). rAF-throttled.
//!
//! The scroll listener is installed exactly once (lazily, on first use) and
//! its value is shared through a module-level signal, so dozens of `TiltCard`s
//! don't each attach their own window listener.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

use crate::motion::motion_value::MotionValue;

thread_local! {
    /// The shared velocity signal, created and wired on first use.
    static SHARED: RefCell<Option<RwSignal<f64>>> = const { RefCell::new(None) };
}

/// Returns a [`MotionValue`] holding the current scroll velocity (px/ms).
pub fn use_scroll_velocity() -> MotionValue {
    SHARED.with(|cell| {
        if let Some(sig) = cell.borrow().as_ref() {
            return *sig;
        }
        let sig = RwSignal::new(0.0);
        install_shared_listener(sig);
        *cell.borrow_mut() = Some(sig);
        sig
    })
}

fn install_shared_listener(velocity: MotionValue) {
    let Some(w) = window() else { return };

    let last_y = Rc::new(Cell::new(w.scroll_y().unwrap_or(0.0)));
    let last_t = Rc::new(Cell::new(crate::utils::raf::now_seconds()));
    let raf_id = Rc::new(Cell::new(0i32));

    let w_for_cb = w.clone();
    let velocity_for_cb = velocity;
    let on_scroll = Closure::<dyn FnMut(web_sys::Event)>::new(move |_: web_sys::Event| {
        if raf_id.get() != 0 {
            return;
        }
        let velocity = velocity_for_cb;
        let w_inner = w_for_cb.clone();
        let w_for_req = w_for_cb.clone();
        let last_y = last_y.clone();
        let last_t = last_t.clone();
        let raf_id_for_cb = raf_id.clone();
        let cb = Closure::<dyn FnMut()>::new(move || {
            raf_id_for_cb.set(0);
            let now = crate::utils::raf::now_seconds();
            let cur_y = w_inner.scroll_y().unwrap_or(0.0);
            let dy = cur_y - last_y.get();
            let dt = now - last_t.get();
            if dt > 0.0 {
                velocity.set(dy / (dt * 1000.0));
            }
            last_y.set(cur_y);
            last_t.set(now);
        });
        let id = w_for_req.request_animation_frame(cb.as_ref().unchecked_ref()).unwrap_or(0);
        raf_id.set(id);
        std::mem::forget(cb);
    });
    let _ = w.add_event_listener_with_callback("scroll", on_scroll.as_ref().unchecked_ref());
    std::mem::forget(on_scroll);
}
