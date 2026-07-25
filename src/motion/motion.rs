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

/// Apply a variant's values to a DOM element as inline styles.
fn apply_variant(el: &web_sys::HtmlElement, v: &Variant) {
    let style = el.style();
    if let Some(t) = v.transform_string() {
        let _ = style.set_property("transform", &t);
    } else {
        let _ = style.set_property("transform", "");
    }
    if let Some(o) = v.opacity {
        let _ = style.set_property("opacity", &format!("{o}"));
    }
    if let Some(r) = v.border_radius {
        let _ = style.set_property("border-radius", &format!("{r}px"));
    }
    if let Some(w) = v.width_pct {
        let _ = style.set_property("width", &format!("{w}%"));
    }
    if let Some(h) = v.height_pct {
        let _ = style.set_property("height", &format!("{h}%"));
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
    #[prop(optional)] children: Option<ChildrenFn>,
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

    // Apply the initial variant on mount.
    let el_ref_init = el_ref.clone();
    let start_variant_init = start_variant.clone();
    Effect::new(move || {
        if let Some(el) = el_ref_init.get() {
            apply_variant(el.as_ref(), &start_variant_init);
        }
    });

    // Kick off the animation.
    let el_ref_anim = el_ref.clone();
    let anim_state_anim = anim_state.clone();
    let anim_state_for_start = anim_state.clone();
    let target_for_anim = target_variant.clone();
    let while_in_view_flag = !while_in_view.is_empty();
    Effect::new(move || {
        let Some(el) = el_ref_anim.get() else { return };
        let Some(target) = target_for_anim.clone() else { return };

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

            let state_tick = anim_state_anim.clone();
            let el_tick = el.clone();
            spawn(move || {
                let done = {
                    let mut s = state_tick.borrow_mut();
                    s.step();
                    apply_variant(el_tick.as_ref(), &s.current);
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
            let el_for_obs = el.clone();
            let mut init = IntersectionObserverInit::new();
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