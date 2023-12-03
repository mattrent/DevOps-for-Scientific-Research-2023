FROM rust:alpine3.18 as builder

WORKDIR /wcrs
COPY . .

RUN apk add musl-dev
RUN cargo build -r
RUN mkdir -p /out && mv ./target/release/wcrs /out/


FROM alpine:3.18

COPY --from=builder /out/wcrs /bin/wcrs

ENTRYPOINT ["/bin/wcrs"]

