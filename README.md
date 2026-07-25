# Avneesh Jadhav — Personal Portfolio

A high-performance personal portfolio built with modern web technologies, featuring smooth scroll animations, vivid gradients, and a Swiss minimalist design system. Written entirely in **Rust**, compiled to **WebAssembly** via **Leptos**.

**Live Site:** *(add your deployed URL here)*

---

## Tech Stack

- **Language:** Rust (compiles to WebAssembly)
- **Framework:** Leptos 0.9 (CSR — client-side rendering)
- **Build Tool:** Trunk
- **Styling:** Tailwind CSS v4 (via `@tailwindcss/cli`)
- **Animations:** Hand-rolled motion core (cubic-Bézier easing, critically-damped springs, `requestAnimationFrame`-driven) — replaces Framer Motion/GSAP
- **Smooth Scroll:** Custom Lenis-style core loop (Rust reimplementation)
- **Icons:** Inline SVG components (Lucide-style + custom social icons)

---

## Getting Started

### Prerequisites

- **Rust** 1.88+ with the `wasm32-unknown-unknown` target
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- **Trunk** (build tool)
  ```bash
  cargo install --locked trunk
  # or: cargo binstall trunk
  ```
- **Node.js** (only for the Tailwind CLI, invoked via `npx` during build — no lockfile needed)

### Installation

```bash
# Dependencies are fetched automatically on first build; no separate install step.
```

### Development

```bash
trunk serve
```
Serves on `http://localhost:1420` with hot-reload.

### Build (release)

```bash
trunk build --release
```
Outputs a static site to `dist/`.

### Lint

```bash
cargo clippy --all-targets -- -D warnings
```

---

## Project Structure

```
.
├── Cargo.toml              # Leptos (csr), leptos-use, web-sys, wasm-bindgen, gloo
├── Cargo.lock
├── Trunk.toml              # Trunk build config + Tailwind CLI pre-build hook
├── index.html              # App shell (head meta, OG, fonts, Trunk link tags)
├── public/                 # Static assets (avatar, CV, images, favicon, robots)
└── src/
    ├── lib.rs              # App() — root layout, wires SmoothScrollProvider + sections
    ├── main.rs             # fn main() -> mount_to_body(App)
    ├── components/         # Leptos components (sections + UI)
    │   ├── hero.rs about.rs philosophy.rs skills.rs
    │   ├── featured_projects.rs projects.rs experience.rs
    │   ├── certifications.rs contact.rs footer.rs
    │   ├── navbar.rs preloader.rs scroll_progress.rs
    │   ├── section_glow.rs tilt_card.rs icons.rs
    │   └── smooth_scroll.rs
    ├── hooks/              # Custom Leptos hooks
    │   └── use_scroll_velocity.rs
    ├── motion/             # Hand-rolled animation core (the WASM compute)
    │   ├── easing.rs       # Cubic-Bézier solver + canonical easing curves
    │   ├── spring.rs       # Critically-damped spring physics
    │   ├── motion_value.rs # MotionValue / useSpring / useTransform
    │   ├── variants.rs     # Variant / Variants / Transition + factories
    │   ├── motion.rs       # <Motion> component (initial/animate/whileInView/stagger)
    │   ├── raf_loop.rs     # Single shared requestAnimationFrame loop
    │   └── animate_presence.rs
    ├── styles/
    │   └── input.css       # Tailwind v4 @theme + design system utilities
    └── utils/
        └── raf.rs          # performance.now() / rAF helpers
```

---

## Design System

The site uses a custom Swiss Minimalist + Vivid Gradients palette defined in `src/styles/input.css`:

| Token | Value | Usage |
|-------|-------|-------|
| `--color-background` | `#FAFAFA` | Page background |
| `--color-surface` | `#FFFFFF` | Cards, panels |
| `--color-text-primary` | `#0A0A0A` | Headings, body |
| `--color-text-secondary` | `#555555` | Captions, labels |
| `--color-accent` | `#FF3366` | Primary accent (pink) |
| `--color-accent-teal` | `#00D4AA` | Secondary accent |
| `--color-accent-indigo` | `#6366F1` | Tertiary accent |

---

## Deployment

This project builds to a static site in the `dist/` folder via `trunk build --release`. Recommended platforms:

- **Vercel** — `vercel --prod` (set build command to `trunk build --release`, output `dist`)
- **Netlify** — Drag & drop `dist/` folder
- **GitHub Pages** — Use GitHub Actions to run `trunk build --release` and publish `dist/`
- **Cloudflare Pages** — Build command `trunk build --release`, output directory `dist`

> **Note:** Do not commit the `dist/` folder to version control. It is generated at build time.

---

## License

© 2026 Avneesh Jadhav. All rights reserved.