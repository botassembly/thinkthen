package io.thinkthen.internal;

/** A copied record representation; kind numbers come from the engine registry. */
public record DriverRecord(int kind, byte[] bytes) {
    public String kindName() { return Contract.recordName(kind); }
    public static DriverRecord of(String kind, byte[] bytes) {
        for (int value = 0; ; value++) {
            String name = Contract.recordName(value);
            if (name == null) throw new IllegalArgumentException("Unknown native record representation");
            if (name.equals(kind)) return new DriverRecord(value,bytes.clone());
        }
    }
}
