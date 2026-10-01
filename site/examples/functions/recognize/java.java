import static java.nio.charset.StandardCharsets.UTF_8;

import java.util.List;
import java.util.Map;
import thinkthen.Door;

void main() {
    try (var tt = new Door()) {
        var spec = """
            {"version": 1, "recognize": {"kinds": {
              "PER": "Part of a person's name.",
              "ORG": "Part of the name of an organization: \
            a company, band, team, agency, \
            government body, or media outlet.",
              "LOC": "Part of the name of a place: \
            a country, region, city, \
            or geographic feature.",
              "MISC": "Part of another named entity: \
            a nationality, an event, a product, \
            or the name of a creative work."}}}""";
        var text = "Maria Chen joined Northwind Freight "
            + "in Chicago last spring.";
        var facts = tt.recognize(
            spec, text.getBytes(UTF_8)).value();
        var entities = (List<?>) facts.get("entities");
        var names = entities.stream()
            .map(one -> (Map<?, ?>) one)
            .map(one -> List.of(
                one.get("text"), one.get("kind")))
            .toList();
        assert names.equals(List.of(
            List.of("Maria Chen", "PER"),
            List.of("Northwind Freight", "ORG"),
            List.of("Chicago", "LOC")));
    }
}
