FROM alpine:latest as certs
RUN apk --update add ca-certificates

FROM clux/muslrust as builder

ENV RUSTFLAGS "-C opt-level=s"
COPY . /volume/
RUN cargo build --release

FROM scratch

WORKDIR /repo

COPY --from=certs /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/release-me /release-me

ENTRYPOINT ["/release-me"]
CMD ["/repo"]
