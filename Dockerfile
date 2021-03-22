FROM rust:1.50-buster as builder
RUN mkdir /src
WORKDIR /src
COPY . /src
RUN cargo build --release

FROM debian:buster-slim
# TINI
ENV TINI_VERSION v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini /tini
RUN chmod +x /tini
ENTRYPOINT ["/tini", "--"]
RUN apt-get -y update \
 && apt-get -y install libssl1.1

# APP
RUN mkdir /app
COPY --from=builder /src/target/release/descaler /app/descaler
WORKDIR /app
CMD ["/app/descaler"]

