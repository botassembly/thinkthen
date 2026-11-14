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
if [ -f example.csproj ]; then cp example.csproj "$work/example.csproj"; else
  cat > "$work/example.csproj" <<'PROJECT'
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net10.0</TargetFramework><ImplicitUsings>enable</ImplicitUsings><Nullable>enable</Nullable></PropertyGroup>
  <ItemGroup><PackageReference Include="Botassembly.ThinkThen" Version="0.2.0" /></ItemGroup>
</Project>
PROJECT
fi
cp show.cs "$work/Program.cs"
printf '%s\n' "<configuration><packageSources><clear /><add key=\"site\" value=\"$packages/csharp/feed\" /></packageSources></configuration>" > "$work/nuget.config"
export DOTNET_CLI_TELEMETRY_OPTOUT=1 DOTNET_NOLOGO=1 DOTNET_SKIP_FIRST_TIME_EXPERIENCE=1 NUGET_PACKAGES="$packages/csharp/packages"
export DOTNET_SYSTEM_GLOBALIZATION_INVARIANT=1 DOTNET_CLI_HOME="$work/dotnet-home"
export MSBUILDDISABLENODEREUSE=1 DOTNET_CLI_USE_MSBUILD_SERVER=0 UseSharedCompilation=false
mkdir -p "$DOTNET_CLI_HOME"
(cd "$work" && dotnet build --verbosity quiet --output "$work/app")
cat > "$work/run.sh" <<'RUN'
set -eu
entry=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
work=$(mktemp -d)
for name in message.txt messages.txt case.json; do
  if [ -f "$entry/$name" ]; then cp "$entry/$name" "$work/$name"; fi
done
export DOTNET_SYSTEM_GLOBALIZATION_INVARIANT=1
(cd "$work" && "${DOTNET_ROOT:+$DOTNET_ROOT/}dotnet" "$entry/app/example.dll") > "$work/output.txt"
cat "$work/output.txt"
if [ -f "$entry/normalize.sh" ] && [ -n "${SITE_ANSWERS_FILE:-}" ]; then
  SITE_CASE_FILE=${SITE_CASE_FILE:-$entry/case.json}
  export SITE_CASE_FILE
  . "$entry/normalize.sh"
fi
RUN
[ "${1:-}" = --prepare ] || bash "$work/run.sh"
