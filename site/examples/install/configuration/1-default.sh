export HOME=/home/reader
unset XDG_CONFIG_HOME XDG_CACHE_HOME

thinkthen status --json |
jq '{configuration: .configuration.path,
  cache: .cache.path,
  usage: .usage.path}'
