"""Private state and process identity operations for THINKTHEN_LIVE_V2."""


def ascii_file(path, length=None):
    try:
        value = read_file(path, 4096).decode("ascii").strip()
    except (OSError, ValueError, UnicodeError):
        refuse("a required machine identity cannot be read")
    if length is not None and (len(value) != length or any(char not in "0123456789abcdef" for char in value)):
        refuse("a required machine identity is invalid")
    return value


def current_boot():
    value = ascii_file(b"/proc/sys/kernel/random/boot_id")
    try:
        return str(uuid.UUID(value))
    except ValueError:
        refuse("the kernel boot identity is invalid")


def proc_fields(pid):
    if os.environ.get("THINKTHEN_LIVE_TEST_UNREADABLE_PID") == str(pid):
        refuse("a process identity cannot be read safely")
    try:
        raw = read_file(f"/proc/{pid}/stat", 8192)
    except FileNotFoundError:
        return None
    except (OSError, ValueError):
        refuse("a process identity cannot be read safely")
    close = raw.rfind(b")")
    fields = raw[close + 2:].split() if close >= 2 else []
    if len(fields) < 20:
        refuse("a process identity cannot be read safely")
    try:
        return {"state": fields[0].decode("ascii"), "pgid": int(fields[2]), "sid": int(fields[3]), "start_ticks": int(fields[19])}
    except (ValueError, UnicodeError):
        refuse("a process identity cannot be read safely")


def process_identity(pid, gate=False):
    fields = proc_fields(pid)
    if fields is None:
        refuse("the live gate exited before its identity was verified")
    value = {"boot_id": current_boot(), "pid": pid, "start_ticks": fields["start_ticks"]}
    if gate:
        value.update({"pgid": fields["pgid"], "sid": fields["sid"]})
    return value


def strict_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError("duplicate key")
        result[key] = value
    return result


def number(value, positive=False):
    return type(value) is int and (value > 0 if positive else value >= 0) and value <= MAXIMUM


def process_number(value, positive=False):
    return type(value) is int and (value > 0 if positive else value >= 0) and value <= 9_223_372_036_854_775_807


def valid_identity(value, gate=False):
    if type(value) is not dict or set(value) != (GATE_KEYS if gate else IDENTITY_KEYS):
        return False
    if not process_number(value["pid"], True) or not process_number(value["start_ticks"]):
        return False
    if gate and (value["pgid"] != value["pid"] or value["sid"] != value["pid"]):
        return False
    if type(value["boot_id"]) is not str:
        return False
    try:
        return str(uuid.UUID(value["boot_id"])) == value["boot_id"]
    except (AttributeError, ValueError, TypeError):
        return False


def validate_state(state, common):
    if type(state) is not dict or set(state) != STATE_KEYS:
        refuse("the live authority state is malformed")
    if type(state["version"]) is not int or state["version"] != 1 or type(state["status"]) is not str or state["status"] not in ("active", "retired"):
        refuse("the live authority state is malformed")
    if type(state["authority_id"]) is not str or type(state["machine_id"]) is not str or type(state["common_dir_hex"]) is not str:
        refuse("the live authority state is malformed")
    try:
        if str(uuid.UUID(state["authority_id"])) != state["authority_id"]:
            raise ValueError
    except (AttributeError, ValueError, TypeError):
        refuse("the live authority state is malformed")
    if not number(state["limit_tokens"], True) or not number(state["charged_tokens"]) or state["charged_tokens"] > state["limit_tokens"]:
        refuse("the live authority state is malformed")
    if state["machine_id"] != ascii_file(b"/etc/machine-id", 32):
        refuse("the live authority belongs to another machine")
    if state["common_dir_hex"] != os.fsencode(common).hex():
        refuse("the live authority belongs to another Git common directory")
    pending = state["pending"]
    if pending is not None:
        if type(pending) is not dict or set(pending) != PENDING_KEYS:
            refuse("the live authority state is malformed")
        malformed = not valid_identity(pending.get("wrapper")) or not valid_identity(pending.get("gate"), True)
        malformed = malformed or pending.get("wrapper", {}).get("boot_id") != pending.get("gate", {}).get("boot_id")
        values = (pending.get("prior_charge"), pending.get("reservation"), pending.get("charged_total"))
        malformed = malformed or pending.get("phase") not in ("pending", "unresolved")
        malformed = malformed or not number(values[0]) or not number(values[1], True) or not number(values[2])
        malformed = malformed or values[0] + values[1] != values[2] or values[2] != state["charged_tokens"]
        if malformed:
            refuse("the live authority state is malformed")
    return state


def require_private(path, directory=False):
    try:
        details = os.lstat(path)
    except OSError:
        refuse("the live authority state is missing or malformed")
    expected = 0o700 if directory else 0o600
    kind_ok = stat.S_ISDIR(details.st_mode) if directory else stat.S_ISREG(details.st_mode)
    if not kind_ok or stat.S_ISLNK(details.st_mode) or details.st_mode & 0o777 != expected:
        refuse("the live authority permissions are unsafe")


def read_state(authority, common):
    require_private(authority, True)
    require_private(os.path.join(authority, b"state.json"))
    try:
        state = json.loads(read_file(os.path.join(authority, b"state.json")), object_pairs_hook=strict_object)
    except (OSError, ValueError, UnicodeError, json.JSONDecodeError):
        refuse("the live authority state is missing or malformed")
    return validate_state(state, common)


def test_hook(name):
    directory = os.environb.get(b"THINKTHEN_LIVE_TEST_DIR")
    selected = os.environ.get("THINKTHEN_LIVE_TEST_POINT")
    if not directory or selected != name:
        return
    open(os.path.join(directory, os.fsencode(name) + b".ready"), "wb").close()
    release = os.path.join(directory, os.fsencode(name) + b".release")
    while not os.path.exists(release):
        time.sleep(0.01)
    os.environ.pop("THINKTHEN_LIVE_TEST_POINT", None)


def replace_state(authority, state):
    path = os.path.join(authority, b"state.json")
    temporary = os.path.join(authority, f"state.{os.getpid()}.{uuid.uuid4().hex}.tmp".encode())
    payload = (json.dumps(state, sort_keys=True, separators=(",", ":")) + "\n").encode("ascii")
    descriptor = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
    try:
        os.write(descriptor, payload)
        if os.environ.get("THINKTHEN_LIVE_FAIL_FSYNC") == "file":
            raise OSError(errno.EIO, "injected")
        os.fsync(descriptor)
    finally:
        os.close(descriptor)
    test_hook("after-temp-sync")
    os.replace(temporary, path)
    test_hook("after-replace")
    descriptor = os.open(authority, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    try:
        if os.environ.get("THINKTHEN_LIVE_FAIL_FSYNC") == "directory":
            raise OSError(errno.EIO, "injected")
        os.fsync(descriptor)
    finally:
        os.close(descriptor)
    test_hook("after-directory-sync")
