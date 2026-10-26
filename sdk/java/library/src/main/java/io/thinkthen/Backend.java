package io.thinkthen;

public enum Backend { SYSTEMONE, OPENAI, STUB; String wireName() { return name().toLowerCase(java.util.Locale.ROOT); } }
