part of 'models.dart';

final class Position extends Carrier {
  final Optional<String?> file;
  final Optional<int> first;
  final Optional<int> last;
  final Optional<List<String>> images;
  const Position({
    this.file = const Optional.absent(),
    this.first = const Optional.absent(),
    this.last = const Optional.absent(),
    this.images = const Optional.absent(),
  });
  factory Position.fromJson(Object? value) {
    final v = readObject(value);
    verify("Position", v, ["file", "first", "last", "images"], []);
    return Position(
      file: v.containsKey("file")
          ? Optional.present((v["file"] == null ? null : readString(v["file"])))
          : const Optional.absent(),
      first: v.containsKey("first")
          ? Optional.present(readInt(v["first"], 1, maxInteger))
          : const Optional.absent(),
      last: v.containsKey("last")
          ? Optional.present(readInt(v["last"], 1, maxInteger))
          : const Optional.absent(),
      images: v.containsKey("images")
          ? Optional.present(readList(v["images"], (v) => readString(v)))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        if (file.present) "file": project(file.value),
        if (first.present) "first": project(first.value),
        if (last.present) "last": project(last.value),
        if (images.present) "images": project(images.value),
      };
}

final class Usage extends Carrier {
  final int input_tokens;
  final int output_tokens;
  const Usage({
    required this.input_tokens,
    required this.output_tokens,
  });
  factory Usage.fromJson(Object? value) {
    final v = readObject(value);
    verify("Usage", v, ["input_tokens", "output_tokens"],
        ["input_tokens", "output_tokens"]);
    return Usage(
      input_tokens: readInt(v["input_tokens"], 0, maxInteger),
      output_tokens: readInt(v["output_tokens"], 0, maxInteger),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "input_tokens": project(input_tokens),
        "output_tokens": project(output_tokens),
      };
}

final class ProfileWarning extends Carrier {
  final String tuned_for;
  final String running;
  const ProfileWarning({
    required this.tuned_for,
    required this.running,
  });
  factory ProfileWarning.fromJson(Object? value) {
    final v = readObject(value);
    verify("ProfileWarning", v, ["tuned_for", "running"],
        ["tuned_for", "running"]);
    return ProfileWarning(
      tuned_for: readString(v["tuned_for"]),
      running: readString(v["running"]),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "tuned_for": project(tuned_for),
        "running": project(running),
      };
}

final class BatchWarning extends Carrier {
  final Batch tuned_for;
  final Batch running;
  const BatchWarning({
    required this.tuned_for,
    required this.running,
  });
  factory BatchWarning.fromJson(Object? value) {
    final v = readObject(value);
    verify(
        "BatchWarning", v, ["tuned_for", "running"], ["tuned_for", "running"]);
    return BatchWarning(
      tuned_for: Batch.fromJson(v["tuned_for"]),
      running: Batch.fromJson(v["running"]),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "tuned_for": project(tuned_for),
        "running": project(running),
      };
}

final class Attempt extends Carrier {
  final int ordinal;
  final Digest request_sha256;
  final int wall_ms;
  final OutcomeKind outcome;
  final SdkRequestId sdk_request_id;
  final Optional<int> status;
  final Optional<int> server_ms;
  final Optional<String> request_id;
  const Attempt({
    required this.ordinal,
    required this.request_sha256,
    required this.wall_ms,
    required this.outcome,
    required this.sdk_request_id,
    this.status = const Optional.absent(),
    this.server_ms = const Optional.absent(),
    this.request_id = const Optional.absent(),
  });
  factory Attempt.fromJson(Object? value) {
    final v = readObject(value);
    verify("Attempt", v, [
      "ordinal",
      "request_sha256",
      "wall_ms",
      "outcome",
      "sdk_request_id",
      "status",
      "server_ms",
      "request_id"
    ], [
      "ordinal",
      "request_sha256",
      "wall_ms",
      "outcome",
      "sdk_request_id"
    ]);
    return Attempt(
      ordinal: readInt(v["ordinal"], 1, maxInteger),
      request_sha256: Digest(readString(v["request_sha256"])),
      wall_ms: readInt(v["wall_ms"], 0, maxInteger),
      outcome: readEnum(v["outcome"], OutcomeKind.values),
      sdk_request_id: SdkRequestId(readString(v["sdk_request_id"])),
      status: v.containsKey("status")
          ? Optional.present(readInt(v["status"], 100, 599))
          : const Optional.absent(),
      server_ms: v.containsKey("server_ms")
          ? Optional.present(readInt(v["server_ms"], 0, maxInteger))
          : const Optional.absent(),
      request_id: v.containsKey("request_id")
          ? Optional.present(readString(v["request_id"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "ordinal": project(ordinal),
        "request_sha256": project(request_sha256),
        "wall_ms": project(wall_ms),
        "outcome": project(outcome),
        "sdk_request_id": project(sdk_request_id),
        if (status.present) "status": project(status.value),
        if (server_ms.present) "server_ms": project(server_ms.value),
        if (request_id.present) "request_id": project(request_id.value),
      };
}

final class Facts extends Carrier {
  final CallId call_id;
  final int records;
  final int requests_sent;
  final int cache_answers;
  final double seconds;
  final Optional<int> input_tokens;
  final Optional<int> output_tokens;
  final Optional<String> model;
  final Optional<String> estimated_cost_usd;
  final Optional<int> command_ms;
  const Facts({
    required this.call_id,
    required this.records,
    required this.requests_sent,
    required this.cache_answers,
    required this.seconds,
    this.input_tokens = const Optional.absent(),
    this.output_tokens = const Optional.absent(),
    this.model = const Optional.absent(),
    this.estimated_cost_usd = const Optional.absent(),
    this.command_ms = const Optional.absent(),
  });
  factory Facts.fromJson(Object? value) {
    final v = readObject(value);
    verify("Facts", v, [
      "call_id",
      "records",
      "requests_sent",
      "cache_answers",
      "seconds",
      "input_tokens",
      "output_tokens",
      "model",
      "estimated_cost_usd",
      "command_ms"
    ], [
      "call_id",
      "records",
      "requests_sent",
      "cache_answers",
      "seconds"
    ]);
    return Facts(
      call_id: CallId(readString(v["call_id"])),
      records: readInt(v["records"], 0, maxInteger),
      requests_sent: readInt(v["requests_sent"], 0, maxInteger),
      cache_answers: readInt(v["cache_answers"], 0, maxInteger),
      seconds: readNumber(v["seconds"], probability: false),
      input_tokens: v.containsKey("input_tokens")
          ? Optional.present(readInt(v["input_tokens"], 0, maxInteger))
          : const Optional.absent(),
      output_tokens: v.containsKey("output_tokens")
          ? Optional.present(readInt(v["output_tokens"], 0, maxInteger))
          : const Optional.absent(),
      model: v.containsKey("model")
          ? Optional.present(readString(v["model"]))
          : const Optional.absent(),
      estimated_cost_usd: v.containsKey("estimated_cost_usd")
          ? Optional.present(readCost(v["estimated_cost_usd"]))
          : const Optional.absent(),
      command_ms: v.containsKey("command_ms")
          ? Optional.present(readInt(v["command_ms"], 0, maxInteger))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "call_id": project(call_id),
        "records": project(records),
        "requests_sent": project(requests_sent),
        "cache_answers": project(cache_answers),
        "seconds": project(seconds),
        if (input_tokens.present) "input_tokens": project(input_tokens.value),
        if (output_tokens.present)
          "output_tokens": project(output_tokens.value),
        if (model.present) "model": project(model.value),
        if (estimated_cost_usd.present)
          "estimated_cost_usd": project(estimated_cost_usd.value),
        if (command_ms.present) "command_ms": project(command_ms.value),
      };
}

final class QuestionSource extends Carrier {
  final OriginKind origin;
  final String answered_by;
  const QuestionSource({
    required this.origin,
    required this.answered_by,
  });
  factory QuestionSource.fromJson(Object? value) {
    final v = readObject(value);
    verify("QuestionSource", v, ["origin", "answered_by"],
        ["origin", "answered_by"]);
    return QuestionSource(
      origin: readEnum(v["origin"], OriginKind.values),
      answered_by: readString(v["answered_by"]),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "origin": project(origin),
        "answered_by": project(answered_by),
      };
}

final class Observed extends Carrier implements Observation {
  final ObservationId observation_id;
  const Observed({
    required this.observation_id,
  });
  factory Observed.fromJson(Object? value) {
    final v = readObject(value);
    verify("Observed", v, ["observation_id"], ["observation_id"]);
    return Observed(
      observation_id: ObservationId(readString(v["observation_id"])),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "observation_id": project(observation_id),
      };
}

final class FailedObservation extends Carrier implements Observation {
  final FailureId failure_id;
  const FailedObservation({
    required this.failure_id,
  });
  factory FailedObservation.fromJson(Object? value) {
    final v = readObject(value);
    verify("FailedObservation", v, ["failure_id"], ["failure_id"]);
    return FailedObservation(
      failure_id: FailureId(readString(v["failure_id"])),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "failure_id": project(failure_id),
      };
}

final class Meta extends Carrier {
  final String tool;
  final String url;
  final String model;
  final int requests_sent;
  final bool cached;
  final List<Digest> requests;
  final int failed_questions;
  final OriginKind? origin;
  final List<QuestionSource> question_sources;
  final List<Observation> observations;
  final Optional<Digest> question_sha256;
  final Optional<Digest> questions_sha256;
  final Optional<String> answered_by;
  final Optional<Usage> usage;
  final Optional<ProfileWarning> profile_warning;
  final Optional<Batch> batch_setting;
  final Optional<BatchWarning> batch_warning;
  final Optional<Digest> context_sha256;
  final Optional<List<Attempt>> attempts;
  const Meta({
    required this.tool,
    required this.url,
    required this.model,
    required this.requests_sent,
    required this.cached,
    required this.requests,
    required this.failed_questions,
    required this.origin,
    required this.question_sources,
    required this.observations,
    this.question_sha256 = const Optional.absent(),
    this.questions_sha256 = const Optional.absent(),
    this.answered_by = const Optional.absent(),
    this.usage = const Optional.absent(),
    this.profile_warning = const Optional.absent(),
    this.batch_setting = const Optional.absent(),
    this.batch_warning = const Optional.absent(),
    this.context_sha256 = const Optional.absent(),
    this.attempts = const Optional.absent(),
  });
  factory Meta.fromJson(Object? value) {
    final v = readObject(value);
    verify("Meta", v, [
      "tool",
      "url",
      "model",
      "requests_sent",
      "cached",
      "requests",
      "failed_questions",
      "origin",
      "question_sources",
      "observations",
      "question_sha256",
      "questions_sha256",
      "answered_by",
      "usage",
      "profile_warning",
      "batch_setting",
      "batch_warning",
      "context_sha256",
      "attempts"
    ], [
      "tool",
      "url",
      "model",
      "requests_sent",
      "cached",
      "requests",
      "failed_questions",
      "origin",
      "question_sources",
      "observations"
    ]);
    return Meta(
      tool: readString(v["tool"]),
      url: readString(v["url"]),
      model: readString(v["model"]),
      requests_sent: readInt(v["requests_sent"], 0, maxInteger),
      cached: readBool(v["cached"]),
      requests: readList(v["requests"], (v) => Digest(readString(v))),
      failed_questions: readInt(v["failed_questions"], 0, maxInteger),
      origin: (v["origin"] == null
          ? null
          : readEnum(v["origin"], OriginKind.values)),
      question_sources:
          readList(v["question_sources"], (v) => QuestionSource.fromJson(v)),
      observations: readList(
          v["observations"],
          (v) => readVariant<Observation>(v, [
                (v) => Observed.fromJson(v),
                (v) => FailedObservation.fromJson(v)
              ])),
      question_sha256: v.containsKey("question_sha256")
          ? Optional.present(Digest(readString(v["question_sha256"])))
          : const Optional.absent(),
      questions_sha256: v.containsKey("questions_sha256")
          ? Optional.present(Digest(readString(v["questions_sha256"])))
          : const Optional.absent(),
      answered_by: v.containsKey("answered_by")
          ? Optional.present(readString(v["answered_by"]))
          : const Optional.absent(),
      usage: v.containsKey("usage")
          ? Optional.present(Usage.fromJson(v["usage"]))
          : const Optional.absent(),
      profile_warning: v.containsKey("profile_warning")
          ? Optional.present(ProfileWarning.fromJson(v["profile_warning"]))
          : const Optional.absent(),
      batch_setting: v.containsKey("batch_setting")
          ? Optional.present(Batch.fromJson(v["batch_setting"]))
          : const Optional.absent(),
      batch_warning: v.containsKey("batch_warning")
          ? Optional.present(BatchWarning.fromJson(v["batch_warning"]))
          : const Optional.absent(),
      context_sha256: v.containsKey("context_sha256")
          ? Optional.present(Digest(readString(v["context_sha256"])))
          : const Optional.absent(),
      attempts: v.containsKey("attempts")
          ? Optional.present(
              readList(v["attempts"], (v) => Attempt.fromJson(v)))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "tool": project(tool),
        "url": project(url),
        "model": project(model),
        "requests_sent": project(requests_sent),
        "cached": project(cached),
        "requests": project(requests),
        "failed_questions": project(failed_questions),
        "origin": project(origin),
        "question_sources": project(question_sources),
        "observations": project(observations),
        if (question_sha256.present)
          "question_sha256": project(question_sha256.value),
        if (questions_sha256.present)
          "questions_sha256": project(questions_sha256.value),
        if (answered_by.present) "answered_by": project(answered_by.value),
        if (usage.present) "usage": project(usage.value),
        if (profile_warning.present)
          "profile_warning": project(profile_warning.value),
        if (batch_setting.present)
          "batch_setting": project(batch_setting.value),
        if (batch_warning.present)
          "batch_warning": project(batch_warning.value),
        if (context_sha256.present)
          "context_sha256": project(context_sha256.value),
        if (attempts.present) "attempts": project(attempts.value),
      };
}
