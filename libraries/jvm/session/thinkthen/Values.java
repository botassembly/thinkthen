package thinkthen;

import java.math.*;
import java.util.*;
import java.util.function.Function;

/** Owned JSON values. Known members are exposed by generated result classes. */
public final class Values {
    private Values() {}
    public interface Value { Object json(); }
    public static class View implements Value {
        private final Object value;
        protected View(Object value) { this.value = freeze(value); }
        public final Object json() { return value; }
        protected final Object required(String member) {
            var object = object(value);
            if (!object.containsKey(member)) throw new IllegalStateException("Missing native member: " + member);
            return object.get(member);
        }
        protected final <T> Presence<T> presence(String member, Function<Object,T> decode) {
            var object = object(value);
            if (!object.containsKey(member)) return Presence.missing();
            Object selected = object.get(member);
            return selected == null ? Presence.nil() : Presence.of(decode.apply(selected));
        }
    }
    public static Object json(Object value) { return value instanceof Value view ? view.json() : value; }
    public static Object freeze(Object value) {
        value = json(value);
        if (value instanceof Map<?,?> object) {
            var result = new LinkedHashMap<String,Object>();
            object.forEach((key, item) -> result.put((String)key, freeze(item)));
            return Collections.unmodifiableMap(result);
        }
        if (value instanceof List<?> array) {
            var result = new ArrayList<Object>();
            array.forEach(item -> result.add(freeze(item)));
            return Collections.unmodifiableList(result);
        }
        return value;
    }
    @SuppressWarnings("unchecked")
    public static Map<String,Object> object(Object value) {
        if (!(value instanceof Map<?,?>)) throw new IllegalStateException("Expected native object");
        return (Map<String,Object>)value;
    }
    public static <T> List<T> list(Object value, Function<Object,T> read) {
        var result = new ArrayList<T>();
        for (Object item : (List<?>)value) result.add(read.apply(item));
        return Collections.unmodifiableList(result);
    }
    public static <T> Map<String,T> map(Object value, Function<Object,T> read) {
        var result = new LinkedHashMap<String,T>();
        object(value).forEach((key,item) -> result.put(key,read.apply(item)));
        return Collections.unmodifiableMap(result);
    }
    public static BigInteger integer(Object value) { return ((BigDecimal)value).toBigIntegerExact(); }
}
