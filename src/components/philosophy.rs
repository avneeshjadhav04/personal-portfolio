//! Philosophy — port of `src/components/Philosophy.tsx`.
//!
//! SplitText per-word reveal reproduced with per-word `<Motion>` + a shared
//! stagger context. The parallax noise background is dropped (cosmetic, low
//! value, hard to reproduce identically without GSAP scrubbing); the section
//! retains its layout and the entrance animation.

use std::collections::HashMap;

use leptos::prelude::*;

use crate::motion::easing::EASE_SMOOTH;
use crate::motion::{Motion, provide_stagger};
use crate::motion::variants::{Transition, Variant};

fn word_variants() -> HashMap<&'static str, Variant> {
    let mut m = HashMap::new();
    m.insert("hidden", Variant { y: Some(40.0), opacity: Some(0.0), ..Variant::new() });
    m.insert(
        "show",
        Variant {
            y: Some(0.0),
            opacity: Some(1.0),
            transition: Transition::new(0.8, EASE_SMOOTH),
            ..Variant::new()
        },
    );
    m
}

fn word_view(text: String) -> impl IntoView {
    view! {
        <Motion variants=word_variants() initial="hidden" while_in_view="show" class="split-word inline-block mr-[0.25em]">{text}</Motion>
    }
}

#[component]
fn SplitTextReveal(
    text: String,
    class: String,
    stagger: f64,
    delay: f64,
) -> impl IntoView {
    let words = text.split(' ').map(|w| w.to_string()).collect::<Vec<_>>();
    let ctx = provide_stagger(stagger, delay);
    provide_context(ctx);
    view! {
        <p class=class>
            {words
                .iter()
                .map(|w| word_view(w.to_string()))
                .collect::<Vec<_>>()}
        </p>
    }
}

#[component]
pub fn Philosophy() -> impl IntoView {
    view! {
        <section id="philosophy" class="relative pt-12 md:pt-20 pb-20 md:pb-32 px-6 overflow-hidden bg-background">
            <div class="relative z-10 max-w-5xl mx-auto">
                <span class="text-[12px] font-mono-accent uppercase tracking-[0.3em] text-text-secondary mb-12 block border-l-2 border-accent pl-4">
                    "My Philosophy"
                </span>
                <div class="space-y-12 md:space-y-16">
                    <SplitTextReveal
                        text="Most AI development focuses on: demos, proof-of-concepts, and blog posts that never see production.".to_string()
                        class="text-2xl md:text-4xl text-text-secondary leading-tight max-w-4xl font-light tracking-tight".to_string()
                        stagger=0.02
                        delay=0.0
                    />
                    <div class="max-w-4xl">
                        <SplitTextReveal
                            text="I focus on:".to_string()
                            class="text-xl md:text-2xl text-text-secondary leading-relaxed mb-6 font-mono-accent uppercase tracking-widest".to_string()
                            stagger=0.03
                            delay=0.2
                        />
                        <SplitTextReveal
                            text="shipping systems that work.".to_string()
                            class="text-5xl sm:text-6xl md:text-8xl lg:text-[8rem] font-bold text-text-primary leading-[0.85] tracking-tighter uppercase".to_string()
                            stagger=0.06
                            delay=0.4
                        />
                    </div>
                </div>
            </div>
        </section>
    }
}