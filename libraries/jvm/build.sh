#!/bin/sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
out="$here/target"
if [ -n "${THINKTHEN_JVM_OUT:-}" ]; then
	case $THINKTHEN_JVM_OUT in /*) ;; *) echo 'jvm build: release output must be absolute' >&2; exit 1 ;; esac
	[ -d "$THINKTHEN_JVM_OUT" ] && [ ! -L "$THINKTHEN_JVM_OUT" ] &&
	[ -z "$(find "$THINKTHEN_JVM_OUT" -mindepth 1 -print -quit)" ] || {
		echo 'jvm build: release output must be an empty real directory' >&2; exit 1;
	}
	out=$THINKTHEN_JVM_OUT
fi
javac=${THINKTHEN_JDK_HOME:+$THINKTHEN_JDK_HOME/bin/javac}
javac=${javac:-$(command -v javac || true)}
jar=${THINKTHEN_JDK_HOME:+$THINKTHEN_JDK_HOME/bin/jar}
jar=${jar:-$(command -v jar || true)}
kotlinc=${THINKTHEN_KOTLIN_HOME:+$THINKTHEN_KOTLIN_HOME/bin/kotlinc}
kotlinc=${kotlinc:-$(command -v kotlinc || true)}
scalac=${THINKTHEN_SCALA_HOME:+$THINKTHEN_SCALA_HOME/bin/scalac}
scalac=${scalac:-$(command -v scalac || true)}
for binary in "$javac" "$jar" "$kotlinc" "$scalac"; do [ -x "$binary" ] || exit 77; done
mkdir -p "$out/classes/door" "$out/classes/kotlin" "$out/classes/scala" "$out/jars"
"$javac" --enable-preview --release 21 -d "$out/classes/door" "$here"/door/thinkthen/*.java
"$jar" --create --file "$out/jars/thinkthen-door.jar" -C "$out/classes/door" .
"$kotlinc" -J-XX:ActiveProcessorCount=2 -jvm-target 21 -classpath "$out/jars/thinkthen-door.jar" "$here"/kotlin/*.kt -d "$out/classes/kotlin"
"$jar" --create --file "$out/jars/thinkthen-kotlin.jar" -C "$out/classes/kotlin" .
"$scalac" -J-XX:ActiveProcessorCount=2 -classpath "$out/jars/thinkthen-door.jar" -d "$out/classes/scala" "$here"/scala/*.scala
"$jar" --create --file "$out/jars/thinkthen-scala.jar" -C "$out/classes/scala" .
