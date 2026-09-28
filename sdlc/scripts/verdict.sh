# `verdict plain|release SURFACE CODE` prints a check's line and fails a failure, and in release mode a "not run" (0128).
verdict() {
	case $3 in
	0) echo "surfaces: pass $2" ;;
	77) [ "$1" = release ] || { echo "surfaces: not run $2 (no host toolchain)"; return 0; }
		echo "surfaces: FAIL $2 (not run)"; return 1 ;;
	*) echo "surfaces: FAIL $2 (exit $3)"; return 1 ;;
	esac
}
