# frozen_string_literal: true
# Generated native requirement. Do not edit.
module ThinkThen
  module NativeContract
    ABI_VERSION = 131073
    ABI_CODE = "usage.abi"
    ABI_MESSAGE = "the native ABI is incompatible"
    FUNCTION_NAMES = {
    1 => "decide",
    2 => "choose",
    3 => "tag",
    4 => "score",
    5 => "filter",
    7 => "find",
    8 => "recognize",
    9 => "relate",
    6 => "rank",
    10 => "annotate",
    }.freeze
    STATUS_NAMES = {
    1 => "answered",
    2 => "unsure",
    3 => "failed",
    }.freeze
    BACKEND_NAMES = {
    1 => "systemone",
    2 => "openai",
    3 => "stub",
    }.freeze
    ID_NAMES = {
    0 => "absent",
    1 => "string",
    2 => "signed",
    3 => "unsigned",
    }.freeze
    RECORD_NAMES = {
    0 => "text",
    1 => "json",
    2 => "missing",
    3 => "invalid",
    }.freeze
    OFFSET_NAMES = {
    0 => "utf8",
    1 => "utf16",
    2 => "code_points",
    3 => "code_points_inclusive",
    }.freeze
    CODE_NAMES = {
    1001 => "usage.threshold",
    1002 => "usage.question",
    1003 => "usage.name",
    1004 => "usage.settings",
    1005 => "usage.backend",
    1006 => "usage.url",
    1007 => "usage.key-missing",
    1008 => "usage.batch",
    1009 => "usage.pointer",
    1010 => "usage.sql-policy",
    1011 => "usage.sql-selection",
    1012 => "usage.abi",
    2001 => "record.json",
    2002 => "record.missing",
    2003 => "record.too-large",
    2004 => "record.invalid",
    3001 => "backend.transport",
    3002 => "backend.status",
    3003 => "backend.reply",
    4001 => "local.cancelled",
    4002 => "local.io",
    4003 => "local.toolbox",
    5001 => "internal.bug",
    }.freeze
  end
end
