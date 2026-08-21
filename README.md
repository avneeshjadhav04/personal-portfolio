# Avneesh Jadhav — Personal Portfolio

A personal portfolio with a Swiss minimalist design system — built as a **single-file static site** for maximum performance.

No frameworks. No build step. No dependencies. One `index.html` containing all markup, styles, and scripts.

---

## Stack

- **HTML5** — semantic, all sections pre-rendered inline (zero JS required for content/SEO)
- **CSS3** — custom design tokens, `@keyframes` animations, `IntersectionObserver`-triggered reveals, gradient mesh, tilt cards
- **Vanilla JavaScript (~6 KB)** — scroll progress, nav state, mobile menu, smooth scroll, word-by-word text reveal, 3D tilt, mouse parallax
- **Google Fonts** — Sora (400–700) and Fira Code (400/500), trimmed to the weights actually used (render-blocking, same as before)

## Why no framework?

The site is 100% static content with ~15 sections. Dropping React, Motion, Lenis, and the entire Node.js toolchain cuts the JS payload from ~200 KB to ~6 KB and makes First Contentful Paint effectively instant.

## Run it

Open `index.html` directly, or serve statically:

```bash
python3 -m http.server 8000
# → http://localhost:8000
```

## Deploy

Upload the folder to any static host (GitHub Pages, Netlify, Vercel, Cloudflare Pages, S3). `404.html` and `robots.txt` are included.
