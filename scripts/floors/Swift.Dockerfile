# Runtime dependencies only. The pinned toolchain is mounted for compilation.
FROM ubuntu:22.04@sha256:0a9887931b0e38af2e3487ed39e29436b0535c4f3f52e57745baaf79d1898956
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    python3 binutils libgcc-11-dev libstdc++-11-dev libatomic1 libcurl4 libedit2 libicu70 libsqlite3-0 libxml2 libstdc++6 \
    libz3-4 libgcc-s1 libc6-dev ca-certificates tzdata && rm -rf /var/lib/apt/lists/*
