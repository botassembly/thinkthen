# Sourced by each release-path and host-setup script that downloads a file. The release
# container's curl 7.61 lacks --retry-all-errors, and plain --retry skips DNS loss. Rehearsal
# run 36864015235 lost a macOS build to one 504 from a DuckDB download that had no retry.

# fetch_url OUT URL [CURL_ARG...]: download URL to OUT with up to three tries, pausing 10 and
# then 20 seconds. A 5xx reply or a connection error tries again. A 4xx reply stops at once.
# On failure it removes OUT and returns 1. The caller still checks the file's digest.
fetch_url() {
	fetch_out=$1 fetch_from=$2
	shift 2
	for fetch_pause in 10 20 0; do
		fetch_code=$(curl -fsSL -w '%{http_code}' -o "$fetch_out" "$@" "$fetch_from") && return 0
		fetch_status=$?
		case $fetch_status:$fetch_code in
		22:4??)
			echo "fetch: $fetch_from returned HTTP $fetch_code; not retried" >&2
			rm -f -- "$fetch_out"
			return 1
			;;
		esac
		[ "$fetch_pause" != 0 ] || break
		echo "fetch: $fetch_from failed (curl exit $fetch_status, HTTP $fetch_code); trying again in $fetch_pause seconds" >&2
		sleep "$fetch_pause"
	done
	echo "fetch: $fetch_from failed after three tries" >&2
	rm -f -- "$fetch_out"
	return 1
}
