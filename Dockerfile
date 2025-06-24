FROM ubuntu:latest AS rust
RUN apt-get update && apt-get install -y build-essential pkg-config libssl-dev curl ca-certificates
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile default
ENV PATH="/root/.cargo/bin:$PATH"

FROM rust AS chef
RUN cargo install cargo-chef
WORKDIR /app
  
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json
  
FROM chef AS builder
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli@0.7.0-alpha.1 --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN dx bundle --package app --platform web --profile release
  
FROM chef AS runtime
COPY --from=builder /app/target/dx/app/release/web/ /usr/local/app/app/
ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080
WORKDIR /usr/local/app