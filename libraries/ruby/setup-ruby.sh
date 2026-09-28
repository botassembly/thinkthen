#!/bin/sh
# One-time setup, by hand, with the network: build the pinned Ruby and
# libyaml from source into ~/.cache/thinkthen-toolchains/ruby. No gate runs
# this script. It refuses an archive whose hash differs from toolchain.env.
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
. "$here/toolchain.env"
. "$here/../../sdlc/scripts/scratch.sh"
tools=$HOME/.cache/thinkthen-toolchains
archives=$tools/ruby/archives
prefix=$tools/ruby/$RUBY_VERSION
if [ -e "$prefix/thinkthen-toolchain.stamp" ]; then
	echo "setup-ruby: refused: $prefix is already complete" >&2
	exit 1
fi
mkdir -p "$archives"
fetch() {
	[ -s "$archives/$2" ] || curl -fsSL --retry 2 -o "$archives/$2" "$1"
}
fetch "$RUBY_URL" "ruby-$RUBY_VERSION.tar.xz"
fetch "$YAML_URL" yaml-0.2.5.tar.gz
echo "$RUBY_SHA256  $archives/ruby-$RUBY_VERSION.tar.xz" | sha256sum -c -
echo "$YAML_SHA256  $archives/yaml-0.2.5.tar.gz" | sha256sum -c -
echo "$YAML_SHA512  $archives/yaml-0.2.5.tar.gz" | sha512sum -c -
work=$(mktemp -d "$tools/ruby/build.XXXX")
tar -xJf "$archives/ruby-$RUBY_VERSION.tar.xz" -C "$work"
tar -xzf "$archives/yaml-0.2.5.tar.gz" -C "$work"
scratch_dir stage "$tools/ruby/stage.XXXX"
configure="--prefix=$prefix --enable-shared --enable-load-relative --disable-install-doc --with-libyaml-source-dir=$work/yaml-0.2.5 --without-fiddle"
cd "$work/ruby-$RUBY_VERSION"
# The words of $configure split on purpose.
# shellcheck disable=SC2086
./configure $configure >"$work/configure.log" 2>&1
make -j 4 >"$work/make.log" 2>&1
make install DESTDIR="$stage" >"$work/install.log" 2>&1
mv "$stage$prefix" "$prefix.tmp"
scratch_remove "$stage"
# Ruby's configure skips a missing extension with only a warning.
"$prefix.tmp/bin/ruby" -rpsych -rzlib -ropenssl -rminitest -e1
printf 'ruby %s\nyaml %s\nyaml512 %s\nconfigure %s\n' "$RUBY_SHA256" "$YAML_SHA256" "$YAML_SHA512" "$configure" \
	>"$prefix.tmp/thinkthen-toolchain.stamp"
mv "$prefix.tmp" "$prefix"
echo "setup-ruby: built $prefix; the build folder $work holds the logs"
