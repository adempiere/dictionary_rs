FROM rust:1.67 as builder

LABEL maintainer="ysenih@erpya.com" \
	description="A Image for start service from rust binary"

WORKDIR /opt/apps/server

COPY . . /opt/apps/server/

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/* && \
    cargo install --path . && \
    mv docker/.env /usr/local/cargo/bin/

FROM debian:bullseye

ENV \
    ADEMPIERE_BACKEND_ENDPOINT="http://0.0.0.0:7878/v1/entities" \
    ADEMPIERE_AUTHORIZATION="Bearer" \
    RUST_LOG="info" \
    DB_HOST="localhost" \
    DB_PORT="5432" \
    DB_NAME="adempiere" \
    DB_USER="adempiere" \
    DB_PASSWORD="adempiere" \
    TZ="America/Caracas"

COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server

WORKDIR /opt/apps/server

COPY --from=builder /usr/local/cargo/bin/.env /opt/apps/server/.env

RUN apt-get update && \ 
    apt-get install -y pkg-config openssl libssl-dev libpq-dev tzdata && \
    rm -rf /var/lib/apt/lists/* && \
    sed -i "s|http://0.0.0.0:7878/v1/entities|$ADEMPIERE_BACKEND_ENDPOINT|g" /opt/apps/server/.env && \
    sed -i "s|Bearer|$ADEMPIERE_AUTHORIZATION|g" /opt/apps/server/.env && \
    sed -i "s|info|$RUST_LOG|g" /opt/apps/server/.env && \
    sed -i "s|db_host|$DB_HOST|g" /opt/apps/server/.env && \
    sed -i "s|db_port|$DB_PORT|g" /opt/apps/server/.env && \
    sed -i "s|db_name|$DB_NAME|g" /opt/apps/server/.env && \
    sed -i "s|db_user|$DB_USER|g" /opt/apps/server/.env && \
    sed -i "s|db_password|$DB_PASSWORD|g" /opt/apps/server/.env && \
    echo "Set Timezone..." && \
	echo $TZ > /etc/timezone

CMD ["server"]