export XDG_CONFIG_HOME="$PWD/config"

thinkthen status --json |
jq .backend.name

export THINKTHEN_BACKEND=ollama
thinkthen status --json |
jq .backend.name
