<?php
declare(strict_types=1);
namespace ThinkThen\Complete;

/** Native constructors/loaders consume this private model after complete API adoption. */
final class Request extends Carrier
{
    public function __construct(public readonly string $function, public readonly Carrier $question,
                                public readonly Selection $input, public readonly Controls $controls) {}
    public function questionJson(): mixed
    {
        if ($this->question instanceof QuestionFile) throw new \LogicException('question files require the native loader');
        return Requests::project($this->question);
    }
}
final class Requests
{
    public static function project(mixed $v): mixed
    {
        if ($v instanceof \BackedEnum) return $v->value;
        if ($v instanceof \JsonSerializable) return self::project($v->jsonSerialize());
        if (is_array($v)) return array_map(self::project(...), $v);
        if ($v instanceof \stdClass) {
            $out = new \stdClass();
            foreach ($v as $k => $value) $out->$k = self::project($value);
            return $out;
        }
        return $v;
    }
    private static function snapshot(Carrier $v): Carrier
    {
        return readModel((new \ReflectionClass($v))->getShortName(), self::project($v));
    }
    private static function build(string $verb, Carrier $q, Selection $input, ?Controls $controls): Request
    {
        $q = self::snapshot($q);
        if (!$input instanceof Carrier) Read::invalid();
        $input = self::snapshot($input);
        $controls = $controls === null ? new Controls() : self::snapshot($controls);
        if (!$input instanceof Selection || !$controls instanceof Controls) Read::invalid();
        $images = $input instanceof ImageInput || ($input instanceof Files && $input->media->present && $input->media->value === MediaKind::IMAGE);
        if ($images && !in_array($verb, ['decide','choose','score'], true)) throw new \InvalidArgumentException('this function is text-only');
        if ($input instanceof CandidateInput && $verb !== 'find') throw new \InvalidArgumentException('candidates require find');
        if ($input instanceof RecordInput && $verb === 'find') throw new \InvalidArgumentException('find requires candidates');
        if ($input instanceof TextInput && in_array($verb, ['filter','rank','find','annotate','relate'], true)) throw new \InvalidArgumentException('this function requires a complete record set');
        if ($input instanceof Files) {
            if (($input->unit === UnitKind::WINDOW) !== $input->window->present) throw new \InvalidArgumentException('window requires its size');
            if ($images && $input->unit !== UnitKind::FILE) throw new \InvalidArgumentException('image sources require file units');
        }
        if ($input instanceof ImageInput && !$input->images) throw new \InvalidArgumentException('images require attachments');
        if ($verb !== 'rank' && $controls->top->present) throw new \InvalidArgumentException('top requires rank');
        if ($verb !== 'find' && $controls->none->present) throw new \InvalidArgumentException('none requires find');
        return new Request($verb, $q, $input, $controls);
    }
    public static function decide(DecideSpec|QuestionFile $question, Selection $input, ?Controls $controls = null): Request
    {
        return self::build('decide', $question, $input, $controls);
    }
    public static function choose(ChooseSpec|QuestionFile $question, Selection $input, ?Controls $controls = null): Request
    {
        return self::build('choose', $question, $input, $controls);
    }
    public static function tag(TagSpec|QuestionFile $question, Selection $input, ?Controls $controls = null): Request
    {
        return self::build('tag', $question, $input, $controls);
    }
    public static function score(ScoreSpec|QuestionFile $question, Selection $input, ?Controls $controls = null): Request
    {
        return self::build('score', $question, $input, $controls);
    }
    public static function filter(DecideSpec|QuestionFile $question, Selection $input, ?Controls $controls = null): Request
    {
        return self::build('filter', $question, $input, $controls);
    }
    public static function rank(DecideSpec|ScoreSpec|QuestionSet|QuestionFile $question, Selection $input, ?Controls $controls = null): Request
    {
        return self::build('rank', $question, $input, $controls);
    }
    public static function find(FindSpec|QuestionFile $question, Selection $input, ?Controls $controls = null): Request
    {
        return self::build('find', $question, $input, $controls);
    }
    public static function annotate(QuestionSet|QuestionFile $question, Selection $input, ?Controls $controls = null): Request
    {
        return self::build('annotate', $question, $input, $controls);
    }
    public static function recognize(RecognitionSpec|QuestionFile $question, Selection $input, ?Controls $controls = null): Request
    {
        return self::build('recognize', $question, $input, $controls);
    }
    public static function relate(RelationSpec|QuestionFile $question, Selection $input, ?Controls $controls = null): Request
    {
        return self::build('relate', $question, $input, $controls);
    }
}
