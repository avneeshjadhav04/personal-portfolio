//! Footer — port of `src/components/Footer.tsx`.

use leptos::prelude::*;

const NAV_LINKS: &[&str] = &["About", "Projects", "Skills", "Experience", "Certifications", "Contact"];

#[component]
pub fn Footer() -> impl IntoView {
    let year = js_sys::Date::new_0().get_full_year();
    view! {
        <footer class="relative bg-text-primary border-t border-border overflow-hidden text-surface">
            <div class="max-w-7xl mx-auto px-6 md:px-10 py-20 md:py-32">
                <div class="grid md:grid-cols-4 gap-16 md:gap-12 mb-24">
                    <div class="md:col-span-2">
                        <h3 class="text-4xl md:text-5xl font-bold tracking-tighter uppercase mb-6">
                            "Avneesh "
                            <span class="text-gradient">"Jadhav."</span>
                        </h3>
                        <p class="text-lg text-surface/70 leading-relaxed max-w-md font-light">
                            "AI Engineer building production systems at the intersection of machine learning, automation, and functional design."
                        </p>
                    </div>
                    <div>
                        <h4 class="text-[12px] font-mono-accent uppercase tracking-[0.3em] text-surface/50 mb-8 block border-l-2 border-accent pl-4">
                            "Navigation"
                        </h4>
                        <ul class="space-y-4">
                            {NAV_LINKS.iter().map(|link| {
                                let href = format!("#{}", link.to_lowercase());
                                view! {
                                    <li>
                                        <a
                                            href=href
                                            class="text-base text-surface hover:text-accent transition-colors font-medium uppercase tracking-wide"
                                        >
                                            {*link}
                                        </a>
                                    </li>
                                }
                            }).collect::<Vec<_>>()}
                        </ul>
                    </div>
                    <div>
                        <h4 class="text-[12px] font-mono-accent uppercase tracking-[0.3em] text-surface/50 mb-8 block border-l-2 border-accent pl-4">
                            "Connect"
                        </h4>
                        <ul class="space-y-4">
                            <li>
                                <a
                                    href="https://www.linkedin.com/in/avneeshjadhav/"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="text-base text-surface hover:text-accent transition-colors font-medium uppercase tracking-wide"
                                >
                                    "LinkedIn"
                                </a>
                            </li>
                            <li>
                                <a
                                    href="https://github.com/avneeshjadhav04"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="text-base text-surface hover:text-accent transition-colors font-medium uppercase tracking-wide"
                                >
                                    "GitHub"
                                </a>
                            </li>
                            <li>
                                <a
                                    href="mailto:avneeshjadhav1@gmail.com"
                                    class="text-base text-surface hover:text-accent transition-colors font-medium uppercase tracking-wide"
                                >
                                    "Email"
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>
                <div class="flex flex-col md:flex-row items-center justify-between gap-6 pt-10 border-t border-surface/20">
                    <p class="text-xs text-surface/50 font-mono-accent uppercase tracking-widest">
                        {format!("© {} Avneesh Jadhav. All rights reserved.", year)}
                    </p>
                    <div class="flex items-center gap-3 bg-surface/10 px-4 py-2">
                        <span class="w-2 h-2 bg-green-500 rounded-full animate-pulse" />
                        <span class="text-[10px] font-mono-accent uppercase tracking-widest text-surface/80">
                            "Website Operational"
                        </span>
                    </div>
                </div>
            </div>
        </footer>
    }
}