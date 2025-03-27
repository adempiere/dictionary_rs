FROM rust:1.85.1-bullseye as builder

LABEL maintainer="ysenih@erpya.com; EdwinBetanc0urt@outlook.com;" \
	description="Microservice with OpenSearch for ADempiere Dictionary using Rust"

ARG BUILD_VERSION "1.0.0"

WORKDIR /opt/apps/server

COPY . . /opt/apps/server/

RUN apt-get update && \
	apt-get install -y \
		pkg-config \
		libssl-dev && \
	rm -rf /var/lib/apt/lists/* && \
    cargo install --config net.git-fetch-with-cli=true --path . && \
    mv docker/.env /usr/local/cargo/bin/


FROM debian:bullseye-20240612

# The argument should be duplicated at this stage
ARG BUILD_VERSION "1.0.0"

ENV \
    RUST_LOG="info" \
	PORT="7878" \
    KAFKA_ENABLED="Y" \
	KAFKA_QUEUES="browser form process window menu_item menu_tree role" \
    ALLOWED_ORIGIN="*" \
    KAFKA_HOST="0.0.0.0:9092" \
    KAFKA_GROUP="default" \
    OPENSEARCH_URL="http://localhost:9200" \
	TZ="America/Caracas" \
	VERSION=${BUILD_VERSION}

COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server

WORKDIR /opt/apps/server

COPY --from=builder /usr/local/cargo/bin/.env /opt/apps/server/.env

RUN apt-get update && \
	apt-get install -y \
		pkg-config \
		openssl \
		libssl-dev \
		tzdata && \
    rm -rf /var/lib/apt/lists/* && \
    sed -i "s|info|$RUST_LOG|g" /opt/apps/server/.env && \
	sed -i "s|7878|$PORT|g" /opt/apps/server/.env && \
    sed -i "s|allowed_origin|$ALLOWED_ORIGIN|g" /opt/apps/server/.env && \
    sed -i "s|kafka_enabled|$KAFKA_ENABLED|g" /opt/apps/server/.env && \
    sed -i "s|kafka_queues|$KAFKA_QUEUES|g" /opt/apps/server/.env && \
    sed -i "s|kafka_host|$KAFKA_HOST|g" /opt/apps/server/.env && \
    sed -i "s|kafka_group|$KAFKA_GROUP|g" /opt/apps/server/.env && \
    sed -i "s|opensearch_url|$OPENSEARCH_URL|g" /opt/apps/server/.env && \
	sed -i "s|1.0.0-dev|$VERSION|g" /opt/apps/server/.env && \
    echo "Set Timezone..." && \
	echo $TZ > /etc/timezone

RUN addgroup adempiere && \
	adduser --disabled-password --gecos "" --ingroup adempiere --no-create-home adempiere && \
	chown -R adempiere /opt/apps/server/ && \
	chmod +x /usr/local/bin/server && \
	echo "Set Timezone..." && \
	echo $TZ > /etc/timezone

USER adempiere

CMD ["server"]
