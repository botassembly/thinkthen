export THINKTHEN_BASE_URL=http://localhost:8080/v1

thinkthen status --json |
jq '.backend | {url, url_source, key_variable}'
