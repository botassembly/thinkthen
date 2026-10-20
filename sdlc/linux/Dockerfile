# The caller selects the architecture's digest from tools.json.
ARG BASE
FROM ${BASE}
LABEL pm.repo=thinkthen pm.ticket=0619
ENV DEBIAN_FRONTEND=noninteractive
# Print the enforced build cgroup limits before installing or compiling tools.
RUN cat /sys/fs/cgroup/cpu.max /sys/fs/cgroup/memory.max
# Docker fetches this verified bootstrap over TLS before apt has trust roots.
ADD https://archive.ubuntu.com/ubuntu/pool/main/c/ca-certificates/ca-certificates_20240203_all.deb /tmp/ca-certificates.deb
RUN echo '641de77d8f142cfd62a1a6f964ba67b20754d3337c480efb529d086075a06c9a  /tmp/ca-certificates.deb' | sha256sum -c - && dpkg-deb --extract /tmp/ca-certificates.deb /tmp/ca-bootstrap && mkdir -p /etc/ssl/certs && cat /tmp/ca-bootstrap/usr/share/ca-certificates/mozilla/*.crt > /etc/ssl/certs/ca-certificates.crt
# One archive snapshot fixes the system packages as well as the base filesystem.
RUN rm /etc/apt/sources.list.d/ubuntu.sources && printf '%s\n' \
    'deb [check-valid-until=no] http://snapshot.ubuntu.com/ubuntu/20260923T150000Z noble main universe' \
    'deb [check-valid-until=no] http://snapshot.ubuntu.com/ubuntu/20260923T150000Z noble-updates main universe' \
    'deb [check-valid-until=no] http://snapshot.ubuntu.com/ubuntu/20260923T150000Z noble-security main universe' \
    > /etc/apt/sources.list && printf 'Package: *\nPin: origin snapshot.ubuntu.com\nPin-Priority: 1001\n' > /etc/apt/preferences.d/snapshot && apt-get update && apt-get install -y --allow-downgrades --no-install-recommends \
    ca-certificates curl git python3 python3-venv build-essential clang libclang-dev \
    libssl-dev libyaml-dev libffi-dev zlib1g-dev pkg-config xz-utils unzip zip \
    libncurses-dev libsqlite3-dev musl-tools util-linux perl && rm -rf /var/lib/apt/lists/*
# Ubuntu's musl-tools supplies musl-gcc for the pinned image's own architecture.
RUN command -v musl-gcc
# The canonical generator invokes Swift's formatter, so retain its binary SDK.
RUN apt-get update && apt-get install -y --no-install-recommends \
    gnupg libedit2 libpython3.12t64 libcurl4t64 libz3-4 tzdata && rm -rf /var/lib/apt/lists/*
COPY sdlc/tools.json /tmp/tools.json
COPY sdlc/tools/ /tmp/tools/
COPY rust-toolchain.toml /opt/thinkthen-image/rust-toolchain.toml
COPY sdlc/linux/install.py /opt/thinkthen-image/sdlc/linux/install.py
COPY sdlc/scripts/tools.py /opt/thinkthen-image/sdlc/scripts/tools.py
COPY conformance/children/children.py /opt/thinkthen-image/conformance/children/children.py
RUN python3 /opt/thinkthen-image/sdlc/linux/install.py /tmp/tools.json
ENV PATH=/opt/thinkthen-tools/bin:/usr/bin:/bin \
    RUSTUP_HOME=/opt/thinkthen-tools/rustup \
    THINKTHEN_LINUX_IMAGE=1
