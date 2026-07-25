//! Navbar — port of `src/components/Navbar.tsx`.
//!
//! Sentinel-driven `scrolled` state, desktop links + CV download + socials, and
//! a mobile menu with an enter/exit animation.

use leptos::html::Div;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{IntersectionObserver, IntersectionObserverInit};

use crate::components::icons::{EmailIcon, GitHubIcon, LinkedInIcon, Menu, X};
use crate::components::smooth_scroll::{ScrollTarget, use_smooth_scroll};

const NAV_LINKS: &[(&str, &str)] = &[
    ("About", "#about"),
    ("Skills", "#skills"),
    ("Projects", "#projects"),
    ("Experience", "#experience"),
    ("Certifications", "#certifications"),
    ("Contact", "#contact"),
];

#[component]
pub fn Navbar() -> impl IntoView {
    let scrolled = RwSignal::new(false);
    let mobile_open = RwSignal::new(false);
    let sentinel_ref = NodeRef::<Div>::new();

    // Sentinel-driven scrolled detection.
    Effect::new(move || {
        let Some(el) = sentinel_ref.get() else { return };
        let scrolled_sig = scrolled;
        let cb = Closure::<dyn FnMut(Vec<web_sys::IntersectionObserverEntry>)>::new(
            move |entries: Vec<web_sys::IntersectionObserverEntry>| {
                if let Some(entry) = entries.into_iter().next() {
                    scrolled_sig.set(!entry.is_intersecting());
                }
            },
        );
        let init = IntersectionObserverInit::new();
        init.set_threshold_f64(0.0);
        if let Ok(obs) = IntersectionObserver::new_with_options(
            cb.as_ref().unchecked_ref(),
            &init,
        ) {
            obs.observe(&el);
            std::mem::forget(cb);
        }
    });

    let smooth = use_smooth_scroll();
    let smooth2 = smooth.clone();

    let handle_nav_click = move |href: String| {
        if let Some(s) = &smooth {
            (s.scroll_to)(ScrollTarget::Selector(href), -80, 1.2);
        }
        mobile_open.set(false);
    };

    let scroll_to_top = move || {
        if let Some(s) = &smooth2 {
            (s.scroll_to)(ScrollTarget::Pixels(0.0), 0, 1.2);
        }
    };

    let nav_classes = move || {
        if scrolled.get() {
            "fixed top-0 left-0 right-0 z-50 transition-all duration-500 bg-surface/90 backdrop-blur-md border-b border-border shadow-sm py-4"
        } else {
            "fixed top-0 left-0 right-0 z-50 transition-all duration-500 bg-transparent py-6"
        }
    };

    let handle_nav_click_clone = handle_nav_click.clone();

    view! {
        <>
            <div
                node_ref=sentinel_ref
                class="absolute top-[80dvh] left-0 w-full h-[1px] pointer-events-none"
                aria_hidden="true"
            />
            <nav class=nav_classes>
                <div class="max-w-7xl mx-auto px-6 md:px-10 flex items-center justify-between">
                    // Logo
                    <button
                        on:click=move |_| scroll_to_top()
                        class="text-xl font-bold tracking-tighter text-text-primary uppercase bg-transparent border-none cursor-pointer hover:opacity-70 transition-opacity"
                    >
                        "Avneesh."
                    </button>

                    // Desktop links
                    <div class="hidden md:flex items-center gap-8">
                        {NAV_LINKS.iter().map(|(name, href)| {
                            let href = href.to_string();
                            let handle = handle_nav_click.clone();
                            view! {
                                <button
                                    on:click=move |_| handle(href.clone())
                                    class="text-sm font-medium tracking-wide uppercase text-text-secondary hover:text-text-primary transition-colors duration-300 relative group bg-transparent border-none cursor-pointer"
                                >
                                    {*name}
                                    <span class="absolute -bottom-1 left-0 w-0 h-[1px] bg-text-primary transition-all duration-300 group-hover:w-full" />
                                </button>
                            }
                        }).collect::<Vec<_>>()}
                    </div>

                    // CTA + socials (desktop)
                    <div class="hidden md:flex items-center gap-6">
                        <a
                            href="/AI%20Engineer-%20Avneesh%20Jadhav%20CV.pdf"
                            download="AI Engineer- Avneesh Jadhav CV.pdf"
                            class="inline-flex items-center gap-2 px-6 py-2.5 rounded-none border border-text-primary text-xs font-semibold uppercase tracking-widest text-text-primary hover:bg-text-primary hover:text-surface transition-colors duration-300"
                        >
                            "Download CV"
                        </a>
                        <div class="flex items-center gap-4">
                            <a
                                href="https://www.linkedin.com/in/avneeshjadhav/"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-text-secondary hover:text-text-primary transition-colors duration-300"
                                aria_label="LinkedIn"
                            >
                                {LinkedInIcon(20, None)}
                            </a>
                            <a
                                href="https://github.com/avneeshjadhav04"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-text-secondary hover:text-text-primary transition-colors duration-300"
                                aria_label="GitHub"
                            >
                                {GitHubIcon(20, None)}
                            </a>
                            <a
                                href="mailto:avneeshjadhav1@gmail.com"
                                class="text-text-secondary hover:text-text-primary transition-colors duration-300"
                                aria_label="Email"
                            >
                                {EmailIcon(20, None)}
                            </a>
                        </div>
                    </div>

                    // Mobile toggle
                    <button
                        class="md:hidden text-text-primary p-2 focus:outline-none"
                        on:click=move |_| mobile_open.set(!mobile_open.get())
                        aria_label="Toggle menu"
                    >
                        {move || {
                            if mobile_open.get() {
                                view! {                                 {X(24, None)} }.into_any()
                            } else {
                                view! {                                 {Menu(24, None)} }.into_any()
                            }
                        }}
                    </button>
                </div>

                // Mobile menu
                <div class="overflow-hidden transition-all duration-300 md:hidden">
                    {move || {
                        if mobile_open.get() {
                            Some(view! {
                                <div
                                    class="absolute top-full left-0 w-full bg-surface border-b border-border p-6 shadow-xl"
                                    style="animation: navMenuIn 0.3s ease-out forwards;"
                                >
                                    <div class="flex flex-col gap-6">
                                        {NAV_LINKS.iter().map(|(name, href)| {
                                            let href = href.to_string();
                                            let handle = handle_nav_click_clone.clone();
                                            view! {
                                                <button
                                                    on:click=move |_| handle(href.clone())
                                                    class="text-2xl font-bold uppercase tracking-tighter text-text-primary hover:text-accent transition-colors text-left bg-transparent border-none cursor-pointer"
                                                >
                                                    {*name}
                                                </button>
                                            }
                                        }).collect::<Vec<_>>()}
                                        <a
                                            href="/AI%20Engineer-%20Avneesh%20Jadhav%20CV.pdf"
                                            download="AI Engineer- Avneesh Jadhav CV.pdf"
                                            class="mt-4 inline-flex items-center justify-center gap-2 px-6 py-4 rounded-none bg-text-primary text-surface font-semibold uppercase tracking-widest"
                                        >
                                            "Download CV"
                                        </a>
                                    </div>
                                </div>
                            })
                        } else {
                            None
                        }
                    }}
                </div>
            </nav>
        </>
    }
}