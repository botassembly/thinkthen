# Builds example.csproj and show.cs in a temporary folder, restoring Botassembly.ThinkThen from
# the feed scripts/prepare-packages.sh staged, with no network.
packages=$(cd ../../../target/site-packages && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
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
export HOME="$work/home" DOTNET_CLI_HOME="$work/home" MSBUILDDISABLENODEREUSE=1 DOTNET_CLI_USE_MSBUILD_SERVER=0 UseSharedCompilation=false
mkdir -p "$HOME"
cd "$work" && dotnet run --verbosity quiet
