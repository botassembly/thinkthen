// Generated named calls and results from canonical contracts; do not edit.
part of thinkthen_session;

mixin NamedCalls {
  Future<OwnedCall> _call(String function, Question question, Input input,
      Options? options, Cancellation? cancellation, Stream<FeedItem>? feed);
  OwnedSession _startSession(Question question, Input input, String function,
      {Options? options});
  OwnedSession decideSession(Question question, Input input,
          {Options? options}) =>
      _startSession(question, input, "decide", options: options);
  Future<CallResult<Object?, DecisionResult>> decide(
      Question question, Input input,
      {Options? options,
      Cancellation? cancellation,
      Stream<FeedItem>? feed}) async {
    final call =
        await _call("decide", question, input, options, cancellation, feed);
    final rows = <DecisionResult>[];
    var aggregated = false;
    for (final packet in call.packets) {
      if (packet is SessionPacketDecideAggregate) {
        if (!aggregated) rows.clear();
        aggregated = true;
        rows.addAll(packet.value);
      } else if (!aggregated && packet is SessionPacketDecideRow)
        rows.add(packet.value);
    }
    final resolved = <BigInt, bool>{};
    for (final packet in call.packets) {
      if (_atPath(packet._fields, ["kind"]) == "observation" &&
          _atPath(packet._fields, ["value", "kind"]) == "row" &&
          _atPath(packet._fields, ["value", "value", "kind"]) == "judgment" &&
          _atPath(packet._fields, ["value", "value", "value", "kind"]) ==
              "decision")
        resolved[_readInteger(_atPath(packet._fields, ["value", "index"]))] =
            _atPath(packet._fields, ["value", "value", "value", "value"]) !=
                null;
    }
    final chances = rows
        .map((row) => resolved[row.index.value ?? BigInt.zero] == true
            ? _numberProbability(
                _atPath(row._fields, ["answer", "probability"]))
            : null)
        .toList(growable: false);
    return CallResult<Object?, DecisionResult>._(
        (input is InputText || input is InputJson)
            ? rows.single.value.value
            : List.unmodifiable(rows.map((row) => row.value.value)),
        (input is InputText || input is InputJson)
            ? _scalarProbability(chances.single)
            : ListProbability(chances),
        rows,
        call);
  }

  OwnedSession chooseSession(Question question, Input input,
          {Options? options}) =>
      _startSession(question, input, "choose", options: options);
  Future<CallResult<Object?, ChoiceResult>> choose(
      Question question, Input input,
      {Options? options,
      Cancellation? cancellation,
      Stream<FeedItem>? feed}) async {
    final call =
        await _call("choose", question, input, options, cancellation, feed);
    final rows = <ChoiceResult>[];
    var aggregated = false;
    for (final packet in call.packets) {
      if (packet is SessionPacketChooseAggregate) {
        if (!aggregated) rows.clear();
        aggregated = true;
        rows.addAll(packet.value);
      } else if (!aggregated && packet is SessionPacketChooseRow)
        rows.add(packet.value);
    }
    final resolved = <BigInt, bool>{};
    for (final packet in call.packets) {
      if (_atPath(packet._fields, ["kind"]) == "observation" &&
          _atPath(packet._fields, ["value", "kind"]) == "row" &&
          _atPath(packet._fields, ["value", "value", "kind"]) == "judgment" &&
          _atPath(packet._fields, ["value", "value", "value", "kind"]) ==
              "choice")
        resolved[_readInteger(_atPath(packet._fields, ["value", "index"]))] =
            _atPath(packet._fields, ["value", "value", "value", "value"]) !=
                null;
    }
    final chances = rows
        .map((row) => resolved[row.index.value ?? BigInt.zero] == true
            ? _numberProbability(
                (_atPath(row._fields, ["answer", "probabilities"])
                    as Map)[_atPath(row._fields, ["answer", "pick"])])
            : null)
        .toList(growable: false);
    return CallResult<Object?, ChoiceResult>._(
        (input is InputText || input is InputJson)
            ? rows.single.value
            : List.unmodifiable(rows.map((row) => row.value)),
        (input is InputText || input is InputJson)
            ? _scalarProbability(chances.single)
            : ListProbability(chances),
        rows,
        call);
  }

  OwnedSession tagSession(Question question, Input input, {Options? options}) =>
      _startSession(question, input, "tag", options: options);
  Future<CallResult<Object?, TagResult>> tag(Question question, Input input,
      {Options? options,
      Cancellation? cancellation,
      Stream<FeedItem>? feed}) async {
    final call =
        await _call("tag", question, input, options, cancellation, feed);
    final rows = <TagResult>[];
    var aggregated = false;
    for (final packet in call.packets) {
      if (packet is SessionPacketTagAggregate) {
        if (!aggregated) rows.clear();
        aggregated = true;
        rows.addAll(packet.value);
      } else if (!aggregated && packet is SessionPacketTagRow)
        rows.add(packet.value);
    }
    return CallResult<Object?, TagResult>._(
        (input is InputText || input is InputJson)
            ? rows.single.value
            : List.unmodifiable(rows.map((row) => row.value)),
        null,
        rows,
        call);
  }

  OwnedSession scoreSession(Question question, Input input,
          {Options? options}) =>
      _startSession(question, input, "score", options: options);
  Future<CallResult<Object?, ScoreResult>> score(Question question, Input input,
      {Options? options,
      Cancellation? cancellation,
      Stream<FeedItem>? feed}) async {
    final call =
        await _call("score", question, input, options, cancellation, feed);
    final rows = <ScoreResult>[];
    var aggregated = false;
    for (final packet in call.packets) {
      if (packet is SessionPacketScoreAggregate) {
        if (!aggregated) rows.clear();
        aggregated = true;
        rows.addAll(packet.value);
      } else if (!aggregated && packet is SessionPacketScoreRow)
        rows.add(packet.value);
    }
    return CallResult<Object?, ScoreResult>._(
        (input is InputText || input is InputJson)
            ? rows.single.value
            : List.unmodifiable(rows.map((row) => row.value)),
        null,
        rows,
        call);
  }

  OwnedSession filterSession(Question question, Input input,
          {Options? options}) =>
      _startSession(question, input, "filter", options: options);
  Future<CallResult<List<Object?>, FilterResult>> filter(
      Question question, Input input,
      {Options? options,
      Cancellation? cancellation,
      Stream<FeedItem>? feed}) async {
    final call =
        await _call("filter", question, input, options, cancellation, feed);
    final rows = <FilterResult>[];
    var aggregated = false;
    for (final packet in call.packets) {
      if (packet is SessionPacketFilterAggregate) {
        if (!aggregated) rows.clear();
        aggregated = true;
        rows.addAll(packet.value);
      } else if (!aggregated && packet is SessionPacketFilterRow)
        rows.add(packet.value);
    }
    final chances = rows
        .where((row) => row.value == true)
        .map((row) =>
            _numberProbability(_atPath(row._fields, ["answer", "probability"])))
        .toList(growable: false);
    return CallResult<List<Object?>, FilterResult>._(
        List<Object?>.unmodifiable(rows
            .where((row) => row.value == true)
            .map((row) => row.input.value)),
        ListProbability(chances),
        rows,
        call);
  }

  OwnedSession rankSession(Question question, Input input,
          {Options? options}) =>
      _startSession(question, input, "rank", options: options);
  Future<CallResult<List<BigInt>, RankResult>> rank(
      Question question, Input input,
      {Options? options,
      Cancellation? cancellation,
      Stream<FeedItem>? feed}) async {
    final call =
        await _call("rank", question, input, options, cancellation, feed);
    final rows = <RankResult>[];
    var aggregated = false;
    for (final packet in call.packets) {
      if (packet is SessionPacketRankAggregate) {
        if (!aggregated) rows.clear();
        aggregated = true;
        rows.addAll(packet.value);
      }
    }
    rows.sort((a, b) => a.value.compareTo(b.value));
    final chances = rows
        .map((row) =>
            _numberProbability(_atPath(row._fields, ["answer", "probability"])))
        .toList(growable: false);
    return CallResult<List<BigInt>, RankResult>._(
        List<BigInt>.unmodifiable(rows.map((row) =>
            row.index.value ??
            (throw StateError("Rank answer has no input index")))),
        ListProbability(chances),
        rows,
        call);
  }

  OwnedSession findSession(Question question, Input input,
          {Options? options}) =>
      _startSession(question, input, "find", options: options);
  Future<CallResult<Object?, Find>> find(Question question, Input input,
      {Options? options,
      Cancellation? cancellation,
      Stream<FeedItem>? feed}) async {
    final call =
        await _call("find", question, input, options, cancellation, feed);
    final rows = <Find>[];
    var aggregated = false;
    for (final packet in call.packets) {
      if (packet is SessionPacketFindAggregate) {
        if (!aggregated) rows.clear();
        aggregated = true;
        rows.add(packet.value);
      }
    }
    final chances = rows
        .map((row) => _atPath(row._fields, ["index"]) != null
            ? _numberProbability(
                (_atPath(row._fields, ["answer", "probabilities"])
                    as Map)[_atPath(row._fields, ["answer", "pick"])])
            : null)
        .toList(growable: false);
    return CallResult<Object?, Find>._(
        rows.single.value, _scalarProbability(chances.single), rows, call);
  }

  OwnedSession annotateSession(Question question, Input input,
          {Options? options}) =>
      _startSession(question, input, "annotate", options: options);
  Future<CallResult<Object?, Annotation>> annotate(
      Question question, Input input,
      {Options? options,
      Cancellation? cancellation,
      Stream<FeedItem>? feed}) async {
    final call =
        await _call("annotate", question, input, options, cancellation, feed);
    final rows = <Annotation>[];
    var aggregated = false;
    for (final packet in call.packets) {
      if (packet is SessionPacketAnnotateAggregate) {
        if (!aggregated) rows.clear();
        aggregated = true;
        rows.addAll(packet.value);
      } else if (!aggregated && packet is SessionPacketAnnotateRow)
        rows.add(packet.value);
    }
    return CallResult<Object?, Annotation>._(
        (input is InputText || input is InputJson)
            ? rows.single._fields['value']
            : List.unmodifiable(rows.map((row) => row._fields['value'])),
        null,
        rows,
        call);
  }

  OwnedSession recognizeSession(Question question, Input input,
          {Options? options}) =>
      _startSession(question, input, "recognize", options: options);
  Future<CallResult<Object?, Recognition>> recognize(
      Question question, Input input,
      {Options? options,
      Cancellation? cancellation,
      Stream<FeedItem>? feed}) async {
    final call =
        await _call("recognize", question, input, options, cancellation, feed);
    final rows = <Recognition>[];
    var aggregated = false;
    for (final packet in call.packets) {
      if (packet is SessionPacketRecognizeAggregate) {
        if (!aggregated) rows.clear();
        aggregated = true;
        rows.addAll(packet.value);
      }
    }
    final chances =
        rows.map((row) => _recognitionProbability(row)).toList(growable: false);
    return CallResult<Object?, Recognition>._(
        (input is InputText || input is InputJson)
            ? rows.single.value
            : List.unmodifiable(rows.map((row) => row.value)),
        (input is InputText || input is InputJson)
            ? ListProbability(chances.single)
            : GroupProbability(chances),
        rows,
        call);
  }

  OwnedSession relateSession(Question question, Input input,
          {Options? options}) =>
      _startSession(question, input, "relate", options: options);
  Future<CallResult<List<RelatedEntityEdge>, Relation>> relate(
      Question question, Input input,
      {Options? options,
      Cancellation? cancellation,
      Stream<FeedItem>? feed}) async {
    final call =
        await _call("relate", question, input, options, cancellation, feed);
    final rows = <Relation>[];
    var aggregated = false;
    for (final packet in call.packets) {
      if (packet is SessionPacketRelateAggregate) {
        if (!aggregated) rows.clear();
        aggregated = true;
        rows.add(packet.value);
      }
    }
    final chances = rows
        .map((row) => row.value
            .map((edge) => edge.probability.toDouble())
            .toList(growable: false))
        .toList(growable: false);
    return CallResult<List<RelatedEntityEdge>, Relation>._(
        rows.single.value, ListProbability(chances.single), rows, call);
  }
}
