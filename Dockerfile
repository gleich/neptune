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

# set timezone to New York
RUN apt-get update && \
    apt-get install -yq tzdata && \
    ln -fs /usr/share/zoneinfo/America/New_York /etc/localtime && \
    dpkg-reconfigure -f noninteractive tzdata
ENV TZ="America/New_York"

# installing gs
RUN apt-get install -yq ghostscript

CMD ["neptune"]
