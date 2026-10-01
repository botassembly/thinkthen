export HOME=/home/reader
export XDG_CONFIG_HOME=settings

thinkthen status --json |
jq .configuration.path
