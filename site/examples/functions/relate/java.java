import static java.nio.charset.StandardCharsets.UTF_8;

import java.util.List;
import java.util.Map;
import thinkthen.Door;
import thinkthen.Json;

void main() {
    try (var tt = new Door()) {
        var sings = """
            {"version": 1, "relate": {"relations": [
              {"name": "sings",
               "source": "singer", "target": "song"}]}}""";
        var names = List.of(
            List.of("Paul McCartney", "singer"),
            List.of("Ringo Starr", "singer"),
            List.of("Yesterday", "song"),
            List.of("Octopus's Garden", "song"));
        var records = names.stream()
            .map(one -> "{\"name\": %s, \"kind\": %s}"
                .formatted(Json.quote(one.get(0)),
                    Json.quote(one.get(1)))
                .getBytes(UTF_8))
            .toArray(byte[][]::new);
        var whoSings = tt.relate(sings, records).value();
        var edges = (List<?>) whoSings.get("edges");
        var pairs = edges.stream()
            .map(one -> (Map<?, ?>) one)
            .map(edge -> List.of(
                (Map<?, ?>) edge.get("source"),
                (Map<?, ?>) edge.get("target")))
            .map(two -> List.of(
                two.get(0).get("name"),
                two.get(1).get("name")))
            .toList();
        assert pairs.equals(List.of(
            List.of("Paul McCartney", "Yesterday"),
            List.of("Ringo Starr", "Octopus's Garden")));
    }
}
