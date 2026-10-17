// Generated from compiler-derived C error constants. Do not edit.
package thinkthen;
public enum ErrorKind {
BACKEND(2), CANCELLED(5), DEADLINE(3), DEFECT(6), LOCAL(4), USAGE(1);
private final int code; ErrorKind(int code) { this.code = code; }
public int code() { return code; }
public static ErrorKind read(int code) { for (ErrorKind kind : values()) if (kind.code == code) return kind;
throw new IllegalStateException("Unknown native error kind"); }
}
