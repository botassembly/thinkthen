part of 'engine.dart';

extension _Execution on Engine {
  DecideViewView _readDecide(Pointer<Void> r, int at) {
    final p = calloc<CDecideViewView>();
    try {
      if (_api.thinkthen_result_decide(r, at, p) != 0)
        throw StateError("native decide view");
      return DecideViewView.copy(p.ref);
    } finally {
      calloc.free(p);
    }
  }

  ChooseViewView _readChoose(Pointer<Void> r, int at) {
    final p = calloc<CChooseViewView>();
    try {
      if (_api.thinkthen_result_choose(r, at, p) != 0)
        throw StateError("native choose view");
      return ChooseViewView.copy(p.ref);
    } finally {
      calloc.free(p);
    }
  }

  TagViewView _readTag(Pointer<Void> r, int at) {
    final p = calloc<CTagViewView>();
    try {
      if (_api.thinkthen_result_tag(r, at, p) != 0)
        throw StateError("native tag view");
      return TagViewView.copy(p.ref);
    } finally {
      calloc.free(p);
    }
  }

  ScoreViewView _readScore(Pointer<Void> r, int at) {
    final p = calloc<CScoreViewView>();
    try {
      if (_api.thinkthen_result_score(r, at, p) != 0)
        throw StateError("native score view");
      return ScoreViewView.copy(p.ref);
    } finally {
      calloc.free(p);
    }
  }

  FilterViewView _readFilter(Pointer<Void> r, int at) {
    final p = calloc<CFilterViewView>();
    try {
      if (_api.thinkthen_result_filter(r, at, p) != 0)
        throw StateError("native filter view");
      return FilterViewView.copy(p.ref);
    } finally {
      calloc.free(p);
    }
  }

  RankViewView _readRank(Pointer<Void> r, int at) {
    final p = calloc<CRankViewView>();
    try {
      if (_api.thinkthen_result_rank(r, at, p) != 0)
        throw StateError("native rank view");
      return RankViewView.copy(p.ref);
    } finally {
      calloc.free(p);
    }
  }

  FindViewView _readFind(Pointer<Void> r, int at) {
    final p = calloc<CFindViewView>();
    try {
      if (_api.thinkthen_result_find(r, at, p) != 0)
        throw StateError("native find view");
      return FindViewView.copy(p.ref);
    } finally {
      calloc.free(p);
    }
  }

  AnnotateViewView _readAnnotate(Pointer<Void> r, int at) {
    final p = calloc<CAnnotateViewView>();
    try {
      if (_api.thinkthen_result_annotate(r, at, p) != 0)
        throw StateError("native annotate view");
      return AnnotateViewView.copy(p.ref);
    } finally {
      calloc.free(p);
    }
  }

  RecognizeViewView _readRecognize(Pointer<Void> r, int at) {
    final p = calloc<CRecognizeViewView>();
    try {
      if (_api.thinkthen_result_recognize(r, at, p) != 0)
        throw StateError("native recognize view");
      return RecognizeViewView.copy(p.ref);
    } finally {
      calloc.free(p);
    }
  }

  RelateViewView _readRelate(Pointer<Void> r, int at) {
    final p = calloc<CRelateViewView>();
    try {
      if (_api.thinkthen_result_relate(r, at, p) != 0)
        throw StateError("native relate view");
      return RelateViewView.copy(p.ref);
    } finally {
      calloc.free(p);
    }
  }

  SummaryView _summary(Pointer<Void> r) {
    final p = calloc<CSummaryView>();
    try {
      if (_api.thinkthen_result_summary(r, p) != 0)
        throw StateError("native summary view");
      return SummaryView.copy(p.ref);
    } finally {
      calloc.free(p);
    }
  }

  DetailsView _details(Pointer<Void> r, int at) {
    final p = calloc<CDetailsView>();
    try {
      if (_api.thinkthen_result_details(r, at, p) != 0)
        throw StateError("native details view");
      return DetailsView.copy(p.ref);
    } finally {
      calloc.free(p);
    }
  }

  QuestionAuthorView _question_author(Pointer<Void> r, int at) {
    final p = calloc<CQuestionAuthorView>();
    try {
      if (_api.thinkthen_result_question_author(r, at, p) != 0)
        throw StateError("native question_author view");
      return QuestionAuthorView.copy(p.ref);
    } finally {
      calloc.free(p);
    }
  }

  ObservationView _observation(Pointer<Void> r, int at) {
    final p = calloc<CObservationView>();
    try {
      if (_api.thinkthen_result_observation(r, at, p) != 0)
        throw StateError("native observation view");
      return ObservationView.copy(p.ref);
    } finally {
      calloc.free(p);
    }
  }

  DetailsView _observation_details(Pointer<Void> r, int at) {
    final p = calloc<CDetailsView>();
    try {
      if (_api.thinkthen_result_observation_details(r, at, p) != 0)
        throw StateError("native observation_details view");
      return DetailsView.copy(p.ref);
    } finally {
      calloc.free(p);
    }
  }

  QuestionAuthorView _observation_author(Pointer<Void> r, int at) {
    final p = calloc<CQuestionAuthorView>();
    try {
      if (_api.thinkthen_result_observation_author(r, at, p) != 0)
        throw StateError("native observation_author view");
      return QuestionAuthorView.copy(p.ref);
    } finally {
      calloc.free(p);
    }
  }

  SourceRecognitionView _source_recognition(Pointer<Void> r, int at) {
    final p = calloc<CSourceRecognitionView>();
    try {
      if (_api.thinkthen_result_source_recognition(r, at, p) != 0)
        throw StateError("native source_recognition view");
      return SourceRecognitionView.copy(p.ref);
    } finally {
      calloc.free(p);
    }
  }

  SourceRelationsView _source_relations(Pointer<Void> r, int at) {
    final p = calloc<CSourceRelationsView>();
    try {
      if (_api.thinkthen_result_source_relations(r, at, p) != 0)
        throw StateError("native source_relations view");
      return SourceRelationsView.copy(p.ref);
    } finally {
      calloc.free(p);
    }
  }

  CompleteResult<T> _copy<T>(
      Pointer<Void> r, String verb, T Function(Pointer<Void>, int) read) {
    final s = _summary(r);
    final rows = <T>[];
    final details = <DetailsView>[];
    final authors = <QuestionAuthorView>[];
    final members = <List<QuestionAuthorView>>[];
    final ranks = <List<RankViewView>>[];
    final recognitions = <SourceRecognitionView?>[];
    final relations = <SourceRelationsView?>[];
    for (var i = 0; i < s.count; ++i) {
      final row = read(r, i);
      rows.add(row);
      details.add(_details(r, i));
      authors.add(_question_author(r, i));
      final ma = <QuestionAuthorView>[];
      if (row is AnnotateViewView) {
        for (var j = 0; j < row.answers.len; ++j) {
          final p = calloc<CQuestionAuthorView>();
          try {
            if (_api.thinkthen_result_member_author(r, i, j, p) != 0)
              throw StateError('native member author');
            ma.add(QuestionAuthorView.copy(p.ref));
          } finally {
            calloc.free(p);
          }
        }
      }
      members.add(ma);
      final rm = <RankViewView>[];
      if (verb == 'rank') {
        final n = calloc<Size>();
        try {
          if (_api.thinkthen_result_rank_member_count(r, i, n) != 0)
            throw StateError('native rank member count');
          for (var j = 0; j < n.value; ++j) {
            final p = calloc<CRankViewView>();
            try {
              if (_api.thinkthen_result_rank_member(r, i, j, p) != 0)
                throw StateError('native rank member');
              rm.add(RankViewView.copy(p.ref));
            } finally {
              calloc.free(p);
            }
          }
        } finally {
          calloc.free(n);
        }
      }
      ranks.add(rm);
      recognitions.add(verb == 'recognize' ? _source_recognition(r, i) : null);
      relations.add(verb == 'relate' ? _source_relations(r, i) : null);
    }
    return CompleteResult(
        s,
        rows,
        List.generate(s.observation_count, (i) => _observation(r, i)),
        details,
        authors,
        members,
        ranks,
        List.generate(s.observation_count, (i) => _observation_details(r, i)),
        List.generate(s.observation_count, (i) => _observation_author(r, i)),
        recognitions,
        relations);
  }

  CompleteResult<T> _execute<T>(
      String verb,
      Question question,
      Source input,
      Controls controls,
      int Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
              Pointer<CControlsView>, Pointer<Pointer<Void>>)
          call,
      T Function(Pointer<Void>, int) read) {
    _live();
    final scope = _Scope();
    final images = <Pointer<Void>>[];
    Pointer<Void> q = nullptr, s = nullptr;
    final out = scope.add(calloc<Pointer<Void>>());
    try {
      q = _question(question, scope);
      s = _source(input, scope, images);
      final c = _controls(controls, scope);
      _check(call(_engine, q, s, c, out));
      return _copy(out.value, verb, read);
    } finally {
      _api.thinkthen_result_free(out.value);
      _api.thinkthen_source_free(s);
      _api.thinkthen_question_free(q);
      for (final im in images) {
        _api.thinkthen_image_free(im);
      }
      scope.close();
    }
  }

  Batch<T> _start<T>(
      String verb,
      Question question,
      Source input,
      Controls controls,
      int Function(Pointer<Void>, Pointer<Void>, Pointer<Void>,
              Pointer<CControlsView>, Pointer<Pointer<Void>>)
          call,
      T Function(Pointer<Void>, int) read) {
    _live();
    final scope = _Scope();
    final images = <Pointer<Void>>[];
    Pointer<Void> q = nullptr, s = nullptr;
    final out = scope.add(calloc<Pointer<Void>>());
    try {
      q = _question(question, scope);
      s = _source(input, scope, images);
      _check(call(_engine, q, s, _controls(controls, scope), out));
      ++_batches;
      return Batch._(this, out.value, verb, read);
    } finally {
      _api.thinkthen_source_free(s);
      _api.thinkthen_question_free(q);
      for (final im in images) {
        _api.thinkthen_image_free(im);
      }
      scope.close();
    }
  }
}
