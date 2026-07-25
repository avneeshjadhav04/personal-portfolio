//! FeaturedProjects — port of `src/components/FeaturedProjects.tsx`.
//!
//! Three featured projects with YouTube embeds, scroll-triggered fade-in.

use std::collections::HashMap;

use leptos::prelude::*;

use crate::components::section_glow::{GlowPosition, GlowSize, SectionGlow};
use crate::motion::easing::EASE_SMOOTH;
use crate::motion::Motion;
use crate::motion::variants::{Transition, Variant};

struct FeaturedProject {
    step: &'static str,
    title: &'static str,
    description: &'static str,
    tags: &'static [&'static str],
    video_id: &'static str,
    link: &'static str,
}

const FEATURED_PROJECTS: &[FeaturedProject] = &[
    FeaturedProject {
        step: "01",
        title: "Project Vulcan: AI Assistant",
        description: "An open-source platform that helps you operate AI with terminal-level access for your daily workflows, making them simpler, more secure, and self-hosted.",
        tags: &["TypeScript", "React", "Rust", "Axum", "SQLite", "AI"],
        video_id: "LhgXK4dvE3w",
        link: "https://project-vulcan.onrender.com/",
    },
    FeaturedProject {
        step: "02",
        title: "Kovero AI: AI Powered Healthcare Claims Assistance Platform",
        description: "A full-fledged user-centric insurance claims assistance platform that simplifies finding the right health policies and helps users prepare for claims. Integrates AI using OCR and transformer-based LLMs for document verification and query resolution.",
        tags: &["Next.js", "React", "TypeScript", "Rust", "Axum", "SQLite", "Docker", "AI"],
        video_id: "9f2AL8xu1NU",
        link: "https://koveroai-alpha.onrender.com/",
    },
    FeaturedProject {
        step: "03",
        title: "LLM From Scratch",
        description: "124M parameter language model trained from scratch on 2B tokens. Built every layer in PyTorch, no Trainer.train(). Validation perplexity 14.8, trained in 5 hours. Live API, weights, and code available.",
        tags: &["PyTorch", "LLM", "Transformers", "AI", "NLP"],
        video_id: "e3es5UZgxq0",
        link: "https://avneeshjadhav04--llm-api.modal.run/",
    },
];

fn card_variants() -> HashMap<&'static str, Variant> {
    let mut m = HashMap::new();
    m.insert("hidden", Variant { y: Some(80.0), opacity: Some(0.0), ..Variant::new() });
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
pub fn FeaturedProjects() -> impl IntoView {
    view! {
        <section id="projects" class="relative bg-background overflow-hidden">
            <SectionGlow color="#FF3366".to_string() position=GlowPosition::Center size=GlowSize::Xl opacity=0.2 animate=true />
            <div class="py-24 md:py-32 px-6 relative z-10">
                <div class="max-w-6xl mx-auto text-center mb-16">
                    <h2 class="text-5xl md:text-7xl font-bold tracking-tighter text-text-primary mb-4 uppercase">
                        "Featured "
                        <span class="text-gradient">"Projects."</span>
                    </h2>
                    <p class="text-xl text-text-secondary max-w-xl mx-auto leading-relaxed font-light">
                        "Real-world systems solving real problems using AI."
                    </p>
                </div>
                <div class="max-w-5xl mx-auto space-y-12 md:space-y-16">
                    {FEATURED_PROJECTS.iter().map(|p| {
                        let src = format!("https://www.youtube-nocookie.com/embed/{}", p.video_id);
                        view! {
                            <Motion variants=card_variants() initial="hidden" while_in_view="show" class="rounded-none bg-surface border border-border overflow-hidden shadow-2xl">
                                <div class="grid md:grid-cols-2 gap-0">
                                    <div class="relative h-64 md:h-auto md:min-h-[480px] flex items-center justify-center overflow-hidden border-b md:border-b-0 md:border-r border-border">
                                        <div class="w-full h-full flex items-center justify-center p-4 md:p-8">
                                            <div class="w-full aspect-video bg-black">
                                                <iframe
                                                    src=src
                                                    title="YouTube video player"
                                                    allow="accelerometer; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                                                    referrerpolicy="strict-origin-when-cross-origin"
                                                    allowfullscreen=true
                                                    attr_loading="lazy"
                                                    class="w-full h-full border-0"
                                                />
                                            </div>
                                        </div>
                                    </div>
                                    <div class="p-8 md:p-12 flex flex-col justify-center">
                                        <span class="text-[10px] font-mono-accent uppercase tracking-[0.25em] text-text-secondary mb-4 block">
                                            {format!("Project {}", p.step)}
                                        </span>
                                        <h3 class="text-3xl md:text-4xl font-bold text-text-primary mb-6 tracking-tight uppercase">
                                            {p.title}
                                        </h3>
                                        <p class="text-lg text-text-secondary leading-relaxed mb-8 font-light">
                                            {p.description}
                                        </p>
                                        <div class="flex flex-wrap gap-2 mb-8">
                                            {p.tags.iter().map(|tag| {
                                                view! {
                                                    <span class="px-3 py-1.5 text-xs font-mono-accent font-medium bg-surface border border-text-primary/20 text-text-primary uppercase tracking-wider">
                                                        {*tag}
                                                    </span>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                        <a
                                            href=p.link
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            class="inline-flex items-center justify-center px-8 py-4 bg-text-primary text-surface font-mono-accent text-sm uppercase tracking-widest hover:bg-accent transition-colors duration-300"
                                        >
                                            "View Project"
                                        </a>
                                    </div>
                                </div>
                            </Motion>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}