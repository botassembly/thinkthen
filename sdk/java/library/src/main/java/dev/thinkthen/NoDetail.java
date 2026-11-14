package dev.thinkthen;

/** The absence of a detail payload. */
public final class NoDetail implements Detail {
    public static final NoDetail INSTANCE = new NoDetail();
    private NoDetail() {}
}
