FROM rust:1.38 as build

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update
RUN apt-get install -y libsqlite3-dev

# Create new project to cache dependencies
WORKDIR /usr/src
RUN USER=root cargo new --bin catnip
WORKDIR /usr/src/catnip
COPY app/Cargo.* ./

# Build dependencies
RUN cargo build --release

# Copy sources and build release
RUN rm -r src
RUN rm -r target/release/deps/catnip*
COPY app/src ./src
RUN cargo build --release


# Final base image...
FROM debian:buster-slim

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update
RUN apt-get install -y libsqlite3-dev libssl1.1 ca-certificates

WORKDIR /catnip/bin
COPY --from=build /usr/src/catnip/target/release/catnip .

WORKDIR /catnip
ENTRYPOINT ["/catnip/bin/catnip"]
