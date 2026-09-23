# The builder image for the Ruby surface: the official trixie Ruby image
# with the pieces the build needs - libclang for rb-sys's bindgen step and
# the workspace's pinned Rust toolchain, installed into the image so the
# build needs nothing from the host. The old host ~/.rustup mount could
# not work from a Mac: it would mount a darwin toolchain into a Linux
# container (surfaces-review-3). Built locally, never pushed; removed
# with docker rmi thinkthen-ruby-builder.
FROM ruby:3.4-trixie
RUN apt-get update && apt-get install -y --no-install-recommends libclang-dev python3 \
    && rm -rf /var/lib/apt/lists/*
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.93.1 --profile minimal \
    --component clippy,rustfmt
ENV PATH="/root/.cargo/bin:${PATH}"
