# Build stage: compile WASM with Trunk
FROM rust:1.88 AS build

WORKDIR /app

# Install wasm target and Trunk
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk@0.21.14

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
