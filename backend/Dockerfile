FROM rust:1.40.0 as builder
ENV USER=root 
# MODE *MUST* be one of "debug" or "release". Default is "debug"
ARG BUILD_MODE="debug"
ENV BUILD_MODE ${BUILD_MODE}
WORKDIR /root/crop
# backend/ 以下のソースコードを全てコピー
COPY . .
## Build whole source code
RUN ./build-with-env.sh && cp target/${BUILD_MODE}/api-server /usr/bin/crop

FROM debian:buster-slim
WORKDIR /home/
RUN apt-get update && apt-get install -y libpq-dev libssl1.1 ca-certificates
COPY --from=builder /usr/bin/crop ./
RUN adduser takatom
USER takatom
ENTRYPOINT ["./crop"]
