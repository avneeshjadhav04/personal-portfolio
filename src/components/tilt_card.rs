//! TiltCard — port of `src/components/TiltCard.tsx`.
//!
//! 3D tilt + glare + scroll-velocity skew, driven by our motion core.

use std::cell::RefCell;
use std::rc::Rc;

use leptos::html::Div;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::hooks::use_scroll_velocity;
use crate::motion::motion_value::{use_motion_template, use_motion_value, use_spring, use_transform};
use crate::motion::spring::SPRING_SMOOTH;

#[component]
pub fn TiltCard(
    children: Children,
    class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let el_ref = NodeRef::<Div>::new();

    // Raw motion values (set by mouse-move handler).
    let rotate_x_raw = use_motion_value(0.0);
    let rotate_y_raw = use_motion_value(0.0);
    let glare_x = use_motion_value(50.0);
    let glare_y = use_motion_value(50.0);

    // Scroll-velocity-driven skew (clamped to ±2deg, scaled by 8).
    let velocity = use_scroll_velocity();
    let skew_x_raw = use_transform(velocity, |v| (v * 8.0).clamp(-2.0, 2.0));

    // Springs smoothing the raw values.
    let rotate_x = use_spring(rotate_x_raw, SPRING_SMOOTH);
    let rotate_y = use_spring(rotate_y_raw, SPRING_SMOOTH);
    let skew_x = use_spring(skew_x_raw, SPRING_SMOOTH);

    // Glare background template.
    let glare_bg = use_motion_template(move || {
        format!(
            "radial-gradient(circle at {}% {}%, rgba(255,255,255,0.08) 0%, transparent 50%)",
            glare_x.get(),
            glare_y.get()
        )
    });

    // rAF-coalesced mouse-move state (ports the TS pendingMove + rafId logic).
    let pending: Rc<RefCell<Option<(f64, f64)>>> = Rc::new(RefCell::new(None));
    let raf_id: Rc<RefCell<i32>> = Rc::new(RefCell::new(0));

    let on_move = move |e: leptos::ev::MouseEvent| {
        *pending.borrow_mut() = Some((e.client_x() as f64, e.client_y() as f64));
        if *raf_id.borrow() != 0 {
            return;
        }
        let pending = pending.clone();
        let raf_id = raf_id.clone();
        let el_ref = el_ref.clone();
        let raw_ax = rotate_x_raw;
        let raw_ay = rotate_y_raw;
        let gx = glare_x;
        let gy = glare_y;
        let cb = wasm_bindgen::closure::Closure::new(move || {
            *raf_id.borrow_mut() = 0;
            let Some((mx, my)) = *pending.borrow() else { return };
            let Some(el) = el_ref.get() else { return };
            let rect = el.get_bounding_client_rect();
            let x = mx - rect.left();
            let y = my - rect.top();
            let cx = rect.width() / 2.0;
            let cy = rect.height() / 2.0;
            raw_ax.set(((y - cy) / cy) * -6.0);
            raw_ay.set(((x - cx) / cx) * 6.0);
            gx.set((x / rect.width()) * 100.0);
            gy.set((y / rect.height()) * 100.0);
        });
        if let Some(w) = web_sys::window() {
            let id = w.request_animation_frame(cb.as_ref().unchecked_ref()).unwrap_or(0);
            *raf_id.borrow_mut() = id;
        }
        std::mem::forget(cb);
    };

    let on_leave = move |_| {
        rotate_x_raw.set(0.0);
        rotate_y_raw.set(0.0);
        glare_x.set(50.0);
        glare_y.set(50.0);
    };

    // Reactive transform string.
    let transform = Memo::new(move |_| {
        format!(
            "perspective(1000px) rotateX({}deg) rotateY({}deg) skewX({}deg)",
            rotate_x.get(),
            rotate_y.get(),
            skew_x.get()
        )
    });

    view! {
        <div
            node_ref=el_ref
            class=format!("relative group {class}")
            style:transform=move || transform.get()
            style:transform-style="preserve-3d"
            style:backface-visibility="hidden"
            style:will-change="transform"
            on:mousemove=on_move
            on:mouseleave=on_leave
        >
            {children()}
            <div
                class="absolute inset-0 rounded-2xl pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity duration-500"
                style:background=move || glare_bg.get()
            />
        </div>
    }
}