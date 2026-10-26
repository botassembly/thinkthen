package io.thinkthen;

public record RecognizeDetail(java.util.List<PieceDetail> pieces, java.util.List<KindDetail> names) implements Detail { public RecognizeDetail { pieces = java.util.List.copyOf(pieces); names = java.util.List.copyOf(names); } }
