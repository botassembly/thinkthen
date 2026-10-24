FROM ubuntu:24.04
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    build-essential bison flex ca-certificates curl pkg-config libjansson-dev \
    python3 python3-psycopg2 && rm -rf /var/lib/apt/lists/*
WORKDIR /tmp
RUN curl --fail --location --silent --show-error https://ftp.postgresql.org/pub/source/v18.6/postgresql-18.6.tar.bz2 -o postgres.tar.bz2 \
    && curl --fail --location --silent --show-error https://ftp.postgresql.org/pub/source/v18.6/postgresql-18.6.tar.bz2.sha256 -o postgres.sha256 \
    && sed 's/postgresql-18.6.tar.bz2/postgres.tar.bz2/' postgres.sha256 | sha256sum -c - \
    && tar xf postgres.tar.bz2 && cd postgresql-18.6 \
    && ./configure --prefix=/opt/thinkthen-pg18 --without-readline --without-zlib --without-icu \
    && make -j4 && make install \
    && cd / && rm -rf /tmp/postgresql-18.6 /tmp/postgres.tar.bz2 /tmp/postgres.sha256
ENV PATH="/opt/thinkthen-pg18/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
