#!/bin/sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
out="$here/target"
command -v javac >/dev/null 2>&1 || exit 77
command -v kotlinc >/dev/null 2>&1 || exit 77
command -v scalac >/dev/null 2>&1 || exit 77
mkdir -p "$out/classes/door" "$out/classes/kotlin" "$out/classes/scala" "$out/jars"
javac --enable-preview --release 21 -d "$out/classes/door" "$here/door/thinkthen/Door.java" "$here/door/thinkthen/ResultEnvelope.java"
jar --create --file "$out/jars/thinkthen-door.jar" -C "$out/classes/door" .
kotlinc -J-XX:ActiveProcessorCount=2 -jvm-target 21 -classpath "$out/jars/thinkthen-door.jar" "$here/kotlin/KotlinCaller.kt" -d "$out/classes/kotlin"
jar --create --file "$out/jars/thinkthen-kotlin.jar" -C "$out/classes/kotlin" .
scalac -J-XX:ActiveProcessorCount=2 -classpath "$out/jars/thinkthen-door.jar" -d "$out/classes/scala" "$here/scala/ScalaCaller.scala"
jar --create --file "$out/jars/thinkthen-scala.jar" -C "$out/classes/scala" .
