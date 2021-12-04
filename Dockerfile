# We use the latest Rust stable release as base image
# Builder image
FROM rust:1.53.0 AS builder
# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /app
# Copy all files from our working environment to our Docker image
COPY . .
ENV SQLX_OFFLINE true
# Let's build our binary!
# We'll use the release profile to make it faaaast
RUN cargo build --release
#Runtime stage
FROM rust:1.53.0 AS Runtime
WORKDIR /app
COPY --from=builder /app/target/release/devapi devapi
COPY configuration configuration
ENV APP_ENVIRONMENT production

# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./devapi"]
