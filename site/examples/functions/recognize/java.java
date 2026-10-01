import static java.nio.charset.StandardCharsets.UTF_8;

import java.util.List;
import java.util.Map;
import thinkthen.Door;

void main() {
    try (var tt = new Door()) {
        var kinds = """
            {"version": 1, "recognize": {"kinds": {
              "person": null,
              "organization": null,
              "place": null}}}""";
        var text = "Maria Chen joined Northwind Freight, "
            + "a company in Chicago.";
        var facts = tt.recognize(
            kinds, text.getBytes(UTF_8)).value();
        var entities = (List<?>) facts.get("entities");
        var names = entities.stream()
            .map(one -> (Map<?, ?>) one)
            .map(one -> List.of(
                one.get("text"), one.get("kind")))
            .toList();
        assert names.equals(List.of(
            List.of("Maria Chen", "person"),
            List.of("Northwind Freight", "organization"),
            List.of("Chicago", "place")));
    }
}
