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

# Install Node.js for Tailwind CSS CLI
RUN apt-get update && apt-get install -y nodejs npm && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source
COPY src ./src
COPY index.html Trunk.toml ./
COPY public ./public
COPY src/styles/input.css ./src/styles/input.css

# Build Tailwind CSS then the WASM bundle
RUN npx --yes @tailwindcss/cli@4.3.3 -i src/styles/input.css -o src/styles/output.css --minify
RUN trunk build --release

# Runtime stage: nginx serving the static dist
FROM nginx:alpine

COPY --from=build /app/dist /usr/share/nginx/html

EXPOSE 8080

CMD ["nginx", "-g", "daemon off;"]
