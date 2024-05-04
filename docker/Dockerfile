FROM rust:1.78.0-bullseye as builder

LABEL maintainer="ysenih@erpya.com; EdwinBetanc0urt@outlook.com;" \
	description="A Image for start service from rust binary"

WORKDIR /opt/apps/server

COPY . . /opt/apps/server/

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/* && \
    cargo install --config net.git-fetch-with-cli=true --path . && \
    mv docker/.env /usr/local/cargo/bin/

FROM debian:bullseye-20240423

ENV \
    RUST_LOG="info" \
	PORT="7878" \
    KAFKA_ENABLED="Y" \
	KAFKA_QUEUES="menu form browser process window" \
    ALLOWED_ORIGIN="*" \
    KAFKA_HOST="0.0.0.0:9092" \
    KAFKA_GROUP="default" \
    OPENSEARCH_URL="http://localhost:9200" \
    TZ="America/Caracas"

COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server

WORKDIR /opt/apps/server

COPY --from=builder /usr/local/cargo/bin/.env /opt/apps/server/.env

RUN apt-get update && \ 
    apt-get install -y pkg-config openssl libssl-dev tzdata && \
    rm -rf /var/lib/apt/lists/* && \
    sed -i "s|info|$RUST_LOG|g" /opt/apps/server/.env && \
	sed -i "s|7878|$PORT|g" /opt/apps/server/.env && \
    sed -i "s|allowed_origin|$ALLOWED_ORIGIN|g" /opt/apps/server/.env && \
    sed -i "s|kafka_enabled|$KAFKA_ENABLED|g" /opt/apps/server/.env && \
    sed -i "s|kafka_queues|$KAFKA_QUEUES|g" /opt/apps/server/.env && \
    sed -i "s|kafka_host|$KAFKA_HOST|g" /opt/apps/server/.env && \
    sed -i "s|kafka_group|$KAFKA_GROUP|g" /opt/apps/server/.env && \
    sed -i "s|opensearch_url|$OPENSEARCH_URL|g" /opt/apps/server/.env && \
    echo "Set Timezone..." && \
	echo $TZ > /etc/timezone

CMD ["server"]
