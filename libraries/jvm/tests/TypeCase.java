import thinkthen.Door;

public final class TypeCase {
    public static void main(String[] args) {
        try (Door door = new Door()) {
            System.out.println(door.call(args[0]));
        } catch (Door.NativeFailure failure) {
            System.out.println("{\"error\":\"" + failure.failure.kind().name().toLowerCase() + "\"}");
        }
    }
}
