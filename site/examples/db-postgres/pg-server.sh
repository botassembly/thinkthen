# Starts a private PostgreSQL 18 server for one example and stops it when the example ends.
# The server reads the extension staged by scripts/prepare-packages.sh. A root build runs
# the server as nobody, because PostgreSQL refuses to run as root.
packages=$(cd ../../../target/site-packages && pwd)
pgbin=$(cat "$packages/postgres/bindir")
pgwork=$(mktemp -d /tmp/thinkthen-pg.XXXXXX)
as=
if [ "$(id -u)" = 0 ]; then chown nobody "$pgwork"; as="runuser -u nobody --"; fi
$as "$pgbin/initdb" -D "$pgwork/data" -U postgres --auth=trust --no-locale -E UTF8 >/dev/null
$as "$pgbin/pg_ctl" -D "$pgwork/data" -l "$pgwork/log" -w -s -o "-k $pgwork -c listen_addresses='' \
  -c extension_control_path='$packages/postgres/share:\$system' \
  -c dynamic_library_path='$packages/postgres/lib:\$libdir'" start
trap '$as "$pgbin/pg_ctl" -D "$pgwork/data" -m immediate -w -s stop; rm -rf "$pgwork"' EXIT
export PGHOST=$pgwork PGUSER=postgres PGDATABASE=postgres PATH="$pgbin:$PATH"
