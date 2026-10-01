export XDG_CONFIG_HOME="$PWD/config"

thinkthen status --backend local-d1 --json |
jq '.backend | {name, url, model, key_variable}'
