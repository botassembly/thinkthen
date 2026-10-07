import 'dart:convert';
import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'abi.dart';
import 'functions.dart';
import 'views.dart';
import 'input.dart';
import 'result.dart';
import 'question.dart';
part 'execution.dart';
part 'batch.dart';
part 'question_adapter.dart';

final class _Scope {
  final List<Pointer> owned = [];
  Pointer<T> add<T extends NativeType>(Pointer<T> p) {
    owned.add(p);
    return p;
  }

  void close() {
    for (final p in owned.reversed) {
      calloc.free(p);
    }
  }

  CStringView string(String value) {
    final bytes = utf8.encode(value);
    final p = add(calloc<Uint8>(bytes.isEmpty ? 1 : bytes.length));
    p.asTypedList(bytes.length).setAll(0, bytes);
    final s = add(calloc<CStringView>()).ref;
    s.data = p;
    s.len = bytes.length;
    return s;
  }

  CContentView content(Content v) {
    final c = add(calloc<CContentView>()).ref;
    c.kind = v.kind;
    c.data = string(v.bytes);
    return c;
  }

  COptionalContentView optional(Content? v) {
    final c = add(calloc<COptionalContentView>()).ref;
    if (v != null) {
      c.present = 1;
      c.value = content(v);
    }
    return c;
  }
}

final class Cancellation {
  final NativeApi _api;
  final Pointer<Void> _pointer;
  bool _closed = false;
  Cancellation._(this._api, this._pointer);
  Pointer<Void> get _live {
    if (_closed) throw StateError('token closed');
    return _pointer;
  }

  /// Borrowed token for native interop; valid until close.
  Pointer<Void> get nativeHandle => _live;
  void fire() => _api.thinkthen_cancel(_live);
  void close() {
    if (!_closed) {
      _api.thinkthen_cancel_token_free(_pointer);
      _closed = true;
    }
  }
}

/// One owned native engine; results contain no borrowed memory.
final class Engine {
  final NativeApi _api;
  late final Pointer<Void> _engine;
  final String _surface;
  bool _closed = false;
  int _batches = 0;
  Engine(String absoluteLibrary, {String? settingsJson})
      : this._(absoluteLibrary, settingsJson, 'dart');

  /// Used by the Flutter facade to identify its actual public door.
  Engine.flutter(String absoluteLibrary, {String? settingsJson})
      : this._(absoluteLibrary, settingsJson, 'flutter');
  Engine._(String absoluteLibrary, String? settingsJson, this._surface)
      : _api = NativeApi(absoluteLibrary) {
    if (!absoluteLibrary.startsWith('/'))
      throw ArgumentError('absolute library path required');
    if (settingsJson == null) {
      _engine = _api.thinkthen_engine_new();
    } else {
      if (settingsJson.contains('\u0000')) throw ArgumentError('interior NUL');
      final p = settingsJson.toNativeUtf8();
      try {
        _engine = _api.thinkthen_engine_new_with(p.cast());
      } finally {
        calloc.free(p);
      }
    }
    if (_engine.address == 0) _fail();
  }
  void _live() {
    if (_closed) throw StateError('engine closed');
  }

  void close() {
    if (_batches != 0) throw StateError('close batches before engine');
    if (!_closed) {
      _api.thinkthen_engine_free(_engine);
      _closed = true;
    }
  }

  Cancellation cancellation() {
    _live();
    return Cancellation._(_api, _api.thinkthen_cancel_token_new());
  }

  void _check(int code) {
    if (code != 0) _fail();
  }

  Never _fail() {
    final out = calloc<Pointer<Void>>();
    try {
      if (_api.thinkthen_error_complete(_engine, out) != 0 ||
          out.value.address == 0)
        throw StateError('native error snapshot unavailable');
      final s = calloc<CSummaryView>();
      try {
        if (_api.thinkthen_result_summary(out.value, s) != 0)
          throw StateError('native failure summary');
        throw CompleteFailure(SummaryView.copy(s.ref));
      } finally {
        calloc.free(s);
        _api.thinkthen_result_free(out.value);
      }
    } finally {
      calloc.free(out);
    }
  }

  Pointer<Void> _question(Question v, _Scope scope) {
    if (v.spec != null) return _constructed(v.spec!, scope);
    final out = scope.add(calloc<Pointer<Void>>());
    final value = scope.string(v.value);
    if (v.method == 'new') {
      final spec = scope.add(calloc<CQuestionSpecView>());
      spec.ref.kind = 7;
      spec.ref.text = scope.content(v.text!);
      spec.ref.none = v.none ? 1 : 0;
      final author = scope.add(calloc<CQuestionAuthorView>());
      if (v.name != null) {
        author.ref.name.present = 1;
        author.ref.name.value = scope.string(v.name!);
      }
      if (v.wordingVersion != null) {
        author.ref.wording_version.present = 1;
        author.ref.wording_version.value = v.wordingVersion!;
      }
      _check(_api.thinkthen_question_new_authored(_engine, spec, author, out));
      return out.value;
    }
    final role = v.role.index + 1;
    _check(switch (v.method) {
      'parse' => _api.thinkthen_question_parse(_engine, role, value, out),
      'load' => _api.thinkthen_question_load(_engine, value, out),
      'load_named' =>
        _api.thinkthen_question_load_named(_engine, role, value, out),
      'load_reference' =>
        _api.thinkthen_question_load_reference(_engine, role, value, out),
      _ => throw StateError('question loader')
    });
    return out.value;
  }

  Pointer<Void> _source(Source v, _Scope scope, List<Pointer<Void>> images) {
    final out = scope.add(calloc<Pointer<Void>>());
    if (v is Files) {
      final paths =
          scope.add(calloc<CStringView>(v.paths.isEmpty ? 1 : v.paths.length));
      for (var i = 0; i < v.paths.length; ++i) {
        paths[i] = scope.string(v.paths[i]);
      }
      final spec = scope.add(calloc<CSourceSpecView>());
      spec.ref.paths.data = paths;
      spec.ref.paths.len = v.paths.length;
      spec.ref.unit = v.unit.index + 1;
      spec.ref.window = v.window;
      _check(v.imageReader
          ? _api.thinkthen_source_image_files(_engine, spec, out)
          : _api.thinkthen_source_files(_engine, spec, out));
    } else if (v is Records) {
      final records = scope
          .add(calloc<CRecordView>(v.records.isEmpty ? 1 : v.records.length));
      for (var i = 0; i < v.records.length; ++i) {
        final r = v.records[i];
        records[i].original = scope.optional(r.original);
        records[i].context = scope.optional(r.context);
        final opts = scope
            .add(calloc<CChoiceView>(r.options.isEmpty ? 1 : r.options.length));
        for (var j = 0; j < r.options.length; ++j) {
          final c = r.options[j];
          opts[j].name = scope.string(c.name);
          opts[j].description = scope.optional(c.description);
          if (c.weight != null) {
            opts[j].weight.present = 1;
            opts[j].weight.value = c.weight!;
          }
        }
        records[i].options.data = opts;
        records[i].options.len = r.options.length;
        final ptrs = scope
            .add(calloc<Pointer<Void>>(r.images.isEmpty ? 1 : r.images.length));
        for (var j = 0; j < r.images.length; ++j) {
          final im = r.images[j];
          final bytes =
              scope.add(calloc<Uint8>(im.bytes.isEmpty ? 1 : im.bytes.length));
          bytes.asTypedList(im.bytes.length).setAll(0, im.bytes);
          final name = scope.add(calloc<COptionalStringView>()).ref;
          if (im.filename != null) {
            name.present = 1;
            name.value = scope.string(im.filename!);
          }
          final handle = scope.add(calloc<Pointer<Void>>());
          _check(_api.thinkthen_image_clone(
              _engine, bytes, im.bytes.length, im.media, name, handle));
          images.add(handle.value);
          ptrs[j] = handle.value;
        }
        records[i].images.data = ptrs;
        records[i].images.len = r.images.length;
      }
      _check(_api.thinkthen_source_records(
          _engine, records, v.records.length, out));
    }
    return out.value;
  }

  Pointer<CControlsView> _controls(Controls v, _Scope scope) {
    final c = scope.add(calloc<CControlsView>());
    c.ref.deadline_ms = v.deadlineMs;
    c.ref.cancel = v.cancel?._live ?? nullptr;
    c.ref.context = scope.optional(v.context);
    if (v.batch != null) {
      c.ref.batch.present = 1;
      c.ref.batch.value = v.batch!;
    }
    c.ref.batch_max = v.batchMax ? 1 : 0;
    c.ref.attempts = v.attempts ? 1 : 0;
    c.ref.surface = scope.string(_surface);
    return c;
  }

  CompleteResult<DecideViewView> decide(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _execute("decide", question, input, controls,
          _api.thinkthen_decide_complete, _readDecide);
  CompleteResult<ChooseViewView> choose(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _execute("choose", question, input, controls,
          _api.thinkthen_choose_complete, _readChoose);
  CompleteResult<TagViewView> tag(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _execute("tag", question, input, controls, _api.thinkthen_tag_complete,
          _readTag);
  CompleteResult<ScoreViewView> score(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _execute("score", question, input, controls,
          _api.thinkthen_score_complete, _readScore);
  CompleteResult<FilterViewView> filter(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _execute("filter", question, input, controls,
          _api.thinkthen_filter_complete, _readFilter);
  CompleteResult<RankViewView> rank(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _execute("rank", question, input, controls, _api.thinkthen_rank_complete,
          _readRank);
  CompleteResult<FindViewView> find(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _execute("find", question, input, controls, _api.thinkthen_find_complete,
          _readFind);
  CompleteResult<AnnotateViewView> annotate(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _execute("annotate", question, input, controls,
          _api.thinkthen_annotate_complete, _readAnnotate);
  CompleteResult<RecognizeViewView> recognize(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _execute("recognize", question, input, controls,
          _api.thinkthen_recognize_complete, _readRecognize);
  CompleteResult<RelateViewView> relate(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _execute("relate", question, input, controls,
          _api.thinkthen_relate_complete, _readRelate);
  Batch<DecideViewView> decideBatch(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _start("decide", question, input, controls,
          _api.thinkthen_decide_batch_start, _readDecide);
  Batch<ChooseViewView> chooseBatch(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _start("choose", question, input, controls,
          _api.thinkthen_choose_batch_start, _readChoose);
  Batch<TagViewView> tagBatch(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _start("tag", question, input, controls, _api.thinkthen_tag_batch_start,
          _readTag);
  Batch<ScoreViewView> scoreBatch(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _start("score", question, input, controls,
          _api.thinkthen_score_batch_start, _readScore);
  Batch<FilterViewView> filterBatch(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _start("filter", question, input, controls,
          _api.thinkthen_filter_batch_start, _readFilter);
  Batch<AnnotateViewView> annotateBatch(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _start("annotate", question, input, controls,
          _api.thinkthen_annotate_batch_start, _readAnnotate);
}
