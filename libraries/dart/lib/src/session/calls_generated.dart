// Generated named calls and answers from canonical contracts; do not edit.
part of thinkthen_session;

mixin NamedCalls {
  Future<OwnedCall> _call(String function, Question question, Input input,
      Options? options, Cancellation? cancellation, Stream<FeedItem>? feed);
  OwnedSession _startSession(Question question, Input input, String function,
      {Options? options});
  OwnedSession decideSession(Question question, Input input,
          {Options? options}) =>
      _startSession(question, input, "decide", options: options);
  Future<OwnedCall> decide(Question question, Input input,
          {Options? options,
          Cancellation? cancellation,
          Stream<FeedItem>? feed}) =>
      _call("decide", question, input, options, cancellation, feed);
  OwnedSession chooseSession(Question question, Input input,
          {Options? options}) =>
      _startSession(question, input, "choose", options: options);
  Future<OwnedCall> choose(Question question, Input input,
          {Options? options,
          Cancellation? cancellation,
          Stream<FeedItem>? feed}) =>
      _call("choose", question, input, options, cancellation, feed);
  OwnedSession tagSession(Question question, Input input, {Options? options}) =>
      _startSession(question, input, "tag", options: options);
  Future<OwnedCall> tag(Question question, Input input,
          {Options? options,
          Cancellation? cancellation,
          Stream<FeedItem>? feed}) =>
      _call("tag", question, input, options, cancellation, feed);
  OwnedSession scoreSession(Question question, Input input,
          {Options? options}) =>
      _startSession(question, input, "score", options: options);
  Future<OwnedCall> score(Question question, Input input,
          {Options? options,
          Cancellation? cancellation,
          Stream<FeedItem>? feed}) =>
      _call("score", question, input, options, cancellation, feed);
  OwnedSession filterSession(Question question, Input input,
          {Options? options}) =>
      _startSession(question, input, "filter", options: options);
  Future<OwnedCall> filter(Question question, Input input,
          {Options? options,
          Cancellation? cancellation,
          Stream<FeedItem>? feed}) =>
      _call("filter", question, input, options, cancellation, feed);
  OwnedSession rankSession(Question question, Input input,
          {Options? options}) =>
      _startSession(question, input, "rank", options: options);
  Future<OwnedCall> rank(Question question, Input input,
          {Options? options,
          Cancellation? cancellation,
          Stream<FeedItem>? feed}) =>
      _call("rank", question, input, options, cancellation, feed);
  OwnedSession findSession(Question question, Input input,
          {Options? options}) =>
      _startSession(question, input, "find", options: options);
  Future<OwnedCall> find(Question question, Input input,
          {Options? options,
          Cancellation? cancellation,
          Stream<FeedItem>? feed}) =>
      _call("find", question, input, options, cancellation, feed);
  OwnedSession annotateSession(Question question, Input input,
          {Options? options}) =>
      _startSession(question, input, "annotate", options: options);
  Future<OwnedCall> annotate(Question question, Input input,
          {Options? options,
          Cancellation? cancellation,
          Stream<FeedItem>? feed}) =>
      _call("annotate", question, input, options, cancellation, feed);
  OwnedSession recognizeSession(Question question, Input input,
          {Options? options}) =>
      _startSession(question, input, "recognize", options: options);
  Future<OwnedCall> recognize(Question question, Input input,
          {Options? options,
          Cancellation? cancellation,
          Stream<FeedItem>? feed}) =>
      _call("recognize", question, input, options, cancellation, feed);
  OwnedSession relateSession(Question question, Input input,
          {Options? options}) =>
      _startSession(question, input, "relate", options: options);
  Future<OwnedCall> relate(Question question, Input input,
          {Options? options,
          Cancellation? cancellation,
          Stream<FeedItem>? feed}) =>
      _call("relate", question, input, options, cancellation, feed);
}
mixin CallAnswers {
  List<SessionPacket> get packets;
  SessionPacketTerminal get terminal;
  void _successful() {
    if (terminal.failure.value case final CallError failure)
      throw CallFailure._result(failure, this as OwnedCall);
  }

  bool? decideAnswer() {
    _successful();
    final answers = <bool?>[];
    for (final packet in packets) {
      if (packet is SessionPacketObservation && packet.function == "decide") {
        final observation = packet.value;
        if (observation is SessionObservationRow) {
          final row = observation.value;
          if (row is SessionObservedRowJudgment &&
              row.value is SessionJudgmentDecision)
            answers.add((row.value as SessionJudgmentDecision).value);
        }
      }
    }
    if (answers.length != 1) throw StateError('Expected exactly one answer');
    return answers.single;
  }

  String? chooseAnswer() {
    _successful();
    final answers = <String?>[];
    for (final packet in packets) {
      if (packet is SessionPacketObservation && packet.function == "choose") {
        final observation = packet.value;
        if (observation is SessionObservationRow) {
          final row = observation.value;
          if (row is SessionObservedRowJudgment &&
              row.value is SessionJudgmentChoice)
            answers.add((row.value as SessionJudgmentChoice).value);
        }
      }
    }
    if (answers.length != 1) throw StateError('Expected exactly one answer');
    return answers.single;
  }

  num scoreAnswer() {
    _successful();
    final answers = <num>[];
    for (final packet in packets) {
      if (packet is SessionPacketObservation && packet.function == "score") {
        final observation = packet.value;
        if (observation is SessionObservationRow) {
          final row = observation.value;
          if (row is SessionObservedRowJudgment &&
              row.value is SessionJudgmentScore)
            answers.add((row.value as SessionJudgmentScore).value);
        }
      }
    }
    if (answers.length != 1) throw StateError('Expected exactly one answer');
    return answers.single;
  }

  List<String> tagAnswer() {
    _successful();
    final answers = <List<String>>[];
    for (final packet in packets) {
      if (packet is SessionPacketObservation && packet.function == "tag") {
        final observation = packet.value;
        if (observation is SessionObservationRow) {
          final row = observation.value;
          if (row is SessionObservedRowJudgment &&
              row.value is SessionJudgmentTags)
            answers.add((row.value as SessionJudgmentTags).value);
        }
      }
    }
    if (answers.length != 1) throw StateError('Expected exactly one answer');
    return answers.single;
  }

  List<Object?> filterAnswer() {
    _successful();
    final answers = <FilterResult>[];
    for (final packet in packets) {
      if (packet is SessionPacketFilterAggregate)
        answers.addAll(packet.value);
      else if (packet is SessionPacketFilterRow) answers.add(packet.value);
    }
    return List.unmodifiable(answers
        .where((row) => row.value == true)
        .map((row) => row.input.value));
  }

  List<Object?> rankAnswer() {
    _successful();
    final answers = <RankResult>[];
    for (final packet in packets) {
      if (packet is SessionPacketRankAggregate) answers.addAll(packet.value);
    }
    answers.sort((a, b) => a.value.compareTo(b.value));
    return List.unmodifiable(answers.map((row) => row.input.value));
  }

  Find findAnswer() {
    _successful();
    final values =
        packets.whereType<SessionPacketFindAggregate>().toList(growable: false);
    if (values.length != 1) throw StateError('Expected exactly one answer');
    final answers = values.single.value;
    return answers;
  }

  List<Annotation> annotateAnswer() {
    _successful();
    final answers = <Annotation>[];
    for (final packet in packets) {
      if (packet is SessionPacketAnnotateAggregate)
        answers.addAll(packet.value);
      else if (packet is SessionPacketAnnotateRow) answers.add(packet.value);
    }
    return List.unmodifiable(answers);
  }

  List<Recognition> recognizeAnswer() {
    _successful();
    final answers = <Recognition>[];
    for (final packet in packets) {
      if (packet is SessionPacketRecognizeAggregate)
        answers.addAll(packet.value);
    }
    return List.unmodifiable(answers);
  }

  Relation relateAnswer() {
    _successful();
    final values = packets
        .whereType<SessionPacketRelateAggregate>()
        .toList(growable: false);
    if (values.length != 1) throw StateError('Expected exactly one answer');
    final answers = values.single.value;
    return answers;
  }
}
