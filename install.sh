#!/bin/sh
# Install the thinkthen command from a verified GitHub release.
#
#   curl -fsSL https://thinkthen.dev/install.sh | sh
#   curl -fsSL https://thinkthen.dev/install.sh | sh -s -- --version 0.2.0
#
# It installs to ~/.local/bin, or THINKTHEN_INSTALL_DIR. It refuses a missing or
# wrong checksum, a staged binary that reports another version, and a symlinked
# folder or binary. It writes a receipt beside the binary, `pending` before the
# swap and `installed` after it. It prints the PATH line and edits no shell
# profile. To uninstall, delete the binary and thinkthen.install.json beside it.
#
# THINKTHEN_INSTALL_BASE and THINKTHEN_INSTALL_API replace https://github.com and
# https://api.github.com, for a mirror or a local copy of a release.
set -eu

REPO=botassembly/thinkthen
BASE=${THINKTHEN_INSTALL_BASE:-https://github.com}
API=${THINKTHEN_INSTALL_API:-https://api.github.com}
INSTALL_DIR=${THINKTHEN_INSTALL_DIR:-$HOME/.local/bin}
VERSION=latest

die() {
	echo "thinkthen install: $*" >&2
	exit 1
}

usage() {
	echo 'usage: install.sh [--version X.Y.Z]'
}

while [ $# -gt 0 ]; do
	case $1 in
	-V | --version)
		[ $# -ge 2 ] || die '--version needs a value'
		VERSION=${2#v}
		shift 2
		;;
	-h | --help)
		usage
		exit 0
		;;
	*)
		usage >&2
		die "unknown argument: $1"
		;;
	esac
done

command -v curl >/dev/null 2>&1 || die 'curl is required'

OS=$(uname -s)
ARCH=$(uname -m)
case $OS/$ARCH in
Linux/x86_64 | Linux/amd64) PLATFORM=linux-x86_64 ;;
Linux/aarch64 | Linux/arm64) PLATFORM=linux-arm64 ;;
Darwin/arm64 | Darwin/aarch64) PLATFORM=macos-arm64 ;;
*) die "no build for $OS $ARCH; thinkthen ships Linux x86_64, Linux arm64, and macOS arm64" ;;
esac

sha256() {
	if command -v sha256sum >/dev/null 2>&1; then
		sha256sum "$1" | awk '{ print tolower($1) }'
	elif command -v shasum >/dev/null 2>&1; then
		shasum -a 256 "$1" | awk '{ print tolower($1) }'
	else
		die 'no SHA-256 tool: install sha256sum or shasum'
	fi
}

# The newest release that is not a draft or a prerelease and holds this
# platform's archive. The API lists releases newest first. A release whose
# archives are still uploading is skipped. Prints nothing on any failure.
latest_from_api() {
	command -v jq >/dev/null 2>&1 || return 0
	curl -fsSL "$API/repos/$REPO/releases" -o "$work/releases.json" 2>/dev/null || return 0
	jq -r --arg platform "$PLATFORM" '
		[.[] | select(.draft == false and .prerelease == false) | . as $r
		     | ($r.tag_name | ltrimstr("v")) as $v
		     | select(any($r.assets[]?.name; . == "thinkthen-\($v)-\($platform).tar.gz"))
		][0].tag_name // empty' "$work/releases.json" 2>/dev/null || true
}

# The tag GitHub's /releases/latest redirect names.
latest_from_redirect() {
	url=$(curl -fsSLI -o /dev/null -w '%{url_effective}' "$BASE/$REPO/releases/latest") \
		|| die 'could not resolve the latest release'
	case $url in
	*/releases/tag/*) echo "${url##*/releases/tag/}" ;;
	*) die "the latest release redirect named no tag: $url" ;;
	esac
}

work=$(mktemp -d)
stage=
receipt_stage=
cleanup() {
	rm -rf -- "$work"
	[ -z "$stage" ] || rm -f -- "$stage"
	[ -z "$receipt_stage" ] || rm -f -- "$receipt_stage"
}
trap cleanup EXIT
trap 'exit 1' HUP INT TERM

if [ "$VERSION" = latest ]; then
	tag=$(latest_from_api)
	[ -n "$tag" ] || tag=$(latest_from_redirect)
	VERSION=${tag#v}
fi
# The version and the folder reach a URL and the receipt, so each keeps a plain form.
case $VERSION in
*[!0-9.]* | .* | *. | *..* | *.*.*.*) die "not a release version: $VERSION; give X.Y.Z" ;;
*.*.*) ;;
*) die "not a release version: $VERSION; give X.Y.Z" ;;
esac
case $INSTALL_DIR in
*\"* | *\\*) die "refusing an install folder with a quote or backslash: $INSTALL_DIR" ;;
esac
# Version zero needs a minor release of at least two.
major=${VERSION%%.*}
remaining=${VERSION#*.}
minor=${remaining%%.*}
if [ "$major" -eq 0 ] && [ "$minor" -lt 2 ]; then
	die 'versions before 0.2.0 have no release; give a version of at least 0.2.0'
fi
ASSET=thinkthen-$VERSION-$PLATFORM.tar.gz
URL=$BASE/$REPO/releases/download/v$VERSION/$ASSET

echo "Downloading $ASSET"
curl -fsSL "$URL" -o "$work/$ASSET" || die "could not download $URL"
curl -fsSL "$BASE/$REPO/releases/download/v$VERSION/SHA256SUMS" -o "$work/checksums" \
	|| die 'could not download the checksum; refusing an unverified install'

expected=$(awk -v name="$ASSET" '
	{
		file = $2; sub(/^\*/, "", file)
		if (file == name) {
			matches++; hash = $1
			if (NF != 2) bad = 1
		}
	}
	END {
		if (!bad && matches == 1 && hash ~ /^[0-9A-Fa-f]+$/ && length(hash) == 64) print tolower(hash)
	}
' "$work/checksums")
[ -n "$expected" ] || die "no unique valid checksum for $ASSET; refusing an unverified install"
[ "$(sha256 "$work/$ASSET")" = "$expected" ] || die "the checksum of $ASSET does not match; refusing to install"
echo 'Checksum verified.'

mkdir "$work/unpacked"
tar -xzf "$work/$ASSET" -C "$work/unpacked" thinkthen
binary=$work/unpacked/thinkthen
[ -f "$binary" ] && [ ! -L "$binary" ] || die "$ASSET holds no thinkthen binary"

[ ! -L "$INSTALL_DIR" ] || die "refusing a symbolic-link install folder: $INSTALL_DIR"
mkdir -p "$INSTALL_DIR"
INSTALL_DIR=$(cd "$INSTALL_DIR" && pwd -P)
installed=$INSTALL_DIR/thinkthen
receipt=$INSTALL_DIR/thinkthen.install.json
for destination in "$installed" "$receipt"; do
	[ ! -L "$destination" ] || die 'refusing a symbolic-link binary or receipt'
	[ ! -e "$destination" ] || [ -f "$destination" ] || die "refusing a non-file install destination: $destination"
done

# A receipt left `pending` by an interrupted run must describe the binary on disk.
field() {
	sed -n "s/^  \"$1\": \"\\([^\"]*\\)\".*/\\1/p" "$receipt"
}
if [ -f "$receipt" ] && [ "$(field state)" = pending ]; then
	[ "$(field executable_path)" = "$installed" ] || die 'the pending receipt names another binary; refusing to continue'
	current=
	[ ! -f "$installed" ] || current=$(sha256 "$installed")
	case $current in
	"" | "$(field old_sha256)" | "$(field new_sha256)") ;;
	*) die 'the pending receipt does not match the installed binary; refusing to continue' ;;
	esac
fi

stage=$(mktemp "$INSTALL_DIR/.thinkthen-stage.XXXXXX")
cp "$binary" "$stage"
chmod 755 "$stage"
reported=$("$stage" --version 2>/dev/null) || die 'the staged binary did not run'
[ "$(printf '%s\n' "$reported" | wc -l)" -eq 1 ] || die 'the staged binary reports more than one line'
case $reported in
"thinkthen $VERSION" | "thinkthen $VERSION ("*")") ;;
*) die "asked for thinkthen $VERSION, and the staged binary reports '$reported'" ;;
esac

new_sha=$(sha256 "$stage")
old_sha=
[ ! -f "$installed" ] || old_sha=$(sha256 "$installed")

write_receipt() {
	receipt_stage=$(mktemp "$INSTALL_DIR/.thinkthen.install.json.XXXXXX")
	chmod 600 "$receipt_stage"
	{
		echo '{'
		echo '  "schema_version": 1,'
		echo "  \"state\": \"$1\","
		echo "  \"executable_path\": \"$installed\","
		echo "  \"version\": \"$VERSION\","
		echo "  \"old_sha256\": \"$old_sha\","
		echo "  \"new_sha256\": \"$new_sha\""
		echo '}'
	} >"$receipt_stage"
	mv -f "$receipt_stage" "$receipt"
	receipt_stage=
	sync 2>/dev/null || true
}

write_receipt pending
mv -f "$stage" "$installed"
stage=
sync 2>/dev/null || true
write_receipt installed

echo "Installed $reported to $installed"
case :$PATH: in
*":$INSTALL_DIR:"*) ;;
*) printf 'Add thinkthen to PATH:\n  export PATH="%s:$PATH"\n' "$INSTALL_DIR" ;;
esac
