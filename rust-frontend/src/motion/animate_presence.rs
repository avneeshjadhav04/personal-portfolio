//! `<AnimatePresence>` — port of Framer Motion's exit-then-unmount wrapper.
//!
//! When the wrapped content is removed, it first animates to its `exit`
//! variant, then unmounts. Used by the Preloader and the Navbar mobile menu.

use std::cell::RefCell;
use std::rc::Rc;

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

use crate::motion::easing::Easing;
use crate::motion::raf_loop::spawn;
use crate::motion::variants::Variant;
use crate::utils::raf::now_seconds;

/// Props for `<AnimatePresence>`.
pub struct AnimatePresenceProps {
    /// Whether the child is currently present. When this flips to `false`,
    /// the exit animation runs and the child is then unmounted.
    pub present: bool,
    /// The variant to animate to on exit.
    pub exit: Variant,
    /// Duration of the exit animation (seconds). Defaults to the exit variant's
    /// transition duration, or 0.5s.
    pub duration: Option<f64>,
    /// Easing for the exit animation.
    pub ease: Easing,
    pub children: Children,
}

/// `<AnimatePresence present=... exit=...>` wraps a single child. When
/// `present` becomes `false`, the child animates to `exit` then unmounts.
#[component]
pub fn AnimatePresence(props: AnimatePresenceProps) -> impl IntoView {
    let AnimatePresenceProps { present, exit, duration, ease, children } = props;

    // Track whether we're animating out so we keep rendering during the exit.
    let exiting = RwSignal::new(false);
    let el_ref = NodeRef::<Element>::new();

    // Watch `present`; when it flips to false, run the exit animation then hide.
    let exit_variant = exit.clone();
    let exit_dur = duration.unwrap_or(exit.transition.duration.max(0.5));
    let exit_ease = ease;
    Effect::new(move || {
        if !present {
            exiting.set(true);
            let Some(el) = el_ref.get() else { return };
            let style = el.unchecked_ref::<Element>().style();
            let start = now_seconds();
            // Snapshot the current "from" values from the element's computed
            // style is complex; instead, animate from the exit variant's
            // inverse (1.0 opacity, no transform) toward the exit variant.
            let from = Variant { opacity: Some(1.0), ..Variant::new() };
            let to = exit_variant.clone();
            let el_for_tick = el.clone();
            spawn(move || {
                let elapsed = now_seconds() - start;
                if elapsed >= exit_dur {
                    // Final frame + signal done.
                    apply_variant(&el_for_tick, &to);
                    exiting.set(false);
                    return true;
                }
                let p = (elapsed / exit_dur).clamp(0.0, 1.0);
                let eased = exit_ease.ease(p);
                let cur = Variant {
                    opacity: Some(1.0 + (to.opacity.unwrap_or(1.0) - 1.0) * eased),
                    ..from.clone()
                };
                apply_variant(&el_for_tick, &cur);
                // Also apply transform fields if the exit variant sets them.
                if let Some(t) = to.transform_string() {
                    let _ = style.set_property("transform", &t);
                }
                false
            });
        }
    });

    // Render the child only while present OR exiting.
    let should_render = move || present || exiting.get();
    let children_view = children();
    view! {
        {move || {
            if should_render() {
                Some(view! { <div node_ref=el_ref>{children_view.clone()}</div> })
            } else {
                None
            }
        }}
    }
}

fn apply_variant(el: &Element, v: &Variant) {
    let style = el.unchecked_ref::<Element>().style();
    if let Some(t) = v.transform_string() {
        let _ = style.set_property("transform", &t);
    }
    if let Some(o) = v.opacity {
        let _ = style.set_property("opacity", &format!("{o}"));
    }
}