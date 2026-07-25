//! Certifications — port of `src/components/Certifications.tsx`.

use std::collections::HashMap;

use leptos::prelude::*;

use crate::components::icons::*;
use crate::components::tilt_card::TiltCard;
use crate::motion::easing::EASE_SMOOTH;
use crate::motion::Motion;
use crate::motion::variants::{Transition, Variant};
use crate::motion::provide_stagger;

type IconFn = fn(i32, Option<&'static str>) -> AnyView;

struct Cert {
    title: &'static str,
    org: &'static str,
    skills: &'static [&'static str],
    icon: IconFn,
    tint: &'static str,
}

const CERTS: &[Cert] = &[
    Cert {
        title: "Machine Learning Specialization",
        org: "Stanford University",
        skills: &["Supervised ML", "Advanced Learning", "Unsupervised Learning", "Recommenders"],
        icon: GraduationCap,
        tint: "from-red-500/30 to-orange-500/5",
    },
    Cert {
        title: "Fundamentals of Deep Learning",
        org: "Nvidia",
        skills: &["Neural Networks", "Deep Learning", "GPU Acceleration"],
        icon: Cpu,
        tint: "from-green-500/30 to-emerald-500/5",
    },
    Cert {
        title: "OCI Generative AI Professional",
        org: "Oracle",
        skills: &["GenAI", "Cloud AI", "LLM Deployment"],
        icon: Cloud,
        tint: "from-red-500/30 to-red-400/5",
    },
    Cert {
        title: "Oracle AI Vector Search Professional",
        org: "Oracle",
        skills: &["Vector DB", "RAG", "Semantic Search"],
        icon: Database,
        tint: "from-red-500/30 to-red-400/5",
    },
    Cert {
        title: "AMCAT Certified Software Engineer",
        org: "IT Services",
        skills: &["Software Engineering", "Problem Solving", "Code Quality"],
        icon: Award,
        tint: "from-blue-500/30 to-cyan-500/5",
    },
    Cert {
        title: "Model Context Protocol",
        org: "Anthropic",
        skills: &["MCP", "AI Integration", "Tool Use"],
        icon: Sparkles,
        tint: "from-amber-500/30 to-yellow-500/5",
    },
    Cert {
        title: "MCP Advanced Topics",
        org: "Anthropic",
        skills: &["Advanced MCP", "AI Architecture", "System Design"],
        icon: BookOpen,
        tint: "from-amber-500/30 to-yellow-500/5",
    },
    Cert {
        title: "Advanced Rust: Managing Projects",
        org: "LinkedIn",
        skills: &["Rust", "Project Management"],
        icon: Award,
        tint: "from-orange-500/30 to-amber-500/5",
    },
    Cert {
        title: "Advanced Linux: The Linux Kernel",
        org: "LinkedIn",
        skills: &["Linux", "System Administration"],
        icon: Terminal,
        tint: "from-emerald-500/30 to-teal-500/5",
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
pub fn Certifications() -> impl IntoView {
    let ctx = provide_stagger(0.08, 0.0);
    provide_context(ctx);
    view! {
        <section id="certifications" class="pt-32 pb-12 md:pt-40 md:pb-20 px-6 relative">
            <div class="max-w-6xl mx-auto">
                <Motion variants=header_variants() initial="hidden" while_in_view="show" class="text-center mb-16">
                    <h2 class="text-3xl md:text-5xl font-bold mb-6 tracking-tight">"Certifications"</h2>
                    <p class="text-text-secondary max-w-2xl mx-auto leading-relaxed">
                        "Industry-recognized certifications from "
                        <span class="font-mono-accent text-accent-teal">"Stanford"</span>
                        ", "
                        <span class="font-mono-accent text-accent-teal">"Nvidia"</span>
                        ", "
                        <span class="font-mono-accent text-accent-teal">"Oracle"</span>
                        ", "
                        <span class="font-mono-accent text-accent-teal">"LinkedIn"</span>
                        ", and "
                        <span class="font-mono-accent text-accent-teal">"Anthropic"</span>
                        "."
                    </p>
                </Motion>
                <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
                    {CERTS.iter().map(|cert| {
                        let icon = cert.icon;
                        view! {
                            <Motion variants=card_variant() initial="hidden" while_in_view="show">
                                <TiltCard class=Some("h-full".to_string())>
                                    <div class=format!(
                                        "group h-full p-6 rounded-2xl bg-surface bg-gradient-to-br {} border border-border transition-all duration-300 hover:-translate-y-1 hover:border-accent-teal/40 hover:shadow-lg",
                                        cert.tint
                                    )>
                                        <div class="flex items-start justify-between mb-5">
                                            <div class="p-3 rounded-xl bg-surface-light border border-border">
                                                {icon(24, Some("text-accent-teal"))}
                                            </div>
                                            <span class="text-xs font-medium text-text-secondary bg-surface-light px-3 py-1 rounded-full border border-border font-mono-accent">
                                                {cert.org}
                                            </span>
                                        </div>
                                        <h4 class="text-lg font-bold text-text-primary mb-4">
                                            {cert.title}
                                        </h4>
                                        <div class="flex flex-wrap gap-2">
                                            {cert.skills.iter().map(|s| {
                                                view! {
                                                    <span class="px-2.5 py-1 text-xs font-medium rounded-full bg-surface-light border border-border text-text-secondary font-mono-accent">
                                                        {*s}
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