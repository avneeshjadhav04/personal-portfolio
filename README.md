# Avneesh Jadhav — Personal Portfolio

A personal portfolio with a Swiss minimalist design system — built as a **static site** for maximum performance.

No frameworks. No build step. No dependencies. One `index.html` containing all markup, styles, and scripts, plus a handful of root-level SEO/agent files (`sitemap.xml`, `robots.txt`, `llms.txt`, `ai.txt`, `.well-known/ai-plugin.json`).

---

## Stack

- **HTML5** — semantic, all sections pre-rendered inline (content is fully visible with JS disabled; JavaScript is progressive enhancement only)
- **CSS3** — custom design tokens, `@keyframes` animations, `IntersectionObserver`-triggered reveals, gradient mesh, tilt cards
- **Vanilla JavaScript (~6 KB)** — scroll progress, nav state, mobile menu, smooth scroll, word-by-word text reveal, 3D tilt, mouse parallax (never required for content)
- **Google Fonts** — Archivo Black, Space Grotesk (400–700) and Space Mono (400/700), trimmed to the weights actually used
- **SEO** — single `h1`, crawlable anchor nav, `author`/`robots` meta, JSON-LD graph (`Person` + `WebSite` + `ProfilePage` + `ItemList`), `sitemap.xml`, `robots.txt`, `rel="me"` profile links, `<noscript>` video fallbacks, `<time>` dates
- **AI-agent ready** — `llms.txt` (plain-text site summary), `ai.txt` (agent usage note), `.well-known/ai-plugin.json` (identity manifest)

## Why no framework?

The site is 100% static content with ~15 sections. Dropping React, Motion, Lenis, and the entire Node.js toolchain cuts the JS payload from ~200 KB to ~6 KB and makes First Contentful Paint effectively instant.

## Run it

Open `index.html` directly, or serve statically:

```bash
python3 -m http.server 8000
# → http://localhost:8000
```

## Deploy

Upload the folder to any static host (GitHub Pages, Netlify, Vercel, Cloudflare Pages, S3). `404.html`, `robots.txt`, `sitemap.xml`, `llms.txt`, `ai.txt`, and `.well-known/ai-plugin.json` are included.

## When you get a domain

1. Use absolute URLs in `sitemap.xml` (`https://yourdomain.com/…`) and add back `Sitemap: https://yourdomain.com/sitemap.xml` to `robots.txt`.
2. Add `<link rel="canonical" href="https://yourdomain.com/">` to `index.html`.
3. Switch `og:url` / `og:image` (and Twitter equivalents) to absolute `https://yourdomain.com/…` URLs.
4. Add the absolute URL as `url` on the JSON-LD `WebSite` node.
5. Verify in Google Search Console and submit the sitemap.
