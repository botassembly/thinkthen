package dev.thinkthen;

import com.fasterxml.jackson.databind.JsonNode;
import java.util.Optional;
import java.util.OptionalDouble;

final class RowDecoder<A,D extends Detail> {
    private final java.util.function.Function<JsonNode,A> answer;
    private final JsonNode question;
    private final java.util.function.Function<JsonNode,D> detail;
    RowDecoder(java.util.function.Function<JsonNode,A> answer, java.util.function.Function<JsonNode,D> detail) {
        this(answer,detail,Values.JSON.createObjectNode());
    }
    private RowDecoder(java.util.function.Function<JsonNode,A> answer, java.util.function.Function<JsonNode,D> detail, JsonNode question) {
        this.answer = answer; this.detail = detail; this.question = question;
    }
    RowDecoder<A,D> forQuestion(JsonNode question) { return new RowDecoder<>(answer,detail,question); }
    private static JsonNode orderedDetail(JsonNode detail, JsonNode question) {
        if (!detail.has("labels")) return detail;
        var result = (com.fasterxml.jackson.databind.node.ObjectNode) detail.deepCopy();
        var labels = Values.JSON.createObjectNode();
        JsonNode options = question.has("choices") ? question.get("choices") : question.path("labels");
        options.forEach(option -> {
            String name = option.isTextual() ? option.asText() : option.get("name").asText();
            labels.set(name,detail.get("labels").get(name));
        });
        result.set("labels",labels);
        return result;
    }
    private static JsonNode orderedFields(JsonNode fields, JsonNode question) {
        var result = (com.fasterxml.jackson.databind.node.ObjectNode) fields.deepCopy();
        if (fields.has("detail")) result.set("detail",orderedDetail(fields.get("detail"),question));
        if (question.path("function").asText().equals("annotate") && fields.path("answer").isObject()) {
            var entries = Values.JSON.createObjectNode();
            question.path("questions").properties().forEach(entry -> entries.set(entry.getKey(),orderedFields(fields.get("answer").get(entry.getKey()),entry.getValue())));
            result.set("answer",entries);
        }
        return result;
    }
    Row<A,D> row(dev.thinkthen.internal.DriverCall.DriverRow packet) {
        try { return decode(packet); }
        catch (RuntimeException cause) { throw Values.error(dev.thinkthen.internal.DriverClient.invalidRow(cause)); }
    }
    private Row<A,D> decode(dev.thinkthen.internal.DriverCall.DriverRow packet) {
        var fields = orderedFields(Values.parse(new String(packet.fields(),java.nio.charset.StandardCharsets.UTF_8)),question);
        var source = fields.path("source");
        var info = new RowInfo(FunctionName.valueOf(fields.get("function").asText().toUpperCase(java.util.Locale.ROOT)),packet.position(),
            fields.has("id") ? Optional.of(Values.json(fields.get("id"))) : Optional.empty(),
            source.isMissingNode() ? Optional.empty() : Optional.of(new Source(source.get("question_identity").asText(),Backend.valueOf(source.get("backend").asText().toUpperCase(java.util.Locale.ROOT)),source.get("model").asText(),source.hasNonNull("name") ? Optional.of(source.get("name").asText()) : Optional.empty())));
        var decoded = outcome(fields,packet.detail().length == 0 ? null : orderedDetail(Values.parse(new String(packet.detail(),java.nio.charset.StandardCharsets.UTF_8)),question));
        return switch (decoded) {
            case AnnotationAnswered<A,D> value -> new Answered<>(info,value.answer(),value.probability(),value.detail());
            case AnnotationUnsure<A,D> value -> new Unsure<>(info,value.probability(),value.detail());
            case AnnotationFailed<A,D> value -> new Failed<>(info,value.error());
        };
    }
    AnnotationOutcome<A,D> outcome(JsonNode fields) { return outcome(fields,fields.get("detail")); }
    private AnnotationOutcome<A,D> outcome(JsonNode fields, @org.jspecify.annotations.Nullable JsonNode rawDetail) {
        String status = fields.get("status").asText();
        if (status.equals("failed")) return new AnnotationFailed<>(Values.error(fields.get("error")));
        Optional<D> decodedDetail = rawDetail == null ? Optional.empty() : Optional.of(detail.apply(rawDetail));
        if (status.equals("unsure")) return new AnnotationUnsure<>(OptionalDouble.of(fields.get("probability").doubleValue()),decodedDetail);
        return new AnnotationAnswered<>(answer.apply(fields.get("answer")),fields.has("probability") ? OptionalDouble.of(fields.get("probability").doubleValue()) : OptionalDouble.empty(),decodedDetail);
    }
}
