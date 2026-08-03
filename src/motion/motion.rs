//! The `<Motion>` component — a port of Framer Motion's `<motion.div>`.
//!
//! Supports a subset of Framer Motion's API sufficient for this portfolio:
//!   - `variants` (named map), `initial`, `animate`, `while_in_view`
//!   - `transition` (per-element override)
//!   - `class`, `id`, `style` (static inline styles)
//!   - parent-driven stagger via [`StaggerContext`]
//!
//! Animations run on the shared rAF loop in [`crate::motion::raf_loop`].

use std::cell::RefCell;
use std::rc::Rc;

use leptos::html::Div;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{IntersectionObserver, IntersectionObserverInit};

use crate::motion::raf_loop::spawn;
use crate::motion::variants::{Transition, Variant, Variants, VIEWPORT_MARGIN, VIEWPORT_ONCE};
use crate::utils::raf::now_seconds;

/// Context a parent `<Motion>` provides to coordinate staggered children.
#[derive(Clone, Copy, Debug)]
pub struct StaggerContext {
    pub next_index: RwSignal<usize>,
    pub stagger: f64,
    pub delay_children: f64,
    pub parent_start: f64,
}

/// Provide a fresh stagger context for children. Returns the context to
/// `provide_context` with in the parent's body.
pub fn provide_stagger(stagger: f64, delay_children: f64) -> StaggerContext {
    StaggerContext {
        next_index: RwSignal::new(0),
        stagger,
        delay_children,
        parent_start: now_seconds(),
    }
}

/// Per-instance animation state held across rAF ticks.
struct AnimState {
    current: Variant,
    target: Variant,
    start: f64,
    transition: Transition,
    done: bool,
}

impl AnimState {
    fn new(from: Variant, to: Variant, transition: Transition) -> Self {
        Self {
            current: from,
            target: to,
            start: now_seconds(),
            transition,
            done: transition.duration <= 0.0,
        }
    }

    /// Advance the animation. Returns `true` when complete.
    fn step(&mut self) -> bool {
        if self.done {
            return true;
        }
        let now = now_seconds();
        let elapsed = now - self.start - self.transition.delay;
        if elapsed < 0.0 {
            return false;
        }
        let p = if self.transition.duration <= 0.0 {
            1.0
        } else {
            (elapsed / self.transition.duration).clamp(0.0, 1.0)
        };
        let eased = self.transition.ease.ease(p);

        let lerp = |from: Option<f64>, to: Option<f64>| -> Option<f64> {
            match (from, to) {
                (Some(a), Some(b)) => Some(a + (b - a) * eased),
                (None, Some(b)) => Some(b),
                _ => None,
            }
        };

        let prev = self.current.clone();
        self.current = Variant {
            opacity: lerp(prev.opacity, self.target.opacity),
            x: lerp(prev.x, self.target.x),
            y: lerp(prev.y, self.target.y),
            scale: lerp(prev.scale, self.target.scale),
            rotate: lerp(prev.rotate, self.target.rotate),
            rotate_x: lerp(prev.rotate_x, self.target.rotate_x),
            rotate_y: lerp(prev.rotate_y, self.target.rotate_y),
            rotate_z: lerp(prev.rotate_z, self.target.rotate_z),
            skew_x: lerp(prev.skew_x, self.target.skew_x),
            skew_y: lerp(prev.skew_y, self.target.skew_y),
            border_radius: lerp(prev.border_radius, self.target.border_radius),
            width_pct: lerp(prev.width_pct, self.target.width_pct),
            height_pct: lerp(prev.height_pct, self.target.height_pct),
            transition: self.target.transition,
        };

        if p >= 1.0 {
            self.done = true;
        }
        self.done
    }
}

/// Compact CSS number formatting: at most 3 decimal places, trailing zeros
/// trimmed (e.g. `1.000` -> `1`, `0.300` -> `0.3`). Keeps style strings short
/// and stable so write-on-change comparisons work.
pub(crate) fn fmt_css(v: f64) -> String {
    let mut s = format!("{v:.3}");
    if s.contains('.') {
        while s.ends_with('0') {
            s.pop();
        }
        if s.ends_with('.') {
            s.pop();
        }
    }
    s
}

/// Cached CSS values for a single animated element. Lets the rAF loop skip DOM
/// style writes when a frame produces the same value as the previous one.
#[derive(Default)]
struct VariantCache {
    transform: Option<String>,
    opacity: Option<String>,
    border_radius: Option<String>,
    width: Option<String>,
    height: Option<String>,
}

/// Apply a variant's values to a DOM element as inline styles, skipping writes
/// whose formatted value is unchanged since the last apply.
fn apply_variant(el: &web_sys::HtmlElement, v: &Variant, cache: &mut VariantCache) {
    let style = el.style();
    if let Some(t) = v.transform_string() {
        if cache.transform.as_deref() != Some(t.as_str()) {
            let _ = style.set_property("transform", &t);
            cache.transform = Some(t);
        }
    } else if cache.transform.take().is_some() {
        let _ = style.set_property("transform", "");
    }
    if let Some(o) = v.opacity {
        let s = fmt_css(o);
        if cache.opacity.as_deref() != Some(s.as_str()) {
            let _ = style.set_property("opacity", &s);
            cache.opacity = Some(s);
        }
    } else if cache.opacity.take().is_some() {
        let _ = style.set_property("opacity", "");
    }
    if let Some(r) = v.border_radius {
        let s = format!("{}px", fmt_css(r));
        if cache.border_radius.as_deref() != Some(s.as_str()) {
            let _ = style.set_property("border-radius", &s);
            cache.border_radius = Some(s);
        }
    }
    if let Some(w) = v.width_pct {
        let s = format!("{}%", fmt_css(w));
        if cache.width.as_deref() != Some(s.as_str()) {
            let _ = style.set_property("width", &s);
            cache.width = Some(s);
        }
    }
    if let Some(h) = v.height_pct {
        let s = format!("{}%", fmt_css(h));
        if cache.height.as_deref() != Some(s.as_str()) {
            let _ = style.set_property("height", &s);
            cache.height = Some(s);
        }
    }
}

fn resolve<'a>(variants: &'a Variants, name: &str) -> Option<&'a Variant> {
    variants.get(name)
}

/// The `<Motion>` component. Pass children inline:
/// `<Motion variants=v initial="hidden" animate="show" class="...">...</Motion>`
#[component]
pub fn Motion(
    #[prop(default = std::collections::HashMap::new())] variants: Variants,
    #[prop(default = "")] initial: &'static str,
    #[prop(default = "")] animate: &'static str,
    #[prop(default = "")] while_in_view: &'static str,
    #[prop(default = Transition::NONE)] transition: Transition,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] id: &'static str,
    #[prop(default = Vec::new())] style: Vec<(&'static str, String)>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let initial_variant = if initial.is_empty() { None } else { resolve(&variants, initial).cloned() };
    let animate_variant = if animate.is_empty() { None } else { resolve(&variants, animate).cloned() };
    let while_in_view_variant =
        if while_in_view.is_empty() { None } else { resolve(&variants, while_in_view).cloned() };

    let target_variant = while_in_view_variant.clone().or_else(|| animate_variant.clone());
    let start_variant = initial_variant.clone().unwrap_or(Variant::new());

    let el_ref = NodeRef::<Div>::new();

    let initial_transition = if transition.duration == 0.0 && transition.delay == 0.0 {
        target_variant.as_ref().map(|v| v.transition).unwrap_or(Transition::NONE)
    } else {
        transition
    };

    let anim_state = Rc::new(RefCell::new(AnimState::new(
        start_variant.clone(),
        target_variant.clone().unwrap_or(Variant::new()),
        initial_transition,
    )));

    // Stagger delay from parent context (if any).
    let stagger_delay = use_context::<StaggerContext>().map(|ctx| {
        let idx = ctx.next_index.get();
        ctx.next_index.set(idx + 1);
        ctx.delay_children + (idx as f64) * ctx.stagger
    });

    // Set up the animation (deferred so the element exists). Applies the
    // start variant, then animates (or jumps straight to the target under
    // `prefers-reduced-motion`).
    let el_ref_anim = el_ref.clone();
    let anim_state_for_start = anim_state.clone();
    let anim_state_anim = anim_state.clone();
    let target_for_anim = target_variant.clone();
    let initial_for_anim = initial_variant.clone();
    let start_variant_for_init = start_variant.clone();
    let while_in_view_flag = !while_in_view.is_empty();
    let transition_for_effect = initial_transition;
    Effect::new(move || {
        let el_ref = el_ref_anim.clone();
        let target = target_for_anim.clone();
        let anim_state_for_start = anim_state_for_start.clone();
        let anim_state_anim_clone = anim_state_anim.clone();
        let initial_variant = initial_for_anim.clone();
        let start_variant_init = start_variant_for_init.clone();
        let transition = transition_for_effect;
        let stagger_delay = stagger_delay;
        let while_in_view_flag = while_in_view_flag;

        let cb = Closure::<dyn FnMut()>::new(move || {
            let Some(el) = el_ref.get() else { return };

            // Reduced motion: skip the animation and show the end state.
            if crate::utils::reduced_motion::reduced_motion() {
                if let Some(target) = target.clone() {
                    let mut cache = VariantCache::default();
                    apply_variant(el.as_ref(), &target, &mut cache);
                }
                return;
            }

            // Apply the start variant so the element mounts hidden.
            let mut init_cache = VariantCache::default();
            apply_variant(el.as_ref(), &start_variant_init, &mut init_cache);

            let Some(target) = target.clone() else { return };

            let anim_state_for_start = anim_state_for_start.clone();
            let anim_state_anim_clone = anim_state_anim_clone.clone();
            let initial_variant = initial_variant.clone();
            let target = target.clone();
            let transition = transition;
            let stagger_delay = stagger_delay;
            let el_for_obs = el.clone();

            let start_fn = move || {
                let mut state = anim_state_for_start.borrow_mut();
                *state = AnimState::new(
                    initial_variant.clone().unwrap_or(Variant::new()),
                    target.clone(),
                    transition,
                );
                if let Some(d) = stagger_delay {
                    state.transition = Transition {
                        delay: state.transition.delay + d,
                        ..state.transition
                    };
                }
                drop(state);

                let state_tick = anim_state_anim_clone.clone();
                let el_tick = el.clone();
                let cache = Rc::new(RefCell::new(VariantCache::default()));
                spawn(move || {
                    let done = {
                        let mut s = state_tick.borrow_mut();
                        s.step();
                        let mut c = cache.borrow_mut();
                        apply_variant(el_tick.as_ref(), &s.current, &mut c);
                        s.done
                    };
                    done
                });
            };

            if !while_in_view_flag {
                start_fn();
            } else if VIEWPORT_ONCE {
                let cb = start_fn;
                let cb_cell = Rc::new(RefCell::new(Some(cb)));
                let init = IntersectionObserverInit::new();
                init.set_root_margin(VIEWPORT_MARGIN);
                let obs_closure =
                    Closure::<dyn FnMut(Vec<web_sys::IntersectionObserverEntry>)>::new(
                        move |entries: Vec<web_sys::IntersectionObserverEntry>| {
                            let Some(entry) = entries.into_iter().next() else { return };
                            if entry.is_intersecting() {
                                if let Some(f) = cb_cell.borrow_mut().take() {
                                    f();
                                }
                            }
                        },
                    );
                if let Ok(obs) = IntersectionObserver::new_with_options(
                    obs_closure.as_ref().unchecked_ref(),
                    &init,
                ) {
                    obs.observe(&el_for_obs);
                    std::mem::forget(obs_closure);
                }
            }
        });

        if let Some(w) = web_sys::window() {
            let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                0,
            );
        }
        std::mem::forget(cb);
    });

    // Build inline style string from `style` prop.
    let style_str = if style.is_empty() {
        None
    } else {
        Some(style.iter().map(|(k, v)| format!("{k}: {v}")).collect::<Vec<_>>().join("; "))
    };

    let class_str = if class.is_empty() { None } else { Some(class) };
    let id_str = if id.is_empty() { None } else { Some(id) };

    view! {
        <div
            node_ref=el_ref
            class=class_str
            id=id_str
            style=style_str
        >
            {children.map(|c| c())}
        </div>
    }
}