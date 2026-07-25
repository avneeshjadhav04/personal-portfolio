//! `<AnimatePresence>` — port of Framer Motion's exit-then-unmount wrapper.
//!
//! When the wrapped content is removed, it first animates to its `exit`
//! variant, then unmounts. Used by the Preloader and the Navbar mobile menu.

use leptos::html::Div;
use leptos::prelude::*;
use web_sys::HtmlElement;

use crate::motion::easing::Easing;
use crate::motion::raf_loop::spawn;
use crate::motion::variants::Variant;
use crate::utils::raf::now_seconds;

/// `<AnimatePresence present=... exit=...>` wraps a single child. When
/// `present` becomes `false`, the child animates to `exit` then unmounts.
#[component]
pub fn AnimatePresence(
    present: Signal<bool>,
    exit: Variant,
    duration: Option<f64>,
    ease: Easing,
    children: ChildrenFn,
) -> impl IntoView {
    // Track whether we're animating out so we keep rendering during the exit.
    let exiting = RwSignal::new(false);
    let el_ref = NodeRef::<Div>::new();

    // Watch `present`; when it flips to false, run the exit animation then hide.
    let exit_variant = exit.clone();
    let exit_dur = duration.unwrap_or(exit.transition.duration.max(0.5));
    let exit_ease = ease;
    Effect::new(move || {
        if !present.get() {
            exiting.set(true);
            let el_ref = el_ref.clone();
            let exit_variant = exit_variant.clone();
            let exit_ease = exit_ease;
            let cb = Closure::<dyn FnMut()>::new(move || {
                let Some(el) = el_ref.get() else { return };
                let start = now_seconds();
                let from = Variant { opacity: Some(1.0), ..Variant::new() };
                let to = exit_variant.clone();
                let el_for_tick = el.clone();
                spawn(move || {
                    let elapsed = now_seconds() - start;
                    if elapsed >= exit_dur {
                        apply_variant(el_for_tick.as_ref(), &to);
                        exiting.set(false);
                        return true;
                    }
                    let p = (elapsed / exit_dur).clamp(0.0, 1.0);
                    let eased = exit_ease.ease(p);
                    let cur = Variant {
                        opacity: Some(1.0 + (to.opacity.unwrap_or(1.0) - 1.0) * eased),
                        ..from.clone()
                    };
                    apply_variant(el_for_tick.as_ref(), &cur);
                    if let Some(t) = to.transform_string() {
                        let el_html: &web_sys::HtmlElement = el_for_tick.as_ref();
                        let _ = el_html.style().set_property("transform", &t);
                    }
                    false
                });
            });
            if let Some(w) = web_sys::window() {
                let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    0,
                );
            }
            std::mem::forget(cb);
        }
    });

    // Render the child only while present OR exiting.
    let should_render = move || present.get() || exiting.get();
    view! {
        {move || {
            if should_render() {
                Some(view! { <div node_ref=el_ref>{children()}</div> })
            } else {
                None
            }
        }}
    }
}

fn apply_variant(el: &HtmlElement, v: &Variant) {
    let style = el.style();
    if let Some(t) = v.transform_string() {
        let _ = style.set_property("transform", &t);
    }
    if let Some(o) = v.opacity {
        let _ = style.set_property("opacity", &format!("{o}"));
    }
}