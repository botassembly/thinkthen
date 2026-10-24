# Online preparation only. Installed-package proof mounts no build checkout.
FROM thinkthen-floor-build:glibc2.28 AS producer
FROM almalinux:8-minimal AS base
RUN microdnf install -y tar gzip unzip libstdc++ && microdnf clean all
FROM base AS node
COPY --from=producer /opt/node /opt/node
FROM base AS ruby
RUN microdnf install -y libyaml openssl-libs readline zlib libffi gdbm && microdnf clean all
COPY --from=producer /opt/ruby /opt/ruby
FROM base AS r
RUN microdnf install -y readline bzip2-libs xz-libs pcre2 libcurl libicu libgfortran libgomp which && microdnf clean all
COPY --from=producer /opt/R /opt/R
FROM base AS managed
RUN microdnf install -y zlib libicu openssl-libs krb5-libs && microdnf clean all
FROM base AS postgres
RUN microdnf install -y util-linux zlib openssl-libs libicu libxml2 && microdnf clean all
COPY --from=producer /opt/thinkthen-pg18 /opt/thinkthen-pg18
FROM ubuntu:22.04 AS swift
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends tar gzip python3 libstdc++6 libatomic1 libcurl4 libedit2 libicu70 libsqlite3-0 libxml2 libz3-4 libgcc-s1 ca-certificates tzdata && rm -rf /var/lib/apt/lists/*

# SwiftPM's source-package rehearsal resolves a local Git tag.
FROM thinkthen-swift-build:6.3.2-ubuntu22.04 AS swift-source
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends git && rm -rf /var/lib/apt/lists/*
