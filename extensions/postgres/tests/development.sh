#!/bin/sh
# Compile the PostgreSQL adapter and run its bounded ownership contracts.
set -eu
: "${THINKTHEN_SQL_TEST_ARCHIVE:?Select the compatible fixture archive}"
: "${THINKTHEN_LOOPBACK:?Select its matching loopback}"
scratch=$(mktemp -d)
cleanup() {
    outcome=$?
    if [ "$outcome" = 0 ]; then rm -rf "$scratch";
    else printf 'PostgreSQL failed files: %s\n' "$scratch" >&2; fi
}
trap cleanup EXIT
trap 'exit 1' HUP INT TERM
docker run --pull=never --rm --network none --label "$(python3 scripts/build_targets.py)" \
    --label "pm.ticket=${PM_JOB%% *}" -v "$PWD:/src:ro" -v "$scratch:/work" \
    -v "$THINKTHEN_SQL_TEST_ARCHIVE:/native/libthinkthen_ffi.a:ro" \
    -v "$THINKTHEN_LOOPBACK:/loopback:ro" "$(python3 scripts/release_images.py floor)" \
    sh -eu -c '
        mkdir -p /work/home "$(pg_config --sharedir)/extension"
        chown -R "$1:$2" /work
        setpriv --reuid "$1" --regid "$2" --clear-groups env HOME=/work/home \
            THINKTHEN_PG_CFLAGS= THINKTHEN_SQL_TEST_BUILD=ON sh /src/extensions/postgres/package.sh
    ' sh "$(id -u)" "$(id -g)"
# The prepared test image owns Python's PostgreSQL driver.
docker run --pull=never --rm --network none --label "$(python3 scripts/build_targets.py)" \
    --label "pm.ticket=${PM_JOB%% *}" -v "$PWD:/src:ro" -v "$scratch:/work" \
    -v "$THINKTHEN_LOOPBACK:/loopback:ro" \
    "$(docker image inspect "${THINKTHEN_POSTGRES_TEST_IMAGE:-thinkthen-postgres-build:18.6}" --format '{{.Id}}')" \
    sh -eu -c '
        cp /work/package/lib/* "$(pg_config --pkglibdir)/"
        cp /work/package/share/extension/* "$(pg_config --sharedir)/extension/"
        if ! getent passwd "$1" >/dev/null; then
            printf "adapter:x:%s:%s::/work/home:/bin/sh\n" "$1" "$2" >> /etc/passwd
        fi
        exec setpriv --reuid "$1" --regid "$2" --clear-groups env -i HOME=/work/home \
            PATH=/opt/thinkthen-pg18/bin:/usr/bin:/bin LANG=C.UTF-8 \
            python3 /src/extensions/postgres/tests/typed_api.py /loopback --group adapter \
            --abi-mismatch-extension /work/build/thinkthen_postgres_mismatch.so \
            --systemone-url-env THINKTHEN_TEST_LOOPBACK_URL --openai-url-env THINKTHEN_TEST_LOOPBACK_URL
    ' sh "$(id -u)" "$(id -g)"
