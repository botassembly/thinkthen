package dev.thinkthen;

import java.util.List;
import java.util.Map;

final class Domain<E> {
    private final List<E> members;
    private final List<String> words;
    private Domain(List<E> members) {
        this.members = List.copyOf(members);
        words = this.members.stream().map(Object::toString).toList();
    }
    static Domain<String> strings(List<String> words) { return new Domain<>(words); }
    static <E extends Enum<E>> Domain<E> of(Class<E> type) { return new Domain<>(List.of(type.getEnumConstants())); }
    List<String> words() { return words; }
    E read(String word) {
        int index = words.indexOf(word);
        if (index < 0) throw new IllegalArgumentException("Answer is outside the question's choices");
        return members.get(index);
    }
    static Map<String,String> descriptions(Map<?,String> values) {
        var result = new java.util.LinkedHashMap<String,String>();
        values.forEach((key,value) -> {
            if (!(key instanceof String || key instanceof Enum<?>)) throw new IllegalArgumentException("Description keys must be choices, labels or levels");
            if (result.putIfAbsent(key.toString(),value) != null) throw new IllegalArgumentException("Repeated description: " + key);
        });
        return Map.copyOf(result);
    }
}
