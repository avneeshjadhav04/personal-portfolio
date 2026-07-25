//! `SectionGlow` — port of `src/components/SectionGlow.tsx`.
//!
//! Decorative blurred colour blob positioned absolutely behind a section.

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum GlowPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

impl GlowPosition {
    fn class(self) -> &'static str {
        match self {
            Self::TopLeft => "-top-[10vw] -left-[10vw]",
            Self::TopRight => "-top-[10vw] -right-[10vw]",
            Self::BottomLeft => "-bottom-[10vw] -left-[10vw]",
            Self::BottomRight => "-bottom-[10vw] -right-[10vw]",
            Self::Center => "top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum GlowSize {
    Sm,
    Md,
    Lg,
    Xl,
}

impl GlowSize {
    fn class(self) -> &'static str {
        match self {
            Self::Sm => "w-[25vw] h-[25vw] min-w-[200px] min-h-[200px]",
            Self::Md => "w-[35vw] h-[35vw] min-w-[280px] min-h-[280px]",
            Self::Lg => "w-[45vw] h-[45vw] min-w-[360px] min-h-[360px]",
            Self::Xl => "w-[55vw] h-[55vw] min-w-[440px] min-h-[440px]",
        }
    }
}

#[component]
pub fn SectionGlow(
    color: String,
    position: GlowPosition,
    size: GlowSize,
    opacity: f64,
    animate: bool,
) -> impl IntoView {
    view! {
        <div
            class=format!(
                "absolute {} {} rounded-full pointer-events-none -z-10 {}",
                position.class(),
                size.class(),
                if animate { "animate-float" } else { "" }
            )
            style:background=color
            style:filter="blur(100px)"
            style:opacity=opacity.to_string()
            aria_hidden="true"
        />
    }
}

impl Default for GlowPosition {
    fn default() -> Self { Self::TopRight }
}
impl Default for GlowSize {
    fn default() -> Self { Self::Lg }
}