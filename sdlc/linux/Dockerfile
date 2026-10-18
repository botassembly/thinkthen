# The caller selects the architecture's digest from tools.json.
ARG BASE
FROM ${BASE}
ENV DEBIAN_FRONTEND=noninteractive
# One archive snapshot fixes the system packages as well as the base filesystem.
RUN rm /etc/apt/sources.list.d/ubuntu.sources && printf '%s\n' \
    'deb [check-valid-until=no] http://snapshot.ubuntu.com/ubuntu/20260923T150000Z noble main universe' \
    'deb [check-valid-until=no] http://snapshot.ubuntu.com/ubuntu/20260923T150000Z noble-updates main universe' \
    'deb [check-valid-until=no] http://snapshot.ubuntu.com/ubuntu/20260923T150000Z noble-security main universe' \
    > /etc/apt/sources.list && apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl git python3 python3-venv build-essential clang libclang-dev \
    libssl-dev libyaml-dev libffi-dev zlib1g-dev pkg-config xz-utils unzip zip \
    bubblewrap gnat-13 gprbuild gnucobol4 libncurses-dev r-base-dev \
    php8.3-cli php8.3-common composer postgresql-16 postgresql-server-dev-16 \
    sqlite3 libsqlite3-dev musl-tools libgtk-3-dev xvfb xauth util-linux perl \
    libcurl4-openssl-dev libxml2-dev && rm -rf /var/lib/apt/lists/*
COPY sdlc/tools.json /tmp/tools.json
COPY sdlc/linux/install.py /tmp/install.py
RUN python3 /tmp/install.py /tmp/tools.json
ENV PATH=/opt/thinkthen-tools/bin:/usr/lib/postgresql/16/bin:/usr/bin:/bin \
    RUSTUP_HOME=/opt/thinkthen-tools/rustup \
    THINKTHEN_LINUX_IMAGE=1
