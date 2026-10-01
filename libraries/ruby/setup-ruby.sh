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
host=$(uname -s)
case $host in Linux) prefix=$tools/ruby/$RUBY_VERSION ;; Darwin) prefix=$tools/ruby/$RUBY_VERSION-$(uname -m)-darwin ;; *) echo "setup-ruby: no route for $host" >&2; exit 77 ;; esac
if [ -e "$prefix/thinkthen-toolchain.stamp" ]; then
	echo "setup-ruby: refused: $prefix is already complete" >&2
	exit 1
fi
mkdir -p "$archives"
fetch() {
	# Rehearsal run 36817514201 lost DNS for cache.ruby-lang.org. curl's --retry skips that error.
	# The release container's curl lacks --retry-all-errors. This loop retries instead.
	[ ! -s "$archives/$2" ] || return 0
	for pause in 10 20 0; do
		curl -fsSL -o "$archives/$2" "$1" && return 0
		[ "$pause" != 0 ] || return 1
		sleep "$pause"
	done
}
fetch "$RUBY_URL" "ruby-$RUBY_VERSION.tar.xz"
fetch "$YAML_URL" yaml-0.2.5.tar.gz
if [ "$host" = Darwin ]; then
	command -v shasum >/dev/null || { echo 'setup-ruby: shasum is missing' >&2; exit 77; }
	openssl=${THINKTHEN_RUBY_OPENSSL:-$(brew --prefix openssl@3 2>/dev/null || true)}
	[ -x "$openssl/bin/openssl" ] || { echo "setup-ruby: OpenSSL $RUBY_OPENSSL_SERIES host prefix is missing" >&2; exit 77; }
	case $("$openssl/bin/openssl" version) in "OpenSSL $RUBY_OPENSSL_SERIES."*) ;; *) echo 'setup-ruby: OpenSSL host version differs from toolchain.env' >&2; exit 77 ;; esac
	DYLD_LIBRARY_PATH=$openssl/lib${DYLD_LIBRARY_PATH:+:$DYLD_LIBRARY_PATH}
	export DYLD_LIBRARY_PATH
	check256() { printf '%s  %s\n' "$1" "$2" | shasum -a 256 -c -; }
	check512() { printf '%s  %s\n' "$1" "$2" | shasum -a 512 -c -; }
	SDKROOT=$(xcrun --sdk macosx --show-sdk-path)
	MACOSX_DEPLOYMENT_TARGET=$RUBY_MACOS_DEPLOYMENT_TARGET
	export SDKROOT MACOSX_DEPLOYMENT_TARGET
else
	check256() { printf '%s  %s\n' "$1" "$2" | sha256sum -c -; }
	check512() { printf '%s  %s\n' "$1" "$2" | sha512sum -c -; }
	if [ -n "${THINKTHEN_RUBY_OPENSSL:-}" ]; then
		openssl=$THINKTHEN_RUBY_OPENSSL
		[ -f "$openssl/include/openssl/ssl.h" ] || { echo 'setup-ruby: OpenSSL headers are missing' >&2; exit 77; }
		LD_LIBRARY_PATH=$openssl/lib64:$openssl/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}
		export LD_LIBRARY_PATH
	fi
fi
check256 "$RUBY_SHA256" "$archives/ruby-$RUBY_VERSION.tar.xz"
check256 "$YAML_SHA256" "$archives/yaml-0.2.5.tar.gz"
check512 "$YAML_SHA512" "$archives/yaml-0.2.5.tar.gz"
# The kept build.XXXX folder holds only logs. The sources build in a scratch folder, which this
# run removes at exit, because the built tree fills about 770 MB (ticket 0366).
work=$(mktemp -d "$tools/ruby/build.XXXX")
scratch_dir source "$tools/ruby/source.XXXX"
tar -xJf "$archives/ruby-$RUBY_VERSION.tar.xz" -C "$source"
tar -xzf "$archives/yaml-0.2.5.tar.gz" -C "$source"
scratch_dir stage "$tools/ruby/stage.XXXX"
configure="--prefix=$prefix --enable-shared --enable-load-relative --disable-install-doc --with-libyaml-source-dir=$source/yaml-0.2.5 --without-fiddle"
if [ -n "${openssl:-}" ]; then
	configure="$configure --with-openssl-dir=$openssl"
	[ "$host" != Linux ] || configure="$configure --with-openssl-lib=$openssl/lib64"
fi
cd "$source/ruby-$RUBY_VERSION"
# keep_logs: copy Ruby's config.log and each extension's mkmf.log, which say why a step failed or
# an extension was skipped, into the kept folder.
keep_logs() {
	[ ! -f config.log ] || cp -- config.log "$work/"
	find ext -name mkmf.log | while IFS= read -r log; do
		name=${log#ext/}
		cp -- "$log" "$work/mkmf-$(printf '%s' "${name%/mkmf.log}" | tr / -).log"
	done
}
# The words of $configure split on purpose.
# shellcheck disable=SC2086
{ ./configure $configure >"$work/configure.log" 2>&1 &&
	make -j 4 >"$work/make.log" 2>&1 &&
	make install DESTDIR="$stage" >"$work/install.log" 2>&1; } ||
	{ keep_logs; echo "setup-ruby: the build failed; $work holds the logs" >&2; exit 1; }
keep_logs
temp_prefix=$prefix.tmp.$$
mv "$stage$prefix" "$temp_prefix"
scratch_remove "$stage"
# Ruby's configure skips a missing extension with only a warning.
"$temp_prefix/bin/ruby" -rpsych -rzlib -ropenssl -rminitest -e1
printf 'ruby %s\nyaml %s\nyaml512 %s\nhost %s\nconfigure %s\n' "$RUBY_SHA256" "$YAML_SHA256" "$YAML_SHA512" "$host:$(uname -m)" "$configure" \
	>"$temp_prefix/thinkthen-toolchain.stamp"
mv "$temp_prefix" "$prefix"
echo "setup-ruby: built $prefix; $work holds the build logs"
