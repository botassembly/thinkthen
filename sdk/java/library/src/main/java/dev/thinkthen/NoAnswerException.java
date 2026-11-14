package dev.thinkthen;

/** No answer was available: uncertainty or a failed record. */
@SuppressWarnings("serial")
public abstract class NoAnswerException extends RuntimeException {
    NoAnswerException(String message) { super(message); }
}
