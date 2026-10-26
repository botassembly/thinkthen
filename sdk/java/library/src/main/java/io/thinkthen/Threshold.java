package io.thinkthen;

public sealed interface Threshold permits Threshold.Cut, Threshold.Band {
    record Cut(double value) implements Threshold {}
    record Band(double low, double high) implements Threshold {}
    static Cut cut(double value) { return new Cut(value); }
    static Band band(double low, double high) { return new Band(low, high); }
}
