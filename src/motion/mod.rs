pub mod animate_presence;
pub mod easing;
pub mod motion;
pub mod motion_value;
pub mod raf_loop;
pub mod spring;
pub mod variants;

pub use animate_presence::{AnimatePresence, AnimatePresenceProps};
pub use easing::{Easing, EASE_IN_OUT, EASE_OUT, EASE_OUT_EXPO, EASE_SMOOTH, EASE_STANDARD};
pub use motion::{Motion, MotionProps, StaggerContext, provide_stagger};
pub use motion_value::{
    MotionTemplate, MotionValue, use_motion_template, use_motion_value, use_spring, use_transform,
};
pub use spring::{SpringConfig, SPRING_SMOOTH, SPRING_SNAPPY};
pub use variants::{
    card_variants, fade_up, hero_stagger_variants, stagger_container, VIEWPORT_MARGIN,
    VIEWPORT_ONCE,
};