# The builder image for the Ruby surface: the official trixie Ruby image
# with the one missing piece, libclang, which rb-sys's bindgen step needs.
# Built locally, never pushed; removed with docker rmi thinkthen-ruby-builder.
FROM ruby:3.4-trixie
RUN apt-get update && apt-get install -y --no-install-recommends libclang-dev \
    && rm -rf /var/lib/apt/lists/*
