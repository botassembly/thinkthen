package dev.thinkthen;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import java.util.Objects;

/** Shared value operations; never consult the caller's mutable item. */
final class ResultValues {
    private ResultValues() {}
    static boolean equal(Row<?,?> row,Object other) {
        return other instanceof Row<?,?> value && row.info().equals(value.info())
            && row.status() == value.status() && row.probability().equals(value.probability())
            && row.detail().equals(value.detail())
            && Objects.equals(ownAnswer(row),ownAnswer(value)) && Objects.equals(ownError(row),ownError(value));
    }
    static int hash(Row<?,?> row) { return Objects.hash(row.info(),row.status(),row.probability(),row.detail(),ownAnswer(row),ownError(row)); }
    private static Object ownAnswer(Row<?,?> row) { return row instanceof Answered<?,?> value ? value.answer() : null; }
    private static Object ownError(Row<?,?> row) { return row instanceof Failed<?,?> value ? value.error() : null; }
    static <A,B,T> Row<B,T> map(Row<A,T> row,java.util.function.Function<A,B> convert) {
        return switch (row) {
            case Answered<A,T> value -> new Answered<>(value.info(),value.item(),convert.apply(value.answer()),value.probability(),value.detail());
            case Unsure<A,T> value -> new Unsure<>(value.info(),value.item(),value.probability(),value.detail());
            case Failed<A,T> value -> new Failed<>(value.info(),value.item(),value.error());
        };
    }
    static ObjectNode json(Row<?,?> row) {
        var result = Values.JSON.createObjectNode();
        result.put("function",row.function().toString().toLowerCase(java.util.Locale.ROOT));
        result.put("status",row.status().toString().toLowerCase(java.util.Locale.ROOT));
        result.put("position",row.position());
        row.id().ifPresent(value -> result.set("id",Values.parse(value.toString())));
        row.source().ifPresent(source -> {
            var value = result.putObject("source");
            value.put("question_identity",source.questionIdentity()); value.put("backend",source.backend().wireName()); value.put("model",source.model());
            source.name().ifPresent(name -> value.put("name",name));
        });
        if (row instanceof Answered<?,?> answered) result.set("answer",answer(answered.answer()));
        row.probability().ifPresent(probability -> result.put("probability",probability));
        if (row instanceof Failed<?,?> failed) {
            var error = result.putObject("error"); error.put("code",failed.error().code().wireName()); error.put("message",failed.error().message());
            failed.error().httpStatus().ifPresent(status -> error.put("http_status",status));
        }
        if (!(row.detail() instanceof NoDetail)) result.set("detail",detail(row.detail()));
        return result;
    }
    private static JsonNode answer(Object value) {
        if (value instanceof java.util.Optional<?> optional) return optional.map(ResultValues::answer).orElse(Values.JSON.nullNode());
        if (value instanceof Enum<?> member) return Values.JSON.getNodeFactory().textNode(member.toString());
        if (value instanceof Span span) return span(span);
        if (value instanceof Entity entity) { var node = mention(entity.mention()); node.put("probability",entity.probability()); return node; }
        if (value instanceof Relation relation) return relation(relation);
        if (value instanceof java.util.List<?> values) { var node = Values.JSON.createArrayNode(); values.forEach(item -> node.add(answer(item))); return node; }
        if (value instanceof Annotations entries) { var node = Values.JSON.createObjectNode(); entries.entries().forEach((name,row) -> {
            var entry = json(row); entry.remove(java.util.List.of("position","id","source")); node.set(name,entry);
        }); return node; }
        return Values.tree(value);
    }
    private static ObjectNode span(Span span) { var node = Values.JSON.createObjectNode(); node.put("text",span.text()); node.put("start",span.savedStart()); node.put("end",span.savedEnd()); return node; }
    private static ObjectNode mention(Mention mention) { var node = span(mention.span()); node.put("kind",mention.kind()); return node; }
    private static ObjectNode relation(Relation relation) {
        var node = Values.JSON.createObjectNode(); node.set("source",mention(relation.source())); node.set("target",mention(relation.target())); node.put("relation",relation.relation()); node.put("probability",relation.probability()); relation.either().ifPresent(value -> node.put("either",value)); return node;
    }
    private static JsonNode detail(Detail detail) {
        if (detail instanceof RelationDetail relations) { var node=Values.JSON.createObjectNode(); node.set("names",answer(relations.names())); node.set("pairs",answer(relations.pairs())); return node; }
        if (detail instanceof RecognizeDetail recognize) {
            var node=Values.JSON.createObjectNode(); var pieces=node.putArray("pieces");
            recognize.pieces().forEach(piece -> { var value=span(piece.span()); value.set("tags",Values.tree(piece.tags())); pieces.add(value); });
            var names=node.putArray("names"); recognize.names().forEach(name -> { var value=Values.JSON.createObjectNode(); value.put("text",name.text()); value.put("start",name.span().savedStart()); value.put("end",name.span().savedEnd()); value.set("kinds",Values.tree(name.kinds())); names.add(value); }); return node;
        }
        return Values.tree(detail);
    }
    static String print(Row<?,?> row) {
        if (row instanceof Unsure<?,?>) return "unsure";
        if (row instanceof Failed<?,?> value) return "failed: " + leaf(value.error().code().wireName()) + ": " + leaf(value.error().message());
        Object value = ((Answered<?,?>)row).answer();
        return switch (row.function()) {
            case SCORE -> java.math.BigDecimal.valueOf(((Number)value).doubleValue()).stripTrailingZeros().toPlainString();
            case TAG -> join((java.util.List<?>)value,ResultValues::leaf);
            case FIND -> ((java.util.Optional<?>)value).map(span -> leaf(((Span)span).text())).orElse("");
            case RECOGNIZE -> join((java.util.List<?>)value,item -> { var name=(Entity)item; return leaf(name.kind()) + ": " + leaf(name.text()) + " (" + name.mention().span().savedStart() + ".." + name.mention().span().savedEnd() + ")"; });
            case RELATE -> join((java.util.List<?>)value,item -> { var relation=(Relation)item; return leaf(relation.source().text()) + " --" + leaf(relation.relation()) + "--> " + leaf(relation.target().text()); });
            case ANNOTATE -> String.join("\t",((Annotations)value).entries().entrySet().stream().sorted(java.util.Map.Entry.comparingByKey(Values::compareNames)).map(entry -> leaf(entry.getKey())+"="+print(entry.getValue())).toList());
            default -> leaf(value);
        };
    }
    private static String leaf(Object value) { return Objects.toString(value).replace('\r',' ').replace('\n',' ').replace('\t',' '); }
    private static String join(java.util.List<?> values,java.util.function.Function<Object,String> print) { return String.join("\t",values.stream().map(print).toList()); }
}
