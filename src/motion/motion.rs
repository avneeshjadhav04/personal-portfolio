//! The `<Motion>` component — a port of Framer Motion's `<motion.div>`.
//!
//! Supports a subset of Framer Motion's API sufficient for this portfolio:
//!   - `variants` (named map), `initial`, `animate`, `while_in_view`
//!   - `transition` (per-element override)
//!   - `class`, `id`, `style` (static inline styles)
//!   - parent-driven stagger via [`StaggerContext`]
//!
//! Animations run on the shared rAF loop in [`crate::motion::raf_loop`].
//!
//! Usage:
//! ```ignore
//! <Motion variants=Some(v) initial="hidden" animate="show" class="...">
//!     <div>"Hello"</div>
//! </Motion>
//! ```

use std::cell::RefCell;
use std::rc::Rc;

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, IntersectionObserver, IntersectionObserverInit};

use crate::motion::easing::Easing;
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
fn apply_variant(el: &Element, v: &Variant) {
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

/// Props for `<Motion>`. Children are passed inline in the `view!` macro.
#[derive(Default)]
pub struct MotionProps {
    pub variants: Option<Variants>,
    pub initial: Option<String>,
    pub animate: Option<String>,
    pub while_in_view: Option<String>,
    pub transition: Option<Transition>,
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: Option<Vec<(&'static str, String)>>,
    pub children: Option<Children>,
}

impl MotionProps {
    /// Builder helper for the common case.
    pub fn new() -> Self { Self::default() }
}

/// The `<Motion>` component. Pass children inline:
/// `<Motion props=MotionProps { ... }>...children...</Motion>`
#[component]
pub fn Motion(#[prop(default)] props: MotionProps) -> impl IntoView {
    let MotionProps {
        variants,
        initial,
        animate,
        while_in_view,
        transition: override_transition,
        class,
        id,
        style,
        children,
    } = props;

    let variants = variants.unwrap_or_default();
    let initial_variant = initial.as_deref().and_then(|n| resolve(&variants, n)).cloned();
    let animate_variant = animate.as_deref().and_then(|n| resolve(&variants, n)).cloned();
    let while_in_view_variant =
        while_in_view.as_deref().and_then(|n| resolve(&variants, n)).cloned();

    let target_variant = while_in_view_variant.clone().or_else(|| animate_variant.clone());
    let start_variant = initial_variant.clone().unwrap_or(Variant::new());

    let el_ref = NodeRef::<Element>::new();

    let initial_transition = override_transition
        .unwrap_or(target_variant.as_ref().map(|v| v.transition).unwrap_or(Transition::NONE));

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
            apply_variant(&el, &start_variant_init);
        }
    });

    // Kick off the animation.
    let el_ref_anim = el_ref.clone();
    let anim_state_anim = anim_state.clone();
    let target_for_anim = target_variant.clone();
    let while_in_view_flag = while_in_view.is_some();
    Effect::new(move || {
        let Some(el) = el_ref_anim.get() else { return };
        let Some(target) = target_for_anim.clone() else { return };

        let start_fn = move || {
            let mut state = anim_state_anim.borrow_mut();
            *state = AnimState::new(
                initial_variant.clone().unwrap_or(Variant::new()),
                target.clone(),
                override_transition.unwrap_or(target.transition),
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
                    apply_variant(&el_tick, &s.current);
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
            let init = IntersectionObserverInit::new();
            init.set_root_margin(VIEWPORT_MARGIN);
            let obs_closure =
                Closure::<dyn FnMut(Vec<web_sys::IntersectionObserverEntry>)>::new(
                    move |entries| {
                        let Some(entry) = entries.into_iter().next() else { return };
                        if entry.is_intersecting() {
                            if let Some(f) = cb_cell.borrow_mut().take() {
                                f();
                            }
                        }
                    },
                );
            if let Some(obs) = IntersectionObserver::new_with_intersection_observer_init(
                obs_closure.as_ref().unchecked_ref(),
                &init,
            ) {
                obs.observe(&el_for_obs);
                std::mem::forget(obs_closure);
            }
        }
    });

    // Build inline style string from `style` prop.
    let style_str = style.map(|s| {
        s.iter().map(|(k, v)| format!("{k}: {v}")).collect::<Vec<_>>().join("; ")
    });

    view! {
        <div
            node_ref=el_ref
            class=class.unwrap_or_default()
            id=Option::map(id, |i| i)
            style=style_str
        >
            {children.map(|c| c())}
        </div>
    }
}