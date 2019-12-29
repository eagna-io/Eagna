FROM rust:1.40.0 as builder
ENV USER=root 
# MODE *MUST* be one of "debug" or "release"
ARG BUILD_MODE
ENV BUILD_MODE ${BUILD_MODE}
WORKDIR /usr/src
RUN cargo new eagna
WORKDIR /usr/src/eagna
RUN touch src/lib.rs 
# Copy only manifest file
COPY Cargo.toml Cargo.lock appengine/build-with-env.sh ./
# Build only dependencies
RUN ./build-with-env.sh
# Copy whole source code
COPY src src
RUN touch src/lib.rs src/main.rs
## Build whole source code
RUN ./build-with-env.sh && cp target/${BUILD_MODE}/eagna /usr/bin/eagna

FROM debian:buster-slim
WORKDIR /home/
RUN apt-get update && apt-get install -y libpq-dev
COPY --from=builder /usr/bin/eagna ./
RUN adduser takatom
USER takatom
ENTRYPOINT ["./eagna"]