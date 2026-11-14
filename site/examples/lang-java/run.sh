set -eu
[ "${SITE_EXECUTE_ONLY:-}" != 1 ] || { echo "Use a prepared executable" >&2; exit 2; }
root=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../.." && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
export SITE_PACKAGES="$packages"
if [ "${1:-}" = --prepare ]; then work=$2; mkdir -p "$work"; else work=$(mktemp -d); fi
case_file=${SITE_CASE_FILE:-../case.json}
if [ -f "$case_file" ]; then
  cp "$case_file" "$work/case.json"
  case_dir=$(CDPATH= cd -- "$(dirname "$case_file")" && pwd)
  basename "$case_dir" > "$work/function-name"
  cp "$root/site/examples/lang-java/normalize.sh" "$work/normalize.sh"
  jq -j '.records[0]' "$case_file" > "$work/message.txt"
  jq -r '.records[]' "$case_file" > "$work/messages.txt"
fi
main=$(sed -n 's/^public class \([[:alnum:]_]*\).*/\1/p' show.java)
test -n "$main"
cp show.java "$work/$main.java"
"$THINKTHEN_BUILD_TOOLS/jvm/jdk-22.0.2+9/bin/javac" --release 22 \
  -cp "$packages/jvm/lib/*" -d "$work" "$work/$main.java"
printf '%s\n' "$main" > "$work/main-class"
cat > "$work/run.sh" <<'RUN'
set -eu
entry=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
work=$(mktemp -d)
for name in message.txt messages.txt case.json; do
  if [ -f "$entry/$name" ]; then cp "$entry/$name" "$work/$name"; fi
done
main=$(cat "$entry/main-class")
(cd "$work" && "$THINKTHEN_BUILD_TOOLS/jvm/jdk-22.0.2+9/bin/java" --enable-native-access=ALL-UNNAMED \
  -cp "$entry:$SITE_PACKAGES/jvm/lib/*" "$main") > "$work/output.txt"
cat "$work/output.txt"
if [ -f "$entry/normalize.sh" ] && [ -n "${SITE_ANSWERS_FILE:-}" ]; then
  SITE_CASE_FILE=${SITE_CASE_FILE:-$entry/case.json}
  export SITE_CASE_FILE
  . "$entry/normalize.sh"
fi
RUN
[ "${1:-}" = --prepare ] || bash "$work/run.sh"
