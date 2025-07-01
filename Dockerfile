FROM ubuntu:latest AS chef
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    pkg-config \
    libssl-dev \
    build-essential \
    gcc \
    libc6-dev \
    git \
    && rm -rf /var/lib/apt/lists/*
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH=$CARGO_HOME/bin:/usr/local/bin:$PATH
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable --profile minimal
RUN . $CARGO_HOME/env
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-chef wasm-bindgen-cli@0.2.100 wasm-pack wasm-opt
RUN curl -LO https://github.com/tailwindlabs/tailwindcss/releases/download/v4.1.11/tailwindcss-linux-arm64 \
    && chmod +x tailwindcss-linux-arm64 \
    && mv tailwindcss-linux-arm64 /usr/local/bin/tailwindcss
WORKDIR /dioxus-app
  
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json
  
FROM chef AS builder
RUN cargo install --git https://github.com/Distortedlogic/dioxus.git dioxus-cli --force --root $CARGO_HOME --features no-downloads

COPY --from=planner /dioxus-app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN dx bundle -p app --release
  
FROM chef AS runtime
COPY --from=builder /dioxus-app/dist/ /usr/local/web/
ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080
WORKDIR /usr/local/web
ENTRYPOINT ["/usr/local/web/app"]