//! Skills — port of `src/components/Skills.tsx`.
//!
//! Staggered grid of skill cards with the TiltCard hover treatment.

use std::collections::HashMap;

use leptos::prelude::*;

use crate::components::icons::*;
use crate::components::section_glow::{GlowPosition, GlowSize, SectionGlow};
use crate::components::tilt_card::TiltCard;
use crate::motion::easing::EASE_SMOOTH;
use crate::motion::motion::{Motion, MotionProps};
use crate::motion::variants::{Transition, Variant};

// Each skill references an icon component by a function pointer so we can
// store them in a static array without HRTB gymnastics.
type IconFn = fn(i32, Option<&str>) -> AnyView;

struct Skill {
    name: &'static str,
    icon: IconFn,
    category: &'static str,
}

fn icon_fn<I: IntoView + 'static>(
    f: impl Fn(i32, Option<&str>) -> I + Copy + 'static,
) -> IconFn {
    move |s, c| f(s, c).into_any()
}

const SKILLS: &[Skill] = &[
    Skill { name: "Python", icon: icon_fn(Terminal::new), category: "Language" },
    Skill { name: "JavaScript", icon: icon_fn(Globe::new), category: "Language" },
    Skill { name: "Rust", icon: icon_fn(Cpu::new), category: "Language" },
    Skill { name: "C++", icon: icon_fn(Cpu::new), category: "Language" },
    Skill { name: "Machine Learning", icon: icon_fn(Cpu::new), category: "AI / ML" },
    Skill { name: "Deep Learning", icon: icon_fn(Layers::new), category: "AI / ML" },
    Skill { name: "Generative AI", icon: icon_fn(Sparkles::new), category: "AI / ML" },
    Skill { name: "Natural Language Processing", icon: icon_fn(Languages::new), category: "AI / ML" },
    Skill { name: "Large Language Models (LLMs)", icon: icon_fn(Brain::new), category: "AI / ML" },
    Skill { name: "Automation", icon: icon_fn(Brain::new), category: "Automation" },
    Skill { name: "n8n", icon: icon_fn(Network::new), category: "Automation" },
    Skill { name: "Linux", icon: icon_fn(Server::new), category: "System" },
    Skill { name: "Git", icon: icon_fn(GitBranch::new), category: "DevOps" },
    Skill { name: "Docker", icon: icon_fn(Package::new), category: "DevOps" },
    Skill { name: "DevOps", icon: icon_fn(Wrench::new), category: "DevOps" },
    Skill { name: "CI/CD", icon: icon_fn(RefreshCw::new), category: "DevOps" },
    Skill { name: "Cloud Computing", icon: icon_fn(Cloud::new), category: "Cloud" },
    Skill { name: "Data Structures", icon: icon_fn(Database::new), category: "CS" },
    Skill { name: "Algorithms", icon: icon_fn(GitBranch::new), category: "CS" },
    Skill { name: "OpenCode", icon: icon_fn(Box::new), category: "Tool" },
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
                <Motion props=MotionProps {
                    variants: Some(header_variants()),
                    initial: Some("hidden".to_string()),
                    while_in_view: Some("show".to_string()),
                    class: Some("text-center mb-16".to_string()),
                    ..Default::default()
                }>
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
                            <Motion props=MotionProps {
                                variants: Some(card_variant()),
                                initial: Some("hidden".to_string()),
                                while_in_view: Some("show".to_string()),
                                ..Default::default()
                            }>
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