//! Animation variant definitions — ports of `src/lib/motion.ts`.
//!
//! A `Variant` is a set of target style values plus a transition describing
//! how to reach them. `Variants` is a named map (the string keys match what the
//! TS code passes to `variants={{ hidden: ..., show: ... }}`).

use std::collections::HashMap;

use crate::motion::easing::Easing;

/// A single CSS-style target plus the transition used to reach it.
#[derive(Clone, Debug, Default)]
pub struct Variant {
    pub opacity: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub scale: Option<f64>,
    pub rotate: Option<f64>,
    pub rotate_x: Option<f64>,
    pub rotate_y: Option<f64>,
    pub rotate_z: Option<f64>,
    pub skew_x: Option<f64>,
    pub skew_y: Option<f64>,
    pub border_radius: Option<f64>,
    pub width_pct: Option<f64>,
    pub height_pct: Option<f64>,
    pub transition: Transition,
}

impl Variant {
    pub const fn new() -> Self {
        Self {
            opacity: None,
            x: None,
            y: None,
            scale: None,
            rotate: None,
            rotate_x: None,
            rotate_y: None,
            rotate_z: None,
            skew_x: None,
            skew_y: None,
            border_radius: None,
            width_pct: None,
            height_pct: None,
            transition: Transition::NONE,
        }
    }

    /// Build a `transform: translate(x,y) scale(...) rotate... skew...` string
    /// from the present fields. `None` fields contribute nothing (so the
    /// element keeps whatever it had).
    pub fn transform_string(&self) -> Option<String> {
        let mut parts: Vec<String> = Vec::new();
        if let Some(x) = self.x {
            parts.push(format!("translateX({x}px)"));
        }
        if let Some(y) = self.y {
            parts.push(format!("translateY({y}px)"));
        }
        if let Some(s) = self.scale {
            parts.push(format!("scale({s})"));
        }
        if let Some(r) = self.rotate {
            parts.push(format!("rotate({r}deg)"));
        }
        if let Some(r) = self.rotate_x {
            parts.push(format!("rotateX({r}deg)"));
        }
        if let Some(r) = self.rotate_y {
            parts.push(format!("rotateY({r}deg)"));
        }
        if let Some(r) = self.rotate_z {
            parts.push(format!("rotateZ({r}deg)"));
        }
        if let Some(s) = self.skew_x {
            parts.push(format!("skewX({s}deg)"));
        }
        if let Some(s) = self.skew_y {
            parts.push(format!("skewY({s}deg)"));
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(" "))
        }
    }
}

/// How to animate from one variant to another.
#[derive(Clone, Copy, Debug)]
pub struct Transition {
    pub duration: f64,
    pub ease: Easing,
    pub delay: f64,
    /// Framer Motion's `staggerChildren`/`delayChildren`, applied by the
    /// parent variant that contains them.
    pub stagger_children: Option<f64>,
    pub delay_children: Option<f64>,
}

impl Transition {
    pub const NONE: Transition = Transition {
        duration: 0.0,
        ease: Easing::Linear,
        delay: 0.0,
        stagger_children: None,
        delay_children: None,
    };

    pub const fn new(duration: f64, ease: Easing) -> Self {
        Self { duration, ease, delay: 0.0, stagger_children: None, delay_children: None }
    }

    pub const fn with_delay(mut self, delay: f64) -> Self {
        self.delay = delay;
        self
    }

    pub const fn with_stagger(mut self, stagger: f64) -> Self {
        self.stagger_children = Some(stagger);
        self
    }

    pub const fn with_delay_children(mut self, delay_children: f64) -> Self {
        self.delay_children = Some(delay_children);
        self
    }
}

impl Default for Transition {
    fn default() -> Self {
        Self::NONE
    }
}

/// Named variants for a single element, e.g. `{ hidden: ..., show: ... }`.
pub type Variants = HashMap<&'static str, Variant>;

/// Helper to build a single-entry variants map.
pub fn variants(hidden: Variant, show: Variant) -> Variants {
    let mut m = Variants::with_capacity(2);
    m.insert("hidden", hidden);
    m.insert("show", show);
    m
}

// ============================================================
// Variant factories — direct ports of `src/lib/motion.ts`.
// ============================================================

use crate::motion::easing::{EASE_SMOOTH, EASE_OUT_EXPO};

/// `fadeUp(distance, duration)` from `src/lib/motion.ts`.
pub fn fade_up(distance: f64, duration: f64) -> Variants {
    variants(
        Variant {
            opacity: Some(0.0),
            y: Some(distance),
            ..Variant::new()
        },
        Variant {
            opacity: Some(1.0),
            y: Some(0.0),
            transition: Transition::new(duration, EASE_SMOOTH),
            ..Variant::new()
        },
    )
}

/// `staggerContainer(stagger, delayChildren)`.
pub fn stagger_container(stagger: f64, delay_children: f64) -> Variants {
    variants(
        Variant::new(),
        Variant {
            transition: Transition::NONE
                .with_stagger(stagger)
                .with_delay_children(delay_children),
            ..Variant::new()
        },
    )
}

/// `cardVariants(distance, duration)`.
pub fn card_variants(distance: f64, duration: f64) -> Variants {
    variants(
        Variant {
            opacity: Some(0.0),
            y: Some(distance),
            scale: Some(0.95),
            ..Variant::new()
        },
        Variant {
            opacity: Some(1.0),
            y: Some(0.0),
            scale: Some(1.0),
            transition: Transition::new(duration, EASE_SMOOTH),
            ..Variant::new()
        },
    )
}

/// Hero entrance variant (y:150, rotateZ:2 -> 0, easeOutExpo, 1.5s).
pub fn hero_stagger_variants() -> Variants {
    variants(
        Variant {
            y: Some(150.0),
            opacity: Some(0.0),
            rotate_z: Some(2.0),
            ..Variant::new()
        },
        Variant {
            y: Some(0.0),
            opacity: Some(1.0),
            rotate_z: Some(0.0),
            transition: Transition::new(1.5, EASE_OUT_EXPO),
            ..Variant::new()
        },
    )
}

/// Shared viewport config for `whileInView` — play once, trigger slightly
/// before the element is fully in view. Equivalent to `{ once: true, margin:
/// '-15% 0px' }`.
pub const VIEWPORT_ONCE: bool = true;
pub const VIEWPORT_MARGIN: &str = "-15% 0px";