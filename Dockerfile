# We use the latest Rust stable release as base image
# Builder image
# docker image with rust and chef installed to generate deps lock file 
FROM lukemathwalker/cargo-chef:latest-rust-1.53.0 as chef
# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /app
FROM chef as planner
# Copy all files from our working environment to our Docker image
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .
ENV SQLX_OFFLINE true
# Let's build our binary!
# We'll use the release profile to make it faaaast
RUN cargo build --release --bin devapi
#Runtime stage
FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/devapi devapi
COPY configuration configuration
ENV APP_ENVIRONMENT production

# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./devapi"]
