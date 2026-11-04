set -eu
[ "${SITE_EXECUTE_ONLY:-}" != 1 ] || { echo "Use prepared Java classes" >&2; exit 2; }
# Runs the source file against the jars scripts/prepare-packages.sh staged, with Java 22.
packages=$(cd ../../../target/site-packages && pwd)
if [ "${1:-}" = --prepare ]; then work=$2; mkdir -p "$work"; else
  work=$(mktemp -d)
  trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
fi
main=$(sed -n 's/^public class \([[:alnum:]_]*\).*/\1/p' show.java)
test -n "$main"
# show.java uses the default backend, as a reader would. Routine runs use the test backend
# named in THINKTHEN_BACKEND, which the Java client does not read itself.
if [ "${THINKTHEN_BACKEND:-}" = stub ]; then
  sed 's/ThinkThen.builder().build()/ThinkThen.builder().backend(Backend.STUB).build()/' show.java > "$work/$main.java"
  grep -q 'Backend.STUB' "$work/$main.java" || { echo 'run.sh: show.java has no ThinkThen.builder().build()' >&2; exit 1; }
else
  cp show.java "$work/$main.java"
fi
"$THINKTHEN_BUILD_TOOLS/jvm/jdk-22.0.2+9/bin/javac" --release 22 \
  -cp "$packages/jvm/lib/*" -d "$work" "$work/$main.java"
printf '%s\n' "$main" > "$work/main-class"
cat > "$work/run.sh" <<'RUN'
entry=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
main=$(cat "$entry/main-class")
exec "$THINKTHEN_BUILD_TOOLS/jvm/jdk-22.0.2+9/bin/java" --enable-native-access=ALL-UNNAMED -cp "$entry:$SITE_PACKAGES/jvm/lib/*" "$main"
RUN
[ "${1:-}" = --prepare ] || bash "$work/run.sh"
