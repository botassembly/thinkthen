package io.thinkthen;

public sealed interface Detail permits NoDetail, ChooseDetail, TagDetail, ScoreDetail, RankDetail, RecognizeDetail, RelateDetail {}
