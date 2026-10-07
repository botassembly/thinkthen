import 'dart:convert';
import 'engine.dart' show Cancellation;
import 'question.dart';

/// Explicit text or arbitrary caller JSON. Neither guesses paths or images.
final class Content {
  final int kind;
  final String bytes;
  const Content.text(this.bytes) : kind = 1;
  Content.json(Object? value)
      : kind = 2,
        bytes = jsonEncode(value);
}

enum LoaderRole {
  atomic,
  set,
  dynamicChoose,
  recognize,
  relate,
  rank,
  rankSet,
  find
}

final class Question {
  const Question.spec(this.spec)
      : method = 'spec',
        role = LoaderRole.atomic,
        value = '',
        text = null,
        none = false,
        name = null,
        wordingVersion = null;
  const Question.find(this.text,
      {this.none = false, this.name, this.wordingVersion})
      : spec = null,
        method = "new",
        role = LoaderRole.find,
        value = "";
  final String method, value;
  final LoaderRole role;
  final QuestionSpec? spec;
  final Content? text;
  final bool none;
  final String? name;
  final int? wordingVersion;
  const Question.saved(this.role, this.value)
      : spec = null,
        method = 'parse',
        text = null,
        none = false,
        name = null,
        wordingVersion = null;
  const Question.file(this.value)
      : spec = null,
        method = 'load',
        role = LoaderRole.atomic,
        text = null,
        none = false,
        name = null,
        wordingVersion = null;
  const Question.named(this.role, this.value)
      : spec = null,
        method = 'load_named',
        text = null,
        none = false,
        name = null,
        wordingVersion = null;
  const Question.reference(this.role, this.value)
      : spec = null,
        method = 'load_reference',
        text = null,
        none = false,
        name = null,
        wordingVersion = null;
}

final class Choice {
  final String name;
  final Content? description;
  final double? weight;
  const Choice(this.name, {this.description, this.weight});
}

final class Image {
  final List<int> bytes;
  final int media;
  final String? filename;
  Image(List<int> bytes, this.media, {this.filename})
      : bytes = List.unmodifiable(bytes) {
    if (bytes.any((byte) => byte < 0 || byte > 255))
      throw ArgumentError('invalid image byte');
  }
}

final class Record {
  final Content? original, context;
  final List<Choice> options;
  final List<Image> images;
  Record(this.original,
      {this.context,
      List<Choice> options = const [],
      List<Image> images = const []})
      : options = List.unmodifiable(options),
        images = List.unmodifiable(images);
}

sealed class Source {}

final class Records extends Source {
  final List<Record> records;
  Records(List<Record> records) : records = List.unmodifiable(records);
}

enum FileUnit { line, window, file, image, jsonl }

final class Files extends Source {
  final List<String> paths;
  final FileUnit unit;
  final int window;
  final bool imageReader;
  Files(List<String> paths,
      {this.unit = FileUnit.line, this.window = 0, this.imageReader = false})
      : paths = List.unmodifiable(paths);
}

final class Controls {
  final int deadlineMs;
  final Cancellation? cancel;
  final Content? context;
  final int? batch;
  final bool batchMax, attempts;
  const Controls(
      {this.deadlineMs = -1,
      this.cancel,
      this.context,
      this.batch,
      this.batchMax = false,
      this.attempts = false});
}
