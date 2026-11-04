ARG BASE
FROM ${BASE}
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends fish=3.7.0-1 fish-common=3.7.0-1 \
    && { dpkg-query -W fish fish-common; apt-cache show fish=3.7.0-1 fish-common=3.7.0-1; sha256sum /usr/bin/fish; } > /installer-inputs.txt \
    && rm -rf /var/lib/apt/lists/*
