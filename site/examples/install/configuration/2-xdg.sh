export HOME=/home/reader
export XDG_CONFIG_HOME=/home/reader/settings
export XDG_CACHE_HOME=/home/reader/scratch

thinkthen status --json |
jq '{configuration: .configuration.path,
  cache: .cache.path,
  usage: .usage.path}'
