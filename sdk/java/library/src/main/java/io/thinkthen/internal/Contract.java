// Generated native requirement. Do not edit.
package io.thinkthen.internal;
final class Contract {
    private Contract() {}
    static final int ABI_VERSION = 131074;
    static final String ABI_CODE = "usage.abi";
    static final String ABI_MESSAGE = "the native ABI is incompatible";
    static String functionName(int value) {
        return switch (value) {
            case 1 -> "decide";
            case 2 -> "choose";
            case 3 -> "tag";
            case 4 -> "score";
            case 5 -> "filter";
            case 7 -> "find";
            case 8 -> "recognize";
            case 9 -> "relate";
            case 6 -> "rank";
            case 10 -> "annotate";
            default -> null;
        };
    }
    static String statusName(int value) {
        return switch (value) {
            case 1 -> "answered";
            case 2 -> "unsure";
            case 3 -> "failed";
            default -> null;
        };
    }
    static String backendName(int value) {
        return switch (value) {
            case 1 -> "systemone";
            case 2 -> "openai";
            case 3 -> "stub";
            default -> null;
        };
    }
    static String idName(int value) {
        return switch (value) {
            case 0 -> "absent";
            case 1 -> "string";
            case 2 -> "signed";
            case 3 -> "unsigned";
            default -> null;
        };
    }
    static String recordName(int value) {
        return switch (value) {
            case 0 -> "text";
            case 1 -> "json";
            case 2 -> "missing";
            case 3 -> "invalid";
            default -> null;
        };
    }
    static String offsetName(int value) {
        return switch (value) {
            case 0 -> "utf8";
            case 1 -> "utf16";
            case 2 -> "code_points";
            case 3 -> "code_points_inclusive";
            default -> null;
        };
    }
    static String codeName(int value) {
        return switch (value) {
            case 1001 -> "usage.threshold";
            case 1002 -> "usage.question";
            case 1003 -> "usage.name";
            case 1004 -> "usage.settings";
            case 1005 -> "usage.backend";
            case 1006 -> "usage.url";
            case 1007 -> "usage.key-missing";
            case 1008 -> "usage.batch";
            case 1009 -> "usage.pointer";
            case 1010 -> "usage.sql-policy";
            case 1011 -> "usage.sql-selection";
            case 1012 -> "usage.abi";
            case 2001 -> "record.json";
            case 2002 -> "record.missing";
            case 2003 -> "record.too-large";
            case 2004 -> "record.invalid";
            case 3001 -> "backend.transport";
            case 3002 -> "backend.status";
            case 3003 -> "backend.reply";
            case 4001 -> "local.cancelled";
            case 4002 -> "local.io";
            case 4003 -> "local.toolbox";
            case 5001 -> "internal.bug";
            default -> null;
        };
    }
}
