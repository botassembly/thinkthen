import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';
import 'package:thinkthen_dart/thinkthen_complete.dart';
import 'native_encode.dart';

Future<void> main(List<String> args) async {
  final v =
      jsonDecode(File(args[0]).readAsStringSync()) as Map<String, dynamic>;
  Engine? engine;
  try {
    engine = Engine(args[1], settingsJson: args[2]);
    print(jsonEncode(await consume(v, engine, args[1])));
  } on CompleteFailure catch (e) {
    print(jsonEncode({'failure': encodeNative(e.summary)}));
  } finally {
    engine?.close();
  }
}

/// This consumer is also invoked through the separately executed Flutter facade.
Future<Map<String, Object?>> consume(
    Map<String, dynamic> v, CompleteApi engine, String library) async {
  Cancellation? token;
  Batch<Object>? batch;
  Pointer<Void>? thread;
  void Function(Pointer<Void>)? join;
  try {
    final verb = v['verb'] as String;
    final question = v['question'] as Map<String, dynamic>;
    final role = switch (verb) {
      'rank' => question.containsKey('questions')
          ? LoaderRole.rankSet
          : LoaderRole.rank,
      'annotate' => LoaderRole.set,
      'find' => LoaderRole.find,
      'recognize' => LoaderRole.recognize,
      'relate' => LoaderRole.relate,
      _ => LoaderRole.atomic
    };
    var q = v['loader'] != null
        ? switch (v['loader']) {
            'load' || 'file' => Question.file(v['reference']),
            'load_named' || 'named' => Question.named(role, v['reference']),
            'load_reference' ||
            'reference' =>
              Question.reference(role, v['reference']),
            _ => throw StateError('loader')
          }
        : v['question_form'] == 'file'
            ? const Question.file('fixture-question.json')
            : Question.saved(role, v['raw'] ?? jsonEncode(question));
    if (verb == 'find' && question['none'] == true) {
      q = Question.find(_content(question['find']),
          none: true,
          name: v['metadata']?['name'],
          wordingVersion: v['metadata']?['wording_version']);
    }
    final root = Platform.environment['TT_REPO']!;
    final injection = v['operation']?['injection'];
    Source source;
    if (v['paths'] != null || injection == 'recording_read_failure') {
      final paths = (v['paths'] ?? ['target/0429-missing-input']) as List;
      source = Files([
        for (final p in paths)
          v['owned_jsonl'] == true ? p as String : '$root/$p'
      ],
          unit: FileUnit.values[(v['source_unit'] ?? 3) - 1],
          window: v['window'] ?? 0,
          imageReader: v['image_reader'] ?? false);
    } else {
      final images = <Image>[
        for (final p in v['image_paths'] ?? [])
          Image(File('$root/$p').readAsBytesSync(),
              v['media'] == 'image/jpeg' ? 1 : 2)
      ];
      final items = v['items'] as List;
      source = Records([
        for (var i = 0; i < items.length; ++i)
          Record(
              v['image_only'] == true
                  ? null
                  : v['caption_files'] == true
                      ? Content.text(File('caption-$i.txt').readAsStringSync())
                      : v['text'] == true && items[i] is String
                          ? Content.text(items[i])
                          : Content.json(items[i]),
              context:
                  v['context_present'] == true ? _content(v['context']) : null,
              options: [
                for (final name in v['candidate_orders']?[i] ?? []) Choice(name)
              ],
              images: images)
      ]);
    }
    if (injection == 'cancel_token' || v['held_cancel'] == true) {
      token = engine.cancellation();
      if (injection == 'cancel_token') token.fire();
    }
    if (v['held_cancel'] == true) {
      final helper =
          DynamicLibrary.open(Platform.environment['TT_CANCEL_HELPER']!);
      final start = helper.lookupFunction<
          Pointer<Void> Function(Pointer<Void>, Pointer<Utf8>, Pointer<Utf8>),
          Pointer<Void> Function(
              Pointer<Void>, Pointer<Utf8>, Pointer<Utf8>)>('cancel_reader');
      join = helper.lookupFunction<Void Function(Pointer<Void>),
          void Function(Pointer<Void>)>('cancel_join');
      final path = library.toNativeUtf8();
      final signal = '${Platform.environment['HOME']}/cancel'.toNativeUtf8();
      try {
        thread = start(token!.nativeHandle, path, signal);
      } finally {
        calloc.free(path);
        calloc.free(signal);
      }
    }
    final controls = Controls(
        deadlineMs: injection == 'expired_deadline' ? 0 : -1,
        cancel: token,
        context:
            v['shared_context'] == null ? null : _content(v['shared_context']),
        attempts: true);
    if (v['incremental'] == true) {
      batch = switch (verb) {
        'decide' => engine.decideBatch(q, source, controls: controls),
        'choose' => engine.chooseBatch(q, source, controls: controls),
        'tag' => engine.tagBatch(q, source, controls: controls),
        'score' => engine.scoreBatch(q, source, controls: controls),
        'filter' => engine.filterBatch(q, source, controls: controls),
        'annotate' => engine.annotateBatch(q, source, controls: controls),
        _ => throw StateError('no aggregate batch')
      };
      final completed = <Object?>[];
      try {
        while (true) {
          final r = batch.next();
          if (r == null) break;
          completed.add(encodeNative(r));
        }
        return {'result': encodeNative(batch.facts()), 'completed': completed};
      } on CompleteFailure catch (e) {
        return {'failure': encodeNative(e.summary), 'completed': completed};
      }
    }
    final result = switch (verb) {
      'decide' => engine.decide(q, source, controls: controls),
      'choose' => engine.choose(q, source, controls: controls),
      'tag' => engine.tag(q, source, controls: controls),
      'score' => engine.score(q, source, controls: controls),
      'filter' => engine.filter(q, source, controls: controls),
      'rank' => engine.rank(q, source, controls: controls),
      'find' => engine.find(q, source, controls: controls),
      'annotate' => engine.annotate(q, source, controls: controls),
      'recognize' => engine.recognize(q, source, controls: controls),
      'relate' => engine.relate(q, source, controls: controls),
      _ => throw StateError('unknown function')
    };
    return {'result': encodeNative(result)};
  } on CompleteFailure catch (e) {
    return {'failure': encodeNative(e.summary)};
  } finally {
    if (thread != null) join!(thread);
    batch?.close();
    token?.close();
  }
}

Content _content(Object? v) => v is String ? Content.text(v) : Content.json(v);
