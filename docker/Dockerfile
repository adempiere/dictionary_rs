FROM rust:1.93.1-bookworm AS builder

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



FROM debian:bookworm-20251117

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

EXPOSE ${PORT}


# Add operative system dependencies
RUN apt-get update && \
	apt-get install -y \
		bash \
		ca-certificates \
		pkg-config \
		openssl \
		libssl-dev \
		tzdata && \
	echo "Set Timezone..." && \
	ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && \
	echo $TZ > /etc/timezone && \
	# Clean up
	apt-get autoremove -y && \
	apt-get clean && \
	rm -rf /var/lib/apt/lists/* \
	rm -rf /tmp/*


WORKDIR /opt/apps/server


COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server

COPY --from=builder /usr/local/cargo/bin/.env /opt/apps/server/.env


# Create env file
RUN sed -i "s|info|$RUST_LOG|g" /opt/apps/server/.env && \
	sed -i "s|7878|$PORT|g" /opt/apps/server/.env && \
	sed -i "s|allowed_origin|$ALLOWED_ORIGIN|g" /opt/apps/server/.env && \
	sed -i "s|kafka_enabled|$KAFKA_ENABLED|g" /opt/apps/server/.env && \
	sed -i "s|kafka_queues|$KAFKA_QUEUES|g" /opt/apps/server/.env && \
	sed -i "s|kafka_host|$KAFKA_HOST|g" /opt/apps/server/.env && \
	sed -i "s|kafka_group|$KAFKA_GROUP|g" /opt/apps/server/.env && \
	sed -i "s|opensearch_url|$OPENSEARCH_URL|g" /opt/apps/server/.env && \
	sed -i "s|1.0.0-dev|$VERSION|g" /opt/apps/server/.env


# Add adempiere as user
RUN addgroup adempiere && \
	adduser --disabled-password --gecos "" --ingroup adempiere --no-create-home adempiere && \
	chown -R adempiere /opt/apps/server/ && \
	chmod +x /usr/local/bin/server && \
	echo "Set Timezone..." && \
	echo $TZ > /etc/timezone

USER adempiere


# Start app
CMD ["server"]
