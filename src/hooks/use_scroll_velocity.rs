//! `useScrollVelocity` — port of `src/hooks/useScrollVelocity.ts`.
//!
//! Returns a motion value tracking scroll velocity (pixels/ms). rAF-throttled.

use leptos::prelude::*;
use web_sys::window;

use crate::motion::motion_value::use_motion_value;
use crate::motion::motion_value::MotionValue;

/// Returns a [`MotionValue`] holding the current scroll velocity (px/ms).
pub fn use_scroll_velocity() -> MotionValue {
    let velocity = use_motion_value(0.0);

    let v = velocity;
    Effect::new(move || {
        let Some(w) = window() else { return };
        let last_y = std::rc::Rc::new(std::cell::Cell::new(w.scroll_y()));
        let last_t = std::rc::Rc::new(std::cell::Cell::new(crate::utils::raf::now_seconds()));
        let raf_id = std::rc::Rc::new(std::cell::Cell::new(0i32));

        let v_inner = v;
        let last_y_c = last_y.clone();
        let last_t_c = last_t.clone();
        let raf_id_c = raf_id.clone();
        let w_c = w.clone();
        let on_scroll = Closure::new(move |_: web_sys::Event| {
            if raf_id_c.get() != 0 {
                return;
            }
            let v_inner = v_inner;
            let last_y = last_y_c.clone();
            let last_t = last_t_c.clone();
            let raf_id = raf_id_c.clone();
            let w = w_c.clone();
            let cb = Closure::new(move || {
                raf_id.set(0);
                let now = crate::utils::raf::now_seconds();
                let cur_y = w.scroll_y();
                let dy = cur_y - last_y.get();
                let dt = now - last_t.get();
                if dt > 0.0 {
                    v_inner.set(dy / (dt * 1000.0));
                }
                last_y.set(cur_y);
                last_t.set(now);
            });
            let id = w.request_animation_frame(cb.as_ref().unchecked_ref()).unwrap_or(0);
            raf_id.set(id);
            std::mem::forget(cb);
        });
        let _ = w.add_event_listener_with_callback("scroll", on_scroll.as_ref().unchecked_ref());
        std::mem::forget(on_scroll);
    });

    velocity
}