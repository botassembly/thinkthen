# frozen_string_literal: true

require "thinkthen/version"
raise LoadError, "thinkthen #{ThinkThen::VERSION}: no compatible native gem was selected. " \
  "Requires Ruby >= 3.4, < 4 on x86_64-linux, aarch64-linux, x86_64-darwin or arm64-darwin " \
  "(macOS 15.0 or later). This fallback cannot run ThinkThen."
