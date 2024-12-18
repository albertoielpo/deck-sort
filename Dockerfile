FROM messense/rust-musl-cross:x86_64-musl AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
# Copy source code from previous stage
COPY . .
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build & cache dependencies
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Copy source code from previous stage
COPY . .
# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl
WORKDIR /
RUN mkdir output
RUN cp /app/target/x86_64-unknown-linux-musl/release/deck-sort /output/deck-sort
CMD ["sh"]