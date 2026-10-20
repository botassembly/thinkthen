// Generated from the Rust result graph. Do not edit.
package thinkthen;
import java.util.*;
import java.math.*;
public final class Results {
    private Results() {
    }
    public static class View extends Values.View {
        protected View(Object value) {
            super(value);
        }
    }
    public static final class Annotation extends View {
        public Annotation(Object value) {
            super(value);
        }
        static Annotation read(Object value) {
            return new Annotation(value);
        }
        public AnswerId answerId() {
            Object value = required("answer_id");
            return AnswerId.read(value);
        }
        public Map<String,AnnotationMember> answers() {
            Object value = required("answers");
            return Values.map(value, item0 -> AnnotationMember.read(item0));
        }
        public Presence<String> file() {
            return presence("file", value -> (String)value);
        }
        public Presence<BigInteger> firstLine() {
            return presence("first_line", value -> Values.integer(value));
        }
        public Presence<BigInteger> index() {
            return presence("index", value -> Values.integer(value));
        }
        public Presence<Object> input() {
            required("input");
            return presence("input", value -> Values.freeze(value));
        }
        public Presence<BigInteger> lastLine() {
            return presence("last_line", value -> Values.integer(value));
        }
        public Meta meta() {
            Object value = required("meta");
            return Meta.read(value);
        }
        public Presence<Position> position() {
            return presence("position", value -> Position.read(value));
        }
        public Version schema() {
            Object value = required("schema");
            return Version.read(value);
        }
        public Presence<PhysicalSource> source() {
            return presence("source", value -> PhysicalSource.read(value));
        }
        public AnnotatedRow value() {
            Object value = required("value");
            return AnnotatedRow.read(value);
        }
    }
    public sealed interface AnnotationMember permits AnnotationMemberAnswerId,AnnotationMemberFailureId {
        static AnnotationMember read(Object value) {
            Map<String,Object> object = Values.object(value);
            if (object.containsKey("answer_id")) return AnnotationMemberAnswerId.read(value);
            if (object.containsKey("failure_id")) return AnnotationMemberFailureId.read(value);
            throw new IllegalStateException("Unknown native result alternative");
        }
    }
    public static final class AnnotationMemberAnswerId extends View implements AnnotationMember {
        public AnnotationMemberAnswerId(Object value) {
            super(value);
        }
        static AnnotationMemberAnswerId read(Object value) {
            return new AnnotationMemberAnswerId(value);
        }
        public Answer answer() {
            Object value = required("answer");
            return Answer.read(value);
        }
        public AnswerId answerId() {
            Object value = required("answer_id");
            return AnswerId.read(value);
        }
        public List<Observation> observations() {
            Object value = required("observations");
            return Values.list(value, item0 -> Observation.read(item0));
        }
        public QuestionDocument question() {
            Object value = required("question");
            return QuestionDocument.read(value);
        }
        public List<QuestionSource> questionSources() {
            Object value = required("question_sources");
            return Values.list(value, item0 -> QuestionSource.read(item0));
        }
        public String request() {
            Object value = required("request");
            return (String)value;
        }
        public Presence<Threshold> threshold() {
            required("threshold");
            return presence("threshold", value -> Threshold.read(value));
        }
        public Presence<Usage> usage() {
            return presence("usage", value -> Usage.read(value));
        }
        public Presence<Value> value() {
            required("value");
            return presence("value", value -> Value.read(value));
        }
    }
    public static final class AnnotationMemberFailureId extends View implements AnnotationMember {
        public AnnotationMemberFailureId(Object value) {
            super(value);
        }
        static AnnotationMemberFailureId read(Object value) {
            return new AnnotationMemberFailureId(value);
        }
        public Failure failure() {
            Object value = required("failure");
            return Failure.read(value);
        }
        public FailureId failureId() {
            Object value = required("failure_id");
            return FailureId.read(value);
        }
        public List<Observation> observations() {
            Object value = required("observations");
            return Values.list(value, item0 -> Observation.read(item0));
        }
        public QuestionDocument question() {
            Object value = required("question");
            return QuestionDocument.read(value);
        }
        public List<QuestionSource> questionSources() {
            Object value = required("question_sources");
            return Values.list(value, item0 -> QuestionSource.read(item0));
        }
        public String request() {
            Object value = required("request");
            return (String)value;
        }
        public Presence<Threshold> threshold() {
            required("threshold");
            return presence("threshold", value -> Threshold.read(value));
        }
        public Presence<Usage> usage() {
            return presence("usage", value -> Usage.read(value));
        }
    }
    public sealed interface AnnotationValue permits AnnotationValueDecision,AnnotationValueChoice,
    AnnotationValueScore,AnnotationValueTags,AnnotationValueFailed {
        static AnnotationValue read(Object value) {
            Map<String,Object> object = Values.object(value);
            if ("decision".equals(object.get("kind"))) return AnnotationValueDecision.read(value);
            if ("choice".equals(object.get("kind"))) return AnnotationValueChoice.read(value);
            if ("score".equals(object.get("kind"))) return AnnotationValueScore.read(value);
            if ("tags".equals(object.get("kind"))) return AnnotationValueTags.read(value);
            if ("failed".equals(object.get("kind"))) return AnnotationValueFailed.read(value);
            throw new IllegalStateException("Unknown native result alternative");
        }
    }
    public static final class AnnotationValueChoice extends View implements AnnotationValue {
        public AnnotationValueChoice(Object value) {
            super(value);
        }
        static AnnotationValueChoice read(Object value) {
            return new AnnotationValueChoice(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public Presence<String> value() {
            required("value");
            return presence("value", value -> (String)value);
        }
    }
    public static final class AnnotationValueDecision extends View implements AnnotationValue {
        public AnnotationValueDecision(Object value) {
            super(value);
        }
        static AnnotationValueDecision read(Object value) {
            return new AnnotationValueDecision(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public Presence<Boolean> value() {
            required("value");
            return presence("value", value -> (Boolean)value);
        }
    }
    public static final class AnnotationValueFailed extends View implements AnnotationValue {
        public AnnotationValueFailed(Object value) {
            super(value);
        }
        static AnnotationValueFailed read(Object value) {
            return new AnnotationValueFailed(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public Failure value() {
            Object value = required("value");
            return Failure.read(value);
        }
    }
    public static final class AnnotationValueScore extends View implements AnnotationValue {
        public AnnotationValueScore(Object value) {
            super(value);
        }
        static AnnotationValueScore read(Object value) {
            return new AnnotationValueScore(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public BigDecimal value() {
            Object value = required("value");
            return (BigDecimal)value;
        }
    }
    public static final class AnnotationValueTags extends View implements AnnotationValue {
        public AnnotationValueTags(Object value) {
            super(value);
        }
        static AnnotationValueTags read(Object value) {
            return new AnnotationValueTags(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public List<String> value() {
            Object value = required("value");
            return Values.list(value, item0 -> (String)item0);
        }
    }
    public static final class AnswerId extends View {
        public AnswerId(Object value) {
            super(value);
        }
        static AnswerId read(Object value) {
            return new AnswerId(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public static final class AnswerResolution extends View {
        public AnswerResolution(Object value) {
            super(value);
        }
        static AnswerResolution read(Object value) {
            return new AnswerResolution(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public sealed interface AnswerResolutions permits AnswerResolutionsString,AnswerResolutionsObject {
        static AnswerResolutions read(Object value) {
            if (value instanceof String) return AnswerResolutionsString.read(value);
            if (value instanceof Map<?,?>) return AnswerResolutionsObject.read(value);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerResolutionsString(AnswerResolution value) implements AnswerResolutions {
        static AnswerResolutionsString read(Object value) {
            return new AnswerResolutionsString(AnswerResolution.read(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerResolutionsObject(Map<String,AnswerResolution> value) implements AnswerResolutions {
        static AnswerResolutionsObject read(Object value) {
            return new AnswerResolutionsObject(Values.map(value, item0 -> AnswerResolution.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public static final class AnswerShape extends View {
        public AnswerShape(Object value) {
            super(value);
        }
        static AnswerShape read(Object value) {
            return new AnswerShape(value);
        }
        public Object value() {
            return Values.freeze(json());
        }
    }
    public sealed interface AnswerShape10 permits AnswerShape10Array,AnswerShape10Null,AnswerShape10Array2 {
        static AnswerShape10 read(Object value) {
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> (union_item_0 instanceof BigDecimal || union_item_0 == null)))) return AnswerShape10Array.read(value);
            if (value == null) return new AnswerShape10Null(null);
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> ((union_item_0 instanceof List<?> && ((List<?>)union_item_0).stream().allMatch(union_item_1 -> (union_item_1 instanceof BigDecimal || union_item_1 == null))) || union_item_0 == null)))) return AnswerShape10Array2.read(value);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape10Array(List<AnswerShape8AnyOf1Items> value) implements AnswerShape10 {
        static AnswerShape10Array read(Object value) {
            return new AnswerShape10Array(Values.list(value, item0 -> AnswerShape8AnyOf1Items.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape10Null(Object value) implements AnswerShape10 {
        static AnswerShape10Null read(Object value) {
            return new AnswerShape10Null(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape10Array2(List<AnswerShape10AnyOf1Items> value) implements AnswerShape10 {
        static AnswerShape10Array2 read(Object value) {
            return new AnswerShape10Array2(Values.list(value, item0 -> AnswerShape10AnyOf1Items.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape10AnyOf1Items permits AnswerShape10AnyOf1ItemsArray,AnswerShape10AnyOf1ItemsNull {
        static AnswerShape10AnyOf1Items read(Object value) {
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> (union_item_0 instanceof BigDecimal || union_item_0 == null)))) return AnswerShape10AnyOf1ItemsArray.read(value);
            if (value == null) return new AnswerShape10AnyOf1ItemsNull(null);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape10AnyOf1ItemsArray(List<AnswerShape8AnyOf1Items> value) implements AnswerShape10AnyOf1Items {
        static AnswerShape10AnyOf1ItemsArray read(Object value) {
            return new AnswerShape10AnyOf1ItemsArray(Values.list(value, item0 -> AnswerShape8AnyOf1Items.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape10AnyOf1ItemsNull(Object value) implements AnswerShape10AnyOf1Items {
        static AnswerShape10AnyOf1ItemsNull read(Object value) {
            return new AnswerShape10AnyOf1ItemsNull(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape11 permits AnswerShape11Array,AnswerShape11Null,AnswerShape11Array2 {
        static AnswerShape11 read(Object value) {
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> union_item_0 instanceof BigDecimal))) return AnswerShape11Array.read(value);
            if (value == null) return new AnswerShape11Null(null);
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> ((union_item_0 instanceof List<?> && ((List<?>)union_item_0).stream().allMatch(union_item_1 -> union_item_1 instanceof BigDecimal)) || union_item_0 == null)))) return AnswerShape11Array2.read(value);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape11Array(List<BigInteger> value) implements AnswerShape11 {
        static AnswerShape11Array read(Object value) {
            return new AnswerShape11Array(Values.list(value, item0 -> Values.integer(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape11Null(Object value) implements AnswerShape11 {
        static AnswerShape11Null read(Object value) {
            return new AnswerShape11Null(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape11Array2(List<AnswerShape11AnyOf1Items> value) implements AnswerShape11 {
        static AnswerShape11Array2 read(Object value) {
            return new AnswerShape11Array2(Values.list(value, item0 -> AnswerShape11AnyOf1Items.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape11AnyOf1Items permits AnswerShape11AnyOf1ItemsArray,AnswerShape11AnyOf1ItemsNull {
        static AnswerShape11AnyOf1Items read(Object value) {
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> union_item_0 instanceof BigDecimal))) return AnswerShape11AnyOf1ItemsArray.read(value);
            if (value == null) return new AnswerShape11AnyOf1ItemsNull(null);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape11AnyOf1ItemsArray(List<BigInteger> value) implements AnswerShape11AnyOf1Items {
        static AnswerShape11AnyOf1ItemsArray read(Object value) {
            return new AnswerShape11AnyOf1ItemsArray(Values.list(value, item0 -> Values.integer(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape11AnyOf1ItemsNull(Object value) implements AnswerShape11AnyOf1Items {
        static AnswerShape11AnyOf1ItemsNull read(Object value) {
            return new AnswerShape11AnyOf1ItemsNull(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public static final class AnswerShape12 extends View {
        public AnswerShape12(Object value) {
            super(value);
        }
        static AnswerShape12 read(Object value) {
            return new AnswerShape12(value);
        }
        public Object value() {
            return Values.freeze(json());
        }
    }
    public static final class AnswerShape12AnyOf1Items extends View {
        public AnswerShape12AnyOf1Items(Object value) {
            super(value);
        }
        static AnswerShape12AnyOf1Items read(Object value) {
            return new AnswerShape12AnyOf1Items(value);
        }
        public Object value() {
            return Values.freeze(json());
        }
    }
    public sealed interface AnswerShape13 permits AnswerShape13Object,AnswerShape13Null,AnswerShape13Array {
        static AnswerShape13 read(Object value) {
            if (value instanceof Map<?,?>) return AnswerShape13Object.read(value);
            if (value == null) return new AnswerShape13Null(null);
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> (union_item_0 instanceof Map<?,
            ?> || union_item_0 == null)))) return AnswerShape13Array.read(value);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape13Object(SettledRecognition value) implements AnswerShape13 {
        static AnswerShape13Object read(Object value) {
            return new AnswerShape13Object(SettledRecognition.read(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape13Null(Object value) implements AnswerShape13 {
        static AnswerShape13Null read(Object value) {
            return new AnswerShape13Null(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape13Array(List<AnswerShape13AnyOf1Items> value) implements AnswerShape13 {
        static AnswerShape13Array read(Object value) {
            return new AnswerShape13Array(Values.list(value, item0 -> AnswerShape13AnyOf1Items.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape13AnyOf1Items permits AnswerShape13AnyOf1ItemsObject,AnswerShape13AnyOf1ItemsNull {
        static AnswerShape13AnyOf1Items read(Object value) {
            if (value instanceof Map<?,?>) return AnswerShape13AnyOf1ItemsObject.read(value);
            if (value == null) return new AnswerShape13AnyOf1ItemsNull(null);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape13AnyOf1ItemsObject(SettledRecognition value) implements AnswerShape13AnyOf1Items {
        static AnswerShape13AnyOf1ItemsObject read(Object value) {
            return new AnswerShape13AnyOf1ItemsObject(SettledRecognition.read(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape13AnyOf1ItemsNull(Object value) implements AnswerShape13AnyOf1Items {
        static AnswerShape13AnyOf1ItemsNull read(Object value) {
            return new AnswerShape13AnyOf1ItemsNull(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape14 permits AnswerShape14Array,AnswerShape14Null,AnswerShape14Array2 {
        static AnswerShape14 read(Object value) {
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> union_item_0 instanceof Map<?,
            ?>))) return AnswerShape14Array.read(value);
            if (value == null) return new AnswerShape14Null(null);
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> ((union_item_0 instanceof List<?> && ((List<?>)union_item_0).stream().allMatch(union_item_1 -> union_item_1 instanceof Map<?,
            ?>)) || union_item_0 == null)))) return AnswerShape14Array2.read(value);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape14Array(SettledRelations value) implements AnswerShape14 {
        static AnswerShape14Array read(Object value) {
            return new AnswerShape14Array(SettledRelations.read(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape14Null(Object value) implements AnswerShape14 {
        static AnswerShape14Null read(Object value) {
            return new AnswerShape14Null(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape14Array2(List<AnswerShape14AnyOf1Items> value) implements AnswerShape14 {
        static AnswerShape14Array2 read(Object value) {
            return new AnswerShape14Array2(Values.list(value, item0 -> AnswerShape14AnyOf1Items.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape14AnyOf1Items permits AnswerShape14AnyOf1ItemsArray,AnswerShape14AnyOf1ItemsNull {
        static AnswerShape14AnyOf1Items read(Object value) {
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> union_item_0 instanceof Map<?,
            ?>))) return AnswerShape14AnyOf1ItemsArray.read(value);
            if (value == null) return new AnswerShape14AnyOf1ItemsNull(null);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape14AnyOf1ItemsArray(SettledRelations value) implements AnswerShape14AnyOf1Items {
        static AnswerShape14AnyOf1ItemsArray read(Object value) {
            return new AnswerShape14AnyOf1ItemsArray(SettledRelations.read(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape14AnyOf1ItemsNull(Object value) implements AnswerShape14AnyOf1Items {
        static AnswerShape14AnyOf1ItemsNull read(Object value) {
            return new AnswerShape14AnyOf1ItemsNull(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape15 permits AnswerShape15Object,AnswerShape15Null,AnswerShape15Array {
        static AnswerShape15 read(Object value) {
            if (value instanceof Map<?,?>) return AnswerShape15Object.read(value);
            if (value == null) return new AnswerShape15Null(null);
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> (union_item_0 instanceof Map<?,
            ?> || union_item_0 == null)))) return AnswerShape15Array.read(value);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape15Object(Map<String,SettledJudgment> value) implements AnswerShape15 {
        static AnswerShape15Object read(Object value) {
            return new AnswerShape15Object(Values.map(value, item0 -> SettledJudgment.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape15Null(Object value) implements AnswerShape15 {
        static AnswerShape15Null read(Object value) {
            return new AnswerShape15Null(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape15Array(List<AnswerShape15AnyOf1Items> value) implements AnswerShape15 {
        static AnswerShape15Array read(Object value) {
            return new AnswerShape15Array(Values.list(value, item0 -> AnswerShape15AnyOf1Items.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape15AnyOf1Items permits AnswerShape15AnyOf1ItemsObject,AnswerShape15AnyOf1ItemsNull {
        static AnswerShape15AnyOf1Items read(Object value) {
            if (value instanceof Map<?,?>) return AnswerShape15AnyOf1ItemsObject.read(value);
            if (value == null) return new AnswerShape15AnyOf1ItemsNull(null);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape15AnyOf1ItemsObject(Map<String,SettledJudgment> value) implements AnswerShape15AnyOf1Items {
        static AnswerShape15AnyOf1ItemsObject read(Object value) {
            return new AnswerShape15AnyOf1ItemsObject(Values.map(value, item0 -> SettledJudgment.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape15AnyOf1ItemsNull(Object value) implements AnswerShape15AnyOf1Items {
        static AnswerShape15AnyOf1ItemsNull read(Object value) {
            return new AnswerShape15AnyOf1ItemsNull(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape2 permits AnswerShape2Number,AnswerShape2Null,AnswerShape2Array {
        static AnswerShape2 read(Object value) {
            if (value instanceof BigDecimal) return AnswerShape2Number.read(value);
            if (value == null) return new AnswerShape2Null(null);
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> (union_item_0 instanceof BigDecimal || union_item_0 == null)))) return AnswerShape2Array.read(value);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape2Number(BigDecimal value) implements AnswerShape2 {
        static AnswerShape2Number read(Object value) {
            return new AnswerShape2Number((BigDecimal)value);
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape2Null(Object value) implements AnswerShape2 {
        static AnswerShape2Null read(Object value) {
            return new AnswerShape2Null(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape2Array(List<AnswerShape8AnyOf1Items> value) implements AnswerShape2 {
        static AnswerShape2Array read(Object value) {
            return new AnswerShape2Array(Values.list(value, item0 -> AnswerShape8AnyOf1Items.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape3 permits AnswerShape3String,AnswerShape3Null,AnswerShape3Array {
        static AnswerShape3 read(Object value) {
            if (value instanceof String) return AnswerShape3String.read(value);
            if (value == null) return new AnswerShape3Null(null);
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> (union_item_0 instanceof String || union_item_0 == null)))) return AnswerShape3Array.read(value);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape3String(String value) implements AnswerShape3 {
        static AnswerShape3String read(Object value) {
            return new AnswerShape3String((String)value);
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape3Null(Object value) implements AnswerShape3 {
        static AnswerShape3Null read(Object value) {
            return new AnswerShape3Null(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape3Array(List<AnswerShape6AnyOf1Items> value) implements AnswerShape3 {
        static AnswerShape3Array read(Object value) {
            return new AnswerShape3Array(Values.list(value, item0 -> AnswerShape6AnyOf1Items.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape4 permits AnswerShape4Integer,AnswerShape4Null,AnswerShape4Array {
        static AnswerShape4 read(Object value) {
            if (value instanceof BigDecimal) return AnswerShape4Integer.read(value);
            if (value == null) return new AnswerShape4Null(null);
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> (union_item_0 instanceof BigDecimal || union_item_0 == null)))) return AnswerShape4Array.read(value);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape4Integer(BigInteger value) implements AnswerShape4 {
        static AnswerShape4Integer read(Object value) {
            return new AnswerShape4Integer(Values.integer(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape4Null(Object value) implements AnswerShape4 {
        static AnswerShape4Null read(Object value) {
            return new AnswerShape4Null(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape4Array(List<AnswerShape4AnyOf1Items> value) implements AnswerShape4 {
        static AnswerShape4Array read(Object value) {
            return new AnswerShape4Array(Values.list(value, item0 -> AnswerShape4AnyOf1Items.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape4AnyOf1Items permits AnswerShape4AnyOf1ItemsInteger,AnswerShape4AnyOf1ItemsNull {
        static AnswerShape4AnyOf1Items read(Object value) {
            if (value instanceof BigDecimal) return AnswerShape4AnyOf1ItemsInteger.read(value);
            if (value == null) return new AnswerShape4AnyOf1ItemsNull(null);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape4AnyOf1ItemsInteger(BigInteger value) implements AnswerShape4AnyOf1Items {
        static AnswerShape4AnyOf1ItemsInteger read(Object value) {
            return new AnswerShape4AnyOf1ItemsInteger(Values.integer(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape4AnyOf1ItemsNull(Object value) implements AnswerShape4AnyOf1Items {
        static AnswerShape4AnyOf1ItemsNull read(Object value) {
            return new AnswerShape4AnyOf1ItemsNull(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape5 permits AnswerShape5String,AnswerShape5Object,AnswerShape5Array {
        static AnswerShape5 read(Object value) {
            if (value instanceof String) return AnswerShape5String.read(value);
            if (value instanceof Map<?,?>) return AnswerShape5Object.read(value);
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> (union_item_0 instanceof String || union_item_0 instanceof Map<?,
            ?>)))) return AnswerShape5Array.read(value);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape5String(AnswerResolution value) implements AnswerShape5 {
        static AnswerShape5String read(Object value) {
            return new AnswerShape5String(AnswerResolution.read(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape5Object(Map<String,AnswerResolution> value) implements AnswerShape5 {
        static AnswerShape5Object read(Object value) {
            return new AnswerShape5Object(Values.map(value, item0 -> AnswerResolution.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape5Array(List<AnswerResolutions> value) implements AnswerShape5 {
        static AnswerShape5Array read(Object value) {
            return new AnswerShape5Array(Values.list(value, item0 -> AnswerResolutions.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape6 permits AnswerShape6String,AnswerShape6Null,AnswerShape6Array {
        static AnswerShape6 read(Object value) {
            if (value instanceof String) return AnswerShape6String.read(value);
            if (value == null) return new AnswerShape6Null(null);
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> (union_item_0 instanceof String || union_item_0 == null)))) return AnswerShape6Array.read(value);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape6String(String value) implements AnswerShape6 {
        static AnswerShape6String read(Object value) {
            return new AnswerShape6String((String)value);
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape6Null(Object value) implements AnswerShape6 {
        static AnswerShape6Null read(Object value) {
            return new AnswerShape6Null(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape6Array(List<AnswerShape6AnyOf1Items> value) implements AnswerShape6 {
        static AnswerShape6Array read(Object value) {
            return new AnswerShape6Array(Values.list(value, item0 -> AnswerShape6AnyOf1Items.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape6AnyOf1Items permits AnswerShape6AnyOf1ItemsString,AnswerShape6AnyOf1ItemsNull {
        static AnswerShape6AnyOf1Items read(Object value) {
            if (value instanceof String) return AnswerShape6AnyOf1ItemsString.read(value);
            if (value == null) return new AnswerShape6AnyOf1ItemsNull(null);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape6AnyOf1ItemsString(String value) implements AnswerShape6AnyOf1Items {
        static AnswerShape6AnyOf1ItemsString read(Object value) {
            return new AnswerShape6AnyOf1ItemsString((String)value);
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape6AnyOf1ItemsNull(Object value) implements AnswerShape6AnyOf1Items {
        static AnswerShape6AnyOf1ItemsNull read(Object value) {
            return new AnswerShape6AnyOf1ItemsNull(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape7 permits AnswerShape7Array,AnswerShape7Null,AnswerShape7Array2 {
        static AnswerShape7 read(Object value) {
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> union_item_0 instanceof String))) return AnswerShape7Array.read(value);
            if (value == null) return new AnswerShape7Null(null);
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> ((union_item_0 instanceof List<?> && ((List<?>)union_item_0).stream().allMatch(union_item_1 -> union_item_1 instanceof String)) || union_item_0 == null)))) return AnswerShape7Array2.read(value);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape7Array(List<String> value) implements AnswerShape7 {
        static AnswerShape7Array read(Object value) {
            return new AnswerShape7Array(Values.list(value, item0 -> (String)item0));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape7Null(Object value) implements AnswerShape7 {
        static AnswerShape7Null read(Object value) {
            return new AnswerShape7Null(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape7Array2(List<AnswerShape7AnyOf1Items> value) implements AnswerShape7 {
        static AnswerShape7Array2 read(Object value) {
            return new AnswerShape7Array2(Values.list(value, item0 -> AnswerShape7AnyOf1Items.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape7AnyOf1Items permits AnswerShape7AnyOf1ItemsArray,AnswerShape7AnyOf1ItemsNull {
        static AnswerShape7AnyOf1Items read(Object value) {
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> union_item_0 instanceof String))) return AnswerShape7AnyOf1ItemsArray.read(value);
            if (value == null) return new AnswerShape7AnyOf1ItemsNull(null);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape7AnyOf1ItemsArray(List<String> value) implements AnswerShape7AnyOf1Items {
        static AnswerShape7AnyOf1ItemsArray read(Object value) {
            return new AnswerShape7AnyOf1ItemsArray(Values.list(value, item0 -> (String)item0));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape7AnyOf1ItemsNull(Object value) implements AnswerShape7AnyOf1Items {
        static AnswerShape7AnyOf1ItemsNull read(Object value) {
            return new AnswerShape7AnyOf1ItemsNull(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape8 permits AnswerShape8Number,AnswerShape8Null,AnswerShape8Array {
        static AnswerShape8 read(Object value) {
            if (value instanceof BigDecimal) return AnswerShape8Number.read(value);
            if (value == null) return new AnswerShape8Null(null);
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> (union_item_0 instanceof BigDecimal || union_item_0 == null)))) return AnswerShape8Array.read(value);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape8Number(BigDecimal value) implements AnswerShape8 {
        static AnswerShape8Number read(Object value) {
            return new AnswerShape8Number((BigDecimal)value);
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape8Null(Object value) implements AnswerShape8 {
        static AnswerShape8Null read(Object value) {
            return new AnswerShape8Null(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape8Array(List<AnswerShape8AnyOf1Items> value) implements AnswerShape8 {
        static AnswerShape8Array read(Object value) {
            return new AnswerShape8Array(Values.list(value, item0 -> AnswerShape8AnyOf1Items.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape8AnyOf1Items permits AnswerShape8AnyOf1ItemsNumber,AnswerShape8AnyOf1ItemsNull {
        static AnswerShape8AnyOf1Items read(Object value) {
            if (value instanceof BigDecimal) return AnswerShape8AnyOf1ItemsNumber.read(value);
            if (value == null) return new AnswerShape8AnyOf1ItemsNull(null);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape8AnyOf1ItemsNumber(BigDecimal value) implements AnswerShape8AnyOf1Items {
        static AnswerShape8AnyOf1ItemsNumber read(Object value) {
            return new AnswerShape8AnyOf1ItemsNumber((BigDecimal)value);
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape8AnyOf1ItemsNull(Object value) implements AnswerShape8AnyOf1Items {
        static AnswerShape8AnyOf1ItemsNull read(Object value) {
            return new AnswerShape8AnyOf1ItemsNull(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape9 permits AnswerShape9Array,AnswerShape9Null,AnswerShape9Array2 {
        static AnswerShape9 read(Object value) {
            if (value instanceof List<?>) return AnswerShape9Array.read(value);
            if (value == null) return new AnswerShape9Null(null);
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> (union_item_0 instanceof List<?> || union_item_0 == null)))) return AnswerShape9Array2.read(value);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape9Array(List<SettledOriginal> value) implements AnswerShape9 {
        static AnswerShape9Array read(Object value) {
            return new AnswerShape9Array(Values.list(value, item0 -> SettledOriginal.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape9Null(Object value) implements AnswerShape9 {
        static AnswerShape9Null read(Object value) {
            return new AnswerShape9Null(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape9Array2(List<AnswerShape9AnyOf1Items> value) implements AnswerShape9 {
        static AnswerShape9Array2 read(Object value) {
            return new AnswerShape9Array2(Values.list(value, item0 -> AnswerShape9AnyOf1Items.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public sealed interface AnswerShape9AnyOf1Items permits AnswerShape9AnyOf1ItemsArray,AnswerShape9AnyOf1ItemsNull {
        static AnswerShape9AnyOf1Items read(Object value) {
            if (value instanceof List<?>) return AnswerShape9AnyOf1ItemsArray.read(value);
            if (value == null) return new AnswerShape9AnyOf1ItemsNull(null);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnswerShape9AnyOf1ItemsArray(List<SettledOriginal> value) implements AnswerShape9AnyOf1Items {
        static AnswerShape9AnyOf1ItemsArray read(Object value) {
            return new AnswerShape9AnyOf1ItemsArray(Values.list(value, item0 -> SettledOriginal.read(item0)));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnswerShape9AnyOf1ItemsNull(Object value) implements AnswerShape9AnyOf1Items {
        static AnswerShape9AnyOf1ItemsNull read(Object value) {
            return new AnswerShape9AnyOf1ItemsNull(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public static final class AnswerShapeAnyOf1Items extends View {
        public AnswerShapeAnyOf1Items(Object value) {
            super(value);
        }
        static AnswerShapeAnyOf1Items read(Object value) {
            return new AnswerShapeAnyOf1Items(value);
        }
        public Object value() {
            return Values.freeze(json());
        }
    }
    public static final class Answers extends View {
        public Answers(Object value) {
            super(value);
        }
        static Answers read(Object value) {
            return new Answers(value);
        }
        public List<RelationMember> questions() {
            Object value = required("questions");
            return Values.list(value, item0 -> RelationMember.read(item0));
        }
    }
    public static final class AtomicArrayOfString extends View {
        public AtomicArrayOfString(Object value) {
            super(value);
        }
        static AtomicArrayOfString read(Object value) {
            return new AtomicArrayOfString(value);
        }
        public Answer answer() {
            Object value = required("answer");
            return Answer.read(value);
        }
        public AnswerId answerId() {
            Object value = required("answer_id");
            return AnswerId.read(value);
        }
        public Presence<List<Image>> images() {
            return presence("images", value -> Values.list(value, item0 -> Image.read(item0)));
        }
        public Presence<BigInteger> index() {
            return presence("index", value -> Values.integer(value));
        }
        public Presence<Object> input() {
            return presence("input", value -> Values.freeze(value));
        }
        public Presence<List<RankMember>> members() {
            return presence("members", value -> Values.list(value, item0 -> RankMember.read(item0)));
        }
        public Meta meta() {
            Object value = required("meta");
            return Meta.read(value);
        }
        public QuestionDocument question() {
            Object value = required("question");
            return QuestionDocument.read(value);
        }
        public Presence<String> questionName() {
            return presence("question_name", value -> (String)value);
        }
        public Version schema() {
            Object value = required("schema");
            return Version.read(value);
        }
        public Presence<PhysicalSource> source() {
            return presence("source", value -> PhysicalSource.read(value));
        }
        public Presence<Threshold> threshold() {
            required("threshold");
            return presence("threshold", value -> Threshold.read(value));
        }
        public List<String> value() {
            Object value = required("value");
            return Values.list(value, item0 -> (String)item0);
        }
    }
    public static final class AtomicDecideValue extends View {
        public AtomicDecideValue(Object value) {
            super(value);
        }
        static AtomicDecideValue read(Object value) {
            return new AtomicDecideValue(value);
        }
        public Answer answer() {
            Object value = required("answer");
            return Answer.read(value);
        }
        public AnswerId answerId() {
            Object value = required("answer_id");
            return AnswerId.read(value);
        }
        public Presence<List<Image>> images() {
            return presence("images", value -> Values.list(value, item0 -> Image.read(item0)));
        }
        public Presence<BigInteger> index() {
            return presence("index", value -> Values.integer(value));
        }
        public Presence<Object> input() {
            return presence("input", value -> Values.freeze(value));
        }
        public Presence<List<RankMember>> members() {
            return presence("members", value -> Values.list(value, item0 -> RankMember.read(item0)));
        }
        public Meta meta() {
            Object value = required("meta");
            return Meta.read(value);
        }
        public QuestionDocument question() {
            Object value = required("question");
            return QuestionDocument.read(value);
        }
        public Presence<String> questionName() {
            return presence("question_name", value -> (String)value);
        }
        public Version schema() {
            Object value = required("schema");
            return Version.read(value);
        }
        public Presence<PhysicalSource> source() {
            return presence("source", value -> PhysicalSource.read(value));
        }
        public Presence<Threshold> threshold() {
            required("threshold");
            return presence("threshold", value -> Threshold.read(value));
        }
        public DecideValue value() {
            Object value = required("value");
            return DecideValue.read(value);
        }
    }
    public static final class AtomicNonZeroUsize extends View {
        public AtomicNonZeroUsize(Object value) {
            super(value);
        }
        static AtomicNonZeroUsize read(Object value) {
            return new AtomicNonZeroUsize(value);
        }
        public Answer answer() {
            Object value = required("answer");
            return Answer.read(value);
        }
        public AnswerId answerId() {
            Object value = required("answer_id");
            return AnswerId.read(value);
        }
        public Presence<List<Image>> images() {
            return presence("images", value -> Values.list(value, item0 -> Image.read(item0)));
        }
        public Presence<BigInteger> index() {
            return presence("index", value -> Values.integer(value));
        }
        public Presence<Object> input() {
            return presence("input", value -> Values.freeze(value));
        }
        public Presence<List<RankMember>> members() {
            return presence("members", value -> Values.list(value, item0 -> RankMember.read(item0)));
        }
        public Meta meta() {
            Object value = required("meta");
            return Meta.read(value);
        }
        public QuestionDocument question() {
            Object value = required("question");
            return QuestionDocument.read(value);
        }
        public Presence<String> questionName() {
            return presence("question_name", value -> (String)value);
        }
        public Version schema() {
            Object value = required("schema");
            return Version.read(value);
        }
        public Presence<PhysicalSource> source() {
            return presence("source", value -> PhysicalSource.read(value));
        }
        public Presence<Threshold> threshold() {
            required("threshold");
            return presence("threshold", value -> Threshold.read(value));
        }
        public BigInteger value() {
            Object value = required("value");
            return Values.integer(value);
        }
    }
    public static final class AtomicNullableString extends View {
        public AtomicNullableString(Object value) {
            super(value);
        }
        static AtomicNullableString read(Object value) {
            return new AtomicNullableString(value);
        }
        public Answer answer() {
            Object value = required("answer");
            return Answer.read(value);
        }
        public AnswerId answerId() {
            Object value = required("answer_id");
            return AnswerId.read(value);
        }
        public Presence<List<Image>> images() {
            return presence("images", value -> Values.list(value, item0 -> Image.read(item0)));
        }
        public Presence<BigInteger> index() {
            return presence("index", value -> Values.integer(value));
        }
        public Presence<Object> input() {
            return presence("input", value -> Values.freeze(value));
        }
        public Presence<List<RankMember>> members() {
            return presence("members", value -> Values.list(value, item0 -> RankMember.read(item0)));
        }
        public Meta meta() {
            Object value = required("meta");
            return Meta.read(value);
        }
        public QuestionDocument question() {
            Object value = required("question");
            return QuestionDocument.read(value);
        }
        public Presence<String> questionName() {
            return presence("question_name", value -> (String)value);
        }
        public Version schema() {
            Object value = required("schema");
            return Version.read(value);
        }
        public Presence<PhysicalSource> source() {
            return presence("source", value -> PhysicalSource.read(value));
        }
        public Presence<Threshold> threshold() {
            required("threshold");
            return presence("threshold", value -> Threshold.read(value));
        }
        public Presence<String> value() {
            required("value");
            return presence("value", value -> (String)value);
        }
    }
    public static final class AtomicBoolean extends View {
        public AtomicBoolean(Object value) {
            super(value);
        }
        static AtomicBoolean read(Object value) {
            return new AtomicBoolean(value);
        }
        public Answer answer() {
            Object value = required("answer");
            return Answer.read(value);
        }
        public AnswerId answerId() {
            Object value = required("answer_id");
            return AnswerId.read(value);
        }
        public Presence<List<Image>> images() {
            return presence("images", value -> Values.list(value, item0 -> Image.read(item0)));
        }
        public Presence<BigInteger> index() {
            return presence("index", value -> Values.integer(value));
        }
        public Presence<Object> input() {
            return presence("input", value -> Values.freeze(value));
        }
        public Presence<List<RankMember>> members() {
            return presence("members", value -> Values.list(value, item0 -> RankMember.read(item0)));
        }
        public Meta meta() {
            Object value = required("meta");
            return Meta.read(value);
        }
        public QuestionDocument question() {
            Object value = required("question");
            return QuestionDocument.read(value);
        }
        public Presence<String> questionName() {
            return presence("question_name", value -> (String)value);
        }
        public Version schema() {
            Object value = required("schema");
            return Version.read(value);
        }
        public Presence<PhysicalSource> source() {
            return presence("source", value -> PhysicalSource.read(value));
        }
        public Presence<Threshold> threshold() {
            required("threshold");
            return presence("threshold", value -> Threshold.read(value));
        }
        public Boolean value() {
            Object value = required("value");
            return (Boolean)value;
        }
    }
    public static final class AtomicDouble extends View {
        public AtomicDouble(Object value) {
            super(value);
        }
        static AtomicDouble read(Object value) {
            return new AtomicDouble(value);
        }
        public Answer answer() {
            Object value = required("answer");
            return Answer.read(value);
        }
        public AnswerId answerId() {
            Object value = required("answer_id");
            return AnswerId.read(value);
        }
        public Presence<List<Image>> images() {
            return presence("images", value -> Values.list(value, item0 -> Image.read(item0)));
        }
        public Presence<BigInteger> index() {
            return presence("index", value -> Values.integer(value));
        }
        public Presence<Object> input() {
            return presence("input", value -> Values.freeze(value));
        }
        public Presence<List<RankMember>> members() {
            return presence("members", value -> Values.list(value, item0 -> RankMember.read(item0)));
        }
        public Meta meta() {
            Object value = required("meta");
            return Meta.read(value);
        }
        public QuestionDocument question() {
            Object value = required("question");
            return QuestionDocument.read(value);
        }
        public Presence<String> questionName() {
            return presence("question_name", value -> (String)value);
        }
        public Version schema() {
            Object value = required("schema");
            return Version.read(value);
        }
        public Presence<PhysicalSource> source() {
            return presence("source", value -> PhysicalSource.read(value));
        }
        public Presence<Threshold> threshold() {
            required("threshold");
            return presence("threshold", value -> Threshold.read(value));
        }
        public BigDecimal value() {
            Object value = required("value");
            return (BigDecimal)value;
        }
    }
    public static final class Attempt extends View {
        public Attempt(Object value) {
            super(value);
        }
        static Attempt read(Object value) {
            return new Attempt(value);
        }
        public BigInteger ordinal() {
            Object value = required("ordinal");
            return Values.integer(value);
        }
        public AttemptOutcome outcome() {
            Object value = required("outcome");
            return AttemptOutcome.read(value);
        }
        public Presence<String> requestId() {
            return presence("request_id", value -> (String)value);
        }
        public String requestSha256() {
            Object value = required("request_sha256");
            return (String)value;
        }
        public SdkRequestId sdkRequestId() {
            Object value = required("sdk_request_id");
            return SdkRequestId.read(value);
        }
        public Presence<BigInteger> serverMs() {
            return presence("server_ms", value -> Values.integer(value));
        }
        public Presence<BigInteger> status() {
            return presence("status", value -> Values.integer(value));
        }
        public BigInteger wallMs() {
            Object value = required("wall_ms");
            return Values.integer(value);
        }
    }
    public sealed interface Batch permits BatchInteger,BatchString {
        static Batch read(Object value) {
            if (value instanceof BigDecimal) return BatchInteger.read(value);
            if (value instanceof String) return BatchString.read(value);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record BatchInteger(BigInteger value) implements Batch {
        static BatchInteger read(Object value) {
            return new BatchInteger(Values.integer(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record BatchString(String value) implements Batch {
        static BatchString read(Object value) {
            return new BatchString((String)value);
        }
        Object json() {
            return Values.json(value);
        }
    }
    public static final class BoundaryMode extends View {
        public BoundaryMode(Object value) {
            super(value);
        }
        static BoundaryMode read(Object value) {
            return new BoundaryMode(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public static final class BoundaryOdds extends View {
        public BoundaryOdds(Object value) {
            super(value);
        }
        static BoundaryOdds read(Object value) {
            return new BoundaryOdds(value);
        }
        public List<PieceOdds> pieces() {
            Object value = required("pieces");
            return Values.list(value, item0 -> PieceOdds.read(item0));
        }
        public List<BoundaryProposal> proposals() {
            Object value = required("proposals");
            return Values.list(value, item0 -> BoundaryProposal.read(item0));
        }
    }
    public static final class BoundaryProposal extends View {
        public BoundaryProposal(Object value) {
            super(value);
        }
        static BoundaryProposal read(Object value) {
            return new BoundaryProposal(value);
        }
        public BigInteger end() {
            Object value = required("end");
            return Values.integer(value);
        }
        public Presence<String> file() {
            return presence("file", value -> (String)value);
        }
        public Presence<BigInteger> firstLine() {
            return presence("first_line", value -> Values.integer(value));
        }
        public Presence<BigInteger> lastLine() {
            return presence("last_line", value -> Values.integer(value));
        }
        public BigInteger length() {
            Object value = required("length");
            return Values.integer(value);
        }
        public BigDecimal probability() {
            Object value = required("probability");
            return (BigDecimal)value;
        }
        public BigInteger start() {
            Object value = required("start");
            return Values.integer(value);
        }
        public String text() {
            Object value = required("text");
            return (String)value;
        }
    }
    public static final class CallError extends View {
        public CallError(Object value) {
            super(value);
        }
        static CallError read(Object value) {
            return new CallError(value);
        }
        public Error error() {
            Object value = required("error");
            return Error.read(value);
        }
        public Presence<Facts> facts() {
            return presence("facts", value -> Facts.read(value));
        }
        public AnswerResolution resolution() {
            Object value = required("resolution");
            return AnswerResolution.read(value);
        }
    }
    public static final class CallId extends View {
        public CallId(Object value) {
            super(value);
        }
        static CallId read(Object value) {
            return new CallId(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public static final class DecideValue extends View {
        public DecideValue(Object value) {
            super(value);
        }
        static DecideValue read(Object value) {
            return new DecideValue(value);
        }
        public Object value() {
            return Values.freeze(json());
        }
    }
    public static final class EntityDocument extends View {
        public EntityDocument(Object value) {
            super(value);
        }
        static EntityDocument read(Object value) {
            return new EntityDocument(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public String name() {
            Object value = required("name");
            return (String)value;
        }
    }
    public static final class Error extends View {
        public Error(Object value) {
            super(value);
        }
        static Error read(Object value) {
            return new Error(value);
        }
        public Presence<EstimatedInputDenial> estimatedInputDenial() {
            return presence("estimated_input_denial", value -> EstimatedInputDenial.read(value));
        }
        public FailureKind kind() {
            Object value = required("kind");
            return FailureKind.read(value);
        }
        public String message() {
            Object value = required("message");
            return (String)value;
        }
        public Boolean retryable() {
            Object value = required("retryable");
            return (Boolean)value;
        }
        public Presence<SendBudgetDenial> sendBudgetDenial() {
            return presence("send_budget_denial", value -> SendBudgetDenial.read(value));
        }
        public Stopped stopped() {
            Object value = required("stopped");
            return Stopped.read(value);
        }
    }
    public sealed interface EstimatedInputDenial permits EstimatedInputDenialInitialRequest,EstimatedInputDenialAdditionalRequest,
    EstimatedInputDenialRetry {
        static EstimatedInputDenial read(Object value) {
            Map<String,Object> object = Values.object(value);
            if ("initial_request".equals(object.get("kind"))) return EstimatedInputDenialInitialRequest.read(value);
            if ("additional_request".equals(object.get("kind"))) return EstimatedInputDenialAdditionalRequest.read(value);
            if ("retry".equals(object.get("kind"))) return EstimatedInputDenialRetry.read(value);
            throw new IllegalStateException("Unknown native result alternative");
        }
    }
    public static final class EstimatedInputDenialAdditionalRequest extends View implements EstimatedInputDenial {
        public EstimatedInputDenialAdditionalRequest(Object value) {
            super(value);
        }
        static EstimatedInputDenialAdditionalRequest read(Object value) {
            return new EstimatedInputDenialAdditionalRequest(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public BigInteger limit() {
            Object value = required("limit");
            return Values.integer(value);
        }
    }
    public static final class EstimatedInputDenialInitialRequest extends View implements EstimatedInputDenial {
        public EstimatedInputDenialInitialRequest(Object value) {
            super(value);
        }
        static EstimatedInputDenialInitialRequest read(Object value) {
            return new EstimatedInputDenialInitialRequest(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public BigInteger limit() {
            Object value = required("limit");
            return Values.integer(value);
        }
    }
    public static final class EstimatedInputDenialRetry extends View implements EstimatedInputDenial {
        public EstimatedInputDenialRetry(Object value) {
            super(value);
        }
        static EstimatedInputDenialRetry read(Object value) {
            return new EstimatedInputDenialRetry(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public BigInteger lastStatus() {
            Object value = required("last_status");
            return Values.integer(value);
        }
        public BigInteger limit() {
            Object value = required("limit");
            return Values.integer(value);
        }
    }
    public static final class Facts extends View {
        public Facts(Object value) {
            super(value);
        }
        static Facts read(Object value) {
            return new Facts(value);
        }
        public Presence<List<Attempt>> attempts() {
            return presence("attempts", value -> Values.list(value, item0 -> Attempt.read(item0)));
        }
        public BigInteger cacheAnswers() {
            Object value = required("cache_answers");
            return Values.integer(value);
        }
        public CallId callId() {
            Object value = required("call_id");
            return CallId.read(value);
        }
        public Presence<String> estimatedCostUsd() {
            return presence("estimated_cost_usd", value -> (String)value);
        }
        public Presence<Boolean> heldModelMismatch() {
            return presence("held_model_mismatch", value -> (Boolean)value);
        }
        public Presence<BigInteger> inputTokens() {
            return presence("input_tokens", value -> Values.integer(value));
        }
        public BigInteger largestRequestBytes() {
            Object value = required("largest_request_bytes");
            return Values.integer(value);
        }
        public Presence<BigInteger> largestRequestEstimatedInputTokens() {
            required("largest_request_estimated_input_tokens");
            return presence("largest_request_estimated_input_tokens", value -> Values.integer(value));
        }
        public Presence<String> model() {
            return presence("model", value -> (String)value);
        }
        public Presence<BigInteger> outputTokens() {
            return presence("output_tokens", value -> Values.integer(value));
        }
        public BigInteger records() {
            Object value = required("records");
            return Values.integer(value);
        }
        public BigInteger requestsSent() {
            Object value = required("requests_sent");
            return Values.integer(value);
        }
        public BigDecimal seconds() {
            Object value = required("seconds");
            return (BigDecimal)value;
        }
        public String tokenEstimateMethod() {
            Object value = required("token_estimate_method");
            return (String)value;
        }
        public Presence<PersistenceObservation> usagePersistence() {
            return presence("usage_persistence", value -> PersistenceObservation.read(value));
        }
    }
    public static final class FailureId extends View {
        public FailureId(Object value) {
            super(value);
        }
        static FailureId read(Object value) {
            return new FailureId(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public static final class Find extends View {
        public Find(Object value) {
            super(value);
        }
        static Find read(Object value) {
            return new Find(value);
        }
        public FindAnswer answer() {
            Object value = required("answer");
            return FindAnswer.read(value);
        }
        public AnswerId answerId() {
            Object value = required("answer_id");
            return AnswerId.read(value);
        }
        public Presence<List<FindCandidate>> candidates() {
            return presence("candidates", value -> Values.list(value, item0 -> FindCandidate.read(item0)));
        }
        public Presence<String> file() {
            return presence("file", value -> (String)value);
        }
        public Presence<BigInteger> firstLine() {
            return presence("first_line", value -> Values.integer(value));
        }
        public Presence<BigInteger> index() {
            required("index");
            return presence("index", value -> Values.integer(value));
        }
        public Presence<BigInteger> lastLine() {
            return presence("last_line", value -> Values.integer(value));
        }
        public Meta meta() {
            Object value = required("meta");
            return Meta.read(value);
        }
        public Presence<Position> position() {
            return presence("position", value -> Position.read(value));
        }
        public FindQuestionDocument question() {
            Object value = required("question");
            return FindQuestionDocument.read(value);
        }
        public Version schema() {
            Object value = required("schema");
            return Version.read(value);
        }
        public Presence<Object> threshold() {
            required("threshold");
            return presence("threshold", value -> Values.freeze(value));
        }
        public Presence<Object> value() {
            required("value");
            return presence("value", value -> Values.freeze(value));
        }
    }
    public static final class FindCandidate extends View {
        public FindCandidate(Object value) {
            super(value);
        }
        static FindCandidate read(Object value) {
            return new FindCandidate(value);
        }
        public Presence<BigInteger> index() {
            required("index");
            return presence("index", value -> Values.integer(value));
        }
        public Presence<Object> input() {
            required("input");
            return presence("input", value -> Values.freeze(value));
        }
        public BigDecimal probability() {
            Object value = required("probability");
            return (BigDecimal)value;
        }
        public Presence<PhysicalSource> source() {
            return presence("source", value -> PhysicalSource.read(value));
        }
    }
    public static final class Image extends View {
        public Image(Object value) {
            super(value);
        }
        static Image read(Object value) {
            return new Image(value);
        }
        public String base64() {
            Object value = required("base64");
            return (String)value;
        }
        public BigInteger height() {
            Object value = required("height");
            return Values.integer(value);
        }
        public ImageMedia media() {
            Object value = required("media");
            return ImageMedia.read(value);
        }
        public BigInteger width() {
            Object value = required("width");
            return Values.integer(value);
        }
    }
    public static final class ImageMedia extends View {
        public ImageMedia(Object value) {
            super(value);
        }
        static ImageMedia read(Object value) {
            return new ImageMedia(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public sealed interface InputDeclaration permits InputDeclarationStringRoot,InputDeclarationObjectRoot {
        static InputDeclaration read(Object value) {
            Map<String,Object> object = Values.object(value);
            if ("string".equals(object.get("type"))) return InputDeclarationStringRoot.read(value);
            if ("object".equals(object.get("type"))) return InputDeclarationObjectRoot.read(value);
            throw new IllegalStateException("Unknown native result alternative");
        }
    }
    public static final class InputDeclarationObjectRoot extends View implements InputDeclaration {
        public InputDeclarationObjectRoot(Object value) {
            super(value);
        }
        static InputDeclarationObjectRoot read(Object value) {
            return new InputDeclarationObjectRoot(value);
        }
        public Map<String,InputPropertyType> properties() {
            Object value = required("properties");
            return Values.map(value, item0 -> InputPropertyType.read(item0));
        }
        public Presence<List<String>> required() {
            return presence("required", value -> Values.list(value, item0 -> (String)item0));
        }
        public ObjectType type() {
            Object value = required("type");
            return ObjectType.read(value);
        }
    }
    public static final class InputDeclarationStringRoot extends View implements InputDeclaration {
        public InputDeclarationStringRoot(Object value) {
            super(value);
        }
        static InputDeclarationStringRoot read(Object value) {
            return new InputDeclarationStringRoot(value);
        }
        public StringType type() {
            Object value = required("type");
            return StringType.read(value);
        }
    }
    public sealed interface InputPropertyType permits InputPropertyTypeString,InputPropertyTypeNumber,
    InputPropertyTypeBoolean,InputPropertyTypeArray {
        static InputPropertyType read(Object value) {
            Map<String,Object> object = Values.object(value);
            if ("string".equals(object.get("type"))) return InputPropertyTypeString.read(value);
            if ("number".equals(object.get("type"))) return InputPropertyTypeNumber.read(value);
            if ("boolean".equals(object.get("type"))) return InputPropertyTypeBoolean.read(value);
            if ("array".equals(object.get("type"))) return InputPropertyTypeArray.read(value);
            throw new IllegalStateException("Unknown native result alternative");
        }
    }
    public static final class InputPropertyTypeArray extends View implements InputPropertyType {
        public InputPropertyTypeArray(Object value) {
            super(value);
        }
        static InputPropertyTypeArray read(Object value) {
            return new InputPropertyTypeArray(value);
        }
        public StringRoot items() {
            Object value = required("items");
            return StringRoot.read(value);
        }
        public String type() {
            Object value = required("type");
            return (String)value;
        }
    }
    public static final class InputPropertyTypeBoolean extends View implements InputPropertyType {
        public InputPropertyTypeBoolean(Object value) {
            super(value);
        }
        static InputPropertyTypeBoolean read(Object value) {
            return new InputPropertyTypeBoolean(value);
        }
        public String type() {
            Object value = required("type");
            return (String)value;
        }
    }
    public static final class InputPropertyTypeNumber extends View implements InputPropertyType {
        public InputPropertyTypeNumber(Object value) {
            super(value);
        }
        static InputPropertyTypeNumber read(Object value) {
            return new InputPropertyTypeNumber(value);
        }
        public String type() {
            Object value = required("type");
            return (String)value;
        }
    }
    public static final class InputPropertyTypeString extends View implements InputPropertyType {
        public InputPropertyTypeString(Object value) {
            super(value);
        }
        static InputPropertyTypeString read(Object value) {
            return new InputPropertyTypeString(value);
        }
        public String type() {
            Object value = required("type");
            return (String)value;
        }
    }
    public static final class Label extends View {
        public Label(Object value) {
            super(value);
        }
        static Label read(Object value) {
            return new Label(value);
        }
        public Presence<Object> description() {
            return presence("description", value -> Values.freeze(value));
        }
        public String name() {
            Object value = required("name");
            return (String)value;
        }
    }
    public static final class Meta extends View {
        public Meta(Object value) {
            super(value);
        }
        static Meta read(Object value) {
            return new Meta(value);
        }
        public Presence<String> answeredBy() {
            return presence("answered_by", value -> (String)value);
        }
        public Presence<List<Attempt>> attempts() {
            return presence("attempts", value -> Values.list(value, item0 -> Attempt.read(item0)));
        }
        public Presence<BatchSetting> batchSetting() {
            return presence("batch_setting", value -> BatchSetting.read(value));
        }
        public Presence<BatchWarning> batchWarning() {
            return presence("batch_warning", value -> BatchWarning.read(value));
        }
        public Boolean cached() {
            Object value = required("cached");
            return (Boolean)value;
        }
        public Presence<String> contextSha256() {
            return presence("context_sha256", value -> (String)value);
        }
        public BigInteger failedQuestions() {
            Object value = required("failed_questions");
            return Values.integer(value);
        }
        public String model() {
            Object value = required("model");
            return (String)value;
        }
        public List<Observation> observations() {
            Object value = required("observations");
            return Values.list(value, item0 -> Observation.read(item0));
        }
        public Presence<Origin> origin() {
            required("origin");
            return presence("origin", value -> Origin.read(value));
        }
        public Presence<ProfileWarning> profileWarning() {
            return presence("profile_warning", value -> ProfileWarning.read(value));
        }
        public Presence<String> questionSha256() {
            return presence("question_sha256", value -> (String)value);
        }
        public List<QuestionSource> questionSources() {
            Object value = required("question_sources");
            return Values.list(value, item0 -> QuestionSource.read(item0));
        }
        public Presence<String> questionsSha256() {
            return presence("questions_sha256", value -> (String)value);
        }
        public List<String> requests() {
            Object value = required("requests");
            return Values.list(value, item0 -> (String)item0);
        }
        public BigInteger requestsSent() {
            Object value = required("requests_sent");
            return Values.integer(value);
        }
        public String tool() {
            Object value = required("tool");
            return (String)value;
        }
        public String url() {
            Object value = required("url");
            return (String)value;
        }
        public Presence<Usage> usage() {
            return presence("usage", value -> Usage.read(value));
        }
    }
    public static final class ObjectRoot extends View {
        public ObjectRoot(Object value) {
            super(value);
        }
        static ObjectRoot read(Object value) {
            return new ObjectRoot(value);
        }
        public Map<String,InputPropertyType> properties() {
            Object value = required("properties");
            return Values.map(value, item0 -> InputPropertyType.read(item0));
        }
        public Presence<List<String>> required() {
            return presence("required", value -> Values.list(value, item0 -> (String)item0));
        }
        public ObjectType type() {
            Object value = required("type");
            return ObjectType.read(value);
        }
    }
    public static final class ObjectType extends View {
        public ObjectType(Object value) {
            super(value);
        }
        static ObjectType read(Object value) {
            return new ObjectType(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public sealed interface Observation permits ObservationObservationId,ObservationFailureId {
        static Observation read(Object value) {
            Map<String,Object> object = Values.object(value);
            if (object.containsKey("observation_id")) return ObservationObservationId.read(value);
            if (object.containsKey("failure_id")) return ObservationFailureId.read(value);
            throw new IllegalStateException("Unknown native result alternative");
        }
    }
    public static final class ObservationId extends View {
        public ObservationId(Object value) {
            super(value);
        }
        static ObservationId read(Object value) {
            return new ObservationId(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public static final class ObservationFailureId extends View implements Observation {
        public ObservationFailureId(Object value) {
            super(value);
        }
        static ObservationFailureId read(Object value) {
            return new ObservationFailureId(value);
        }
        public FailureId failureId() {
            Object value = required("failure_id");
            return FailureId.read(value);
        }
    }
    public static final class ObservationObservationId extends View implements Observation {
        public ObservationObservationId(Object value) {
            super(value);
        }
        static ObservationObservationId read(Object value) {
            return new ObservationObservationId(value);
        }
        public ObservationId observationId() {
            Object value = required("observation_id");
            return ObservationId.read(value);
        }
    }
    public static final class Origin extends View {
        public Origin(Object value) {
            super(value);
        }
        static Origin read(Object value) {
            return new Origin(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public static final class PersistenceObservation extends View {
        public PersistenceObservation(Object value) {
            super(value);
        }
        static PersistenceObservation read(Object value) {
            return new PersistenceObservation(value);
        }
        public Presence<String> advice() {
            return presence("advice", value -> (String)value);
        }
        public String observedAt() {
            Object value = required("observed_at");
            return (String)value;
        }
        public UsagePersistence state() {
            Object value = required("state");
            return UsagePersistence.read(value);
        }
    }
    public static final class PhysicalSource extends View {
        public PhysicalSource(Object value) {
            super(value);
        }
        static PhysicalSource read(Object value) {
            return new PhysicalSource(value);
        }
        public String file() {
            Object value = required("file");
            return (String)value;
        }
        public Presence<BigInteger> firstLine() {
            return presence("first_line", value -> Values.integer(value));
        }
        public Presence<BigInteger> lastLine() {
            return presence("last_line", value -> Values.integer(value));
        }
    }
    public static final class Position extends View {
        public Position(Object value) {
            super(value);
        }
        static Position read(Object value) {
            return new Position(value);
        }
        public Presence<String> file() {
            required("file");
            return presence("file", value -> (String)value);
        }
        public Presence<BigInteger> first() {
            return presence("first", value -> Values.integer(value));
        }
        public Presence<List<String>> images() {
            return presence("images", value -> Values.list(value, item0 -> (String)item0));
        }
        public Presence<BigInteger> last() {
            return presence("last", value -> Values.integer(value));
        }
    }
    public static final class QuestionName extends View {
        public QuestionName(Object value) {
            super(value);
        }
        static QuestionName read(Object value) {
            return new QuestionName(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public static final class QuestionSource extends View {
        public QuestionSource(Object value) {
            super(value);
        }
        static QuestionSource read(Object value) {
            return new QuestionSource(value);
        }
        public String answeredBy() {
            Object value = required("answered_by");
            return (String)value;
        }
        public Presence<BigInteger> batchSize() {
            return presence("batch_size", value -> Values.integer(value));
        }
        public Origin origin() {
            Object value = required("origin");
            return Origin.read(value);
        }
    }
    public static final class RankMember extends View {
        public RankMember(Object value) {
            super(value);
        }
        static RankMember read(Object value) {
            return new RankMember(value);
        }
        public String name() {
            Object value = required("name");
            return (String)value;
        }
        public RankMemberResult result() {
            Object value = required("result");
            return RankMemberResult.read(value);
        }
    }
    public static final class RankMemberResult extends View {
        public RankMemberResult(Object value) {
            super(value);
        }
        static RankMemberResult read(Object value) {
            return new RankMemberResult(value);
        }
        public Answer answer() {
            Object value = required("answer");
            return Answer.read(value);
        }
        public AnswerId answerId() {
            Object value = required("answer_id");
            return AnswerId.read(value);
        }
        public Presence<List<Image>> images() {
            return presence("images", value -> Values.list(value, item0 -> Image.read(item0)));
        }
        public Meta meta() {
            Object value = required("meta");
            return Meta.read(value);
        }
        public QuestionDocument question() {
            Object value = required("question");
            return QuestionDocument.read(value);
        }
        public Version schema() {
            Object value = required("schema");
            return Version.read(value);
        }
        public Presence<PhysicalSource> source() {
            return presence("source", value -> PhysicalSource.read(value));
        }
        public Presence<Object> threshold() {
            required("threshold");
            return presence("threshold", value -> Values.freeze(value));
        }
        public BigInteger value() {
            Object value = required("value");
            return Values.integer(value);
        }
    }
    public static final class Recognition extends View {
        public Recognition(Object value) {
            super(value);
        }
        static Recognition read(Object value) {
            return new Recognition(value);
        }
        public RecognitionOdds answer() {
            Object value = required("answer");
            return RecognitionOdds.read(value);
        }
        public AnswerId answerId() {
            Object value = required("answer_id");
            return AnswerId.read(value);
        }
        public Presence<String> file() {
            return presence("file", value -> (String)value);
        }
        public Presence<BigInteger> firstLine() {
            return presence("first_line", value -> Values.integer(value));
        }
        public Presence<BigInteger> index() {
            return presence("index", value -> Values.integer(value));
        }
        public Presence<Object> input() {
            return presence("input", value -> Values.freeze(value));
        }
        public Presence<BigInteger> lastLine() {
            return presence("last_line", value -> Values.integer(value));
        }
        public Meta meta() {
            Object value = required("meta");
            return Meta.read(value);
        }
        public Presence<Position> position() {
            return presence("position", value -> Position.read(value));
        }
        public RecognizeQuestionDocument question() {
            Object value = required("question");
            return RecognizeQuestionDocument.read(value);
        }
        public Version schema() {
            Object value = required("schema");
            return Version.read(value);
        }
        public Presence<PhysicalSource> source() {
            return presence("source", value -> PhysicalSource.read(value));
        }
        public Recognize value() {
            Object value = required("value");
            return Recognize.read(value);
        }
    }
    public static final class RecognitionEdgeDocument extends View {
        public RecognitionEdgeDocument(Object value) {
            super(value);
        }
        static RecognitionEdgeDocument read(Object value) {
            return new RecognitionEdgeDocument(value);
        }
        public Boolean either() {
            Object value = required("either");
            return (Boolean)value;
        }
        public BigDecimal probability() {
            Object value = required("probability");
            return (BigDecimal)value;
        }
        public String relation() {
            Object value = required("relation");
            return (String)value;
        }
        public Entity source() {
            Object value = required("source");
            return Entity.read(value);
        }
        public Entity target() {
            Object value = required("target");
            return Entity.read(value);
        }
    }
    public static final class RecognitionMode extends View {
        public RecognitionMode(Object value) {
            super(value);
        }
        static RecognitionMode read(Object value) {
            return new RecognitionMode(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public sealed interface RecognitionOdds permits RecognitionOddsRecognizeAnswer,RecognitionOddsBoundaryOdds {
        static RecognitionOdds read(Object value) {
            Map<String,Object> object = Values.object(value);
            if (object.containsKey("names") && object.containsKey("pairs") && object.containsKey("pieces") && object.containsKey("proposals")) return RecognitionOddsRecognizeAnswer.read(value);
            if (object.containsKey("pieces") && object.containsKey("proposals") && !object.containsKey("names") && !object.containsKey("pairs")) return RecognitionOddsBoundaryOdds.read(value);
            throw new IllegalStateException("Unknown native result alternative");
        }
    }
    public static final class RecognitionOddsBoundaryOdds extends View implements RecognitionOdds {
        public RecognitionOddsBoundaryOdds(Object value) {
            super(value);
        }
        static RecognitionOddsBoundaryOdds read(Object value) {
            return new RecognitionOddsBoundaryOdds(value);
        }
        public List<PieceOdds> pieces() {
            Object value = required("pieces");
            return Values.list(value, item0 -> PieceOdds.read(item0));
        }
        public List<BoundaryProposal> proposals() {
            Object value = required("proposals");
            return Values.list(value, item0 -> BoundaryProposal.read(item0));
        }
    }
    public static final class RecognitionOddsRecognizeAnswer extends View implements RecognitionOdds {
        public RecognitionOddsRecognizeAnswer(Object value) {
            super(value);
        }
        static RecognitionOddsRecognizeAnswer read(Object value) {
            return new RecognitionOddsRecognizeAnswer(value);
        }
        public List<NameOdds> names() {
            Object value = required("names");
            return Values.list(value, item0 -> NameOdds.read(item0));
        }
        public List<PairOdds> pairs() {
            Object value = required("pairs");
            return Values.list(value, item0 -> PairOdds.read(item0));
        }
        public List<PieceOdds> pieces() {
            Object value = required("pieces");
            return Values.list(value, item0 -> PieceOdds.read(item0));
        }
        public List<RecognitionProposal> proposals() {
            Object value = required("proposals");
            return Values.list(value, item0 -> RecognitionProposal.read(item0));
        }
    }
    public static final class RecognitionProposal extends View {
        public RecognitionProposal(Object value) {
            super(value);
        }
        static RecognitionProposal read(Object value) {
            return new RecognitionProposal(value);
        }
        public BigInteger end() {
            Object value = required("end");
            return Values.integer(value);
        }
        public Boolean kept() {
            Object value = required("kept");
            return (Boolean)value;
        }
        public Presence<String> kind() {
            return presence("kind", value -> (String)value);
        }
        public Presence<Place> selected() {
            return presence("selected", value -> Place.read(value));
        }
        public BigDecimal spanProbability() {
            Object value = required("span_probability");
            return (BigDecimal)value;
        }
        public BigInteger start() {
            Object value = required("start");
            return Values.integer(value);
        }
        public Presence<BigDecimal> strength() {
            return presence("strength", value -> (BigDecimal)value);
        }
    }
    public static final class RecognitionStageContext extends View {
        public RecognitionStageContext(Object value) {
            super(value);
        }
        static RecognitionStageContext read(Object value) {
            return new RecognitionStageContext(value);
        }
        public Presence<String> boundary() {
            return presence("boundary", value -> (String)value);
        }
        public Presence<String> kindEdge() {
            return presence("kind_edge", value -> (String)value);
        }
        public Presence<String> relation() {
            return presence("relation", value -> (String)value);
        }
    }
    public static final class Relation extends View {
        public Relation(Object value) {
            super(value);
        }
        static Relation read(Object value) {
            return new Relation(value);
        }
        public Answers answer() {
            Object value = required("answer");
            return Answers.read(value);
        }
        public AnswerId answerId() {
            Object value = required("answer_id");
            return AnswerId.read(value);
        }
        public Presence<String> file() {
            return presence("file", value -> (String)value);
        }
        public Presence<BigInteger> firstLine() {
            return presence("first_line", value -> Values.integer(value));
        }
        public Presence<BigInteger> index() {
            return presence("index", value -> Values.integer(value));
        }
        public Presence<Object> input() {
            return presence("input", value -> Values.freeze(value));
        }
        public Presence<List<SessionInputSource>> inputSources() {
            return presence("input_sources", value -> Values.list(value, item0 -> SessionInputSource.read(item0)));
        }
        public Presence<BigInteger> lastLine() {
            return presence("last_line", value -> Values.integer(value));
        }
        public Meta meta() {
            Object value = required("meta");
            return Meta.read(value);
        }
        public Presence<Position> position() {
            return presence("position", value -> Position.read(value));
        }
        public RelateQuestionDocument question() {
            Object value = required("question");
            return RelateQuestionDocument.read(value);
        }
        public Version schema() {
            Object value = required("schema");
            return Version.read(value);
        }
        public List<RelatedEntityEdge> value() {
            Object value = required("value");
            return Values.list(value, item0 -> RelatedEntityEdge.read(item0));
        }
    }
    public static final class RelationDirection extends View {
        public RelationDirection(Object value) {
            super(value);
        }
        static RelationDirection read(Object value) {
            return new RelationDirection(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public sealed interface RelationEndpoint permits RelationEndpointRelatedEntity,RelationEndpointSource {
        static RelationEndpoint read(Object value) {
            Map<String,Object> object = Values.object(value);
            if (object.containsKey("kind") && object.containsKey("name") && !object.containsKey("file") && !object.containsKey("ordinal") && !object.containsKey("record")) return RelationEndpointRelatedEntity.read(value);
            if (object.containsKey("file") && object.containsKey("kind") && object.containsKey("name") && object.containsKey("ordinal") && object.containsKey("record")) return RelationEndpointSource.read(value);
            throw new IllegalStateException("Unknown native result alternative");
        }
    }
    public static final class RelationEndpointRelatedEntity extends View implements RelationEndpoint {
        public RelationEndpointRelatedEntity(Object value) {
            super(value);
        }
        static RelationEndpointRelatedEntity read(Object value) {
            return new RelationEndpointRelatedEntity(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public String name() {
            Object value = required("name");
            return (String)value;
        }
    }
    public static final class RelationEndpointSource extends View implements RelationEndpoint {
        public RelationEndpointSource(Object value) {
            super(value);
        }
        static RelationEndpointSource read(Object value) {
            return new RelationEndpointSource(value);
        }
        public Presence<String> file() {
            required("file");
            return presence("file", value -> (String)value);
        }
        public Presence<BigInteger> firstLine() {
            return presence("first_line", value -> Values.integer(value));
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public Presence<BigInteger> lastLine() {
            return presence("last_line", value -> Values.integer(value));
        }
        public String name() {
            Object value = required("name");
            return (String)value;
        }
        public BigInteger ordinal() {
            Object value = required("ordinal");
            return Values.integer(value);
        }
        public Object record() {
            Object value = required("record");
            return Values.freeze(value);
        }
    }
    public sealed interface RelationMember permits RelationMemberAnswerId,RelationMemberFailureId {
        static RelationMember read(Object value) {
            Map<String,Object> object = Values.object(value);
            if (object.containsKey("answer_id")) return RelationMemberAnswerId.read(value);
            if (object.containsKey("failure_id")) return RelationMemberFailureId.read(value);
            throw new IllegalStateException("Unknown native result alternative");
        }
    }
    public static final class RelationMemberAnswerId extends View implements RelationMember {
        public RelationMemberAnswerId(Object value) {
            super(value);
        }
        static RelationMemberAnswerId read(Object value) {
            return new RelationMemberAnswerId(value);
        }
        public RelationDirection direction() {
            Object value = required("direction");
            return RelationDirection.read(value);
        }
        public RelationMethod method() {
            Object value = required("method");
            return RelationMethod.read(value);
        }
        public List<Observation> observations() {
            Object value = required("observations");
            return Values.list(value, item0 -> Observation.read(item0));
        }
        public QuestionDocument question() {
            Object value = required("question");
            return QuestionDocument.read(value);
        }
        public List<QuestionSource> questionSources() {
            Object value = required("question_sources");
            return Values.list(value, item0 -> QuestionSource.read(item0));
        }
        public String reads() {
            Object value = required("reads");
            return (String)value;
        }
        public String relation() {
            Object value = required("relation");
            return (String)value;
        }
        public String request() {
            Object value = required("request");
            return (String)value;
        }
        public RelatedEntity source() {
            Object value = required("source");
            return RelatedEntity.read(value);
        }
        public Presence<RelatedEntity> target() {
            required("target");
            return presence("target", value -> RelatedEntity.read(value));
        }
        public Threshold threshold() {
            Object value = required("threshold");
            return Threshold.read(value);
        }
        public Presence<Usage> usage() {
            return presence("usage", value -> Usage.read(value));
        }
        public Boolean accepted() {
            Object value = required("accepted");
            return (Boolean)value;
        }
        public Answer answer() {
            Object value = required("answer");
            return Answer.read(value);
        }
        public AnswerId answerId() {
            Object value = required("answer_id");
            return AnswerId.read(value);
        }
        public BigDecimal probability() {
            Object value = required("probability");
            return (BigDecimal)value;
        }
        public String sourceName() {
            return (String)Values.object(required("source")).get("name");
        }
    }
    public static final class RelationMemberFailureId extends View implements RelationMember {
        public RelationMemberFailureId(Object value) {
            super(value);
        }
        static RelationMemberFailureId read(Object value) {
            return new RelationMemberFailureId(value);
        }
        public RelationDirection direction() {
            Object value = required("direction");
            return RelationDirection.read(value);
        }
        public RelationMethod method() {
            Object value = required("method");
            return RelationMethod.read(value);
        }
        public List<Observation> observations() {
            Object value = required("observations");
            return Values.list(value, item0 -> Observation.read(item0));
        }
        public QuestionDocument question() {
            Object value = required("question");
            return QuestionDocument.read(value);
        }
        public List<QuestionSource> questionSources() {
            Object value = required("question_sources");
            return Values.list(value, item0 -> QuestionSource.read(item0));
        }
        public String reads() {
            Object value = required("reads");
            return (String)value;
        }
        public String relation() {
            Object value = required("relation");
            return (String)value;
        }
        public String request() {
            Object value = required("request");
            return (String)value;
        }
        public RelatedEntity source() {
            Object value = required("source");
            return RelatedEntity.read(value);
        }
        public Presence<RelatedEntity> target() {
            required("target");
            return presence("target", value -> RelatedEntity.read(value));
        }
        public Threshold threshold() {
            Object value = required("threshold");
            return Threshold.read(value);
        }
        public Presence<Usage> usage() {
            return presence("usage", value -> Usage.read(value));
        }
        public Failure failure() {
            Object value = required("failure");
            return Failure.read(value);
        }
        public FailureId failureId() {
            Object value = required("failure_id");
            return FailureId.read(value);
        }
        public String sourceName() {
            return (String)Values.object(required("source")).get("name");
        }
    }
    public static final class RelationMethod extends View {
        public RelationMethod(Object value) {
            super(value);
        }
        static RelationMethod read(Object value) {
            return new RelationMethod(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public static final class RequestFunction extends View {
        public RequestFunction(Object value) {
            super(value);
        }
        static RequestFunction read(Object value) {
            return new RequestFunction(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public static final class SdkRequestId extends View {
        public SdkRequestId(Object value) {
            super(value);
        }
        static SdkRequestId read(Object value) {
            return new SdkRequestId(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public sealed interface SendBudgetDenial permits SendBudgetDenialBeforeFirstSend,SendBudgetDenialBeforeAdditionalSend,
    SendBudgetDenialBeforeRetry {
        static SendBudgetDenial read(Object value) {
            Map<String,Object> object = Values.object(value);
            if ("before_first_send".equals(object.get("kind"))) return SendBudgetDenialBeforeFirstSend.read(value);
            if ("before_additional_send".equals(object.get("kind"))) return SendBudgetDenialBeforeAdditionalSend.read(value);
            if ("before_retry".equals(object.get("kind"))) return SendBudgetDenialBeforeRetry.read(value);
            throw new IllegalStateException("Unknown native result alternative");
        }
    }
    public static final class SendBudgetDenialBeforeAdditionalSend extends View implements SendBudgetDenial {
        public SendBudgetDenialBeforeAdditionalSend(Object value) {
            super(value);
        }
        static SendBudgetDenialBeforeAdditionalSend read(Object value) {
            return new SendBudgetDenialBeforeAdditionalSend(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
    }
    public static final class SendBudgetDenialBeforeFirstSend extends View implements SendBudgetDenial {
        public SendBudgetDenialBeforeFirstSend(Object value) {
            super(value);
        }
        static SendBudgetDenialBeforeFirstSend read(Object value) {
            return new SendBudgetDenialBeforeFirstSend(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
    }
    public static final class SendBudgetDenialBeforeRetry extends View implements SendBudgetDenial {
        public SendBudgetDenialBeforeRetry(Object value) {
            super(value);
        }
        static SendBudgetDenialBeforeRetry read(Object value) {
            return new SendBudgetDenialBeforeRetry(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public BigInteger lastStatus() {
            Object value = required("last_status");
            return Values.integer(value);
        }
    }
    public static final class SettledDecision extends View {
        public SettledDecision(Object value) {
            super(value);
        }
        static SettledDecision read(Object value) {
            return new SettledDecision(value);
        }
        public Object value() {
            return Values.freeze(json());
        }
    }
    public sealed interface SettledJudgment permits SettledJudgmentBoolean,SettledJudgmentNull,
    SettledJudgmentString,SettledJudgmentNumber,SettledJudgmentArray {
        static SettledJudgment read(Object value) {
            if (value instanceof Boolean) return SettledJudgmentBoolean.read(value);
            if (value == null) return new SettledJudgmentNull(null);
            if (value instanceof String) return SettledJudgmentString.read(value);
            if (value instanceof BigDecimal) return SettledJudgmentNumber.read(value);
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> union_item_0 instanceof String))) return SettledJudgmentArray.read(value);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record SettledJudgmentBoolean(Boolean value) implements SettledJudgment {
        static SettledJudgmentBoolean read(Object value) {
            return new SettledJudgmentBoolean((Boolean)value);
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record SettledJudgmentNull(Object value) implements SettledJudgment {
        static SettledJudgmentNull read(Object value) {
            return new SettledJudgmentNull(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record SettledJudgmentString(String value) implements SettledJudgment {
        static SettledJudgmentString read(Object value) {
            return new SettledJudgmentString((String)value);
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record SettledJudgmentNumber(BigDecimal value) implements SettledJudgment {
        static SettledJudgmentNumber read(Object value) {
            return new SettledJudgmentNumber((BigDecimal)value);
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record SettledJudgmentArray(List<String> value) implements SettledJudgment {
        static SettledJudgmentArray read(Object value) {
            return new SettledJudgmentArray(Values.list(value, item0 -> (String)item0));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public static final class SettledOriginal extends View {
        public SettledOriginal(Object value) {
            super(value);
        }
        static SettledOriginal read(Object value) {
            return new SettledOriginal(value);
        }
        public Object value() {
            return Values.freeze(json());
        }
    }
    public sealed interface SettledRecognition permits SettledRecognitionEntities,SettledRecognitionModeProposals {
        static SettledRecognition read(Object value) {
            Map<String,Object> object = Values.object(value);
            if (object.containsKey("entities") && !object.containsKey("mode") && !object.containsKey("proposals")) return SettledRecognitionEntities.read(value);
            if (object.containsKey("mode") && object.containsKey("proposals") && !object.containsKey("entities")) return SettledRecognitionModeProposals.read(value);
            throw new IllegalStateException("Unknown native result alternative");
        }
    }
    public static final class SettledRecognitionEntities extends View implements SettledRecognition {
        public SettledRecognitionEntities(Object value) {
            super(value);
        }
        static SettledRecognitionEntities read(Object value) {
            return new SettledRecognitionEntities(value);
        }
        public List<Entity> entities() {
            Object value = required("entities");
            return Values.list(value, item0 -> Entity.read(item0));
        }
        public Presence<List<EntityEdge>> relations() {
            return presence("relations", value -> Values.list(value, item0 -> EntityEdge.read(item0)));
        }
    }
    public static final class SettledRecognitionModeProposals extends View implements SettledRecognition {
        public SettledRecognitionModeProposals(Object value) {
            super(value);
        }
        static SettledRecognitionModeProposals read(Object value) {
            return new SettledRecognitionModeProposals(value);
        }
        public BoundaryMode mode() {
            Object value = required("mode");
            return BoundaryMode.read(value);
        }
        public List<BoundaryProposal> proposals() {
            Object value = required("proposals");
            return Values.list(value, item0 -> BoundaryProposal.read(item0));
        }
    }
    public static final class SettledRelations extends View {
        public SettledRelations(Object value) {
            super(value);
        }
        static SettledRelations read(Object value) {
            return new SettledRelations(value);
        }
        public List<RelatedEntityEdge> value() {
            return Values.list(json(), item0 -> RelatedEntityEdge.read(item0));
        }
    }
    public static final class StopCause extends View {
        public StopCause(Object value) {
            super(value);
        }
        static StopCause read(Object value) {
            return new StopCause(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public static final class Stopped extends View {
        public Stopped(Object value) {
            super(value);
        }
        static Stopped read(Object value) {
            return new Stopped(value);
        }
        public Presence<BigInteger> at() {
            return presence("at", value -> Values.integer(value));
        }
        public StopCause cause() {
            Object value = required("cause");
            return StopCause.read(value);
        }
        public Boolean retryable() {
            Object value = required("retryable");
            return (Boolean)value;
        }
        public Presence<BigInteger> status() {
            return presence("status", value -> Values.integer(value));
        }
    }
    public static final class StringRoot extends View {
        public StringRoot(Object value) {
            super(value);
        }
        static StringRoot read(Object value) {
            return new StringRoot(value);
        }
        public StringType type() {
            Object value = required("type");
            return StringType.read(value);
        }
    }
    public static final class StringType extends View {
        public StringType(Object value) {
            super(value);
        }
        static StringType read(Object value) {
            return new StringType(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public static final class Usage extends View {
        public Usage(Object value) {
            super(value);
        }
        static Usage read(Object value) {
            return new Usage(value);
        }
        public Presence<BigInteger> inputTokens() {
            return presence("input_tokens", value -> Values.integer(value));
        }
        public Presence<BigInteger> outputTokens() {
            return presence("output_tokens", value -> Values.integer(value));
        }
    }
    public static final class UsagePersistence extends View {
        public UsagePersistence(Object value) {
            super(value);
        }
        static UsagePersistence read(Object value) {
            return new UsagePersistence(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public static final class Verb extends View {
        public Verb(Object value) {
            super(value);
        }
        static Verb read(Object value) {
            return new Verb(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public static final class Version extends View {
        public Version(Object value) {
            super(value);
        }
        static Version read(Object value) {
            return new Version(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public static final class WordingVersion extends View {
        public WordingVersion(Object value) {
            super(value);
        }
        static WordingVersion read(Object value) {
            return new WordingVersion(value);
        }
        public BigInteger value() {
            return Values.integer(json());
        }
    }
    public sealed interface AnnotatedField permits AnnotatedFieldBoolean,AnnotatedFieldNull,AnnotatedFieldString,
    AnnotatedFieldArray,AnnotatedFieldNumber,AnnotatedFieldObject {
        static AnnotatedField read(Object value) {
            if (value instanceof Boolean) return AnnotatedFieldBoolean.read(value);
            if (value == null) return new AnnotatedFieldNull(null);
            if (value instanceof String) return AnnotatedFieldString.read(value);
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> union_item_0 instanceof String))) return AnnotatedFieldArray.read(value);
            if (value instanceof BigDecimal) return AnnotatedFieldNumber.read(value);
            if (value instanceof Map<?,?>) return AnnotatedFieldObject.read(value);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record AnnotatedFieldBoolean(Boolean value) implements AnnotatedField {
        static AnnotatedFieldBoolean read(Object value) {
            return new AnnotatedFieldBoolean((Boolean)value);
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnnotatedFieldNull(Object value) implements AnnotatedField {
        static AnnotatedFieldNull read(Object value) {
            return new AnnotatedFieldNull(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnnotatedFieldString(String value) implements AnnotatedField {
        static AnnotatedFieldString read(Object value) {
            return new AnnotatedFieldString((String)value);
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnnotatedFieldArray(List<String> value) implements AnnotatedField {
        static AnnotatedFieldArray read(Object value) {
            return new AnnotatedFieldArray(Values.list(value, item0 -> (String)item0));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnnotatedFieldNumber(BigDecimal value) implements AnnotatedField {
        static AnnotatedFieldNumber read(Object value) {
            return new AnnotatedFieldNumber((BigDecimal)value);
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record AnnotatedFieldObject(Failed value) implements AnnotatedField {
        static AnnotatedFieldObject read(Object value) {
            return new AnnotatedFieldObject(Failed.read(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public static final class AnnotatedRow extends View {
        public AnnotatedRow(Object value) {
            super(value);
        }
        static AnnotatedRow read(Object value) {
            return new AnnotatedRow(value);
        }
        public Map<String,AnnotatedField> value() {
            return Values.map(json(), item0 -> AnnotatedField.read(item0));
        }
    }
    public sealed interface Answer permits AnswerYesNo,AnswerChoice,AnswerTag,AnswerScore {
        static Answer read(Object value) {
            Map<String,Object> object = Values.object(value);
            if ("yes_no".equals(object.get("kind"))) return AnswerYesNo.read(value);
            if ("choice".equals(object.get("kind"))) return AnswerChoice.read(value);
            if ("tag".equals(object.get("kind"))) return AnswerTag.read(value);
            if ("score".equals(object.get("kind"))) return AnswerScore.read(value);
            throw new IllegalStateException("Unknown native result alternative");
        }
    }
    public static final class AnswerChoice extends View implements Answer {
        public AnswerChoice(Object value) {
            super(value);
        }
        static AnswerChoice read(Object value) {
            return new AnswerChoice(value);
        }
        public Presence<BigDecimal> confidence() {
            return presence("confidence", value -> (BigDecimal)value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public String pick() {
            Object value = required("pick");
            return (String)value;
        }
        public Map<String,BigDecimal> probabilities() {
            Object value = required("probabilities");
            return Values.map(value, item0 -> (BigDecimal)item0);
        }
    }
    public static final class AnswerScore extends View implements Answer {
        public AnswerScore(Object value) {
            super(value);
        }
        static AnswerScore read(Object value) {
            return new AnswerScore(value);
        }
        public Presence<BigDecimal> confidence() {
            return presence("confidence", value -> (BigDecimal)value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public String level() {
            Object value = required("level");
            return (String)value;
        }
        public Map<String,BigDecimal> probabilities() {
            Object value = required("probabilities");
            return Values.map(value, item0 -> (BigDecimal)item0);
        }
    }
    public static final class AnswerTag extends View implements Answer {
        public AnswerTag(Object value) {
            super(value);
        }
        static AnswerTag read(Object value) {
            return new AnswerTag(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public Map<String,BigDecimal> probabilities() {
            Object value = required("probabilities");
            return Values.map(value, item0 -> (BigDecimal)item0);
        }
    }
    public static final class AnswerYesNo extends View implements Answer {
        public AnswerYesNo(Object value) {
            super(value);
        }
        static AnswerYesNo read(Object value) {
            return new AnswerYesNo(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public BigDecimal probability() {
            Object value = required("probability");
            return (BigDecimal)value;
        }
    }
    public static final class AttemptOutcome extends View {
        public AttemptOutcome(Object value) {
            super(value);
        }
        static AttemptOutcome read(Object value) {
            return new AttemptOutcome(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public sealed interface BatchSetting permits BatchSettingInteger,BatchSettingString {
        static BatchSetting read(Object value) {
            if (value instanceof BigDecimal) return BatchSettingInteger.read(value);
            if (value instanceof String) return BatchSettingString.read(value);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record BatchSettingInteger(BigInteger value) implements BatchSetting {
        static BatchSettingInteger read(Object value) {
            return new BatchSettingInteger(Values.integer(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record BatchSettingString(String value) implements BatchSetting {
        static BatchSettingString read(Object value) {
            return new BatchSettingString((String)value);
        }
        Object json() {
            return Values.json(value);
        }
    }
    public static final class BatchWarning extends View {
        public BatchWarning(Object value) {
            super(value);
        }
        static BatchWarning read(Object value) {
            return new BatchWarning(value);
        }
        public BatchSetting running() {
            Object value = required("running");
            return BatchSetting.read(value);
        }
        public BatchSetting tunedFor() {
            Object value = required("tuned_for");
            return BatchSetting.read(value);
        }
    }
    public static final class Entity extends View {
        public Entity(Object value) {
            super(value);
        }
        static Entity read(Object value) {
            return new Entity(value);
        }
        public BigInteger end() {
            Object value = required("end");
            return Values.integer(value);
        }
        public Presence<String> file() {
            return presence("file", value -> (String)value);
        }
        public Presence<BigInteger> firstLine() {
            return presence("first_line", value -> Values.integer(value));
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public Presence<BigInteger> lastLine() {
            return presence("last_line", value -> Values.integer(value));
        }
        public BigInteger length() {
            Object value = required("length");
            return Values.integer(value);
        }
        public BigInteger start() {
            Object value = required("start");
            return Values.integer(value);
        }
        public BigDecimal strength() {
            Object value = required("strength");
            return (BigDecimal)value;
        }
        public String text() {
            Object value = required("text");
            return (String)value;
        }
    }
    public static final class EntityEdge extends View {
        public EntityEdge(Object value) {
            super(value);
        }
        static EntityEdge read(Object value) {
            return new EntityEdge(value);
        }
        public Presence<Boolean> either() {
            return presence("either", value -> (Boolean)value);
        }
        public BigDecimal probability() {
            Object value = required("probability");
            return (BigDecimal)value;
        }
        public String relation() {
            Object value = required("relation");
            return (String)value;
        }
        public Entity source() {
            Object value = required("source");
            return Entity.read(value);
        }
        public Entity target() {
            Object value = required("target");
            return Entity.read(value);
        }
    }
    public static final class Failed extends View {
        public Failed(Object value) {
            super(value);
        }
        static Failed read(Object value) {
            return new Failed(value);
        }
        public Failure failed() {
            Object value = required("failed");
            return Failure.read(value);
        }
    }
    public static final class Failure extends View {
        public Failure(Object value) {
            super(value);
        }
        static Failure read(Object value) {
            return new Failure(value);
        }
        public FailureCause cause() {
            Object value = required("cause");
            return FailureCause.read(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
    }
    public static final class FailureCause extends View {
        public FailureCause(Object value) {
            super(value);
        }
        static FailureCause read(Object value) {
            return new FailureCause(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public static final class FailureKind extends View {
        public FailureKind(Object value) {
            super(value);
        }
        static FailureKind read(Object value) {
            return new FailureKind(value);
        }
        public String value() {
            return (String)json();
        }
    }
    public static final class FindAnswer extends View {
        public FindAnswer(Object value) {
            super(value);
        }
        static FindAnswer read(Object value) {
            return new FindAnswer(value);
        }
        public Presence<BigDecimal> confidence() {
            return presence("confidence", value -> (BigDecimal)value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public String pick() {
            Object value = required("pick");
            return (String)value;
        }
        public Map<String,BigDecimal> probabilities() {
            Object value = required("probabilities");
            return Values.map(value, item0 -> (BigDecimal)item0);
        }
    }
    public static final class FindQuestionDocument extends View {
        public FindQuestionDocument(Object value) {
            super(value);
        }
        static FindQuestionDocument read(Object value) {
            return new FindQuestionDocument(value);
        }
        public Presence<Batch> batch() {
            return presence("batch", value -> Batch.read(value));
        }
        public Presence<InputDeclaration> contextSchema() {
            return presence("context_schema", value -> InputDeclaration.read(value));
        }
        public Presence<InputDeclaration> itemSchema() {
            return presence("item_schema", value -> InputDeclaration.read(value));
        }
        public Presence<List<Label>> labelDetails() {
            return presence("label_details", value -> Values.list(value, item0 -> Label.read(item0)));
        }
        public Presence<String> model() {
            return presence("model", value -> (String)value);
        }
        public Presence<QuestionName> name() {
            return presence("name", value -> QuestionName.read(value));
        }
        public Boolean none() {
            Object value = required("none");
            return (Boolean)value;
        }
        public Presence<List<String>> on() {
            return presence("on", value -> Values.list(value, item0 -> (String)item0));
        }
        public Presence<String> profile() {
            return presence("profile", value -> (String)value);
        }
        public Object text() {
            Object value = required("text");
            return Values.freeze(value);
        }
        public String verb() {
            Object value = required("verb");
            return (String)value;
        }
        public Presence<WordingVersion> wordingVersion() {
            return presence("wording_version", value -> WordingVersion.read(value));
        }
    }
    public static final class NameOdds extends View {
        public NameOdds(Object value) {
            super(value);
        }
        static NameOdds read(Object value) {
            return new NameOdds(value);
        }
        public Presence<Map<String,BigDecimal>> edges() {
            required("edges");
            return presence("edges", value -> Values.map(value, item0 -> (BigDecimal)item0));
        }
        public BigInteger end() {
            Object value = required("end");
            return Values.integer(value);
        }
        public Presence<Map<String,BigDecimal>> kinds() {
            required("kinds");
            return presence("kinds", value -> Values.map(value, item0 -> (BigDecimal)item0));
        }
        public BigInteger start() {
            Object value = required("start");
            return Values.integer(value);
        }
    }
    public static final class PairOdds extends View {
        public PairOdds(Object value) {
            super(value);
        }
        static PairOdds read(Object value) {
            return new PairOdds(value);
        }
        public BigDecimal probability() {
            Object value = required("probability");
            return (BigDecimal)value;
        }
        public String relation() {
            Object value = required("relation");
            return (String)value;
        }
        public Place source() {
            Object value = required("source");
            return Place.read(value);
        }
        public Place target() {
            Object value = required("target");
            return Place.read(value);
        }
    }
    public static final class PieceOdds extends View {
        public PieceOdds(Object value) {
            super(value);
        }
        static PieceOdds read(Object value) {
            return new PieceOdds(value);
        }
        public BigInteger end() {
            Object value = required("end");
            return Values.integer(value);
        }
        public BigInteger start() {
            Object value = required("start");
            return Values.integer(value);
        }
        public Map<String,BigDecimal> tags() {
            Object value = required("tags");
            return Values.map(value, item0 -> (BigDecimal)item0);
        }
    }
    public static final class Place extends View {
        public Place(Object value) {
            super(value);
        }
        static Place read(Object value) {
            return new Place(value);
        }
        public BigInteger end() {
            Object value = required("end");
            return Values.integer(value);
        }
        public BigInteger start() {
            Object value = required("start");
            return Values.integer(value);
        }
    }
    public static final class Plan extends View {
        public Plan(Object value) {
            super(value);
        }
        static Plan read(Object value) {
            return new Plan(value);
        }
        public BigInteger estimatedBytes() {
            Object value = required("estimated_bytes");
            return Values.integer(value);
        }
        public TokenBand estimatedInputTokens() {
            Object value = required("estimated_input_tokens");
            return TokenBand.read(value);
        }
        public Presence<String> firstBodyUtf8() {
            required("first_body_utf8");
            return presence("first_body_utf8", value -> (String)value);
        }
        public BigInteger largestRequestBytes() {
            Object value = required("largest_request_bytes");
            return Values.integer(value);
        }
        public BigInteger largestRequestEstimatedInputTokens() {
            Object value = required("largest_request_estimated_input_tokens");
            return Values.integer(value);
        }
        public BigInteger records() {
            Object value = required("records");
            return Values.integer(value);
        }
        public BigInteger requests() {
            Object value = required("requests");
            return Values.integer(value);
        }
        public String tokenEstimateMethod() {
            Object value = required("token_estimate_method");
            return (String)value;
        }
        public Boolean upperBound() {
            Object value = required("upper_bound");
            return (Boolean)value;
        }
    }
    public static final class ProfileWarning extends View {
        public ProfileWarning(Object value) {
            super(value);
        }
        static ProfileWarning read(Object value) {
            return new ProfileWarning(value);
        }
        public String running() {
            Object value = required("running");
            return (String)value;
        }
        public String tunedFor() {
            Object value = required("tuned_for");
            return (String)value;
        }
    }
    public sealed interface QuestionDocument permits QuestionDocumentDecide,QuestionDocumentChoose,
    QuestionDocumentTag,QuestionDocumentScore {
        static QuestionDocument read(Object value) {
            Map<String,Object> object = Values.object(value);
            if ("decide".equals(object.get("verb"))) return QuestionDocumentDecide.read(value);
            if ("choose".equals(object.get("verb"))) return QuestionDocumentChoose.read(value);
            if ("tag".equals(object.get("verb"))) return QuestionDocumentTag.read(value);
            if ("score".equals(object.get("verb"))) return QuestionDocumentScore.read(value);
            throw new IllegalStateException("Unknown native result alternative");
        }
    }
    public static final class QuestionDocumentChoose extends View implements QuestionDocument {
        public QuestionDocumentChoose(Object value) {
            super(value);
        }
        static QuestionDocumentChoose read(Object value) {
            return new QuestionDocumentChoose(value);
        }
        public Presence<Batch> batch() {
            return presence("batch", value -> Batch.read(value));
        }
        public Presence<InputDeclaration> contextSchema() {
            return presence("context_schema", value -> InputDeclaration.read(value));
        }
        public Presence<InputDeclaration> itemSchema() {
            return presence("item_schema", value -> InputDeclaration.read(value));
        }
        public Presence<List<Label>> labelDetails() {
            return presence("label_details", value -> Values.list(value, item0 -> Label.read(item0)));
        }
        public Presence<String> model() {
            return presence("model", value -> (String)value);
        }
        public Presence<QuestionName> name() {
            return presence("name", value -> QuestionName.read(value));
        }
        public Presence<List<String>> on() {
            return presence("on", value -> Values.list(value, item0 -> (String)item0));
        }
        public Presence<String> profile() {
            return presence("profile", value -> (String)value);
        }
        public Presence<WordingVersion> wordingVersion() {
            return presence("wording_version", value -> WordingVersion.read(value));
        }
        public List<String> options() {
            Object value = required("options");
            return Values.list(value, item0 -> (String)item0);
        }
        public Object text() {
            Object value = required("text");
            return Values.freeze(value);
        }
        public String verb() {
            Object value = required("verb");
            return (String)value;
        }
    }
    public static final class QuestionDocumentDecide extends View implements QuestionDocument {
        public QuestionDocumentDecide(Object value) {
            super(value);
        }
        static QuestionDocumentDecide read(Object value) {
            return new QuestionDocumentDecide(value);
        }
        public Presence<Batch> batch() {
            return presence("batch", value -> Batch.read(value));
        }
        public Presence<InputDeclaration> contextSchema() {
            return presence("context_schema", value -> InputDeclaration.read(value));
        }
        public Presence<InputDeclaration> itemSchema() {
            return presence("item_schema", value -> InputDeclaration.read(value));
        }
        public Presence<List<Label>> labelDetails() {
            return presence("label_details", value -> Values.list(value, item0 -> Label.read(item0)));
        }
        public Presence<String> model() {
            return presence("model", value -> (String)value);
        }
        public Presence<QuestionName> name() {
            return presence("name", value -> QuestionName.read(value));
        }
        public Presence<List<String>> on() {
            return presence("on", value -> Values.list(value, item0 -> (String)item0));
        }
        public Presence<String> profile() {
            return presence("profile", value -> (String)value);
        }
        public Presence<WordingVersion> wordingVersion() {
            return presence("wording_version", value -> WordingVersion.read(value));
        }
        public Presence<Object> falseValue() {
            return presence("false", value -> Values.freeze(value));
        }
        public Object text() {
            Object value = required("text");
            return Values.freeze(value);
        }
        public Presence<Object> trueValue() {
            return presence("true", value -> Values.freeze(value));
        }
        public String verb() {
            Object value = required("verb");
            return (String)value;
        }
    }
    public static final class QuestionDocumentScore extends View implements QuestionDocument {
        public QuestionDocumentScore(Object value) {
            super(value);
        }
        static QuestionDocumentScore read(Object value) {
            return new QuestionDocumentScore(value);
        }
        public Presence<Batch> batch() {
            return presence("batch", value -> Batch.read(value));
        }
        public Presence<InputDeclaration> contextSchema() {
            return presence("context_schema", value -> InputDeclaration.read(value));
        }
        public Presence<InputDeclaration> itemSchema() {
            return presence("item_schema", value -> InputDeclaration.read(value));
        }
        public Presence<List<Label>> labelDetails() {
            return presence("label_details", value -> Values.list(value, item0 -> Label.read(item0)));
        }
        public Presence<String> model() {
            return presence("model", value -> (String)value);
        }
        public Presence<QuestionName> name() {
            return presence("name", value -> QuestionName.read(value));
        }
        public Presence<List<String>> on() {
            return presence("on", value -> Values.list(value, item0 -> (String)item0));
        }
        public Presence<String> profile() {
            return presence("profile", value -> (String)value);
        }
        public Presence<WordingVersion> wordingVersion() {
            return presence("wording_version", value -> WordingVersion.read(value));
        }
        public List<String> levels() {
            Object value = required("levels");
            return Values.list(value, item0 -> (String)item0);
        }
        public Object text() {
            Object value = required("text");
            return Values.freeze(value);
        }
        public String verb() {
            Object value = required("verb");
            return (String)value;
        }
    }
    public static final class QuestionDocumentTag extends View implements QuestionDocument {
        public QuestionDocumentTag(Object value) {
            super(value);
        }
        static QuestionDocumentTag read(Object value) {
            return new QuestionDocumentTag(value);
        }
        public Presence<Batch> batch() {
            return presence("batch", value -> Batch.read(value));
        }
        public Presence<InputDeclaration> contextSchema() {
            return presence("context_schema", value -> InputDeclaration.read(value));
        }
        public Presence<InputDeclaration> itemSchema() {
            return presence("item_schema", value -> InputDeclaration.read(value));
        }
        public Presence<List<Label>> labelDetails() {
            return presence("label_details", value -> Values.list(value, item0 -> Label.read(item0)));
        }
        public Presence<String> model() {
            return presence("model", value -> (String)value);
        }
        public Presence<QuestionName> name() {
            return presence("name", value -> QuestionName.read(value));
        }
        public Presence<List<String>> on() {
            return presence("on", value -> Values.list(value, item0 -> (String)item0));
        }
        public Presence<String> profile() {
            return presence("profile", value -> (String)value);
        }
        public Presence<WordingVersion> wordingVersion() {
            return presence("wording_version", value -> WordingVersion.read(value));
        }
        public List<String> labels() {
            Object value = required("labels");
            return Values.list(value, item0 -> (String)item0);
        }
        public Object text() {
            Object value = required("text");
            return Values.freeze(value);
        }
        public String verb() {
            Object value = required("verb");
            return (String)value;
        }
    }
    public sealed interface Recognize permits RecognizeEntities,RecognizeModeProposals {
        static Recognize read(Object value) {
            Map<String,Object> object = Values.object(value);
            if (object.containsKey("entities") && !object.containsKey("mode") && !object.containsKey("proposals")) return RecognizeEntities.read(value);
            if (object.containsKey("mode") && object.containsKey("proposals") && !object.containsKey("entities")) return RecognizeModeProposals.read(value);
            throw new IllegalStateException("Unknown native result alternative");
        }
    }
    public static final class RecognizeAnswer extends View {
        public RecognizeAnswer(Object value) {
            super(value);
        }
        static RecognizeAnswer read(Object value) {
            return new RecognizeAnswer(value);
        }
        public List<NameOdds> names() {
            Object value = required("names");
            return Values.list(value, item0 -> NameOdds.read(item0));
        }
        public List<PairOdds> pairs() {
            Object value = required("pairs");
            return Values.list(value, item0 -> PairOdds.read(item0));
        }
        public List<PieceOdds> pieces() {
            Object value = required("pieces");
            return Values.list(value, item0 -> PieceOdds.read(item0));
        }
        public List<RecognitionProposal> proposals() {
            Object value = required("proposals");
            return Values.list(value, item0 -> RecognitionProposal.read(item0));
        }
    }
    public static final class RecognizeQuestionDocument extends View {
        public RecognizeQuestionDocument(Object value) {
            super(value);
        }
        static RecognizeQuestionDocument read(Object value) {
            return new RecognizeQuestionDocument(value);
        }
        public Presence<Batch> batch() {
            return presence("batch", value -> Batch.read(value));
        }
        public Presence<InputDeclaration> contextSchema() {
            return presence("context_schema", value -> InputDeclaration.read(value));
        }
        public Presence<Object> entityDefinition() {
            return presence("entity_definition", value -> Values.freeze(value));
        }
        public Presence<Object> instructions() {
            return presence("instructions", value -> Values.freeze(value));
        }
        public Presence<InputDeclaration> itemSchema() {
            return presence("item_schema", value -> InputDeclaration.read(value));
        }
        public Map<String,Object> kinds() {
            Object value = required("kinds");
            return Values.map(value, item0 -> Values.freeze(item0));
        }
        public Presence<List<Label>> labelDetails() {
            return presence("label_details", value -> Values.list(value, item0 -> Label.read(item0)));
        }
        public Presence<RecognitionMode> mode() {
            return presence("mode", value -> RecognitionMode.read(value));
        }
        public Presence<String> model() {
            return presence("model", value -> (String)value);
        }
        public Presence<QuestionName> name() {
            return presence("name", value -> QuestionName.read(value));
        }
        public Presence<List<String>> on() {
            return presence("on", value -> Values.list(value, item0 -> (String)item0));
        }
        public Presence<String> profile() {
            return presence("profile", value -> (String)value);
        }
        public Presence<Threshold> relationThreshold() {
            return presence("relation_threshold", value -> Threshold.read(value));
        }
        public Presence<List<RelationRule>> relations() {
            return presence("relations", value -> Values.list(value, item0 -> RelationRule.read(item0)));
        }
        public Presence<BigInteger> snippetPieces() {
            return presence("snippet_pieces", value -> Values.integer(value));
        }
        public Presence<RecognitionStageContext> stageContext() {
            return presence("stage_context", value -> RecognitionStageContext.read(value));
        }
        public Threshold threshold() {
            Object value = required("threshold");
            return Threshold.read(value);
        }
        public Verb verb() {
            Object value = required("verb");
            return Verb.read(value);
        }
        public Presence<WordingVersion> wordingVersion() {
            return presence("wording_version", value -> WordingVersion.read(value));
        }
    }
    public static final class RecognizeEntities extends View implements Recognize {
        public RecognizeEntities(Object value) {
            super(value);
        }
        static RecognizeEntities read(Object value) {
            return new RecognizeEntities(value);
        }
        public List<Entity> entities() {
            Object value = required("entities");
            return Values.list(value, item0 -> Entity.read(item0));
        }
        public Presence<List<EntityEdge>> relations() {
            return presence("relations", value -> Values.list(value, item0 -> EntityEdge.read(item0)));
        }
    }
    public static final class RecognizeModeProposals extends View implements Recognize {
        public RecognizeModeProposals(Object value) {
            super(value);
        }
        static RecognizeModeProposals read(Object value) {
            return new RecognizeModeProposals(value);
        }
        public BoundaryMode mode() {
            Object value = required("mode");
            return BoundaryMode.read(value);
        }
        public List<BoundaryProposal> proposals() {
            Object value = required("proposals");
            return Values.list(value, item0 -> BoundaryProposal.read(item0));
        }
    }
    public static final class RelateFields extends View {
        public RelateFields(Object value) {
            super(value);
        }
        static RelateFields read(Object value) {
            return new RelateFields(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public String name() {
            Object value = required("name");
            return (String)value;
        }
    }
    public static final class RelateQuestionDocument extends View {
        public RelateQuestionDocument(Object value) {
            super(value);
        }
        static RelateQuestionDocument read(Object value) {
            return new RelateQuestionDocument(value);
        }
        public Presence<Batch> batch() {
            return presence("batch", value -> Batch.read(value));
        }
        public Presence<InputDeclaration> contextSchema() {
            return presence("context_schema", value -> InputDeclaration.read(value));
        }
        public Presence<RelateFields> fields() {
            required("fields");
            return presence("fields", value -> RelateFields.read(value));
        }
        public Presence<InputDeclaration> itemSchema() {
            return presence("item_schema", value -> InputDeclaration.read(value));
        }
        public Presence<List<Label>> labelDetails() {
            return presence("label_details", value -> Values.list(value, item0 -> Label.read(item0)));
        }
        public Presence<String> model() {
            return presence("model", value -> (String)value);
        }
        public Presence<QuestionName> name() {
            return presence("name", value -> QuestionName.read(value));
        }
        public Presence<List<String>> on() {
            return presence("on", value -> Values.list(value, item0 -> (String)item0));
        }
        public Presence<String> profile() {
            return presence("profile", value -> (String)value);
        }
        public List<RelationRule> relations() {
            Object value = required("relations");
            return Values.list(value, item0 -> RelationRule.read(item0));
        }
        public Threshold threshold() {
            Object value = required("threshold");
            return Threshold.read(value);
        }
        public String verb() {
            Object value = required("verb");
            return (String)value;
        }
        public Presence<WordingVersion> wordingVersion() {
            return presence("wording_version", value -> WordingVersion.read(value));
        }
    }
    public static final class RelatedEntity extends View {
        public RelatedEntity(Object value) {
            super(value);
        }
        static RelatedEntity read(Object value) {
            return new RelatedEntity(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public String name() {
            Object value = required("name");
            return (String)value;
        }
    }
    public static final class RelatedEntityEdge extends View {
        public RelatedEntityEdge(Object value) {
            super(value);
        }
        static RelatedEntityEdge read(Object value) {
            return new RelatedEntityEdge(value);
        }
        public Presence<Boolean> either() {
            return presence("either", value -> (Boolean)value);
        }
        public BigDecimal probability() {
            Object value = required("probability");
            return (BigDecimal)value;
        }
        public String relation() {
            Object value = required("relation");
            return (String)value;
        }
        public RelationEndpoint source() {
            Object value = required("source");
            return RelationEndpoint.read(value);
        }
        public RelationEndpoint target() {
            Object value = required("target");
            return RelationEndpoint.read(value);
        }
        public String sourceName() {
            return (String)Values.object(required("source")).get("name");
        }
        public String targetName() {
            return (String)Values.object(required("target")).get("name");
        }
    }
    public static final class RelationRule extends View {
        public RelationRule(Object value) {
            super(value);
        }
        static RelationRule read(Object value) {
            return new RelationRule(value);
        }
        public Boolean either() {
            Object value = required("either");
            return (Boolean)value;
        }
        public String name() {
            Object value = required("name");
            return (String)value;
        }
        public String reads() {
            Object value = required("reads");
            return (String)value;
        }
        public Presence<Boolean> single() {
            return presence("single", value -> (Boolean)value);
        }
        public String source() {
            Object value = required("source");
            return (String)value;
        }
        public String target() {
            Object value = required("target");
            return (String)value;
        }
    }
    public static final class SessionAnnotation extends View {
        public SessionAnnotation(Object value) {
            super(value);
        }
        static SessionAnnotation read(Object value) {
            return new SessionAnnotation(value);
        }
        public String name() {
            Object value = required("name");
            return (String)value;
        }
        public AnnotationValue value() {
            Object value = required("value");
            return AnnotationValue.read(value);
        }
    }
    public static final class SessionInputSource extends View {
        public SessionInputSource(Object value) {
            super(value);
        }
        static SessionInputSource read(Object value) {
            return new SessionInputSource(value);
        }
        public BigInteger index() {
            Object value = required("index");
            return Values.integer(value);
        }
        public PhysicalSource source() {
            Object value = required("source");
            return PhysicalSource.read(value);
        }
    }
    public sealed interface SessionJudgment permits SessionJudgmentDecision,SessionJudgmentChoice,
    SessionJudgmentScore,SessionJudgmentTags {
        static SessionJudgment read(Object value) {
            Map<String,Object> object = Values.object(value);
            if ("decision".equals(object.get("kind"))) return SessionJudgmentDecision.read(value);
            if ("choice".equals(object.get("kind"))) return SessionJudgmentChoice.read(value);
            if ("score".equals(object.get("kind"))) return SessionJudgmentScore.read(value);
            if ("tags".equals(object.get("kind"))) return SessionJudgmentTags.read(value);
            throw new IllegalStateException("Unknown native result alternative");
        }
    }
    public static final class SessionJudgmentChoice extends View implements SessionJudgment {
        public SessionJudgmentChoice(Object value) {
            super(value);
        }
        static SessionJudgmentChoice read(Object value) {
            return new SessionJudgmentChoice(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public Presence<String> value() {
            required("value");
            return presence("value", value -> (String)value);
        }
    }
    public static final class SessionJudgmentDecision extends View implements SessionJudgment {
        public SessionJudgmentDecision(Object value) {
            super(value);
        }
        static SessionJudgmentDecision read(Object value) {
            return new SessionJudgmentDecision(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public Presence<Boolean> value() {
            required("value");
            return presence("value", value -> (Boolean)value);
        }
    }
    public static final class SessionJudgmentScore extends View implements SessionJudgment {
        public SessionJudgmentScore(Object value) {
            super(value);
        }
        static SessionJudgmentScore read(Object value) {
            return new SessionJudgmentScore(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public BigDecimal value() {
            Object value = required("value");
            return (BigDecimal)value;
        }
    }
    public static final class SessionJudgmentTags extends View implements SessionJudgment {
        public SessionJudgmentTags(Object value) {
            super(value);
        }
        static SessionJudgmentTags read(Object value) {
            return new SessionJudgmentTags(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public List<String> value() {
            Object value = required("value");
            return Values.list(value, item0 -> (String)item0);
        }
    }
    public static final class SessionNamedProbability extends View {
        public SessionNamedProbability(Object value) {
            super(value);
        }
        static SessionNamedProbability read(Object value) {
            return new SessionNamedProbability(value);
        }
        public String name() {
            Object value = required("name");
            return (String)value;
        }
        public BigDecimal probability() {
            Object value = required("probability");
            return (BigDecimal)value;
        }
    }
    public sealed interface SessionObservation permits SessionObservationQuestion,SessionObservationRow {
        static SessionObservation read(Object value) {
            Map<String,Object> object = Values.object(value);
            if ("question".equals(object.get("kind"))) return SessionObservationQuestion.read(value);
            if ("row".equals(object.get("kind"))) return SessionObservationRow.read(value);
            throw new IllegalStateException("Unknown native result alternative");
        }
    }
    public static final class SessionObservationQuestion extends View implements SessionObservation {
        public SessionObservationQuestion(Object value) {
            super(value);
        }
        static SessionObservationQuestion read(Object value) {
            return new SessionObservationQuestion(value);
        }
        public SessionQuestionDetail detail() {
            Object value = required("detail");
            return SessionQuestionDetail.read(value);
        }
        public BigInteger index() {
            Object value = required("index");
            return Values.integer(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public Presence<String> member() {
            return presence("member", value -> (String)value);
        }
        public BigInteger position() {
            Object value = required("position");
            return Values.integer(value);
        }
        public Presence<String> stage() {
            return presence("stage", value -> (String)value);
        }
    }
    public static final class SessionObservationRow extends View implements SessionObservation {
        public SessionObservationRow(Object value) {
            super(value);
        }
        static SessionObservationRow read(Object value) {
            return new SessionObservationRow(value);
        }
        public BigInteger index() {
            Object value = required("index");
            return Values.integer(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public SessionObservedRow value() {
            Object value = required("value");
            return SessionObservedRow.read(value);
        }
    }
    public sealed interface SessionObservedRow permits SessionObservedRowJudgment,SessionObservedRowAnnotated,
    SessionObservedRowRecognized,SessionObservedRowFind,SessionObservedRowRelations {
        static SessionObservedRow read(Object value) {
            Map<String,Object> object = Values.object(value);
            if ("judgment".equals(object.get("kind"))) return SessionObservedRowJudgment.read(value);
            if ("annotated".equals(object.get("kind"))) return SessionObservedRowAnnotated.read(value);
            if ("recognized".equals(object.get("kind"))) return SessionObservedRowRecognized.read(value);
            if ("find".equals(object.get("kind"))) return SessionObservedRowFind.read(value);
            if ("relations".equals(object.get("kind"))) return SessionObservedRowRelations.read(value);
            throw new IllegalStateException("Unknown native result alternative");
        }
    }
    public static final class SessionObservedRowAnnotated extends View implements SessionObservedRow {
        public SessionObservedRowAnnotated(Object value) {
            super(value);
        }
        static SessionObservedRowAnnotated read(Object value) {
            return new SessionObservedRowAnnotated(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public List<SessionAnnotation> value() {
            Object value = required("value");
            return Values.list(value, item0 -> SessionAnnotation.read(item0));
        }
    }
    public static final class SessionObservedRowFind extends View implements SessionObservedRow {
        public SessionObservedRowFind(Object value) {
            super(value);
        }
        static SessionObservedRowFind read(Object value) {
            return new SessionObservedRowFind(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public Presence<BigInteger> value() {
            required("value");
            return presence("value", value -> Values.integer(value));
        }
    }
    public static final class SessionObservedRowJudgment extends View implements SessionObservedRow {
        public SessionObservedRowJudgment(Object value) {
            super(value);
        }
        static SessionObservedRowJudgment read(Object value) {
            return new SessionObservedRowJudgment(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public SessionJudgment value() {
            Object value = required("value");
            return SessionJudgment.read(value);
        }
    }
    public static final class SessionObservedRowRecognized extends View implements SessionObservedRow {
        public SessionObservedRowRecognized(Object value) {
            super(value);
        }
        static SessionObservedRowRecognized read(Object value) {
            return new SessionObservedRowRecognized(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public SessionRecognition value() {
            Object value = required("value");
            return SessionRecognition.read(value);
        }
    }
    public static final class SessionObservedRowRelations extends View implements SessionObservedRow {
        public SessionObservedRowRelations(Object value) {
            super(value);
        }
        static SessionObservedRowRelations read(Object value) {
            return new SessionObservedRowRelations(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public List<SessionRelationEdge> value() {
            Object value = required("value");
            return Values.list(value, item0 -> SessionRelationEdge.read(item0));
        }
    }
    public sealed interface SessionPacket permits SessionPacketDecideRow,SessionPacketChooseRow,
    SessionPacketTagRow,SessionPacketScoreRow,SessionPacketFilterRow,SessionPacketAnnotateRow,SessionPacketDecideAggregate,
    SessionPacketChooseAggregate,SessionPacketTagAggregate,SessionPacketScoreAggregate,SessionPacketFilterAggregate,
    SessionPacketRankAggregate,SessionPacketFindAggregate,SessionPacketAnnotateAggregate,SessionPacketRecognizeAggregate,
    SessionPacketRelateAggregate,SessionPacketObservation,SessionPacketTerminal {
        static SessionPacket read(Object value) {
            Map<String,Object> object = Values.object(value);
            if ("decide".equals(object.get("function")) && "row".equals(object.get("kind"))) return SessionPacketDecideRow.read(value);
            if ("choose".equals(object.get("function")) && "row".equals(object.get("kind"))) return SessionPacketChooseRow.read(value);
            if ("tag".equals(object.get("function")) && "row".equals(object.get("kind"))) return SessionPacketTagRow.read(value);
            if ("score".equals(object.get("function")) && "row".equals(object.get("kind"))) return SessionPacketScoreRow.read(value);
            if ("filter".equals(object.get("function")) && "row".equals(object.get("kind"))) return SessionPacketFilterRow.read(value);
            if ("annotate".equals(object.get("function")) && "row".equals(object.get("kind"))) return SessionPacketAnnotateRow.read(value);
            if ("decide".equals(object.get("function")) && "aggregate".equals(object.get("kind"))) return SessionPacketDecideAggregate.read(value);
            if ("choose".equals(object.get("function")) && "aggregate".equals(object.get("kind"))) return SessionPacketChooseAggregate.read(value);
            if ("tag".equals(object.get("function")) && "aggregate".equals(object.get("kind"))) return SessionPacketTagAggregate.read(value);
            if ("score".equals(object.get("function")) && "aggregate".equals(object.get("kind"))) return SessionPacketScoreAggregate.read(value);
            if ("filter".equals(object.get("function")) && "aggregate".equals(object.get("kind"))) return SessionPacketFilterAggregate.read(value);
            if ("rank".equals(object.get("function")) && "aggregate".equals(object.get("kind"))) return SessionPacketRankAggregate.read(value);
            if ("find".equals(object.get("function")) && "aggregate".equals(object.get("kind"))) return SessionPacketFindAggregate.read(value);
            if ("annotate".equals(object.get("function")) && "aggregate".equals(object.get("kind"))) return SessionPacketAnnotateAggregate.read(value);
            if ("recognize".equals(object.get("function")) && "aggregate".equals(object.get("kind"))) return SessionPacketRecognizeAggregate.read(value);
            if ("relate".equals(object.get("function")) && "aggregate".equals(object.get("kind"))) return SessionPacketRelateAggregate.read(value);
            if ("observation".equals(object.get("kind"))) return SessionPacketObservation.read(value);
            if ("terminal".equals(object.get("kind"))) return SessionPacketTerminal.read(value);
            throw new IllegalStateException("Unknown native result alternative");
        }
    }
    public static final class SessionPacketAnnotateAggregate extends View implements SessionPacket {
        public SessionPacketAnnotateAggregate(Object value) {
            super(value);
        }
        static SessionPacketAnnotateAggregate read(Object value) {
            return new SessionPacketAnnotateAggregate(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public List<Annotation> value() {
            Object value = required("value");
            return Values.list(value, item0 -> Annotation.read(item0));
        }
    }
    public static final class SessionPacketAnnotateRow extends View implements SessionPacket {
        public SessionPacketAnnotateRow(Object value) {
            super(value);
        }
        static SessionPacketAnnotateRow read(Object value) {
            return new SessionPacketAnnotateRow(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public Annotation value() {
            Object value = required("value");
            return Annotation.read(value);
        }
    }
    public static final class SessionPacketChooseAggregate extends View implements SessionPacket {
        public SessionPacketChooseAggregate(Object value) {
            super(value);
        }
        static SessionPacketChooseAggregate read(Object value) {
            return new SessionPacketChooseAggregate(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public List<AtomicNullableString> value() {
            Object value = required("value");
            return Values.list(value, item0 -> AtomicNullableString.read(item0));
        }
    }
    public static final class SessionPacketChooseRow extends View implements SessionPacket {
        public SessionPacketChooseRow(Object value) {
            super(value);
        }
        static SessionPacketChooseRow read(Object value) {
            return new SessionPacketChooseRow(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public AtomicNullableString value() {
            Object value = required("value");
            return AtomicNullableString.read(value);
        }
    }
    public static final class SessionPacketDecideAggregate extends View implements SessionPacket {
        public SessionPacketDecideAggregate(Object value) {
            super(value);
        }
        static SessionPacketDecideAggregate read(Object value) {
            return new SessionPacketDecideAggregate(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public List<AtomicDecideValue> value() {
            Object value = required("value");
            return Values.list(value, item0 -> AtomicDecideValue.read(item0));
        }
    }
    public static final class SessionPacketDecideRow extends View implements SessionPacket {
        public SessionPacketDecideRow(Object value) {
            super(value);
        }
        static SessionPacketDecideRow read(Object value) {
            return new SessionPacketDecideRow(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public AtomicDecideValue value() {
            Object value = required("value");
            return AtomicDecideValue.read(value);
        }
    }
    public static final class SessionPacketFilterAggregate extends View implements SessionPacket {
        public SessionPacketFilterAggregate(Object value) {
            super(value);
        }
        static SessionPacketFilterAggregate read(Object value) {
            return new SessionPacketFilterAggregate(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public List<AtomicBoolean> value() {
            Object value = required("value");
            return Values.list(value, item0 -> AtomicBoolean.read(item0));
        }
    }
    public static final class SessionPacketFilterRow extends View implements SessionPacket {
        public SessionPacketFilterRow(Object value) {
            super(value);
        }
        static SessionPacketFilterRow read(Object value) {
            return new SessionPacketFilterRow(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public AtomicBoolean value() {
            Object value = required("value");
            return AtomicBoolean.read(value);
        }
    }
    public static final class SessionPacketFindAggregate extends View implements SessionPacket {
        public SessionPacketFindAggregate(Object value) {
            super(value);
        }
        static SessionPacketFindAggregate read(Object value) {
            return new SessionPacketFindAggregate(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public Find value() {
            Object value = required("value");
            return Find.read(value);
        }
    }
    public static final class SessionPacketObservation extends View implements SessionPacket {
        public SessionPacketObservation(Object value) {
            super(value);
        }
        static SessionPacketObservation read(Object value) {
            return new SessionPacketObservation(value);
        }
        public RequestFunction function() {
            Object value = required("function");
            return RequestFunction.read(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public SessionObservation value() {
            Object value = required("value");
            return SessionObservation.read(value);
        }
    }
    public static final class SessionPacketRankAggregate extends View implements SessionPacket {
        public SessionPacketRankAggregate(Object value) {
            super(value);
        }
        static SessionPacketRankAggregate read(Object value) {
            return new SessionPacketRankAggregate(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public List<AtomicNonZeroUsize> value() {
            Object value = required("value");
            return Values.list(value, item0 -> AtomicNonZeroUsize.read(item0));
        }
    }
    public static final class SessionPacketRecognizeAggregate extends View implements SessionPacket {
        public SessionPacketRecognizeAggregate(Object value) {
            super(value);
        }
        static SessionPacketRecognizeAggregate read(Object value) {
            return new SessionPacketRecognizeAggregate(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public List<Recognition> value() {
            Object value = required("value");
            return Values.list(value, item0 -> Recognition.read(item0));
        }
    }
    public static final class SessionPacketRelateAggregate extends View implements SessionPacket {
        public SessionPacketRelateAggregate(Object value) {
            super(value);
        }
        static SessionPacketRelateAggregate read(Object value) {
            return new SessionPacketRelateAggregate(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public Relation value() {
            Object value = required("value");
            return Relation.read(value);
        }
    }
    public static final class SessionPacketScoreAggregate extends View implements SessionPacket {
        public SessionPacketScoreAggregate(Object value) {
            super(value);
        }
        static SessionPacketScoreAggregate read(Object value) {
            return new SessionPacketScoreAggregate(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public List<AtomicDouble> value() {
            Object value = required("value");
            return Values.list(value, item0 -> AtomicDouble.read(item0));
        }
    }
    public static final class SessionPacketScoreRow extends View implements SessionPacket {
        public SessionPacketScoreRow(Object value) {
            super(value);
        }
        static SessionPacketScoreRow read(Object value) {
            return new SessionPacketScoreRow(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public AtomicDouble value() {
            Object value = required("value");
            return AtomicDouble.read(value);
        }
    }
    public static final class SessionPacketTagAggregate extends View implements SessionPacket {
        public SessionPacketTagAggregate(Object value) {
            super(value);
        }
        static SessionPacketTagAggregate read(Object value) {
            return new SessionPacketTagAggregate(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public List<AtomicArrayOfString> value() {
            Object value = required("value");
            return Values.list(value, item0 -> AtomicArrayOfString.read(item0));
        }
    }
    public static final class SessionPacketTagRow extends View implements SessionPacket {
        public SessionPacketTagRow(Object value) {
            super(value);
        }
        static SessionPacketTagRow read(Object value) {
            return new SessionPacketTagRow(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public AtomicArrayOfString value() {
            Object value = required("value");
            return AtomicArrayOfString.read(value);
        }
    }
    public static final class SessionPacketTerminal extends View implements SessionPacket {
        public SessionPacketTerminal(Object value) {
            super(value);
        }
        static SessionPacketTerminal read(Object value) {
            return new SessionPacketTerminal(value);
        }
        public Presence<SettledAnswer> answer() {
            required("answer");
            return presence("answer", value -> SettledAnswer.read(value));
        }
        public Presence<Facts> facts() {
            return presence("facts", value -> Facts.read(value));
        }
        public Presence<CallError> failure() {
            return presence("failure", value -> CallError.read(value));
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public AnswerResolution resolution() {
            Object value = required("resolution");
            return AnswerResolution.read(value);
        }
    }
    public sealed interface SessionProbabilities permits SessionProbabilitiesYesNo,SessionProbabilitiesNamed {
        static SessionProbabilities read(Object value) {
            Map<String,Object> object = Values.object(value);
            if ("yes_no".equals(object.get("kind"))) return SessionProbabilitiesYesNo.read(value);
            if ("named".equals(object.get("kind"))) return SessionProbabilitiesNamed.read(value);
            throw new IllegalStateException("Unknown native result alternative");
        }
    }
    public static final class SessionProbabilitiesNamed extends View implements SessionProbabilities {
        public SessionProbabilitiesNamed(Object value) {
            super(value);
        }
        static SessionProbabilitiesNamed read(Object value) {
            return new SessionProbabilitiesNamed(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public List<SessionNamedProbability> value() {
            Object value = required("value");
            return Values.list(value, item0 -> SessionNamedProbability.read(item0));
        }
    }
    public static final class SessionProbabilitiesYesNo extends View implements SessionProbabilities {
        public SessionProbabilitiesYesNo(Object value) {
            super(value);
        }
        static SessionProbabilitiesYesNo read(Object value) {
            return new SessionProbabilitiesYesNo(value);
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public BigDecimal value() {
            Object value = required("value");
            return (BigDecimal)value;
        }
    }
    public static final class SessionQuestionDetail extends View {
        public SessionQuestionDetail(Object value) {
            super(value);
        }
        static SessionQuestionDetail read(Object value) {
            return new SessionQuestionDetail(value);
        }
        public Presence<AnswerId> answerId() {
            return presence("answer_id", value -> AnswerId.read(value));
        }
        public Boolean cached() {
            Object value = required("cached");
            return (Boolean)value;
        }
        public Presence<BigDecimal> confidence() {
            return presence("confidence", value -> (BigDecimal)value);
        }
        public BigInteger failedQuestions() {
            Object value = required("failed_questions");
            return Values.integer(value);
        }
        public Presence<Failure> failure() {
            return presence("failure", value -> Failure.read(value));
        }
        public Presence<FailureId> failureId() {
            return presence("failure_id", value -> FailureId.read(value));
        }
        public Presence<Object> input() {
            return presence("input", value -> Values.freeze(value));
        }
        public Presence<PhysicalSource> inputSource() {
            return presence("input_source", value -> PhysicalSource.read(value));
        }
        public List<SessionInputSource> inputSources() {
            Object value = required("input_sources");
            return Values.list(value, item0 -> SessionInputSource.read(item0));
        }
        public List<Object> inputs() {
            Object value = required("inputs");
            return Values.list(value, item0 -> Values.freeze(item0));
        }
        public String model() {
            Object value = required("model");
            return (String)value;
        }
        public List<Observation> observations() {
            Object value = required("observations");
            return Values.list(value, item0 -> Observation.read(item0));
        }
        public Presence<SessionProbabilities> probabilities() {
            return presence("probabilities", value -> SessionProbabilities.read(value));
        }
        public QuestionDocument question() {
            Object value = required("question");
            return QuestionDocument.read(value);
        }
        public String questionSha256() {
            Object value = required("question_sha256");
            return (String)value;
        }
        public List<QuestionSource> questionSources() {
            Object value = required("question_sources");
            return Values.list(value, item0 -> QuestionSource.read(item0));
        }
        public Presence<String> rawPick() {
            return presence("raw_pick", value -> (String)value);
        }
        public Presence<Usage> reportedUsage() {
            return presence("reported_usage", value -> Usage.read(value));
        }
        public List<String> requests() {
            Object value = required("requests");
            return Values.list(value, item0 -> (String)item0);
        }
        public BigInteger requestsSent() {
            Object value = required("requests_sent");
            return Values.integer(value);
        }
        public Presence<Threshold> threshold() {
            return presence("threshold", value -> Threshold.read(value));
        }
        public String url() {
            Object value = required("url");
            return (String)value;
        }
        public Presence<TokenUsage> usage() {
            return presence("usage", value -> TokenUsage.read(value));
        }
        public Presence<Value> value() {
            return presence("value", value -> Value.read(value));
        }
    }
    public static final class SessionRecognition extends View {
        public SessionRecognition(Object value) {
            super(value);
        }
        static SessionRecognition read(Object value) {
            return new SessionRecognition(value);
        }
        public List<Entity> entities() {
            Object value = required("entities");
            return Values.list(value, item0 -> Entity.read(item0));
        }
        public RecognitionMode mode() {
            Object value = required("mode");
            return RecognitionMode.read(value);
        }
        public Presence<List<BoundaryProposal>> proposals() {
            return presence("proposals", value -> Values.list(value, item0 -> BoundaryProposal.read(item0)));
        }
        public Presence<List<RecognitionEdgeDocument>> relations() {
            return presence("relations", value -> Values.list(value, item0 -> RecognitionEdgeDocument.read(item0)));
        }
    }
    public static final class SessionRelationEdge extends View {
        public SessionRelationEdge(Object value) {
            super(value);
        }
        static SessionRelationEdge read(Object value) {
            return new SessionRelationEdge(value);
        }
        public Boolean either() {
            Object value = required("either");
            return (Boolean)value;
        }
        public BigDecimal probability() {
            Object value = required("probability");
            return (BigDecimal)value;
        }
        public String relation() {
            Object value = required("relation");
            return (String)value;
        }
        public EntityDocument source() {
            Object value = required("source");
            return EntityDocument.read(value);
        }
        public EntityDocument target() {
            Object value = required("target");
            return EntityDocument.read(value);
        }
        public String sourceName() {
            return (String)Values.object(required("source")).get("name");
        }
        public String targetName() {
            return (String)Values.object(required("target")).get("name");
        }
    }
    public sealed interface SettledAnswer permits SettledAnswerDecide,SettledAnswerChoose,SettledAnswerTag,
    SettledAnswerScore,SettledAnswerFilter,SettledAnswerRank,SettledAnswerFind,SettledAnswerRecognize,
    SettledAnswerRelate,SettledAnswerAnnotate {
        static SettledAnswer read(Object value) {
            Map<String,Object> object = Values.object(value);
            if ("decide".equals(object.get("function"))) return SettledAnswerDecide.read(value);
            if ("choose".equals(object.get("function"))) return SettledAnswerChoose.read(value);
            if ("tag".equals(object.get("function"))) return SettledAnswerTag.read(value);
            if ("score".equals(object.get("function"))) return SettledAnswerScore.read(value);
            if ("filter".equals(object.get("function"))) return SettledAnswerFilter.read(value);
            if ("rank".equals(object.get("function"))) return SettledAnswerRank.read(value);
            if ("find".equals(object.get("function"))) return SettledAnswerFind.read(value);
            if ("recognize".equals(object.get("function"))) return SettledAnswerRecognize.read(value);
            if ("relate".equals(object.get("function"))) return SettledAnswerRelate.read(value);
            if ("annotate".equals(object.get("function"))) return SettledAnswerAnnotate.read(value);
            throw new IllegalStateException("Unknown native result alternative");
        }
    }
    public static final class SettledAnswerAnnotate extends View implements SettledAnswer {
        public SettledAnswerAnnotate(Object value) {
            super(value);
        }
        static SettledAnswerAnnotate read(Object value) {
            return new SettledAnswerAnnotate(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public Presence<AnswerShape4> index() {
            required("index");
            return presence("index", value -> AnswerShape4.read(value));
        }
        public Presence<AnswerShape3> pick() {
            required("pick");
            return presence("pick", value -> AnswerShape3.read(value));
        }
        public Presence<AnswerShape2> probability() {
            required("probability");
            return presence("probability", value -> AnswerShape2.read(value));
        }
        public AnswerShape5 resolution() {
            Object value = required("resolution");
            return AnswerShape5.read(value);
        }
        public Presence<AnswerShape15> value() {
            required("value");
            return presence("value", value -> AnswerShape15.read(value));
        }
    }
    public static final class SettledAnswerChoose extends View implements SettledAnswer {
        public SettledAnswerChoose(Object value) {
            super(value);
        }
        static SettledAnswerChoose read(Object value) {
            return new SettledAnswerChoose(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public Presence<AnswerShape4> index() {
            required("index");
            return presence("index", value -> AnswerShape4.read(value));
        }
        public Presence<AnswerShape3> pick() {
            required("pick");
            return presence("pick", value -> AnswerShape3.read(value));
        }
        public Presence<AnswerShape2> probability() {
            required("probability");
            return presence("probability", value -> AnswerShape2.read(value));
        }
        public AnswerShape5 resolution() {
            Object value = required("resolution");
            return AnswerShape5.read(value);
        }
        public Presence<AnswerShape6> value() {
            required("value");
            return presence("value", value -> AnswerShape6.read(value));
        }
    }
    public static final class SettledAnswerDecide extends View implements SettledAnswer {
        public SettledAnswerDecide(Object value) {
            super(value);
        }
        static SettledAnswerDecide read(Object value) {
            return new SettledAnswerDecide(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public Presence<AnswerShape4> index() {
            required("index");
            return presence("index", value -> AnswerShape4.read(value));
        }
        public Presence<AnswerShape3> pick() {
            required("pick");
            return presence("pick", value -> AnswerShape3.read(value));
        }
        public Presence<AnswerShape2> probability() {
            required("probability");
            return presence("probability", value -> AnswerShape2.read(value));
        }
        public AnswerShape5 resolution() {
            Object value = required("resolution");
            return AnswerShape5.read(value);
        }
        public AnswerShape value() {
            Object value = required("value");
            return AnswerShape.read(value);
        }
    }
    public static final class SettledAnswerFilter extends View implements SettledAnswer {
        public SettledAnswerFilter(Object value) {
            super(value);
        }
        static SettledAnswerFilter read(Object value) {
            return new SettledAnswerFilter(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public Presence<AnswerShape4> index() {
            required("index");
            return presence("index", value -> AnswerShape4.read(value));
        }
        public Presence<AnswerShape3> pick() {
            required("pick");
            return presence("pick", value -> AnswerShape3.read(value));
        }
        public Presence<AnswerShape10> probability() {
            required("probability");
            return presence("probability", value -> AnswerShape10.read(value));
        }
        public AnswerShape5 resolution() {
            Object value = required("resolution");
            return AnswerShape5.read(value);
        }
        public Presence<AnswerShape9> value() {
            required("value");
            return presence("value", value -> AnswerShape9.read(value));
        }
    }
    public static final class SettledAnswerFind extends View implements SettledAnswer {
        public SettledAnswerFind(Object value) {
            super(value);
        }
        static SettledAnswerFind read(Object value) {
            return new SettledAnswerFind(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public Presence<AnswerShape4> index() {
            required("index");
            return presence("index", value -> AnswerShape4.read(value));
        }
        public Presence<AnswerShape3> pick() {
            required("pick");
            return presence("pick", value -> AnswerShape3.read(value));
        }
        public Presence<AnswerShape2> probability() {
            required("probability");
            return presence("probability", value -> AnswerShape2.read(value));
        }
        public AnswerShape5 resolution() {
            Object value = required("resolution");
            return AnswerShape5.read(value);
        }
        public AnswerShape12 value() {
            Object value = required("value");
            return AnswerShape12.read(value);
        }
    }
    public static final class SettledAnswerRank extends View implements SettledAnswer {
        public SettledAnswerRank(Object value) {
            super(value);
        }
        static SettledAnswerRank read(Object value) {
            return new SettledAnswerRank(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public Presence<AnswerShape4> index() {
            required("index");
            return presence("index", value -> AnswerShape4.read(value));
        }
        public Presence<AnswerShape3> pick() {
            required("pick");
            return presence("pick", value -> AnswerShape3.read(value));
        }
        public Presence<AnswerShape10> probability() {
            required("probability");
            return presence("probability", value -> AnswerShape10.read(value));
        }
        public AnswerShape5 resolution() {
            Object value = required("resolution");
            return AnswerShape5.read(value);
        }
        public Presence<AnswerShape11> value() {
            required("value");
            return presence("value", value -> AnswerShape11.read(value));
        }
    }
    public static final class SettledAnswerRecognize extends View implements SettledAnswer {
        public SettledAnswerRecognize(Object value) {
            super(value);
        }
        static SettledAnswerRecognize read(Object value) {
            return new SettledAnswerRecognize(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public Presence<AnswerShape4> index() {
            required("index");
            return presence("index", value -> AnswerShape4.read(value));
        }
        public Presence<AnswerShape3> pick() {
            required("pick");
            return presence("pick", value -> AnswerShape3.read(value));
        }
        public Presence<AnswerShape10> probability() {
            required("probability");
            return presence("probability", value -> AnswerShape10.read(value));
        }
        public AnswerShape5 resolution() {
            Object value = required("resolution");
            return AnswerShape5.read(value);
        }
        public Presence<AnswerShape13> value() {
            required("value");
            return presence("value", value -> AnswerShape13.read(value));
        }
    }
    public static final class SettledAnswerRelate extends View implements SettledAnswer {
        public SettledAnswerRelate(Object value) {
            super(value);
        }
        static SettledAnswerRelate read(Object value) {
            return new SettledAnswerRelate(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public Presence<AnswerShape4> index() {
            required("index");
            return presence("index", value -> AnswerShape4.read(value));
        }
        public Presence<AnswerShape3> pick() {
            required("pick");
            return presence("pick", value -> AnswerShape3.read(value));
        }
        public Presence<AnswerShape10> probability() {
            required("probability");
            return presence("probability", value -> AnswerShape10.read(value));
        }
        public AnswerShape5 resolution() {
            Object value = required("resolution");
            return AnswerShape5.read(value);
        }
        public Presence<AnswerShape14> value() {
            required("value");
            return presence("value", value -> AnswerShape14.read(value));
        }
    }
    public static final class SettledAnswerScore extends View implements SettledAnswer {
        public SettledAnswerScore(Object value) {
            super(value);
        }
        static SettledAnswerScore read(Object value) {
            return new SettledAnswerScore(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public Presence<AnswerShape4> index() {
            required("index");
            return presence("index", value -> AnswerShape4.read(value));
        }
        public Presence<AnswerShape3> pick() {
            required("pick");
            return presence("pick", value -> AnswerShape3.read(value));
        }
        public Presence<AnswerShape2> probability() {
            required("probability");
            return presence("probability", value -> AnswerShape2.read(value));
        }
        public AnswerShape5 resolution() {
            Object value = required("resolution");
            return AnswerShape5.read(value);
        }
        public Presence<AnswerShape8> value() {
            required("value");
            return presence("value", value -> AnswerShape8.read(value));
        }
    }
    public static final class SettledAnswerTag extends View implements SettledAnswer {
        public SettledAnswerTag(Object value) {
            super(value);
        }
        static SettledAnswerTag read(Object value) {
            return new SettledAnswerTag(value);
        }
        public String function() {
            Object value = required("function");
            return (String)value;
        }
        public Presence<AnswerShape4> index() {
            required("index");
            return presence("index", value -> AnswerShape4.read(value));
        }
        public Presence<AnswerShape3> pick() {
            required("pick");
            return presence("pick", value -> AnswerShape3.read(value));
        }
        public Presence<AnswerShape2> probability() {
            required("probability");
            return presence("probability", value -> AnswerShape2.read(value));
        }
        public AnswerShape5 resolution() {
            Object value = required("resolution");
            return AnswerShape5.read(value);
        }
        public Presence<AnswerShape7> value() {
            required("value");
            return presence("value", value -> AnswerShape7.read(value));
        }
    }
    public static final class SourceRelationEndpoint extends View {
        public SourceRelationEndpoint(Object value) {
            super(value);
        }
        static SourceRelationEndpoint read(Object value) {
            return new SourceRelationEndpoint(value);
        }
        public Presence<String> file() {
            required("file");
            return presence("file", value -> (String)value);
        }
        public Presence<BigInteger> firstLine() {
            return presence("first_line", value -> Values.integer(value));
        }
        public String kind() {
            Object value = required("kind");
            return (String)value;
        }
        public Presence<BigInteger> lastLine() {
            return presence("last_line", value -> Values.integer(value));
        }
        public String name() {
            Object value = required("name");
            return (String)value;
        }
        public BigInteger ordinal() {
            Object value = required("ordinal");
            return Values.integer(value);
        }
        public Object record() {
            Object value = required("record");
            return Values.freeze(value);
        }
    }
    public sealed interface Threshold permits ThresholdNumber,ThresholdString {
        static Threshold read(Object value) {
            if (value instanceof BigDecimal) return ThresholdNumber.read(value);
            if (value instanceof String) return ThresholdString.read(value);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record ThresholdNumber(BigDecimal value) implements Threshold {
        static ThresholdNumber read(Object value) {
            return new ThresholdNumber((BigDecimal)value);
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record ThresholdString(String value) implements Threshold {
        static ThresholdString read(Object value) {
            return new ThresholdString((String)value);
        }
        Object json() {
            return Values.json(value);
        }
    }
    public static final class TokenBand extends View {
        public TokenBand(Object value) {
            super(value);
        }
        static TokenBand read(Object value) {
            return new TokenBand(value);
        }
        public BigInteger lower() {
            Object value = required("lower");
            return Values.integer(value);
        }
        public BigInteger upper() {
            Object value = required("upper");
            return Values.integer(value);
        }
    }
    public static final class TokenUsage extends View {
        public TokenUsage(Object value) {
            super(value);
        }
        static TokenUsage read(Object value) {
            return new TokenUsage(value);
        }
        public BigInteger inputTokens() {
            Object value = required("input_tokens");
            return Values.integer(value);
        }
        public BigInteger outputTokens() {
            Object value = required("output_tokens");
            return Values.integer(value);
        }
    }
    public sealed interface Value permits ValueBoolean,ValueNull,ValueString,ValueArray,ValueNumber {
        static Value read(Object value) {
            if (value instanceof Boolean) return ValueBoolean.read(value);
            if (value == null) return new ValueNull(null);
            if (value instanceof String) return ValueString.read(value);
            if ((value instanceof List<?> && ((List<?>)value).stream().allMatch(union_item_0 -> union_item_0 instanceof String))) return ValueArray.read(value);
            if (value instanceof BigDecimal) return ValueNumber.read(value);
            throw new IllegalStateException("Unknown native value alternative");
        }
    }
    public record ValueBoolean(Boolean value) implements Value {
        static ValueBoolean read(Object value) {
            return new ValueBoolean((Boolean)value);
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record ValueNull(Object value) implements Value {
        static ValueNull read(Object value) {
            return new ValueNull(Values.freeze(value));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record ValueString(String value) implements Value {
        static ValueString read(Object value) {
            return new ValueString((String)value);
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record ValueArray(List<String> value) implements Value {
        static ValueArray read(Object value) {
            return new ValueArray(Values.list(value, item0 -> (String)item0));
        }
        Object json() {
            return Values.json(value);
        }
    }
    public record ValueNumber(BigDecimal value) implements Value {
        static ValueNumber read(Object value) {
            return new ValueNumber((BigDecimal)value);
        }
        Object json() {
            return Values.json(value);
        }
    }
    public static final class ImmediateFacts extends View {
        public ImmediateFacts(Object value) {
            super(value);
        }
        static ImmediateFacts read(Object value) {
            return new ImmediateFacts(value);
        }
        public Presence<List<Attempt>> attempts() {
            return presence("attempts", value -> Values.list(value, item0 -> Attempt.read(item0)));
        }
        public BigInteger cacheAnswers() {
            Object value = required("cache_answers");
            return Values.integer(value);
        }
        public Presence<CallId> callId() {
            return presence("call_id", value -> CallId.read(value));
        }
        public Presence<String> estimatedCostUsd() {
            return presence("estimated_cost_usd", value -> (String)value);
        }
        public Presence<Boolean> heldModelMismatch() {
            return presence("held_model_mismatch", value -> (Boolean)value);
        }
        public Presence<BigInteger> inputTokens() {
            return presence("input_tokens", value -> Values.integer(value));
        }
        public BigInteger largestRequestBytes() {
            Object value = required("largest_request_bytes");
            return Values.integer(value);
        }
        public Presence<BigInteger> largestRequestEstimatedInputTokens() {
            required("largest_request_estimated_input_tokens");
            return presence("largest_request_estimated_input_tokens", value -> Values.integer(value));
        }
        public Presence<String> model() {
            return presence("model", value -> (String)value);
        }
        public Presence<BigInteger> outputTokens() {
            return presence("output_tokens", value -> Values.integer(value));
        }
        public BigInteger records() {
            Object value = required("records");
            return Values.integer(value);
        }
        public BigInteger requestsSent() {
            Object value = required("requests_sent");
            return Values.integer(value);
        }
        public BigDecimal seconds() {
            Object value = required("seconds");
            return (BigDecimal)value;
        }
        public String tokenEstimateMethod() {
            Object value = required("token_estimate_method");
            return (String)value;
        }
        public Presence<PersistenceObservation> usagePersistence() {
            return presence("usage_persistence", value -> PersistenceObservation.read(value));
        }
    }
    static Object tree(Object value) {
        if (value instanceof AnswerResolutionsString item) return Values.json(item.value());
        if (value instanceof AnswerResolutionsObject item) return Values.json(item.value());
        if (value instanceof AnswerShape10Array item) return Values.json(item.value());
        if (value instanceof AnswerShape10Null item) return Values.json(item.value());
        if (value instanceof AnswerShape10Array2 item) return Values.json(item.value());
        if (value instanceof AnswerShape10AnyOf1ItemsArray item) return Values.json(item.value());
        if (value instanceof AnswerShape10AnyOf1ItemsNull item) return Values.json(item.value());
        if (value instanceof AnswerShape11Array item) return Values.json(item.value());
        if (value instanceof AnswerShape11Null item) return Values.json(item.value());
        if (value instanceof AnswerShape11Array2 item) return Values.json(item.value());
        if (value instanceof AnswerShape11AnyOf1ItemsArray item) return Values.json(item.value());
        if (value instanceof AnswerShape11AnyOf1ItemsNull item) return Values.json(item.value());
        if (value instanceof AnswerShape13Object item) return Values.json(item.value());
        if (value instanceof AnswerShape13Null item) return Values.json(item.value());
        if (value instanceof AnswerShape13Array item) return Values.json(item.value());
        if (value instanceof AnswerShape13AnyOf1ItemsObject item) return Values.json(item.value());
        if (value instanceof AnswerShape13AnyOf1ItemsNull item) return Values.json(item.value());
        if (value instanceof AnswerShape14Array item) return Values.json(item.value());
        if (value instanceof AnswerShape14Null item) return Values.json(item.value());
        if (value instanceof AnswerShape14Array2 item) return Values.json(item.value());
        if (value instanceof AnswerShape14AnyOf1ItemsArray item) return Values.json(item.value());
        if (value instanceof AnswerShape14AnyOf1ItemsNull item) return Values.json(item.value());
        if (value instanceof AnswerShape15Object item) return Values.json(item.value());
        if (value instanceof AnswerShape15Null item) return Values.json(item.value());
        if (value instanceof AnswerShape15Array item) return Values.json(item.value());
        if (value instanceof AnswerShape15AnyOf1ItemsObject item) return Values.json(item.value());
        if (value instanceof AnswerShape15AnyOf1ItemsNull item) return Values.json(item.value());
        if (value instanceof AnswerShape2Number item) return Values.json(item.value());
        if (value instanceof AnswerShape2Null item) return Values.json(item.value());
        if (value instanceof AnswerShape2Array item) return Values.json(item.value());
        if (value instanceof AnswerShape3String item) return Values.json(item.value());
        if (value instanceof AnswerShape3Null item) return Values.json(item.value());
        if (value instanceof AnswerShape3Array item) return Values.json(item.value());
        if (value instanceof AnswerShape4Integer item) return Values.json(item.value());
        if (value instanceof AnswerShape4Null item) return Values.json(item.value());
        if (value instanceof AnswerShape4Array item) return Values.json(item.value());
        if (value instanceof AnswerShape4AnyOf1ItemsInteger item) return Values.json(item.value());
        if (value instanceof AnswerShape4AnyOf1ItemsNull item) return Values.json(item.value());
        if (value instanceof AnswerShape5String item) return Values.json(item.value());
        if (value instanceof AnswerShape5Object item) return Values.json(item.value());
        if (value instanceof AnswerShape5Array item) return Values.json(item.value());
        if (value instanceof AnswerShape6String item) return Values.json(item.value());
        if (value instanceof AnswerShape6Null item) return Values.json(item.value());
        if (value instanceof AnswerShape6Array item) return Values.json(item.value());
        if (value instanceof AnswerShape6AnyOf1ItemsString item) return Values.json(item.value());
        if (value instanceof AnswerShape6AnyOf1ItemsNull item) return Values.json(item.value());
        if (value instanceof AnswerShape7Array item) return Values.json(item.value());
        if (value instanceof AnswerShape7Null item) return Values.json(item.value());
        if (value instanceof AnswerShape7Array2 item) return Values.json(item.value());
        if (value instanceof AnswerShape7AnyOf1ItemsArray item) return Values.json(item.value());
        if (value instanceof AnswerShape7AnyOf1ItemsNull item) return Values.json(item.value());
        if (value instanceof AnswerShape8Number item) return Values.json(item.value());
        if (value instanceof AnswerShape8Null item) return Values.json(item.value());
        if (value instanceof AnswerShape8Array item) return Values.json(item.value());
        if (value instanceof AnswerShape8AnyOf1ItemsNumber item) return Values.json(item.value());
        if (value instanceof AnswerShape8AnyOf1ItemsNull item) return Values.json(item.value());
        if (value instanceof AnswerShape9Array item) return Values.json(item.value());
        if (value instanceof AnswerShape9Null item) return Values.json(item.value());
        if (value instanceof AnswerShape9Array2 item) return Values.json(item.value());
        if (value instanceof AnswerShape9AnyOf1ItemsArray item) return Values.json(item.value());
        if (value instanceof AnswerShape9AnyOf1ItemsNull item) return Values.json(item.value());
        if (value instanceof BatchInteger item) return Values.json(item.value());
        if (value instanceof BatchString item) return Values.json(item.value());
        if (value instanceof SettledJudgmentBoolean item) return Values.json(item.value());
        if (value instanceof SettledJudgmentNull item) return Values.json(item.value());
        if (value instanceof SettledJudgmentString item) return Values.json(item.value());
        if (value instanceof SettledJudgmentNumber item) return Values.json(item.value());
        if (value instanceof SettledJudgmentArray item) return Values.json(item.value());
        if (value instanceof AnnotatedFieldBoolean item) return Values.json(item.value());
        if (value instanceof AnnotatedFieldNull item) return Values.json(item.value());
        if (value instanceof AnnotatedFieldString item) return Values.json(item.value());
        if (value instanceof AnnotatedFieldArray item) return Values.json(item.value());
        if (value instanceof AnnotatedFieldNumber item) return Values.json(item.value());
        if (value instanceof AnnotatedFieldObject item) return Values.json(item.value());
        if (value instanceof BatchSettingInteger item) return Values.json(item.value());
        if (value instanceof BatchSettingString item) return Values.json(item.value());
        if (value instanceof ThresholdNumber item) return Values.json(item.value());
        if (value instanceof ThresholdString item) return Values.json(item.value());
        if (value instanceof ValueBoolean item) return Values.json(item.value());
        if (value instanceof ValueNull item) return Values.json(item.value());
        if (value instanceof ValueString item) return Values.json(item.value());
        if (value instanceof ValueArray item) return Values.json(item.value());
        if (value instanceof ValueNumber item) return Values.json(item.value());
        return value;
    }
    public static final class CallResult<V,P,R> extends Engine.OwnedCall {
        private final V value;
        private final P probability;
        private final List<R> results;
        private final Facts facts;
        private CallResult(Engine.OwnedCall call, V value, P probability, List<R> results) {
            super(call.packets(), call.terminal());
            this.value = value;
            this.probability = probability;
            this.results = Collections.unmodifiableList(new ArrayList<>(results));
            this.facts = call.terminal().facts().value();
        }
        public V value() {
            return value;
        }
        public P probability() {
            return probability;
        }
        public List<R> results() {
            return results;
        }
        public Facts facts() {
            return facts;
        }
    }
    private static <T> T optional(Presence<T> value) {
        return value.state() == Presence.State.VALUE ? value.value() : null;
    }
    private static <T> List<T> immutable(List<T> values) {
        return Collections.unmodifiableList(values);
    }
    private static Double probability(BigDecimal value) {
        return value == null ? null : value.doubleValue();
    }
    static CallResult<List<Object>,List<Double>,AtomicDecideValue> projectDecide(Engine.OwnedCall call) {
        var results = new ArrayList<AtomicDecideValue>();
        for (SessionPacket packet : call.packets()) {
            if (packet instanceof SessionPacketDecideAggregate aggregate) results.addAll(aggregate.value());
            else if (packet instanceof SessionPacketDecideRow row) results.add(row.value());
        }
        var values = new ArrayList<Object>();
        var probabilities = new ArrayList<Double>();
        for (AtomicDecideValue row : results) {
            values.add(row.value().value());
            probabilities.add(resolved(call, "decide", optional(row.index())) ? (row.answer() instanceof AnswerYesNo odds ? probability(odds.probability()) : null) : null);
        }
        return new CallResult<>(call, immutable(values), immutable(probabilities), results);
    }
    static CallResult<Object,Double,AtomicDecideValue> projectDecideScalar(Engine.OwnedCall call) {
        var result = projectDecide(call);
        if (result.results().size() != 1) throw new IllegalStateException("Expected one input result");
        return new CallResult<>(call, result.value().getFirst(), result.probability().getFirst(), result.results());
    }
    static CallResult<List<String>,List<Double>,AtomicNullableString> projectChoose(Engine.OwnedCall call) {
        var results = new ArrayList<AtomicNullableString>();
        for (SessionPacket packet : call.packets()) {
            if (packet instanceof SessionPacketChooseAggregate aggregate) results.addAll(aggregate.value());
            else if (packet instanceof SessionPacketChooseRow row) results.add(row.value());
        }
        var values = new ArrayList<String>();
        var probabilities = new ArrayList<Double>();
        for (AtomicNullableString row : results) {
            values.add(optional(row.value()));
            probabilities.add(resolved(call, "choose", optional(row.index())) ? (row.answer() instanceof AnswerChoice odds ? probability(odds.probabilities().get(odds.pick())) : null) : null);
        }
        return new CallResult<>(call, immutable(values), immutable(probabilities), results);
    }
    static CallResult<String,Double,AtomicNullableString> projectChooseScalar(Engine.OwnedCall call) {
        var result = projectChoose(call);
        if (result.results().size() != 1) throw new IllegalStateException("Expected one input result");
        return new CallResult<>(call, result.value().getFirst(), result.probability().getFirst(), result.results());
    }
    static CallResult<List<List<String>>,Void,AtomicArrayOfString> projectTag(Engine.OwnedCall call) {
        var results = new ArrayList<AtomicArrayOfString>();
        for (SessionPacket packet : call.packets()) {
            if (packet instanceof SessionPacketTagAggregate aggregate) results.addAll(aggregate.value());
            else if (packet instanceof SessionPacketTagRow row) results.add(row.value());
        }
        var values = new ArrayList<List<String>>();
        for (AtomicArrayOfString row : results) {
            values.add(row.value());
        }
        return new CallResult<>(call, immutable(values), null, results);
    }
    static CallResult<List<String>,Void,AtomicArrayOfString> projectTagScalar(Engine.OwnedCall call) {
        var result = projectTag(call);
        if (result.results().size() != 1) throw new IllegalStateException("Expected one input result");
        return new CallResult<>(call, result.value().getFirst(), null, result.results());
    }
    static CallResult<List<BigDecimal>,Void,AtomicDouble> projectScore(Engine.OwnedCall call) {
        var results = new ArrayList<AtomicDouble>();
        for (SessionPacket packet : call.packets()) {
            if (packet instanceof SessionPacketScoreAggregate aggregate) results.addAll(aggregate.value());
            else if (packet instanceof SessionPacketScoreRow row) results.add(row.value());
        }
        var values = new ArrayList<BigDecimal>();
        for (AtomicDouble row : results) {
            values.add(row.value());
        }
        return new CallResult<>(call, immutable(values), null, results);
    }
    static CallResult<BigDecimal,Void,AtomicDouble> projectScoreScalar(Engine.OwnedCall call) {
        var result = projectScore(call);
        if (result.results().size() != 1) throw new IllegalStateException("Expected one input result");
        return new CallResult<>(call, result.value().getFirst(), null, result.results());
    }
    static CallResult<List<Object>,List<Double>,AtomicBoolean> projectFilter(Engine.OwnedCall call) {
        var results = new ArrayList<AtomicBoolean>();
        for (SessionPacket packet : call.packets()) {
            if (packet instanceof SessionPacketFilterAggregate aggregate) results.addAll(aggregate.value());
            else if (packet instanceof SessionPacketFilterRow row) results.add(row.value());
        }
        var values = new ArrayList<Object>();
        var probabilities = new ArrayList<Double>();
        for (AtomicBoolean row : results) {
            values.add(optional(row.input()));
            probabilities.add(row.answer() instanceof AnswerYesNo odds ? probability(odds.probability()) : null);
        }
        return new CallResult<>(call, immutable(values), immutable(probabilities), results);
    }
    static CallResult<List<BigInteger>,List<Double>,AtomicNonZeroUsize> projectRank(Engine.OwnedCall call) {
        var results = new ArrayList<AtomicNonZeroUsize>();
        for (SessionPacket packet : call.packets()) {
            if (packet instanceof SessionPacketRankAggregate aggregate) results.addAll(aggregate.value());
        }
        results.sort(Comparator.comparing(AtomicNonZeroUsize::value));
        var values = new ArrayList<BigInteger>();
        var probabilities = new ArrayList<Double>();
        for (AtomicNonZeroUsize row : results) {
            values.add(optional(row.index()));
            probabilities.add(row.answer() instanceof AnswerYesNo odds ? probability(odds.probability()) : null);
        }
        return new CallResult<>(call, immutable(values), immutable(probabilities), results);
    }
    static CallResult<Object,Double,Find> projectFind(Engine.OwnedCall call) {
        var results = new ArrayList<Find>();
        for (SessionPacket packet : call.packets()) {
            if (packet instanceof SessionPacketFindAggregate aggregate) results.add(aggregate.value());
        }
        if (results.size() != 1) throw new IllegalStateException("Expected one aggregate result");
        var row = results.getFirst();
        return new CallResult<>(call, optional(row.value()), optional(row.index()) == null ? null : (row.answer() instanceof FindAnswer odds ? probability(odds.probabilities().get(odds.pick())) : null),
        results);
    }
    static CallResult<List<AnnotatedRow>,Void,Annotation> projectAnnotate(Engine.OwnedCall call) {
        var results = new ArrayList<Annotation>();
        for (SessionPacket packet : call.packets()) {
            if (packet instanceof SessionPacketAnnotateAggregate aggregate) results.addAll(aggregate.value());
            else if (packet instanceof SessionPacketAnnotateRow row) results.add(row.value());
        }
        var values = new ArrayList<AnnotatedRow>();
        for (Annotation row : results) {
            values.add(row.value());
        }
        return new CallResult<>(call, immutable(values), null, results);
    }
    static CallResult<AnnotatedRow,Void,Annotation> projectAnnotateScalar(Engine.OwnedCall call) {
        var result = projectAnnotate(call);
        if (result.results().size() != 1) throw new IllegalStateException("Expected one input result");
        return new CallResult<>(call, result.value().getFirst(), null, result.results());
    }
    static CallResult<List<Recognize>,List<List<Double>>,Recognition> projectRecognize(Engine.OwnedCall call) {
        var results = new ArrayList<Recognition>();
        for (SessionPacket packet : call.packets()) {
            if (packet instanceof SessionPacketRecognizeAggregate aggregate) results.addAll(aggregate.value());
        }
        var values = new ArrayList<Recognize>();
        var probabilities = new ArrayList<List<Double>>();
        for (Recognition row : results) {
            values.add(row.value());
            probabilities.add(recognitionProbabilities(row));
        }
        return new CallResult<>(call, immutable(values), immutable(probabilities), results);
    }
    static CallResult<Recognize,List<Double>,Recognition> projectRecognizeScalar(Engine.OwnedCall call) {
        var result = projectRecognize(call);
        if (result.results().size() != 1) throw new IllegalStateException("Expected one input result");
        return new CallResult<>(call, result.value().getFirst(), result.probability().getFirst(), result.results());
    }
    static CallResult<List<RelatedEntityEdge>,List<Double>,Relation> projectRelate(Engine.OwnedCall call) {
        var results = new ArrayList<Relation>();
        for (SessionPacket packet : call.packets()) {
            if (packet instanceof SessionPacketRelateAggregate aggregate) results.add(aggregate.value());
        }
        if (results.size() != 1) throw new IllegalStateException("Expected one aggregate result");
        var row = results.getFirst();
        return new CallResult<>(call, row.value(), row.value().stream().map(edge -> probability(edge.probability())).toList(),
        results);
    }
    private static List<Double> recognitionProbabilities(Recognition row) {
        if (row.value() instanceof RecognizeModeProposals boundary) return boundary.proposals().stream().map(proposal -> probability(proposal.probability())).toList();
        var entities = ((RecognizeEntities)row.value()).entities();
        var proposals = row.answer() instanceof RecognitionOddsRecognizeAnswer odds ? odds.proposals() : List.<RecognitionProposal>of();
        var probabilities = new ArrayList<Double>();
        for (var entity : entities) {
            Double probability = null;
            for (var proposal : proposals) {
                var selected = optional(proposal.selected());
                if (proposal.kept() && selected != null && selected.start().equals(entity.start()) && selected.end().equals(entity.end()) && Objects.equals(optional(proposal.kind()),
                entity.kind())) {
                    probability = probability(proposal.spanProbability());
                    break;
                }
            }
            probabilities.add(probability);
        }
        return immutable(probabilities);
    }
    private static Object path(Object value, String[] members) {
        for (String member : members) {
            if (!(value instanceof Map<?,?> object)) return null;
            value = object.get(member);
        } return value;
    }
    private static boolean resolved(Engine.OwnedCall call, String function, BigInteger index) {
        for (var item : call.packets()) {
            var packet = Values.object(Values.json(item));
            if (!function.equals(packet.get("function"))) continue;
            switch (function) {
                case "decide" -> {
                    if (!("observation".equals(path(packet, new String[]{
                        "kind"
                    })) && "row".equals(path(packet, new String[]{
                        "value", "kind"
                    })) && "judgment".equals(path(packet, new String[]{
                        "value", "value", "kind"
                    })) && "decision".equals(path(packet, new String[]{
                        "value", "value", "value", "kind"
                    })))) continue;
                    Object nativeIndex = path(packet, new String[]{
                        "value", "index"
                    });
                    BigInteger actual = nativeIndex instanceof BigDecimal number ? number.toBigIntegerExact() : BigInteger.ZERO;
                    if (!actual.equals(index == null ? BigInteger.ZERO : index)) continue;
                    return path(packet, new String[]{
                        "value", "value", "value", "value"
                    }) != null;
                }
                case "choose" -> {
                    if (!("observation".equals(path(packet, new String[]{
                        "kind"
                    })) && "row".equals(path(packet, new String[]{
                        "value", "kind"
                    })) && "judgment".equals(path(packet, new String[]{
                        "value", "value", "kind"
                    })) && "choice".equals(path(packet, new String[]{
                        "value", "value", "value", "kind"
                    })))) continue;
                    Object nativeIndex = path(packet, new String[]{
                        "value", "index"
                    });
                    BigInteger actual = nativeIndex instanceof BigDecimal number ? number.toBigIntegerExact() : BigInteger.ZERO;
                    if (!actual.equals(index == null ? BigInteger.ZERO : index)) continue;
                    return path(packet, new String[]{
                        "value", "value", "value", "value"
                    }) != null;
                }
                default -> {
                }
            }
        }
        return false;
    }
}
