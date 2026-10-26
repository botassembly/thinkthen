package io.thinkthen;

public sealed interface JsonValue permits JsonNull, JsonBoolean, JsonNumber, JsonString, JsonArray, JsonObject {
    static JsonValue parse(String json) { return Values.json(Values.parse(json)); }
}
