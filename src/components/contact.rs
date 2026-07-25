//! Contact — port of `src/components/Contact.tsx`.

use std::collections::HashMap;

use leptos::prelude::*;

use crate::components::icons::*;
use crate::motion::easing::EASE_OUT;
use crate::motion::Motion;
use crate::motion::variants::{Transition, Variant};

type IconFn = fn(i32, Option<&'static str>) -> AnyView;

struct ContactRow {
    icon: IconFn,
    label: &'static str,
    value: &'static str,
    href: Option<&'static str>,
    external: bool,
}

const ROWS: &[ContactRow] = &[
    ContactRow {
        icon: EmailIcon,
        label: "Email",
        value: "avneeshjadhav1@gmail.com",
        href: Some("mailto:avneeshjadhav1@gmail.com"),
        external: false,
    },
    ContactRow {
        icon: Phone,
        label: "Phone",
        value: "+91 95454 57385",
        href: Some("tel:+919545457385"),
        external: false,
    },
    ContactRow {
        icon: MapPin,
        label: "Location",
        value: "Pune, Maharashtra, India",
        href: None,
        external: false,
    },
    ContactRow {
        icon: LinkedInIcon,
        label: "LinkedIn",
        value: "linkedin.com/in/avneeshjadhav",
        href: Some("https://www.linkedin.com/in/avneeshjadhav/"),
        external: true,
    },
    ContactRow {
        icon: GitHubIcon,
        label: "GitHub",
        value: "github.com/avneeshjadhav04",
        href: Some("https://github.com/avneeshjadhav04"),
        external: true,
    },
];

fn variants() -> HashMap<&'static str, Variant> {
    let mut m = HashMap::new();
    m.insert("hidden", Variant { y: Some(30.0), opacity: Some(0.0), ..Variant::new() });
    m.insert(
        "show",
        Variant {
            y: Some(0.0),
            opacity: Some(1.0),
            transition: Transition::new(0.7, EASE_OUT),
            ..Variant::new()
        },
    );
    m
}

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <section id="contact" class="pt-16 pb-24 md:pt-24 md:pb-32 px-6 relative">
            <div class="max-w-6xl mx-auto">
                <Motion variants=Some(variants()) initial="hidden" while_in_view="show" class="text-center mb-16">
                    <h2 class="text-3xl md:text-5xl font-bold mb-16 tracking-tight text-center">
                        "Get in Touch"
                    </h2>
                </Motion>
                <div class="max-w-2xl mx-auto">
                    <Motion variants=Some(variants()) initial="hidden" while_in_view="show" class="space-y-6">
                        {ROWS.iter().map(|row| {
                            let icon = row.icon;
                            view! {
                                <div class="flex items-center gap-4 p-4 rounded-xl glass-card">
                                    <div class="p-3 rounded-xl bg-accent-teal/10 border border-accent-teal/20">
                                        {icon(20, Some("text-accent-teal"))}
                                    </div>
                                    <div>
                                        <p class="text-sm text-text-secondary font-mono-accent">{row.label}</p>
                                        {if let Some(href) = row.href {
                                            let target = if row.external { "_blank" } else { "" };
                                            let rel = if row.external { "noopener noreferrer" } else { "" };
                                            view! {
                                                <a
                                                    href=href
                                                    target=target
                                                    rel=rel
                                                    class="font-medium hover:text-accent-teal transition-colors link-underline"
                                                >
                                                    {row.value}
                                                </a>
                                            }.into_any()
                                        } else {
                                            view! { <p class="font-medium">{row.value}</p> }.into_any()
                                        }}
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </Motion>
                </div>
            </div>
        </section>
    }
}