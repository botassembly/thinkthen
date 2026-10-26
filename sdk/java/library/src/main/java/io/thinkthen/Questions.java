package io.thinkthen;

final class Questions {
    private Questions() {}
    static com.fasterxml.jackson.databind.JsonNode definition(Question<?,?> question) {
        return switch (question) {
            case DecideQuestion value -> value.definition();
            case ChooseQuestion value -> value.definition();
            case TagQuestion value -> value.definition();
            case ScoreQuestion value -> value.definition();
            case FilterQuestion value -> value.definition();
            case RankQuestion value -> value.definition();
            case FindQuestion value -> value.definition();
            case RecognizeQuestion value -> value.definition();
            case RelateQuestion value -> value.definition();
        };
    }
    static @org.jspecify.annotations.Nullable String name(Question<?,?> question) {
        return switch (question) {
            case DecideQuestion value -> value.name();
            case ChooseQuestion value -> value.name();
            case TagQuestion value -> value.name();
            case ScoreQuestion value -> value.name();
            case FilterQuestion value -> value.name();
            case RankQuestion value -> value.name();
            case FindQuestion value -> value.name();
            case RecognizeQuestion value -> value.name();
            case RelateQuestion value -> value.name();
        };
    }
    static @org.jspecify.annotations.Nullable Question<?,?> decode(com.fasterxml.jackson.databind.JsonNode value) {
        return switch (value.path("function").asText()) {
            case "decide" -> new DecideQuestion(value,null);
            case "choose" -> new ChooseQuestion(value,null);
            case "tag" -> new TagQuestion(value,null);
            case "score" -> new ScoreQuestion(value,null);
            case "filter" -> new FilterQuestion(value,null);
            case "rank" -> new RankQuestion(value,null);
            case "find" -> new FindQuestion(value,null);
            case "recognize" -> new RecognizeQuestion(value,null);
            case "relate" -> new RelateQuestion(value,null);
            default -> null;
        };
    }
}
