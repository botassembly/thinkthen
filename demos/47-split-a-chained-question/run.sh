#!/usr/bin/env bash
set -euo pipefail
unset THINKTHEN_API_KEY THINKTHEN_BASE_URL

song=${1:?name one song}
case "$song" in
  'I Me Mine')
    set -- 'a=With the Beatles' 'b=Revolver' 'c=Magical Mystery Tour' 'd=Let It Be'
    ;;
  "It Won't Be Long")
    set -- 'a=Rubber Soul' 'b=With the Beatles' 'c=Revolver' 'd=Yellow Submarine'
    ;;
  *) printf 'unknown example song\n' >&2; exit 2 ;;
esac

album_key=$(printf %s "$song" | thinkthen choose \
  'The text is the title of a song by the Beatles. Which was the first album to include it?' \
  --option "$1" --option "$2" --option "$3" --option "$4" \
  --raw --model jev-latest --url https://api.typesafe.ai/v1 --replay recording/) && first_code=0 || first_code=$?
case "$first_code" in
  0) ;;
  3) printf 'needs review: first album not sure\n' >&2; exit 3 ;;
  *) exit "$first_code" ;;
esac

case "$album_key" in
  a) album=${1#*=} ;;
  b) album=${2#*=} ;;
  c) album=${3#*=} ;;
  d) album=${4#*=} ;;
  *) printf 'unknown album option\n' >&2; exit 2 ;;
esac

set --
for year in 1963 1964 1965 1966 1967 1968 1969 1970; do
  set -- "$@" --option "$year=$year"
done
answer=$(printf %s "$album" | thinkthen choose \
  'The text names an album by the Beatles. In what year was it first released?' \
  "$@" --raw --model jev-latest --url https://api.typesafe.ai/v1 --replay recording/)
printf '%s -> %s -> %s\n' "$song" "$album" "$answer"
