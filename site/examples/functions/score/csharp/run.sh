# Runs show.cs as a .NET 10 console program, restoring Botassembly.ThinkThen from the feed
# scripts/prepare-packages.sh staged, with no network.
# The records come from the function's case.json, staged as the file show.cs reads.
# The run fails when show.cs's question or options differ from case.json, then
# turns the printed answers into the JSON every language's example must match.
set -e
packages=$(cd ../../../../../target/site-packages && pwd)
shown=$(pwd)/show.cs
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
jq -r '[.question?, .choices[]?, .labels[]?, .levels[]?, .kinds[]?,
        (.rules[]? | .source, .target, .relation),
        (.questions? // {} | keys[]),
        (.questions[]? | .question, .choices[]?, .levels[]?)]
       | .[] | select(. != null)' "$SITE_CASE_FILE" |
  while IFS= read -r wording; do
    grep -qF "\"$wording\"" "$shown" || { echo "show.cs does not use \"$wording\" from case.json" >&2; exit 1; }
  done
jq -j '.records | if length == 1 then .[0] else error("expected one record") end' "$SITE_CASE_FILE" > "$work/message.txt"
cp "$shown" "$work/Program.cs"
printf '%s\n' '<Project Sdk="Microsoft.NET.Sdk">' \
  '  <PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net10.0</TargetFramework><ImplicitUsings>enable</ImplicitUsings><Nullable>enable</Nullable></PropertyGroup>' \
  '  <ItemGroup><PackageReference Include="Botassembly.ThinkThen" Version="0.2.0" /></ItemGroup>' \
  '</Project>' > "$work/example.csproj"
printf '%s\n' "<configuration><packageSources><clear /><add key=\"site\" value=\"$packages/csharp/feed\" /></packageSources></configuration>" > "$work/nuget.config"
export DOTNET_CLI_TELEMETRY_OPTOUT=1 DOTNET_NOLOGO=1 DOTNET_SKIP_FIRST_TIME_EXPERIENCE=1 NUGET_PACKAGES="$packages/csharp/packages"
# Invariant culture prints numbers with a decimal point. Each run gets its own HOME and no
# build servers, because two .NET runs that share one HOME can remove each other's state.
export DOTNET_SYSTEM_GLOBALIZATION_INVARIANT=1
export HOME="$work/home" DOTNET_CLI_HOME="$work/home" MSBUILDDISABLENODEREUSE=1 DOTNET_CLI_USE_MSBUILD_SERVER=0 UseSharedCompilation=false
mkdir -p "$HOME"
(cd "$work" && dotnet run --verbosity quiet) > "$work/output.txt"
cat "$work/output.txt"
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" 'split("\n") | map(select(length > 0)) | map(fromjson)' \
  "$work/output.txt" > "$SITE_ANSWERS_FILE"
