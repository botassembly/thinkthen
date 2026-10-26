package io.thinkthen;

public sealed interface AnnotationQuestion<A,D extends Detail> extends Question<A,D> permits DecideQuestion, ChooseQuestion, TagQuestion, ScoreQuestion, FilterQuestion, FindQuestion, RecognizeQuestion, RelateQuestion {}
