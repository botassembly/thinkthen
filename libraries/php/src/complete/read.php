<?php
declare(strict_types=1);
namespace ThinkThen\Complete;
require_once __DIR__ . '/type_dispatch.php';

/** Strict known-field reader; unknown additive output fields are ignored. */
final class Read
{
    public static function invalid(): never { throw new \UnexpectedValueException('invalid typed carrier'); }
    public static function object(mixed $v): \stdClass
    {
        if (!$v instanceof \stdClass) self::invalid();
        return $v;
    }
    public static function json(mixed $v): mixed
    {
        // Snapshot caller JSON; no host question, file, image or pointer parser.
        try { return json_decode(json_encode($v, JSON_THROW_ON_ERROR), false, 512, JSON_THROW_ON_ERROR | JSON_BIGINT_AS_STRING); }
        catch (\JsonException) { self::invalid(); }
    }
    public static function integer(mixed $v, int $low = 0, int $high = PHP_INT_MAX): int
    {
        if (!is_int($v) || $v < $low || $v > $high) self::invalid();
        return $v;
    }
    public static function number(mixed $v, bool $probability = false): float
    {
        if ((!is_int($v) && !is_float($v)) || !is_finite((float)$v) || ($probability && ($v < 0 || $v > 1))) self::invalid();
        return (float)$v;
    }
    public static function field(\stdClass $v, string $name, string $type): mixed
    {
        if (!property_exists($v, $name)) self::invalid();
        return self::value($type, $v->$name);
    }
    public static function optional(\stdClass $v, string $name, string $type): Optional
    {
        return property_exists($v, $name) ? new Optional(true, self::value($type, $v->$name)) : new Optional();
    }
    public static function value(string $type, mixed $v): mixed
    {
        if ($type === 'str|[str]') return Pointers::fromJson($v);
        if (isset(ALIASES[$type]) || str_contains($type, '|')) {
            foreach (ALIASES[$type] ?? explode('|', $type) as $variant) {
                try { return self::value($variant, $v); } catch (\UnexpectedValueException) {}
            }
            self::invalid();
        }
        if (str_starts_with($type, '[')) {
            if (!is_array($v) || !array_is_list($v)) self::invalid();
            return array_map(fn($item) => self::value(substr($type, 1, -1), $item), $v);
        }
        if (str_starts_with($type, '{')) return match ($type) {
            '{probability}' => Probabilities::fromJson($v), '{description}' => Descriptions::fromJson($v),
            '{AnnotationEntry}' => AnnotationMembers::fromJson($v), '{AnnotatedValue}' => AnnotationValues::fromJson($v),
            '{AnnotationSpec}' => QuestionMembers::fromJson($v), default => self::invalid(),
        };
        if (in_array($type, ['CallId','SdkRequestId','ObservationId','FailureId','AnswerId','Digest'], true)) {
            if (!is_string($v)) self::invalid();
            $class = __NAMESPACE__ . '\\' . $type;
            return new $class($v);
        }
        $enum = match ($type) {
            'image_format' => MediaType::class, 'origin' => OriginKind::class, 'outcome' => OutcomeKind::class, 'cause' => CauseKind::class,
            'error_kind' => FailureKind::class, 'unit' => UnitKind::class, 'media' => MediaKind::class,
            'method' => MethodKind::class, 'direction' => DirectionKind::class, 'stop_cause' => StopCauseKind::class,
            default => null,
        };
        if ($enum !== null) return is_string($v) ? ($enum::tryFrom($v) ?? self::invalid()) : self::invalid();
        if (str_starts_with($type, '=')) {
            if ($v !== ($type === '=true' ? true : substr($type, 1))) self::invalid();
            return $v;
        }
        return match ($type) {
            'str', 'bytes' => is_string($v) ? $v : self::invalid(),
            'text' => $v !== null ? Authored::fromJson($v) : self::invalid(),
            'description' => Authored::fromJson($v), 'json' => self::json($v),
            'null' => $v === null ? null : self::invalid(), 'bool' => is_bool($v) ? $v : self::invalid(),
            'deadline' => self::integer($v, -1), 'uint' => self::integer($v), 'positive' => self::integer($v, 1), 'one' => self::integer($v, 1, 1),
            'http_status' => self::integer($v, 100, 599),
            'number' => self::number($v), 'probability' => self::number($v, true),
            'threshold' => Threshold::fromJson($v), 'batch' => Batch::fromJson($v), 'Labels' => Labels::fromJson($v),
            'SuccessValue' => !$v instanceof \stdClass ? ActionValue::fromJson($v) : self::invalid(),
            'AnnotatedValue' => ActionValue::fromJson($v),
            'cost' => is_string($v) && preg_match('/\A[0-9]+\.[0-9]{6}\z/D', $v) ? $v : self::invalid(),
            default => readModel($type, $v),
        };
    }
    /** @param list<string> $fields @param list<string> $required */
    public static function shape(string $type, \stdClass $v, array $fields, array $required): void
    {
        foreach ($required as $key) if (!property_exists($v, $key)) self::invalid();
        $input = str_ends_with($type, 'Spec') || str_ends_with($type, 'Member') || in_array($type,
            ['Controls','QuestionSet','QuestionFile','Files','ImageInput','ImageBytes','TextInput','RecordInput','CandidateInput','RecognitionPlan','RelationPlan','PlanRule'], true);
        if ($input) foreach ($v as $key => $_) if (!in_array((string)$key, $fields, true)) self::invalid();
        if (property_exists($v, 'proxy')) self::invalid();
        $success = in_array($type, ['Observed','AnnotationSuccess','RelationSuccess'], true);
        $failure = in_array($type, ['FailedObservation','AnnotationFailure','RelationFailure'], true);
        if (($success && property_exists($v, 'failure_id')) || ($failure && property_exists($v, 'answer_id')) ||
            ($type === 'FailedObservation' && property_exists($v, 'observation_id'))) self::invalid();
        if ($type === 'Meta') self::meta($v);
        if (in_array($type, ['Entity','Span','PieceOdds','NameOdds'], true)) {
            if (self::integer($v->end) < self::integer($v->start)) self::invalid();
            if ($type === 'Entity' && $v->length !== $v->end - $v->start) self::invalid();
        }
        foreach ([['first','last'],['first_line','last_line']] as [$first,$last]) {
            if (property_exists($v,$first) !== property_exists($v,$last)) self::invalid();
            if (property_exists($v,$first) && (!property_exists($v,'file') || self::integer($v->$last,1) < self::integer($v->$first,1))) self::invalid();
        }
        if (in_array($type,['ChooseSpec','TagSpec','ChooseMember','TagMember','RecognitionSpec','RelationSpec','TagResult','FilterResult','RecognizeQuestion','RelateQuestion'],true)) {
            foreach (['threshold','relation_threshold'] as $key) if (property_exists($v,$key) && ((!is_int($v->$key) && !is_float($v->$key)) || $v->$key <= 0 || $v->$key > 1)) self::invalid();
        }
        if ($type === 'ChooseResult' && $v->threshold !== null && !is_int($v->threshold) && !is_float($v->threshold)) self::invalid();
        if ($type === 'DecideResult' && $v->threshold === null) self::invalid();
        if ($type === 'RankResult' && (!in_array($v->question->verb ?? null,['decide','score'],true) || !in_array($v->answer->kind ?? null,['yes_no','score'],true) || ($v->answer->kind === 'score') !== ($v->question->verb === 'score'))) self::invalid();
        if (str_ends_with($type, 'Result') && (($type === 'AnnotateResult') !== property_exists(self::object($v->meta),'questions_sha256'))) self::invalid();
        if ($type === 'Facts' && self::number($v->seconds) < 0) self::invalid();
    }
    private static function meta(\stdClass $v): void
    {
        foreach (['requests','question_sources','observations'] as $k) if (!is_array($v->$k)) self::invalid();
        if (count($v->requests) !== count($v->question_sources) || count($v->requests) !== count($v->observations)) self::invalid();
        if (property_exists($v,'question_sha256') === property_exists($v,'questions_sha256')) self::invalid();
        $origins=[]; $names=[]; $failed=0;
        foreach ($v->question_sources as $s) {
            $s=self::object($s);
            if (!in_array($s->origin ?? null,['live','cache','replay'],true) || !is_string($s->answered_by ?? null)) self::invalid();
            $origins[]=$s->origin; $names[]=$s->answered_by;
        }
        foreach ($v->observations as $o) if (property_exists(self::object($o),'failure_id')) ++$failed;
        if ($v->failed_questions !== $failed) self::invalid();
        if (!$origins) {
            if ($v->origin !== null || $v->cached !== false || $v->requests_sent !== 0 || property_exists($v,'answered_by')) self::invalid();
        } else {
            $origin=in_array('live',$origins,true) ? 'live' : (in_array('replay',$origins,true) ? 'replay' : 'cache');
            if ($v->origin !== $origin || $v->cached !== ($origin !== 'live')) self::invalid();
            $names=array_values(array_unique($names));
            if (count($names)===1 ? ($v->answered_by ?? null)!==$names[0] : property_exists($v,'answered_by')) self::invalid();
        }
    }
}
