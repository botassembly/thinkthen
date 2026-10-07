<?php
declare(strict_types=1);
namespace ThinkThen\Complete;

abstract class DecisionValue extends Carrier {}
final class BooleanDecision extends DecisionValue {
    public function __construct(public readonly bool $value) {}
    public function jsonSerialize(): bool { return $this->value; }
}
final class AuthoredDecision extends DecisionValue {
    public function __construct(public readonly JsonContent $meaning) {}
    public function jsonSerialize(): mixed { return $this->meaning->jsonSerialize(); }
}
final class UnresolvedDecision extends DecisionValue {
    public function jsonSerialize(): mixed { return null; }
}
final class AuthoredValue extends ActionValue {
    public function __construct(public readonly JsonContent $meaning) {}
    public function jsonSerialize(): mixed { return $this->meaning->jsonSerialize(); }
}
function readDecision(mixed $value, mixed $question): DecisionValue
{
    $q=Read::object($question);
    if (property_exists($q,'true') || property_exists($q,'false')) {
        // The complete C discriminator is required for authored-null vs uncertainty.
        if ($value===null) Read::invalid();
        return new AuthoredDecision(JsonContent::fromJson($value));
    }
    return $value===null ? new UnresolvedDecision() : new BooleanDecision(Read::value('bool',$value));
}
function readAction(mixed $value,mixed $question): ActionValue
{
    $q=Read::object($question);
    if ($q->verb==='decide' && (property_exists($q,'true') || property_exists($q,'false'))) {
        if ($value===null) Read::invalid();
        return new AuthoredValue(JsonContent::fromJson($value));
    }
    $valid=match($q->verb) {
        'decide'=>is_bool($value) || $value===null,
        'choose'=>is_string($value) || $value===null,
        'tag'=>is_array($value) && array_is_list($value),
        'score'=>is_int($value) || is_float($value),
        default=>false,
    };
    if (!$valid) Read::invalid();
    return ActionValue::fromJson($value);
}
function readAnnotationValues(mixed $value,mixed $answers): AnnotationValues
{
    $v=Read::object($value);$entries=Read::object($answers);$out=[];
    if(count(get_object_vars($v))!==count(get_object_vars($entries))) Read::invalid();
    foreach($v as $name=>$x) {
        if(!property_exists($entries,(string)$name)) Read::invalid();
        $e=Read::object($entries->$name);
        $out[]=new AnnotationValuesEntry((string)$name,property_exists($e,'failure_id') ? ActionValue::fromJson($x) : readAction($x,$e->question));
    }
    return AnnotationValues::fromEntries($out);
}
