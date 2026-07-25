//! Skills — port of `src/components/Skills.tsx`.
//!
//! Staggered grid of skill cards with the TiltCard hover treatment.

use std::collections::HashMap;

use leptos::prelude::*;

use crate::components::icons::*;
use crate::components::section_glow::{GlowPosition, GlowSize, SectionGlow};
use crate::components::tilt_card::TiltCard;
use crate::motion::easing::EASE_SMOOTH;
use crate::motion::Motion;
use crate::motion::variants::{Transition, Variant};

type IconFn = fn(i32, Option<&'static str>) -> AnyView;

struct Skill {
    name: &'static str,
    icon: IconFn,
    category: &'static str,
}

const SKILLS: &[Skill] = &[
    Skill { name: "Python", icon: Terminal, category: "Language" },
    Skill { name: "JavaScript", icon: Globe, category: "Language" },
    Skill { name: "Rust", icon: Cpu, category: "Language" },
    Skill { name: "C++", icon: Cpu, category: "Language" },
    Skill { name: "Machine Learning", icon: Cpu, category: "AI / ML" },
    Skill { name: "Deep Learning", icon: Layers, category: "AI / ML" },
    Skill { name: "Generative AI", icon: Sparkles, category: "AI / ML" },
    Skill { name: "Natural Language Processing", icon: Languages, category: "AI / ML" },
    Skill { name: "Large Language Models (LLMs)", icon: Brain, category: "AI / ML" },
    Skill { name: "Automation", icon: Brain, category: "Automation" },
    Skill { name: "n8n", icon: Network, category: "Automation" },
    Skill { name: "Linux", icon: ServerIcon, category: "System" },
    Skill { name: "Git", icon: GitBranch, category: "DevOps" },
    Skill { name: "Docker", icon: Package, category: "DevOps" },
    Skill { name: "DevOps", icon: Wrench, category: "DevOps" },
    Skill { name: "CI/CD", icon: RefreshCw, category: "DevOps" },
    Skill { name: "Cloud Computing", icon: Cloud, category: "Cloud" },
    Skill { name: "Data Structures", icon: Database, category: "CS" },
    Skill { name: "Algorithms", icon: GitBranch, category: "CS" },
    Skill { name: "OpenCode", icon: BoxIcon, category: "Tool" },
];

fn header_variants() -> HashMap<&'static str, Variant> {
    let mut m = HashMap::new();
    m.insert("hidden", Variant { y: Some(20.0), opacity: Some(0.0), ..Variant::new() });
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
        Variant { y: Some(30.0), scale: Some(0.95), opacity: Some(0.0), ..Variant::new() },
    );
    m.insert(
        "show",
        Variant {
            y: Some(0.0),
            scale: Some(1.0),
            opacity: Some(1.0),
            transition: Transition::new(0.5, EASE_SMOOTH),
            ..Variant::new()
        },
    );
    m
}

#[component]
pub fn Skills() -> impl IntoView {
    view! {
        <section id="skills" class="pt-20 pb-24 md:pt-32 md:pb-32 px-6 relative overflow-hidden">
            <SectionGlow color="#00D4AA".to_string() position=GlowPosition::BottomLeft size=GlowSize::Lg opacity=0.3 animate=true />
            <div class="max-w-6xl mx-auto relative z-10">
                <Motion variants=Some(header_variants()) initial="hidden" while_in_view="show" class="text-center mb-16">
                    <h2 class="text-3xl md:text-5xl font-bold mb-16 tracking-tight text-center">
                        "Skills & Technologies"
                    </h2>
                </Motion>
                <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                    {SKILLS.iter().map(|skill| {
                        let name = skill.name;
                        let category = skill.category;
                        let icon = skill.icon;
                        view! {
                            <Motion variants=Some(card_variant()) initial="hidden" while_in_view="show">
                                <TiltCard class=Some("h-full".to_string())>
                                    <div class="group h-full p-6 rounded-2xl glass-card breathe-border cursor-pointer">
                                        {icon(28, Some("text-text-secondary group-hover:text-accent-teal transition-colors duration-300 mb-4"))}
                                        <h4 class="font-semibold text-text-primary mb-2 text-sm md:text-base">{name}</h4>
                                        <span class="text-xs text-text-secondary font-mono-accent">{category}</span>
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