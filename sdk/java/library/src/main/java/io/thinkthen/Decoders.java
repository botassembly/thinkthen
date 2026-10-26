package io.thinkthen;

import com.fasterxml.jackson.databind.JsonNode;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Optional;

final class Decoders {
    private Decoders() {}
    private static NoDetail noDetail(JsonNode ignored) { throw new IllegalStateException("Unexpected native detail"); }
    static final RowDecoder<Boolean,NoDetail> DECIDE = new RowDecoder<>(JsonNode::booleanValue,Decoders::noDetail);
    static final RowDecoder<Boolean,NoDetail> FILTER = DECIDE;
    static final RowDecoder<String,ChooseDetail> CHOOSE = new RowDecoder<>(JsonNode::asText,value -> new ChooseDetail(Values.doubles(value.get("labels"))));
    static final RowDecoder<List<String>,TagDetail> TAG = new RowDecoder<>(value -> list(value,JsonNode::asText),value -> new TagDetail(Values.doubles(value.get("labels"))));
    static final RowDecoder<Double,ScoreDetail> SCORE = new RowDecoder<>(JsonNode::doubleValue,value -> new ScoreDetail(Values.levels(value.get("levels"))));
    static final RowDecoder<Long,RankDetail> RANK = new RowDecoder<>(JsonNode::longValue,value -> new RankDetail(Values.levels(value.get("levels"))));
    static final RowDecoder<Optional<Span>,NoDetail> FIND = new RowDecoder<>(value -> value.isNull() ? Optional.empty() : Optional.of(Values.span(value)),Decoders::noDetail);
    static final RowDecoder<List<Entity>,RecognizeDetail> RECOGNIZE = new RowDecoder<>(value -> list(value,Values::entity),Decoders::recognizeDetail);
    static final RowDecoder<List<Relation>,RelateDetail> RELATE = new RowDecoder<>(value -> list(value,Values::relation),value -> new RelateDetail(list(value.get("names"),Values::entity),list(value.get("pairs"),Values::relation)));
    static final RowDecoder<Annotations,NoDetail> ANNOTATE = new RowDecoder<>(Decoders::annotations,Decoders::noDetail);
    private static <T> List<T> list(JsonNode array,java.util.function.Function<JsonNode,T> decode) {
        var result = new ArrayList<T>(); array.forEach(value -> result.add(decode.apply(value))); return List.copyOf(result);
    }
    private static RecognizeDetail recognizeDetail(JsonNode value) {
        return new RecognizeDetail(list(value.get("pieces"),piece -> {
            var tags = new LinkedHashMap<BoundaryTag,Double>();
            piece.get("tags").properties().forEach(entry -> tags.put(BoundaryTag.valueOf(entry.getKey()),entry.getValue().doubleValue()));
            return new PieceDetail(Values.span(piece),tags);
        }),list(value.get("names"),name -> new KindDetail(name.get("start").intValue(),name.get("end").intValue(),Values.doubles(name.get("kinds")))));
    }
    private static Annotations annotations(JsonNode value) {
        var result = new LinkedHashMap<String,AnnotationEntry>();
        value.properties().forEach(entry -> {
            JsonNode item = entry.getValue();
            AnnotationEntry outcome = switch (item.get("function").asText()) {
                case "decide" -> new AnnotationEntry.Decide(DECIDE.outcome(item));
                case "choose" -> new AnnotationEntry.Choose(CHOOSE.outcome(item));
                case "tag" -> new AnnotationEntry.Tag(TAG.outcome(item));
                case "score" -> new AnnotationEntry.Score(SCORE.outcome(item));
                case "filter" -> new AnnotationEntry.Filter(FILTER.outcome(item));
                case "find" -> new AnnotationEntry.Find(FIND.outcome(item));
                case "recognize" -> new AnnotationEntry.Recognize(RECOGNIZE.outcome(item));
                case "relate" -> new AnnotationEntry.Relate(RELATE.outcome(item));
                default -> throw new IllegalStateException("Unknown annotation function");
            };
            result.put(entry.getKey(),outcome);
        });
        return new Annotations(result);
    }
}
