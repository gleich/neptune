FROM rust as BUILDER

COPY . app
WORKDIR app

RUN cargo build --release

FROM golang:1.18

COPY --from=builder /app/target/release/neptune .
COPY --from=builder /app/assets .

# install rmapi
RUN git clone https://github.com/juruen/rmapi
WORKDIR rmapi
RUN go build . \
    && mv rmapi /bin/
WORKDIR /

CMD ["neptune"]
