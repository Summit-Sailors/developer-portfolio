FROM ubuntu:latest AS chef
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    wget \
    pkg-config \
    libssl-dev \
    build-essential \
    gcc \
    libc6-dev \
    xz-utils \
    git \
    && rm -rf /var/lib/apt/lists/*
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH=$CARGO_HOME/bin:$PATH
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain 1.87.0 --profile minimal
RUN . $CARGO_HOME/env
RUN cargo install cargo-chef cargo-binstall
WORKDIR /dioxus-app
  
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json
  
FROM chef AS builder
RUN cargo binstall dioxus-cli@0.7.0-alpha.1 -y --root $CARGO_HOME

COPY --from=planner /dioxus-app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN dx bundle -p app
  
FROM chef AS runtime
COPY --from=builder /dioxus-app/dist/ /usr/local/web/
ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080
WORKDIR /usr/local/web
ENTRYPOINT ["/usr/local/web/app"]