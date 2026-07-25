//! Lucide-style SVG icons used across the portfolio.
//!
//! Each is a Leptos component taking a `size` prop (default 24) and an optional
//! `class`. Ports the subset of `lucide-react` imports used in the original
//! `src/components/`: Bot, Stethoscope, Phone, Calendar, Globe, Home,
//! CreditCard, Cpu, Menu, X.
//!
//! Plus the custom social icons from `src/components/Icons.tsx`:
//! LinkedInIcon, GitHubIcon, EmailIcon.

use leptos::prelude::*;

fn stroke_props(size: i32) -> (&'static str, &'static str) {
    ("viewBox", "0 0 24 24")
}

/// Shared wrapper for stroke-style icons (lucide family).
fn lucide(
    size: i32,
    class: Option<&str>,
    paths: &'static str,
    fill: &'static str,
    stroke: bool,
) -> impl IntoView {
    let _ = stroke_props(size);
    view! {
        <svg
            width=size
            height=size
            viewBox="0 0 24 24"
            fill=fill
            stroke={if stroke { Some("currentColor") } else { None }}
            stroke_width={if stroke { Some(2) } else { None }}
            stroke_linecap="round"
            stroke_linejoin="round"
            class=class.unwrap_or("")
            inner_html=paths
        />
    }
}

/// Shared wrapper for fill-style icons (custom social icons).
fn icon_fill(size: i32, class: Option<&str>, path: &'static str) -> impl IntoView {
    view! {
        <svg
            width=size
            height=size
            viewBox="0 0 24 24"
            fill="currentColor"
            class=class.unwrap_or("")
            inner_html=path
        />
    }
}

// ============================================================
// Lucide stroke icons.
// ============================================================

#[component]
pub fn Bot(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<path d="M12 8V4H8"/><rect width="16" height="12" x="4" y="8" rx="2"/><path d="M2 14h2"/><path d="M20 14h2"/><path d="M15 13v2"/><path d="M9 13v2"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn Stethoscope(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<path d="M4.8 2.3A.3.3 0 1 0 5 2H4a2 2 0 0 0-2 2v5a6 6 0 0 0 6 6v0a6 6 0 0 0 6-6V4a2 2 0 0 0-2-2h-1a.2.2 0 1 0 .3.3"/><path d="M8 15v1a6 6 0 0 0 6 6v0a6 6 0 0 0 6-6v-4"/><circle cx="20" cy="10" r="2"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn Phone(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn Calendar(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<rect width="18" height="18" x="3" y="4" rx="2"/><path d="M16 2v4"/><path d="M8 2v4"/><path d="M3 10h18"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn Globe(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/><path d="M2 12h20"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn Home(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><path d="M9 22V12h6v10"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn CreditCard(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<rect width="20" height="14" x="2" y="5" rx="2"/><path d="M2 10h20"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn Cpu(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<path d="M12 20v2"/><path d="M12 2v2"/><path d="M17 20v2"/><path d="M17 2v2"/><path d="M2 12h2"/><path d="M2 17h2"/><path d="M2 7h2"/><path d="M20 12h2"/><path d="M20 17h2"/><path d="M20 7h2"/><path d="M7 20v2"/><path d="M7 2v2"/><rect width="7" height="7" x="7" y="7" rx="1"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn Menu(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(size, class, r#"<line x1="4" x2="20" y1="12" y2="12"/><line x1="4" x2="20" y1="6" y2="6"/><line x1="4" x2="20" y1="18" y2="18"/>"#, "none", true)
}

#[component]
pub fn X(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(size, class, r#"<path d="M18 6 6 18"/><path d="M6 6l12 12"/>"#, "none", true)
}

// ============================================================
// Additional lucide icons used by Skills / Certifications / Experience / Contact.
// ============================================================

#[component]
pub fn Briefcase(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<path d="M16 20V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16"/><rect width="20" height="14" x="2" y="8" rx="2"/><path d="M2 17h20"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn Award(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<path d="m15.477 12.89 1.515 8.526a.5.5 0 0 1-.81.47l-3.58-2.687a1 1 0 0 0-1.197 0l-3.586 2.686a.5.5 0 0 1-.81-.469l1.514-8.526"/><circle cx="12" cy="8" r="6"/><path d="M12 2v2"/><path d="M9.5 8h5"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn GraduationCap(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<path d="M21.42 10.922a1 1 0 0 0-.019-1.838L12.83 5.18a2 2 0 0 0-1.66 0L2.6 9.08a1 1 0 0 0 0 1.832l8.57 3.908a2 2 0 0 0 1.66 0z"/><path d="M22 10v6"/><path d="M6 12.97V20a6 6 0 0 0 12 0v-7"/><path d="M22 10v6"/><path d="M12 12v6"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn Cloud(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<path d="M17.5 19H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9Z"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn Database(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M3 5V19A9 3 0 0 0 21 19V5"/><path d="M3 12A9 3 0 0 0 21 12"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn BookOpen(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<path d="M12 7v14"/><path d="M3 18a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h5a4 4 0 0 1 4 4 4 4 0 0 1 4-4h5a1 1 0 0 1 1 1v13a1 1 0 0 1-1 1h-6a3 3 0 0 0-3 3 3 3 0 0 0-3-3z"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn Sparkles(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<path d="M9.937 15.5A2 2 0 0 0 8.5 14.063l-6.135-1.582a.5.5 0 0 1 0-.962L8.5 9.936A2 2 0 0 0 9.937 8.5l1.582-6.135a.5.5 0 0 1 .963 0L14.063 8.5A2 2 0 0 0 15.5 9.937l6.135 1.581a.5.5 0 0 1 0 .964L15.5 14.063a2 2 0 0 0-1.437 1.437l-1.582 6.135a.5.5 0 0 1-.963 0z"/><path d="M20 3v4"/><path d="M22 5h-4"/><path d="M4 17v2"/><path d="M5 18H3"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn Layers(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<path d="M12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.832l8.57 3.908a2 2 0 0 0 1.66 0l8.58-3.908a1 1 0 0 0 0-1.832z"/><path d="M2 12.5l8.57 3.908a2 2 0 0 0 1.66 0L21 12.5"/><path d="M2 17.5l8.57 3.908a2 2 0 0 0 1.66 0L21 17.5"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn Terminal(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<path d="m6 16 4-4-4-4"/><path d="M2 16h10"/><rect width="14" height="14" x="8" y="8" rx="2"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn Box(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<path d="M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z"/><path d="m3.3 7 8.7 5 8.7-5"/><path d="M12 22V12"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn GitBranch(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<line x1="6" x2="6" y1="3" y2="15"/><circle cx="18" cy="6" r="3"/><circle cx="6" cy="18" r="3"/><path d="M18 9a9 9 0 0 1-9 9"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn Server(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<rect width="20" height="8" x="2" y="2" rx="2" ry="2"/><rect width="20" height="8" x="2" y="14" rx="2" ry="2"/><line x1="6" x2="6.01" y1="6" y2="6"/><line x1="6" x2="6.01" y1="18" y2="18"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn Network(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<rect x="16" y="16" width="6" height="6" rx="1"/><rect x="2" y="16" width="6" height="6" rx="1"/><rect x="9" y="2" width="6" height="6" rx="1"/><path d="M5 16v-3a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v3"/><path d="M12 12V9"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn Package(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<path d="m11.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.832l8.57 3.908a2 2 0 0 0 1.66 0l8.58-3.908a1 1 0 0 0 0-1.832z"/><path d="M2 12.5l8.57 3.908a2 2 0 0 0 1.66 0L21 12.5"/><path d="M2 17.5l8.57 3.908a2 2 0 0 0 1.66 0L21 17.5"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn Wrench(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn RefreshCw(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/><path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/><path d="M3 21v-5h5"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn Languages(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<path d="m5 8 6 6"/><path d="m4 14 6-6 2-3"/><path d="M2 5h12"/><path d="M7 2h1"/><path d="m22 22-5-10-5 10"/><path d="M14 18h6"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn Brain(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<path d="M12 5a3 3 0 1 0-5.997.125 4 4 0 0 0-2.526 5.77 4 4 0 0 0 .556 6.588A4 4 0 1 0 12 18Z"/><path d="M12 5a3 3 0 1 1 5.997.125 4 4 0 0 1 2.526 5.77 4 4 0 0 1-.556 6.588A4 4 0 1 1 12 18Z"/><path d="M15 13a4.5 4.5 0 0 1-3-4 4.5 4.5 0 0 1-3 4"/>"#,
        "none",
        true,
    )
}

#[component]
pub fn MapPin(size: i32, class: Option<&str>) -> impl IntoView {
    lucide(
        size,
        class,
        r#"<path d="M20 10c0 4.993-5.539 10.193-7.399 11.799a1 1 0 0 1-1.202 0C9.539 20.193 4 14.993 4 10a8 8 0 0 1 16 0"/><circle cx="12" cy="10" r="3"/>"#,
        "none",
        true,
    )
}

// NOTE: `Phone` (lucide stroke icon) is defined above; the custom `PhoneIcon`
// name in lucide-react is the same stroke icon. We reuse the `Phone` component.

// ============================================================
// Custom social icons (from src/components/Icons.tsx).
// ============================================================

#[component]
pub fn LinkedInIcon(size: i32, class: Option<&str>) -> impl IntoView {
    icon_fill(
        size,
        class,
        r#"<path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 01-2.063-2.065 2.064 2.064 0 112.063 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/>"#,
    )
}

#[component]
pub fn EmailIcon(size: i32, class: Option<&str>) -> impl IntoView {
    view! {
        <svg
            width=size
            height=size
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke_width=2
            stroke_linecap="round"
            stroke_linejoin="round"
            class=class.unwrap_or("")
            inner_html=r#"<rect width="20" height="16" x="2" y="4" rx="2"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/>"#
        />
    }
}

#[component]
pub fn GitHubIcon(size: i32, class: Option<&str>) -> impl IntoView {
    icon_fill(
        size,
        class,
        r#"<path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"/>"#,
    )
}