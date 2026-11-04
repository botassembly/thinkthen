package dev.thinkthen;

/**
 * The question identity, backend, model and optional caller-supplied question name.
 * @param questionIdentity the questionIdentity value
 * @param backend the backend value
 * @param model the model value
 * @param name the name value
 */
public record Source(String questionIdentity, Backend backend, String model, java.util.Optional<String> name) {}
