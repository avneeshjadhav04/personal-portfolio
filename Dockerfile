# Build stage: compile WASM with Trunk
FROM rust:1.88 AS build

WORKDIR /app

# Install wasm target
RUN rustup target add wasm32-unknown-unknown

# Download pre-built Trunk binary (avoids compiling lightningcss which has cssparser version conflicts on Rust 1.88)
RUN wget -q https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz \
    && tar xzf trunk-x86_64-unknown-linux-gnu.tar.gz \
    && mv trunk /usr/local/bin/trunk \
    && rm trunk-x86_64-unknown-linux-gnu.tar.gz

# Download Tailwind CSS v4 standalone CLI (no Node.js needed)
RUN wget -q https://github.com/tailwindlabs/tailwindcss/releases/download/v4.0.0/tailwindcss-linux-x64 \
    -O /usr/local/bin/tailwindcss && chmod +x /usr/local/bin/tailwindcss

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source
COPY src ./src
COPY index.html Trunk.toml ./
COPY public ./public
COPY src/styles/input.css ./src/styles/input.css

# Build Tailwind CSS then the WASM bundle
RUN tailwindcss -i src/styles/input.css -o src/styles/output.css --minify
RUN mkdir -p /root/.cache/trunk/wasm-opt-version_123/bin && \
    printf '#!/bin/sh\ncp "$1" "$2"\n' > /root/.cache/trunk/wasm-opt-version_123/bin/wasm-opt && \
    chmod +x /root/.cache/trunk/wasm-opt-version_123/bin/wasm-opt && \
    trunk build --release

# Runtime stage: nginx serving the static dist
FROM nginx:alpine

COPY --from=build /app/dist /usr/share/nginx/html

EXPOSE 8080

CMD ["nginx", "-g", "daemon off;"]
