# Build stage
FROM rust:latest AS builder
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
WORKDIR /app
COPY . .
RUN trunk build --release

# Serve stage
FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
