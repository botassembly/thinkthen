package dev.thinkthen;

/**
 * A typed Detail value.
 */
public sealed interface Detail permits NoDetail, LabelDetail, LevelDetail, RecognizeDetail, RelationDetail {}
