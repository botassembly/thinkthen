part of 'models.dart';

final class Files extends Carrier implements Selection {
  final List<String> paths;
  final UnitKind unit;
  final Optional<int> window;
  final Optional<MediaKind> media;
  const Files({
    required this.paths,
    required this.unit,
    this.window = const Optional.absent(),
    this.media = const Optional.absent(),
  });
  factory Files.fromJson(Object? value) {
    final v = readObject(value);
    verify("Files", v, ["paths", "unit", "window", "media"], ["paths", "unit"]);
    return Files(
      paths: readList(v["paths"], (v) => readString(v)),
      unit: readEnum(v["unit"], UnitKind.values),
      window: v.containsKey("window")
          ? Optional.present(readInt(v["window"], 1, maxInteger))
          : const Optional.absent(),
      media: v.containsKey("media")
          ? Optional.present(readEnum(v["media"], MediaKind.values))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "paths": project(paths),
        "unit": project(unit),
        if (window.present) "window": project(window.value),
        if (media.present) "media": project(media.value),
      };
}

final class TextInput extends Carrier implements Selection {
  final Authored text;
  const TextInput({
    required this.text,
  });
  factory TextInput.fromJson(Object? value) {
    final v = readObject(value);
    verify("TextInput", v, ["text"], ["text"]);
    return TextInput(
      text: readText(v["text"]),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "text": project(text),
      };
}

final class RecordInput extends Carrier implements Selection {
  final List<Object?> records;
  final Optional<Object?> context;
  const RecordInput({
    required this.records,
    this.context = const Optional.absent(),
  });
  factory RecordInput.fromJson(Object? value) {
    final v = readObject(value);
    verify("RecordInput", v, ["records", "context"], ["records"]);
    return RecordInput(
      records: readList(v["records"], (v) => readJson(v)),
      context: v.containsKey("context")
          ? Optional.present(readJson(v["context"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "records": project(records),
        if (context.present) "context": project(context.value),
      };
}

final class CandidateInput extends Carrier implements Selection {
  final List<Object?> units;
  final Optional<Object?> context;
  const CandidateInput({
    required this.units,
    this.context = const Optional.absent(),
  });
  factory CandidateInput.fromJson(Object? value) {
    final v = readObject(value);
    verify("CandidateInput", v, ["units", "context"], ["units"]);
    return CandidateInput(
      units: readList(v["units"], (v) => readJson(v)),
      context: v.containsKey("context")
          ? Optional.present(readJson(v["context"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "units": project(units),
        if (context.present) "context": project(context.value),
      };
}

final class ImageBytes extends Carrier {
  final List<int> data;
  final MediaType media;
  final Optional<String> name;
  const ImageBytes({
    required this.data,
    required this.media,
    this.name = const Optional.absent(),
  });
  factory ImageBytes.fromJson(Object? value) {
    final v = readObject(value);
    verify("ImageBytes", v, ["data", "media", "name"], ["data", "media"]);
    return ImageBytes(
      data: readBytes(v["data"]),
      media: readEnum(v["media"], MediaType.values),
      name: v.containsKey("name")
          ? Optional.present(readString(v["name"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "data": project(data),
        "media": project(media),
        if (name.present) "name": project(name.value),
      };
}

final class ImageInput extends Carrier implements Selection {
  final List<ImageBytes> images;
  final Optional<Object?> text;
  const ImageInput({
    required this.images,
    this.text = const Optional.absent(),
  });
  factory ImageInput.fromJson(Object? value) {
    final v = readObject(value);
    verify("ImageInput", v, ["images", "text"], ["images"]);
    return ImageInput(
      images: readList(v["images"], (v) => ImageBytes.fromJson(v)),
      text: v.containsKey("text")
          ? Optional.present(readJson(v["text"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "images": project(images),
        if (text.present) "text": project(text.value),
      };
}

final class Controls extends Carrier {
  final Optional<Batch> batch;
  final Optional<Object?> context;
  final Optional<Pointers> on;
  final Optional<Threshold> threshold;
  final Optional<int> top;
  final Optional<bool> none;
  final Optional<String> model;
  final Optional<bool> attempts;
  final Optional<int> deadline_ms;
  const Controls({
    this.batch = const Optional.absent(),
    this.context = const Optional.absent(),
    this.on = const Optional.absent(),
    this.threshold = const Optional.absent(),
    this.top = const Optional.absent(),
    this.none = const Optional.absent(),
    this.model = const Optional.absent(),
    this.attempts = const Optional.absent(),
    this.deadline_ms = const Optional.absent(),
  });
  factory Controls.fromJson(Object? value) {
    final v = readObject(value);
    verify("Controls", v, [
      "batch",
      "context",
      "on",
      "threshold",
      "top",
      "none",
      "model",
      "attempts",
      "deadline_ms"
    ], []);
    return Controls(
      batch: v.containsKey("batch")
          ? Optional.present(Batch.fromJson(v["batch"]))
          : const Optional.absent(),
      context: v.containsKey("context")
          ? Optional.present(readJson(v["context"]))
          : const Optional.absent(),
      on: v.containsKey("on")
          ? Optional.present(Pointers.fromJson(v["on"]))
          : const Optional.absent(),
      threshold: v.containsKey("threshold")
          ? Optional.present(Threshold.fromJson(v["threshold"]))
          : const Optional.absent(),
      top: v.containsKey("top")
          ? Optional.present(readInt(v["top"], 1, maxInteger))
          : const Optional.absent(),
      none: v.containsKey("none")
          ? Optional.present(readBool(v["none"]))
          : const Optional.absent(),
      model: v.containsKey("model")
          ? Optional.present(readString(v["model"]))
          : const Optional.absent(),
      attempts: v.containsKey("attempts")
          ? Optional.present(readBool(v["attempts"]))
          : const Optional.absent(),
      deadline_ms: v.containsKey("deadline_ms")
          ? Optional.present(readInt(v["deadline_ms"], -1, maxInteger))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        if (batch.present) "batch": project(batch.value),
        if (context.present) "context": project(context.value),
        if (on.present) "on": project(on.value),
        if (threshold.present) "threshold": project(threshold.value),
        if (top.present) "top": project(top.value),
        if (none.present) "none": project(none.value),
        if (model.present) "model": project(model.value),
        if (attempts.present) "attempts": project(attempts.value),
        if (deadline_ms.present) "deadline_ms": project(deadline_ms.value),
      };
}
