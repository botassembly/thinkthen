package dev.thinkthen;

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
    static final RowDecoder<String,LabelDetail> CHOOSE = new RowDecoder<>(JsonNode::asText,value -> new LabelDetail(Values.doubles(value.get("labels"))));
    static final RowDecoder<List<String>,LabelDetail> TAG = new RowDecoder<>(value -> list(value,JsonNode::asText),value -> new LabelDetail(Values.doubles(value.get("labels"))));
    static final RowDecoder<Double,LevelDetail> SCORE = new RowDecoder<>(JsonNode::doubleValue,value -> new LevelDetail(Values.levels(value.get("levels"))));
    static final RowDecoder<Long,LevelDetail> RANK = new RowDecoder<>(JsonNode::longValue,value -> new LevelDetail(Values.levels(value.get("levels"))));
    static final RowDecoder<Optional<Span>,NoDetail> FIND = new RowDecoder<>((value,canonical) -> value.isNull() ? Optional.empty() : Optional.of(Values.span(value,canonical)),(host,canonical) -> noDetail(host));
    static final RowDecoder<List<Entity>,RecognizeDetail> RECOGNIZE = new RowDecoder<>((value,canonical) -> paired(value,canonical,Values::entity),Decoders::recognizeDetail);
    static final RowDecoder<List<Relation>,RelationDetail> RELATE = new RowDecoder<>((value,canonical) -> paired(value,canonical,Values::relation),(value,canonical) -> new RelationDetail(paired(value.get("names"),canonical.get("names"),Values::entity),paired(value.get("pairs"),canonical.get("pairs"),Values::relation)));
    static final RowDecoder<Annotations,NoDetail> ANNOTATE = new RowDecoder<>(ignored -> { throw new IllegalStateException("Annotation answer requires row context"); },Decoders::noDetail);
    static <T> List<T> list(JsonNode array,java.util.function.Function<JsonNode,T> decode) {
        var result = new ArrayList<T>(); array.forEach(value -> result.add(decode.apply(value))); return List.copyOf(result);
    }
    private static <T> List<T> paired(JsonNode host,JsonNode canonical,java.util.function.BiFunction<JsonNode,JsonNode,T> decode) {
        var result = new ArrayList<T>();
        for (int i=0;i<host.size();i++) result.add(decode.apply(host.get(i),canonical.get(i)));
        return List.copyOf(result);
    }
    private static RecognizeDetail recognizeDetail(JsonNode value,JsonNode canonical) {
        return new RecognizeDetail(paired(value.get("pieces"),canonical.get("pieces"),(piece,saved) -> {
            var tags = new LinkedHashMap<BoundaryTag,Double>();
            piece.get("tags").properties().forEach(entry -> tags.put(BoundaryTag.valueOf(entry.getKey()),entry.getValue().doubleValue()));
            return new PieceDetail(Values.span(piece,saved),tags);
        }),paired(value.get("names"),canonical.get("names"),(name,saved) -> new KindDetail(Values.span(name,saved),Values.doubles(name.get("kinds")))));
    }
    static Annotations annotations(JsonNode value,JsonNode canonical,JsonNode questions,RowInfo parent,Object original) {
        var result = new LinkedHashMap<String,Row<?,?>>();
        value.properties().forEach(entry -> {
            JsonNode item = entry.getValue();
            var function = FunctionName.valueOf(item.get("function").asText().toUpperCase(java.util.Locale.ROOT));
            var info = new RowInfo(function,parent.position(),parent.id(),parent.source());
            Row<?,?> outcome = switch (function) {
                case DECIDE -> DECIDE.outcome(item,canonical.get(entry.getKey()),info,original);
                case CHOOSE -> CHOOSE.outcome(item,canonical.get(entry.getKey()),info,original);
                case TAG -> TAG.outcome(item,canonical.get(entry.getKey()),info,original);
                case SCORE -> SCORE.outcome(item,canonical.get(entry.getKey()),info,original);
                case FIND -> FIND.outcome(item,canonical.get(entry.getKey()),info,original);
                case RECOGNIZE -> RECOGNIZE.outcome(item,canonical.get(entry.getKey()),info,original);
                case RELATE -> RELATE.outcome(item,canonical.get(entry.getKey()),info,original);
                default -> throw new IllegalStateException("Unknown annotation function");
            };
            result.put(entry.getKey(),outcome);
        });
        var domains = new LinkedHashMap<String,List<String>>();
        questions.properties().forEach(entry -> {
            String function=entry.getValue().path("function").asText();
            if (function.equals("choose") || function.equals("tag")) domains.put(entry.getKey(),Questions.words(entry.getValue(),function.equals("choose") ? "choices" : "labels"));
        });
        return new Annotations(result,domains);
    }
}
