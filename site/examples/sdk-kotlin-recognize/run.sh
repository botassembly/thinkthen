# Compile the unchanged example against the staged Kotlin package, offline.
set -eu
[ "${SITE_EXECUTE_ONLY:-}" != 1 ] || { echo "Use a prepared executable" >&2; exit 2; }
root=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../.." && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
java=$THINKTHEN_BUILD_TOOLS/jvm/jdk-22.0.2+9/bin/java
if [ "${1:-}" = --prepare ]; then work=$2; mkdir -p "$work"; else
  work=$(mktemp -d)
  trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
fi
trap 'exit 1' HUP INT TERM
classpath=$(printf '%s:' "$packages"/jvm/lib/*.jar)
"$java" -cp "$packages/jvm/kotlinc/*" org.jetbrains.kotlin.cli.jvm.K2JVMCompiler \
  -no-stdlib -no-reflect -jvm-target 22 -cp "$classpath" -d "$work" show.kt
if [ -n "${SITE_CASE_FILE:-}" ]; then
  python3 -B - "$SITE_CASE_FILE" "$work" <<'PYINPUT'
import json
from pathlib import Path
import sys

case = json.loads(Path(sys.argv[1]).read_text())
work = Path(sys.argv[2])
(work / "message.txt").write_text(case["records"][0])
(work / "messages.txt").write_text("\n".join(case["records"]) + "\n")
PYINPUT
fi
cat > "$work/run.sh" <<'RUN'
entry=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
classpath=$(printf '%s:' "$SITE_PACKAGES"/jvm/lib/*.jar)
exec "$THINKTHEN_BUILD_TOOLS/jvm/jdk-22.0.2+9/bin/java" --enable-native-access=ALL-UNNAMED -cp "$entry:$classpath" ShowKt
RUN
if [ "${1:-}" != --prepare ]; then (cd "$work" && bash run.sh) > "$work/output.txt"; cat "$work/output.txt"; fi
