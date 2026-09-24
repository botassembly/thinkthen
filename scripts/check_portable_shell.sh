#!/usr/bin/env bash
# Shell spellings that break on macOS stay out of the tracked scripts
# (surfaces-review-7 R3-32, R4-10). Three spellings fail:
#   sed -i with no attached suffix: BSD sed reads the next word as the
#     suffix. Use perl -pi, or sed -i.bak and remove the .bak file.
#   %N in a date format: BSD date has no nanoseconds. Use perl's
#     Time::HiRes.
#   a bare timeout command: macOS ships it only as gtimeout. Resolve it
#     once into $TIMEOUT.
# A backslash continuation joins the next line, and a hit prints the
# joined line under the number of its first line. Comment lines are
# skipped. A sed call fails when any of its words is -i with no attached
# suffix, alone or ending a bundle (-Ei, -ni), including -i'' and -i"",
# or --in-place; the words are read after quoted text is blanked, up to
# the next ; | & ( ) or backtick. A quoted "timeout" is still timeout.
# A line that names a spelling as data ends with the comment
# "# portable-shell: data". The mark counts only in this checker and in
# scripts/test_gate_lib.sh, and only on its own line; anywhere else it is
# ignored (surfaces-review-7 verifier).
# With file arguments, only those files are read, and a missing one
# fails. With none, every tracked *.sh file and every tracked file whose
# first line is a sh or bash shebang is read; a tree without git, or one
# with no script, fails rather than passing empty.
# Usage: bash scripts/check_portable_shell.sh [FILE...]
set -euo pipefail
cd "$(dirname "$0")/.."
if [ $# -eq 0 ]; then
  listed=$(git ls-files 2>/dev/null) \
    || { echo "FAIL     no git here to list the scripts; pass the files to read"; exit 1; }
  set -- $(printf '%s\n' "$listed" | perl -nle '
    my ($h, $first);
    $first = <$h> if !/\.sh$/ and -f and open($h, "<", $_);
    print if /\.sh$/ or ($first // "") =~ m{^#!\S*(?:\s+\S+)*?[/\s](?:ba)?sh\b};
  ')
  [ $# -gt 0 ] || { echo "FAIL     no shell script to read"; exit 1; }
fi
# The spellings, as a Perl program that prints "N:line" for each hit.
scan='
  my $marked = shift @ARGV;
  my ($held, $start) = ("", 0);
  sub check {
    my ($n, $line) = @_;
    return if $line =~ /^\s*#/;
    return if $marked and $line =~ /#\s*portable-shell: data$/;
    my $code = $line =~ s/(["\x27])(sed|timeout)\1/$2/gr;
    my $hit = $code =~ /\bdate\b[^|;]*%[-_0^#]*[0-9]*N/
      || $code =~ /(?:^|[^-\$\w.\/])\\?timeout\s+[-0-9"\x27\$]/;
    $code =~ s/-i(?:\x27\x27|"")/-i/g;
    $code =~ s/\x27[^\x27]*\x27/Q/g;
    $code =~ s/"(?:[^"\\]|\\.)*"/Q/g;
    for my $part (split /[;|&()`]/, $code) {
      next unless $part =~ /(?:^|[\s\\])sed\s+(.*)/;
      $hit ||= grep { /^(?:-[A-Za-z]*i|--in-place(?:=.*)?)$/ } split " ", $1;
    }
    print "$n:$line\n" if $hit;
  }
  while (my $line = <STDIN>) {
    chomp $line;
    $start ||= $.;
    if ($line =~ s/\\$//) { $held .= $line; next }
    check($start, $held . $line);
    ($held, $start) = ("", 0);
  }
  check($start, $held) if $start;
'
bad=0
for f in "$@"; do
  if [ ! -f "$f" ]; then
    echo "FAIL     $f: no such file"
    bad=1
    continue
  fi
  marked=0
  case $f in
  scripts/check_portable_shell.sh | scripts/test_gate_lib.sh) marked=1 ;;
  esac
  hits=$(perl -e "$scan" "$marked" <"$f")
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits" | sed "s|^|FAIL     $f:|"
    bad=1
  fi
done
[ "$bad" -eq 0 ] && echo "ok:      $# shell scripts use no GNU-only sed -i, date %N, or bare timeout"  # portable-shell: data
exit "$bad"
