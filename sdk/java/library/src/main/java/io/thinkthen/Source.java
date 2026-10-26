package io.thinkthen;

public record Source(String questionIdentity, Backend backend, String model, java.util.Optional<String> name) {}
