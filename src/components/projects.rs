//! Projects (all projects) — port of `src/components/Projects.tsx`.

use std::collections::HashMap;

use leptos::prelude::*;

use crate::components::icons::*;
use crate::components::section_glow::{GlowPosition, GlowSize, SectionGlow};
use crate::components::tilt_card::TiltCard;
use crate::motion::easing::EASE_SMOOTH;
use crate::motion::motion::{Motion, MotionProps, provide_stagger};
use crate::motion::variants::{Transition, Variant};

type IconFn = fn(i32, Option<&str>) -> AnyView;
fn icon_fn<I: IntoView + 'static>(f: impl Fn(i32, Option<&str>) -> I + Copy + 'static) -> IconFn {
    move |s, c| f(s, c).into_any()
}

struct Project {
    title: &'static str,
    description: &'static str,
    tags: &'static [&'static str],
    icon: IconFn,
    year: &'static str,
}

const PROJECTS: &[Project] = &[
    Project {
        title: "Autonomous Lead Generation System",
        description: "A fully automated client outreach and lead generation system that identifies prospects, personalizes outreach, and manages follow-ups using AI agents.",
        tags: &["Automation", "Python", "APIs", "n8n"],
        icon: icon_fn(BotIcon::new),
        year: "2026",
    },
    Project {
        title: "Voice AI Call Handler",
        description: "A fully automated inbound call handling system using voice AI, capable of understanding caller intent, answering queries, and routing calls intelligently.",
        tags: &["Voice AI", "VAPI", "APIs", "n8n", "Prompt Engineering"],
        icon: icon_fn(PhoneIcon::new),
        year: "Feb 2025",
    },
    Project {
        title: "AI Lead Qualification Agent",
        description: "An intelligent appointment setter and lead qualification agent that engages prospects, scores leads, and books meetings autonomously.",
        tags: &["AI Agents", "Automation", "APIs", "n8n"],
        icon: icon_fn(CalendarIcon::new),
        year: "Jun 2025",
    },
    Project {
        title: "LinkedIn Content Automation",
        description: "An automated system that generates, schedules, and posts engaging LinkedIn content using AI-driven copywriting and trend analysis.",
        tags: &["Automation", "Content AI", "APIs", "n8n"],
        icon: icon_fn(GlobeIcon::new),
        year: "Jul 2025",
    },
    Project {
        title: "Housing Price Predictor",
        description: "A Linear Regression-based model that predicts housing prices from structured data features, demonstrating core ML fundamentals with clean data pipelines.",
        tags: &["Linear Regression", "Python", "scikit-learn", "Pandas"],
        icon: icon_fn(HomeIcon::new),
        year: "Feb 2024",
    },
    Project {
        title: "Credit Card Fraud Detection",
        description: "A Binary Classification model that detects fraudulent credit card transactions with high precision, using feature engineering and ensemble techniques.",
        tags: &["Binary Classification", "Python", "ML", "Pandas"],
        icon: icon_fn(CreditCardIcon::new),
        year: "Jan 2024",
    },
    Project {
        title: "Project Vulcan: AI Assistant",
        description: "An open-source platform that helps you operate AI with terminal-level access for your daily workflows, making them simpler, more secure, and self-hosted.",
        tags: &["TypeScript", "React", "Rust", "Axum", "SQLite", "AI"],
        icon: icon_fn(CpuIcon::new),
        year: "2026",
    },
    Project {
        title: "Kovero AI: AI Powered Healthcare Claims Assistance Platform",
        description: "A full-fledged user-centric insurance claims assistance platform that simplifies finding the right health policies and helps users prepare for claims. Integrates AI using OCR and transformer-based LLMs for document verification and query resolution.",
        tags: &["Next.js", "React", "TypeScript", "Rust", "Axum", "SQLite", "Docker", "AI"],
        icon: icon_fn(StethoscopeIcon::new),
        year: "2026",
    },
    Project {
        title: "LLM From Scratch",
        description: "124M parameter language model trained from scratch on 2B tokens. Built every layer in PyTorch, no Trainer.train(). Validation perplexity 14.8, trained in 5 hours. Live API, weights, and code available.",
        tags: &["PyTorch", "LLM", "Transformers", "AI", "NLP"],
        icon: icon_fn(CpuIcon::new),
        year: "2026",
    },
];

fn header_variants() -> HashMap<&'static str, Variant> {
    let mut m = HashMap::new();
    m.insert("hidden", Variant { y: Some(30.0), opacity: Some(0.0), ..Variant::new() });
    m.insert(
        "show",
        Variant {
            y: Some(0.0),
            opacity: Some(1.0),
            transition: Transition::new(0.7, EASE_SMOOTH),
            ..Variant::new()
        },
    );
    m
}

fn card_variant() -> HashMap<&'static str, Variant> {
    let mut m = HashMap::new();
    m.insert(
        "hidden",
        Variant { y: Some(50.0), scale: Some(0.95), opacity: Some(0.0), ..Variant::new() },
    );
    m.insert(
        "show",
        Variant {
            y: Some(0.0),
            scale: Some(1.0),
            opacity: Some(1.0),
            transition: Transition::new(0.6, EASE_SMOOTH),
            ..Variant::new()
        },
    );
    m
}

#[component]
pub fn Projects() -> impl IntoView {
    // Provide a stagger context so children animate in sequence.
    let ctx = provide_stagger(0.1, 0.0);
    provide_context(ctx);
    view! {
        <section id="all-projects" class="pt-32 pb-12 md:pt-40 md:pb-20 px-6 relative overflow-hidden">
            <SectionGlow color="#8B5CF6".to_string() position=GlowPosition::TopLeft size=GlowSize::Lg opacity=0.3 animate=true />
            <div class="max-w-6xl mx-auto relative z-10">
                <Motion props=MotionProps {
                    variants: Some(header_variants()),
                    initial: Some("hidden".to_string()),
                    while_in_view: Some("show".to_string()),
                    class: Some("text-center mb-16".to_string()),
                    ..Default::default()
                }>
                    <h2 class="text-3xl md:text-5xl font-bold mb-6 tracking-tight">"All Projects"</h2>
                    <p class="text-text-secondary max-w-2xl mx-auto leading-relaxed">
                        "A showcase of hands-on work across "
                        <span class="font-mono-accent text-accent-teal">"AI automation"</span>
                        ", machine learning, and intelligent systems."
                    </p>
                </Motion>
                <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
                    {PROJECTS.iter().map(|p| {
                        let icon = p.icon;
                        view! {
                            <Motion props=MotionProps {
                                variants: Some(card_variant()),
                                initial: Some("hidden".to_string()),
                                while_in_view: Some("show".to_string()),
                                ..Default::default()
                            }>
                                <TiltCard class=Some("h-full".to_string())>
                                    <div class="group h-full p-6 rounded-2xl glass-card cursor-pointer">
                                        <div class="flex items-start justify-between mb-5">
                                            <div class="p-3 rounded-xl bg-accent-teal/10 border border-accent-teal/20">
                                                {icon(24, Some("text-accent-teal"))}
                                            </div>
                                            <span class="text-xs font-medium text-text-secondary bg-background/60 px-3 py-1 rounded-full border border-border font-mono-accent">
                                                {p.year}
                                            </span>
                                        </div>
                                        <h4 class="text-xl font-bold text-text-primary group-hover:text-gradient transition-all duration-300 mb-3">
                                            {p.title}
                                        </h4>
                                        <p class="text-sm text-text-secondary leading-relaxed mb-6">
                                            {p.description}
                                        </p>
                                        <div class="flex flex-wrap gap-2">
                                            {p.tags.iter().map(|tag| {
                                                view! {
                                                    <span class="px-3 py-1 text-xs font-medium rounded-full bg-background/60 border border-border text-text-secondary font-mono-accent">
                                                        {*tag}
                                                    </span>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    </div>
                                </TiltCard>
                            </Motion>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}