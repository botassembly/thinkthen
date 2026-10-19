# verdict SURFACE CODE [PREFIX]: only manifest-declared unsupported checks may skip.
verdict() {
    python3 "$REPO/sdlc/scripts/check_verdict.py" verdict "$1" "$2" --prefix "${3:-surfaces}"
}
