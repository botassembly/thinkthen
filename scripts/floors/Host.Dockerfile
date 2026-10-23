# Local Linux orchestration only; product compilation uses the baseline image.
FROM ubuntu:24.04@sha256:68ee22004a9109b44973313b02ff7b1de250f1de0f5ecb628d73f579aaff33ac
ENV DEBIAN_FRONTEND=noninteractive PIP_BREAK_SYSTEM_PACKAGES=1
RUN apt-get update && apt-get install -y --no-install-recommends \
    sudo docker.io docker-buildx git python3 python3-pip python3-venv curl ca-certificates \
    build-essential cmake ninja-build pkg-config unzip xz-utils libssl-dev libyaml-dev zlib1g-dev libicu-dev \
    && rm -rf /var/lib/apt/lists/*
ARG UV_VERSION
RUN python3 -m pip install --no-cache-dir "uv==$UV_VERSION"
