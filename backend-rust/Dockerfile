FROM rust:1.35.0 as builder
ENV USER=root 
# MODE *MUST* be one of "develop" or "release"
ARG BUILD_MODE
ENV BUILD_MODE ${BUILD_MODE}
RUN apt-get update
WORKDIR /usr/src
RUN cargo new eagna
WORKDIR /usr/src/eagna
RUN touch src/lib.rs 
# Copy only manifest file
COPY Cargo.toml Cargo.lock build-with-env.sh ./
# Build only dependencies
RUN ./build-with-env.sh
# Copy whole source code
COPY src src
RUN touch src/lib.rs src/main.rs
## Build whole source code
RUN ./build-with-env.sh

FROM rust:1.35.0
WORKDIR /root/
COPY --from=builder /usr/src/eagna/target/debug/eagna ./
CMD ["./eagna"]
