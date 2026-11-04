set -o pipefail
# One record of 100,000 letters.
head -c 100000 /dev/zero | tr '\0' 'a' |
  thinkthen decide --json 'Is this text about billing?' |
  jq '{status, error}'
