package io.thinkthen.internal;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import org.jspecify.annotations.Nullable;

public final class Question {
    private final JsonNode value;
    private Question(JsonNode value) { this.value = value.deepCopy(); }
    public static Question inline(String text) { return inlineValue(text); }
    public static Question toolbox(JsonNode value, @Nullable String name) {
        JsonNode selected = name == null && value.size() == 1 ? value.elements().next() : value.path(name == null ? "" : name);
        return new Question(selected);
    }
    static Question inlineValue(Object value) { return new Question(Json.questionTree(value)); }
    ObjectNode json(String function) {
        if (value.isObject() && value.has("function")) return ((ObjectNode) value).deepCopy();
        var result = Json.MAPPER.createObjectNode().put("function", function);
        switch (function) {
            case "recognize" -> result.set("kinds", value.deepCopy());
            case "relate" -> result.set("rules", value.deepCopy());
            case "annotate" -> result.set("questions", value.deepCopy());
            default -> result.set("question", value.deepCopy());
        }
        return result;
    }
}
