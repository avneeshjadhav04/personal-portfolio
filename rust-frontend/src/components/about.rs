//! About — port of `src/components/About.tsx`.
//!
//! GSAP ScrollTrigger entrances (image scale/rotateY, text y/opacity stagger)
//! are reproduced with `<Motion while_in_view>`.

use std::collections::HashMap;

use leptos::prelude::*;

use crate::components::section_glow::{GlowPosition, GlowSize, SectionGlow};
use crate::motion::easing::EASE_SMOOTH;
use crate::motion::motion::Motion;
use crate::motion::motion::MotionProps;
use crate::motion::variants::{Transition, Variant};

fn image_variants() -> std::collections::HashMap<&'static str, Variant> {
    let mut m = HashMap::new();
    m.insert(
        "hidden",
        Variant { scale: Some(0.9), opacity: Some(0.0), rotate_y: Some(5.0), ..Variant::new() },
    );
    m.insert(
        "show",
        Variant {
            scale: Some(1.0),
            opacity: Some(1.0),
            rotate_y: Some(0.0),
            transition: Transition::new(1.5, EASE_SMOOTH),
            ..Variant::new()
        },
    );
    m
}

fn text_variants() -> std::collections::HashMap<&'static str, Variant> {
    let mut m = HashMap::new();
    m.insert("hidden", Variant { y: Some(50.0), opacity: Some(0.0), ..Variant::new() });
    m.insert(
        "show",
        Variant {
            y: Some(0.0),
            opacity: Some(1.0),
            transition: Transition::new(1.0, EASE_SMOOTH),
            ..Variant::new()
        },
    );
    m
}

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about" class="pt-32 md:pt-48 pb-12 md:pb-20 px-6 relative bg-background border-b border-border overflow-hidden">
            <SectionGlow color="#FF9933".to_string() position=GlowPosition::TopRight size=GlowSize::Lg opacity=0.3 animate=true />
            <div class="max-w-7xl mx-auto relative z-10">
                <div class="grid lg:grid-cols-2 gap-16 lg:gap-24 items-start">
                    <Motion props=MotionProps {
                        variants: Some(image_variants()),
                        initial: Some("hidden".to_string()),
                        while_in_view: Some("show".to_string()),
                        class: Some("about-image-container relative".to_string()),
                        ..Default::default()
                    }>
                        <div class="aspect-[4/5] relative overflow-hidden bg-surface border border-text-primary/10 p-2">
                            <img
                                src="/avatar.jfif"
                                alt="Avneesh Jadhav"
                                class="w-full h-full object-cover filter grayscale hover:grayscale-0 transition-all duration-700"
                                loading="lazy"
                            />
                        </div>
                        <div class="absolute -bottom-4 -right-4 w-24 h-24 bg-gradient-to-br from-accent to-accent-glow blur-2xl opacity-50 pointer-events-none" />
                    </Motion>
                    <div class="flex flex-col justify-center h-full">
                        <Motion props=MotionProps {
                            variants: Some(text_variants()),
                            initial: Some("hidden".to_string()),
                            while_in_view: Some("show".to_string()),
                            class: Some("about-text-reveal text-5xl md:text-7xl font-bold tracking-tighter text-text-primary mb-8 uppercase leading-[0.9]".to_string()),
                            ..Default::default()
                        }>
                            <span>"Hi, I'm "</span>
                            <span class="text-gradient">"Avneesh."</span>
                        </Motion>
                        <Motion props=MotionProps {
                            variants: Some(text_variants()),
                            initial: Some("hidden".to_string()),
                            while_in_view: Some("show".to_string()),
                            transition: Some(Transition::new(1.0, EASE_SMOOTH).with_delay(0.1)),
                            class: Some("about-text-reveal text-lg text-text-secondary leading-relaxed mb-4".to_string()),
                            ..Default::default()
                        }>
                            "Technology has always fascinated me from the start. From opening up toys as a child to dissecting complex projects these days, it's always interesting to see how things work."
                        </Motion>
                        <Motion props=MotionProps {
                            variants: Some(text_variants()),
                            initial: Some("hidden".to_string()),
                            while_in_view: Some("show".to_string()),
                            transition: Some(Transition::new(1.0, EASE_SMOOTH).with_delay(0.2)),
                            class: Some("about-text-reveal text-lg text-text-secondary leading-relaxed mb-4".to_string()),
                            ..Default::default()
                        }>
                            "In college, theory was good, but there came a point where I shifted from studying text to actually engineering solutions."
                        </Motion>
                        <Motion props=MotionProps {
                            variants: Some(text_variants()),
                            initial: Some("hidden".to_string()),
                            while_in_view: Some("show".to_string()),
                            transition: Some(Transition::new(1.0, EASE_SMOOTH).with_delay(0.3)),
                            class: Some("about-text-reveal text-lg text-text-secondary leading-relaxed mb-12".to_string()),
                            ..Default::default()
                        }>
                            "Which brings me to the present. Currently, I'm focused on AI implementation."
                        </Motion>
                        <Motion props=MotionProps {
                            variants: Some(text_variants()),
                            initial: Some("hidden".to_string()),
                            while_in_view: Some("show".to_string()),
                            transition: Some(Transition::new(1.0, EASE_SMOOTH).with_delay(0.4)),
                            class: Some("about-text-reveal text-lg text-text-secondary leading-relaxed border-t border-border pt-8".to_string()),
                            ..Default::default()
                        }>
                            <span>"I am based out of "</span>
                            <strong class="text-text-primary font-medium">"Pune, Maharashtra, India"</strong>
                            "."
                        </Motion>
                    </div>
                </div>
            </div>
        </section>
    }
}