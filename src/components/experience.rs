//! Experience & Achievements — port of `src/components/Experience.tsx`.

use std::collections::HashMap;

use leptos::prelude::*;

use crate::components::icons::*;
use crate::components::section_glow::{GlowPosition, GlowSize, SectionGlow};
use crate::components::tilt_card::TiltCard;
use crate::motion::easing::EASE_SMOOTH;
use crate::motion::Motion;
use crate::motion::variants::{Transition, Variant};
use crate::motion::provide_stagger;

struct Experience {
    role: &'static str,
    company: &'static str,
    period: &'static str,
    description: &'static str,
    skills: &'static [&'static str],
    highlights: &'static [&'static str],
}

const EXPERIENCES: &[Experience] = &[
    Experience {
        role: "Technical Team Member",
        company: "Association of Computer Engineering Students",
        period: "2023 - 2024",
        description: "Active member of the university technical society, contributing to the technical growth of the student community through workshops, mentoring, and event organization.",
        skills: &["Workshops", "Mentoring", "Event Organizing", "Content Curation"],
        highlights: &[
            "Conducted workshops on technical skill improvement",
            "Mentored juniors in programming and AI/ML",
            "Organized upcoming technical events and hackathons",
            "Managed content curation for society platforms",
        ],
    },
];

struct Achievement {
    title: &'static str,
    desc: &'static str,
    image: &'static str,
}

const ACHIEVEMENTS: &[Achievement] = &[
    Achievement {
        title: "Research Paper Selection",
        desc: "Paper selected for the 11th ICTIS conference in Bangkok, Thailand",
        image: "/photo-conference.jpg",
    },
    Achievement {
        title: "HackerRank Golden Badges",
        desc: "Golden badges in Python, C++, Java, and Problem Solving",
        image: "/hackerrank.png",
    },
    Achievement {
        title: "LeetCode Milestone",
        desc: "279 problems solved demonstrating strong DSA skills",
        image: "/leetcode.png",
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

fn item_variant() -> HashMap<&'static str, Variant> {
    let mut m = HashMap::new();
    m.insert("hidden", Variant { x: Some(-40.0), opacity: Some(0.0), ..Variant::new() });
    m.insert(
        "show",
        Variant {
            x: Some(0.0),
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
        Variant { y: Some(40.0), scale: Some(0.95), opacity: Some(0.0), ..Variant::new() },
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
pub fn ExperienceSection() -> impl IntoView {
    let ctx = provide_stagger(0.2, 0.0);
    provide_context(ctx);
    view! {
        <section id="experience" class="pt-16 pb-24 md:pt-24 md:pb-32 px-6 relative overflow-hidden">
            <SectionGlow color="#6366F1".to_string() position=GlowPosition::BottomRight size=GlowSize::Lg opacity=0.3 animate=true />
            <div class="max-w-4xl mx-auto relative z-10">
                <Motion variants=header_variants() initial="hidden" while_in_view="show" class="text-center mb-16">
                    <h2 class="text-3xl md:text-5xl font-bold mb-16 tracking-tight text-center">
                        "Experience & Achievements"
                    </h2>
                </Motion>
                <div class="relative border-l border-border ml-4 md:ml-0 md:pl-0 mb-20">
                    {EXPERIENCES.iter().map(|exp| {
                        view! {
                            <Motion variants=item_variant() initial="hidden" while_in_view="show" class="mb-12 md:mb-16 last:mb-0 relative pl-8 md:pl-12">
                                <div class="absolute left-[-5px] md:left-[-5px] top-2 w-3 h-3 rounded-full bg-accent-teal shadow-[0_0_15px_rgba(20,184,166,0.6)]" />
                                <div class="flex flex-col md:flex-row md:items-center gap-2 md:gap-4 mb-3">
                                    <div class="flex items-center gap-3">
                                        <div class="p-2 rounded-lg bg-surface-light border border-border">
                                            {Briefcase(18, Some("text-accent-teal"))}
                                        </div>
                                        <h4 class="text-xl font-bold text-text-primary">{exp.role}</h4>
                                    </div>
                                    <span class="hidden md:block text-border">"|"</span>
                                    <span class="text-accent-indigo font-medium">{exp.company}</span>
                                </div>
                                <div class="flex items-center gap-2 text-sm text-text-secondary mb-4 ml-[3px] font-mono-accent">
                                    {Calendar(14, None)}
                                    {exp.period}
                                </div>
                                <p class="text-text-secondary leading-relaxed mb-4 ml-[3px]">
                                    {exp.description}
                                </p>
                                <ul class="space-y-2 ml-[3px] mb-4">
                                    {exp.highlights.iter().map(|h| {
                                        view! {
                                            <li class="flex items-start gap-2 text-sm text-text-secondary">
                                                <span class="mt-1.5 w-1.5 h-1.5 rounded-full bg-accent-teal shrink-0 shadow-[0_0_5px_rgba(20,184,166,0.5)]" />
                                                {*h}
                                            </li>
                                        }
                                    }).collect::<Vec<_>>()}
                                </ul>
                                <div class="flex flex-wrap gap-2 ml-[3px]">
                                    {exp.skills.iter().map(|s| {
                                        view! {
                                            <span class="px-3 py-1 text-xs font-medium rounded-full bg-accent-teal/10 text-accent-teal border border-accent-teal/20 font-mono-accent">
                                                {*s}
                                            </span>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </Motion>
                        }
                    }).collect::<Vec<_>>()}
                </div>
                <Motion variants=header_variants() initial="hidden" while_in_view="show" class="text-center mb-10">
                    <h3 class="text-2xl md:text-3xl font-bold mb-16 text-center">"Key Achievements"</h3>
                </Motion>
                <div class="grid md:grid-cols-3 gap-6">
                    {ACHIEVEMENTS.iter().map(|ach| {
                        view! {
                            <Motion variants=card_variant() initial="hidden" while_in_view="show">
                                <TiltCard class=Some("h-full".to_string())>
                                    <div class="group h-full rounded-2xl glass-card overflow-hidden cursor-pointer">
                                        <div class="aspect-[16/10] overflow-hidden bg-surface-light">
                                            <img
                                                src=ach.image
                                                alt=ach.title
                                                class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110"
                                            />
                                        </div>
                                        <div class="p-6">
                                            <div class="inline-flex p-3 rounded-xl bg-accent-teal/10 border border-accent-teal/20 mb-4">
                                                {Award(24, Some("text-accent-teal"))}
                                            </div>
                                            <h4 class="font-bold text-text-primary mb-2">{ach.title}</h4>
                                            <p class="text-sm text-text-secondary">{ach.desc}</p>
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