package io.thinkthen;

import com.fasterxml.jackson.databind.JsonNode;
import java.util.Optional;
import java.util.OptionalDouble;

final class RowDecoder<A,D extends Detail> {
    private final java.util.function.Function<JsonNode,A> answer;
    private final java.util.function.Function<JsonNode,D> detail;
    RowDecoder(java.util.function.Function<JsonNode,A> answer, java.util.function.Function<JsonNode,D> detail) {
        this.answer = answer; this.detail = detail;
    }
    Row<A,D> row(io.thinkthen.internal.DriverCall.DriverRow packet) {
        try { return decode(packet); }
        catch (RuntimeException cause) { throw Values.error(io.thinkthen.internal.DriverClient.invalidRow(cause)); }
    }
    private Row<A,D> decode(io.thinkthen.internal.DriverCall.DriverRow packet) {
        var fields = Values.parse(new String(packet.fields(),java.nio.charset.StandardCharsets.UTF_8));
        var source = fields.path("source");
        var info = new RowInfo(Function.valueOf(fields.get("function").asText().toUpperCase(java.util.Locale.ROOT)),packet.position(),
            fields.has("id") ? Optional.of(Values.json(fields.get("id"))) : Optional.empty(),
            source.isMissingNode() ? Optional.empty() : Optional.of(new Source(source.get("question_identity").asText(),Backend.valueOf(source.get("backend").asText().toUpperCase(java.util.Locale.ROOT)),source.get("model").asText(),source.hasNonNull("name") ? Optional.of(source.get("name").asText()) : Optional.empty())));
        var decoded = outcome(fields,packet.detail().length == 0 ? null : Values.parse(new String(packet.detail(),java.nio.charset.StandardCharsets.UTF_8)));
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
        if (status.equals("unsure")) return new AnnotationUnsure<>(fields.get("probability").doubleValue(),decodedDetail);
        return new AnnotationAnswered<>(answer.apply(fields.get("answer")),fields.has("probability") ? OptionalDouble.of(fields.get("probability").doubleValue()) : OptionalDouble.empty(),decodedDetail);
    }
}
