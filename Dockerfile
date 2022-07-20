FROM rust as BUILDER

COPY . app
WORKDIR app

RUN cargo build --release

FROM golang:1.18

COPY --from=builder /app/target/release/neptune /bin/
COPY --from=builder /app/assets /assets/

# install rmapi
RUN git clone https://github.com/juruen/rmapi
WORKDIR rmapi
RUN go build . \
    && mv rmapi /bin/
WORKDIR /

CMD ["neptune"]
