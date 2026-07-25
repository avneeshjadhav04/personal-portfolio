//! Hero — port of `src/components/Hero.tsx`.
//!
//! Mouse-parallax tilt on the headline group, staggered entrance, scroll
//! indicator. Uses our motion core instead of Framer Motion.

use std::collections::HashMap;

use leptos::prelude::*;
use web_sys::window;

use crate::motion::easing::{EASE_IN_OUT, EASE_OUT_EXPO, EASE_SMOOTH};
use crate::motion::motion::Motion;
use crate::motion::motion_value::{use_motion_value, use_spring};
use crate::motion::spring::SpringConfig;
use crate::motion::variants::{Transition, Variant, Variants};

fn v(hidden: Variant, show: Variant) -> Variants {
    let mut m = HashMap::new();
    m.insert("hidden", hidden);
    m.insert("show", show);
    m
}

fn hero_stagger_variants() -> Variants {
    v(
        Variant { y: Some(150.0), opacity: Some(0.0), rotate_z: Some(2.0), ..Variant::new() },
        Variant {
            y: Some(0.0),
            opacity: Some(1.0),
            rotate_z: Some(0.0),
            transition: Transition::new(1.5, EASE_OUT_EXPO),
            ..Variant::new()
        },
    )
}

fn divider_variants() -> Variants {
    v(
        Variant { scale: Some(0.0), ..Variant::new() },
        Variant {
            scale: Some(1.0),
            transition: Transition::new(1.2, EASE_IN_OUT),
            ..Variant::new()
        },
    )
}

fn sub_variants() -> Variants {
    v(
        Variant { y: Some(30.0), opacity: Some(0.0), ..Variant::new() },
        Variant {
            y: Some(0.0),
            opacity: Some(1.0),
            transition: Transition::new(1.0, EASE_SMOOTH),
            ..Variant::new()
        },
    )
}

fn cta_variants() -> Variants {
    v(
        Variant { y: Some(20.0), opacity: Some(0.0), ..Variant::new() },
        Variant {
            y: Some(0.0),
            opacity: Some(1.0),
            transition: Transition::new(0.8, EASE_SMOOTH),
            ..Variant::new()
        },
    )
}

fn cta_group_variants() -> Variants {
    let mut m = HashMap::new();
    m.insert("hidden", Variant::new());
    m.insert(
        "show",
        Variant {
            transition: Transition::NONE.with_stagger(0.1),
            ..Variant::new()
        },
    );
    m
}

fn scroll_variants() -> Variants {
    v(
        Variant { y: Some(-20.0), opacity: Some(0.0), ..Variant::new() },
        Variant {
            y: Some(0.0),
            opacity: Some(1.0),
            transition: Transition::new(1.0, EASE_SMOOTH),
            ..Variant::new()
        },
    )
}

fn hero_container_variants() -> Variants {
    let mut m = HashMap::new();
    m.insert("hidden", Variant::new());
    m.insert(
        "show",
        Variant {
            transition: Transition::NONE
                .with_delay_children(0.25)
                .with_stagger(0.32),
            ..Variant::new()
        },
    );
    m
}

#[component]
pub fn Hero() -> impl IntoView {
    // Mouse-parallax motion values, smoothed with springs (stiffness 50, damping 20).
    let mouse_x = use_motion_value(0.0);
    let mouse_y = use_motion_value(0.0);
    let smooth_x = use_spring(mouse_x, SpringConfig::new(50.0, 20.0).with_rest_delta(0.001));
    let smooth_y = use_spring(mouse_y, SpringConfig::new(50.0, 20.0).with_rest_delta(0.001));

    // Global mousemove listener.
    let mx = mouse_x;
    let my = mouse_y;
    Effect::new(move || {
        let Some(w) = window() else { return };
        let mx = mx;
        let my = my;
        let cb = wasm_bindgen::closure::Closure::new(move |e: web_sys::MouseEvent| {
            let cx = w.inner_width().unwrap_or_default().as_f64().unwrap_or(0.0) / 2.0;
            let cy = w.inner_height().unwrap_or_default().as_f64().unwrap_or(0.0) / 2.0;
            mx.set((e.client_x() as f64 - cx) * 0.02);
            my.set((e.client_y() as f64 - cy) * 0.02);
        });
        let _ = w.add_event_listener_with_callback("mousemove", cb.as_ref().unchecked_ref());
        std::mem::forget(cb);
    });

    let parallax = Memo::new(move |_| {
        format!("translate({}px, {}px)", smooth_x.get(), smooth_y.get())
    });

    view! {
        <section class="relative w-full min-h-[100dvh] flex items-center justify-center overflow-hidden pt-20">
            <Motion props=crate::motion::motion::MotionProps {
                variants: Some(hero_container_variants()),
                initial: Some("hidden".to_string()),
                animate: Some("show".to_string()),
                class: Some("relative z-10 w-full max-w-7xl mx-auto px-6 md:px-10 flex flex-col items-center text-center".to_string()),
                ..Default::default()
            }>
                <div class="parallax-layer overflow-hidden mb-2" style:transform=move || parallax.get() style:will-change="transform">
                    <Motion props=crate::motion::motion::MotionProps {
                        variants: Some(hero_stagger_variants()),
                        initial: Some("hidden".to_string()),
                        animate: Some("show".to_string()),
                        class: Some("hero-stagger text-[4rem] sm:text-[6rem] md:text-[8rem] lg:text-[10rem] font-bold tracking-tighter leading-[0.85] text-text-primary uppercase".to_string()),
                        ..Default::default()
                    }>"Avneesh"</Motion>
                </div>
                <div class="parallax-layer overflow-hidden mb-8 flex flex-col md:flex-row items-center gap-4 md:gap-8" style:transform=move || parallax.get() style:will-change="transform">
                    <Motion props=crate::motion::motion::MotionProps {
                        variants: Some(hero_stagger_variants()),
                        initial: Some("hidden".to_string()),
                        animate: Some("show".to_string()),
                        class: Some("hero-stagger text-[4rem] sm:text-[6rem] md:text-[8rem] lg:text-[10rem] font-bold tracking-tighter leading-[0.85] text-gradient uppercase".to_string()),
                        ..Default::default()
                    }>"Jadhav."</Motion>
                </div>
                <Motion props=crate::motion::motion::MotionProps {
                    variants: Some(divider_variants()),
                    initial: Some("hidden".to_string()),
                    animate: Some("show".to_string()),
                    class: Some("hero-divider w-full max-w-2xl h-[1px] bg-text-primary/20 mb-10".to_string()),
                    style: Some(vec![("transform-origin", "left center".to_string())]),
                    ..Default::default()
                }>{}
                </Motion>
                <Motion props=crate::motion::motion::MotionProps {
                    variants: Some(sub_variants()),
                    initial: Some("hidden".to_string()),
                    animate: Some("show".to_string()),
                    class: Some("hero-sub text-lg md:text-2xl text-text-secondary max-w-2xl leading-relaxed mb-12 font-light".to_string()),
                    ..Default::default()
                }>
                    <span>
                        "An "
                        <strong class="text-text-primary font-medium">"AI Engineer and Full-Stack Developer"</strong>
                        " focused on building AI-native applications, automation systems, and scalable software products."
                    </span>
                </Motion>
                <Motion props=crate::motion::motion::MotionProps {
                    variants: Some(cta_group_variants()),
                    initial: Some("hidden".to_string()),
                    animate: Some("show".to_string()),
                    class: Some("flex flex-wrap items-center justify-center gap-6".to_string()),
                    ..Default::default()
                }>
                    <Motion props=crate::motion::motion::MotionProps {
                        variants: Some(cta_variants()),
                        initial: Some("hidden".to_string()),
                        animate: Some("show".to_string()),
                        class: Some("hero-cta inline-flex items-center justify-center px-8 py-4 rounded-none border border-text-primary bg-text-primary text-surface font-mono-accent text-sm uppercase tracking-widest hover:bg-transparent hover:text-text-primary transition-colors duration-300".to_string()),
                        ..Default::default()
                    }>
                        <a href="#projects">"View Projects"</a>
                    </Motion>
                    <Motion props=crate::motion::motion::MotionProps {
                        variants: Some(cta_variants()),
                        initial: Some("hidden".to_string()),
                        animate: Some("show".to_string()),
                        class: Some("hero-cta inline-flex items-center justify-center px-8 py-4 rounded-none border border-text-primary/20 bg-transparent text-text-primary font-mono-accent text-sm uppercase tracking-widest hover:border-text-primary transition-colors duration-300".to_string()),
                        ..Default::default()
                    }>
                        <a href="#contact">"Get in Touch"</a>
                    </Motion>
                </Motion>
            </Motion>
            <Motion props=crate::motion::motion::MotionProps {
                variants: Some(scroll_variants()),
                initial: Some("hidden".to_string()),
                animate: Some("show".to_string()),
                transition: Some(Transition::new(1.0, EASE_SMOOTH).with_delay(1.95)),
                class: Some("hero-scroll absolute bottom-10 left-1/2 -translate-x-1/2 z-10".to_string()),
                ..Default::default()
            }>
                <div class="flex flex-col items-center gap-3 text-text-primary/50 hover:text-text-primary transition-colors">
                    <span class="text-[10px] font-mono-accent uppercase tracking-[0.3em]">"Scroll"</span>
                    <div class="w-[1px] h-12 bg-text-primary/30 relative overflow-hidden">
                        <div class="absolute top-0 left-0 w-full h-full bg-text-primary animate-[shimmer_2s_infinite]" style:transform-origin="top" />
                    </div>
                </div>
            </Motion>
        </section>
    }
}