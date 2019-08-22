FROM rust:1.36.0 as builder
ENV USER=root 
# MODE *MUST* be one of "debug" or "release"
ARG BUILD_MODE
ENV BUILD_MODE ${BUILD_MODE}
RUN apt-get update
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

FROM rust:1.36.0
WORKDIR /root/
COPY --from=builder /usr/bin/eagna ./
RUN apt-get update && apt-get install -y python3-pip && pip3 install google-cloud-storage
COPY appengine/fetch_secrets.py appengine/entrypoint.sh ./
CMD ["./entrypoint.sh"]
