// Generated native requirement. Do not edit.
package dev.thinkthen.internal;
final class Contract {
    private Contract() {}
    static final int ABI_VERSION = 131076;
    static final String ABI_CODE = "usage.abi";
    static final String ABI_MESSAGE = "the native ABI is incompatible";
    static final String CLOSED_CODE = "usage.closed";
    static final String CLOSED_MESSAGE = "client: expected an open client; received a closed client";
    static final String LIBRARY_VARIABLE = "THINKTHEN_LIBRARY";
    static final String LIBRARY_PATH_MESSAGE = "THINKTHEN_LIBRARY must be an absolute path";
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
            case 1013 -> "usage.closed";
            case 1014 -> "usage.function";
            case 1015 -> "usage.record-kind";
            case 1016 -> "usage.offsets";
            case 1017 -> "usage.argument";
            case 1018 -> "usage.option";
            case 1019 -> "usage.column";
            case 1020 -> "usage.sheet";
            case 1021 -> "usage.output";
            case 1022 -> "usage.table";
            case 2001 -> "record.json";
            case 2002 -> "record.missing";
            case 2003 -> "record.too-large";
            case 2004 -> "record.invalid";
            case 2005 -> "record.empty";
            case 3001 -> "backend.transport";
            case 3002 -> "backend.status";
            case 3003 -> "backend.reply";
            case 4001 -> "local.cancelled";
            case 4002 -> "local.io";
            case 4003 -> "local.toolbox";
            case 4004 -> "local.accounting";
            case 4005 -> "local.request-limit";
            case 5001 -> "internal.bug";
            default -> null;
        };
    }
}
