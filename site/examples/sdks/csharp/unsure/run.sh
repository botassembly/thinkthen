#!/bin/sh
set -eu
[ "${SITE_EXECUTE_ONLY:-}" != 1 ] || { echo "Use a prepared C# program" >&2; exit 2; }
# Builds example.csproj and show.cs in a temporary folder, restoring Botassembly.ThinkThen from
# the feed scripts/prepare-packages.sh staged, with no network.
root=$(cd ../../../../.. && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
if [ "${1:-}" = --prepare ]; then
  work=$2
  mkdir -p "$work"
else
  work=$(mktemp -d)
  trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
fi
cp example.csproj "$work/"
# show.cs uses the default backend, as a reader would. Routine runs use the test backend
# named in THINKTHEN_BACKEND, which the C# client does not read itself.
if [ "${THINKTHEN_BACKEND:-}" = stub ]; then
  sed 's/new ThinkThenClient()/new ThinkThenClient(new ThinkThenOptions { Backend = Backend.Stub })/' show.cs > "$work/show.cs"
  grep -q 'Backend.Stub' "$work/show.cs" || { echo 'run.sh: show.cs has no new ThinkThenClient()' >&2; exit 1; }
else
  cp show.cs "$work/"
fi
printf '%s\n' "<configuration><packageSources><clear /><add key=\"site\" value=\"$packages/csharp/feed\" /></packageSources></configuration>" > "$work/nuget.config"
export DOTNET_CLI_TELEMETRY_OPTOUT=1 DOTNET_NOLOGO=1 DOTNET_SKIP_FIRST_TIME_EXPERIENCE=1 NUGET_PACKAGES="$packages/csharp/packages"
# .NET keeps named-mutex state under HOME/.dotnet, and two .NET runs that share one HOME
# can remove each other's folders there. Each example gets its own HOME and no build servers.
export DOTNET_SYSTEM_GLOBALIZATION_INVARIANT=1
export HOME="$work/home" DOTNET_CLI_HOME="$work/home" MSBUILDDISABLENODEREUSE=1 DOTNET_CLI_USE_MSBUILD_SERVER=0 UseSharedCompilation=false
mkdir -p "$HOME"
(cd "$work" && dotnet build --verbosity quiet --output "$work/app")
cat > "$work/run.sh" <<'RUN'
#!/bin/sh
export DOTNET_SYSTEM_GLOBALIZATION_INVARIANT=1
exec "${DOTNET_ROOT:+$DOTNET_ROOT/}dotnet" "$(dirname "$0")/app/example.dll"
RUN
[ "${1:-}" = --prepare ] || sh "$work/run.sh"
