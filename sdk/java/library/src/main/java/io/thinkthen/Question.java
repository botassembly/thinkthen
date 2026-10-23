package io.thinkthen;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import org.jspecify.annotations.Nullable;

public final class Question {
    private final JsonNode value;
    private Question(ObjectNode value) { this.value = value.deepCopy(); }
    public static Question inline(String text) { return inlineValue(text); }
    public static Question toolbox(JsonNode value, @Nullable String name) {
        var question = Json.MAPPER.createObjectNode().put("type", "toolbox");
        question.set("value", value.deepCopy());
        if (name != null) question.put("name", name);
        return new Question(question);
    }
    public static Question reference(String name) {
        return new Question(Json.MAPPER.createObjectNode().put("type", "reference").put("name", name));
    }
    static Question inlineValue(Object value) {
        var question = Json.MAPPER.createObjectNode().put("type", "inline");
        question.set("value", Json.MAPPER.valueToTree(value));
        return new Question(question);
    }
    JsonNode json() { return value.deepCopy(); }
}
