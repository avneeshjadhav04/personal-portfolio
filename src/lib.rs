pub mod components;
pub mod hooks;
pub mod motion;
pub mod utils;

use leptos::prelude::*;

use components::{
    About, Certifications, Contact, ExperienceSection, FeaturedProjects, Footer, Hero, Navbar,
    Philosophy, Preloader, Projects, ScrollProgress, Skills, SmoothScrollProvider,
};

#[component]
pub fn App() -> impl IntoView {
    let loaded = RwSignal::new(false);

    view! {
        <SmoothScrollProvider>
            <div class="relative min-h-screen bg-background text-text-primary selection:bg-accent/30">
                {move || {
                    if !loaded.get() {
                        Some(view! {
                            <Preloader on_complete=Callback::new(move |_| loaded.set(true)) />
                        })
                    } else {
                        None
                    }
                }}
                // GPU-Optimized Gradient Background Mesh
                <div class="gradient-mesh">
                    <div class="gradient-blob blob-1" />
                    <div class="gradient-blob blob-2" />
                    <div class="gradient-blob blob-3" />
                </div>
                <ScrollProgress />
                <Navbar />
                <main>
                    <div class="section-wrap"><Hero /></div>
                    <div class="section-divider" />
                    <div class="section-wrap"><About /></div>
                    <div class="section-divider" />
                    <div class="section-wrap"><Philosophy /></div>
                    <div class="section-divider" />
                    <div class="section-wrap"><Skills /></div>
                    <div class="section-divider" />
                    <div class="section-wrap"><FeaturedProjects /></div>
                    <div class="section-divider" />
                    <div class="section-wrap"><Projects /></div>
                    <div class="section-divider" />
                    <div class="section-wrap"><ExperienceSection /></div>
                    <div class="section-divider" />
                    <div class="section-wrap"><Certifications /></div>
                    <div class="section-divider" />
                    <div class="section-wrap"><Contact /></div>
                </main>
                <Footer />
            </div>
        </SmoothScrollProvider>
    }
}