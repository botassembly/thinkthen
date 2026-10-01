import java.math.BigDecimal;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.Map;
import thinkthen.Door;
import thinkthen.Json;

void main() throws Exception {
    try (var tt = new Door()) {
        var form = Files.readString(Path.of("form.json"));
        var body = "Steps: click Log in. Nobody gets in.";
        var triage = Json.parseObject(tt.call(
            "{\"annotate\": " + form
            + ", \"records\": [" + Json.quote(body) + "]}"))
            .get("value");
        assert triage.equals(List.of(Map.of(
            "steps", true,
            "area", "login",
            "impact", new BigDecimal("1.98"))));
    }
}
