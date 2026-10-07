<?php
declare(strict_types=1);
namespace ThinkThen\Complete;
final class DescriptionsEntry extends Carrier
{
    public function __construct(public readonly string $name, public readonly Authored $value) {}
}
final class Descriptions extends Carrier
{
    /** @param list<DescriptionsEntry> $entries */
    private function __construct(public readonly array $entries) {}
    public static function fromJson(mixed $v): self
    {
        $out = [];
        foreach (Read::object($v) as $name => $value) $out[] = new DescriptionsEntry((string)$name, Read::value('description', $value));
        return new self($out);
    }
    public function get(string $name): ?Authored
    {
        foreach ($this->entries as $entry) if ($entry->name === $name) return $entry->value;
        return null;
    }
    public function jsonSerialize(): \stdClass
    {
        $out = new \stdClass();
        foreach ($this->entries as $entry) $out->{$entry->name} = $entry->value;
        return $out;
    }
}
final class AnnotationMembersEntry extends Carrier
{
    public function __construct(public readonly string $name, public readonly AnnotationEntry $value) {}
}
final class AnnotationMembers extends Carrier
{
    /** @param list<AnnotationMembersEntry> $entries */
    private function __construct(public readonly array $entries) {}
    public static function fromJson(mixed $v): self
    {
        $out = [];
        foreach (Read::object($v) as $name => $value) $out[] = new AnnotationMembersEntry((string)$name, Read::value('AnnotationEntry', $value));
        return new self($out);
    }
    public function get(string $name): ?AnnotationEntry
    {
        foreach ($this->entries as $entry) if ($entry->name === $name) return $entry->value;
        return null;
    }
    public function jsonSerialize(): \stdClass
    {
        $out = new \stdClass();
        foreach ($this->entries as $entry) $out->{$entry->name} = $entry->value;
        return $out;
    }
}
final class AnnotationValuesEntry extends Carrier
{
    public function __construct(public readonly string $name, public readonly ActionValue $value) {}
}
final class AnnotationValues extends Carrier
{
    /** @param list<AnnotationValuesEntry> $entries */
    public static function fromEntries(array $entries): self { return new self($entries); }
    /** @param list<AnnotationValuesEntry> $entries */
    private function __construct(public readonly array $entries) {}
    public static function fromJson(mixed $v): self
    {
        $out = [];
        foreach (Read::object($v) as $name => $value) $out[] = new AnnotationValuesEntry((string)$name, Read::value('AnnotatedValue', $value));
        return new self($out);
    }
    public function get(string $name): ?ActionValue
    {
        foreach ($this->entries as $entry) if ($entry->name === $name) return $entry->value;
        return null;
    }
    public function jsonSerialize(): \stdClass
    {
        $out = new \stdClass();
        foreach ($this->entries as $entry) $out->{$entry->name} = $entry->value;
        return $out;
    }
}
final class QuestionMembersEntry extends Carrier
{
    public function __construct(public readonly string $name, public readonly AnnotationSpec $value) {}
}
final class QuestionMembers extends Carrier
{
    /** @param list<QuestionMembersEntry> $entries */
    private function __construct(public readonly array $entries) {}
    public static function fromJson(mixed $v): self
    {
        $out = [];
        foreach (Read::object($v) as $name => $value) $out[] = new QuestionMembersEntry((string)$name, Read::value('AnnotationSpec', $value));
        return new self($out);
    }
    public function get(string $name): ?AnnotationSpec
    {
        foreach ($this->entries as $entry) if ($entry->name === $name) return $entry->value;
        return null;
    }
    public function jsonSerialize(): \stdClass
    {
        $out = new \stdClass();
        foreach ($this->entries as $entry) $out->{$entry->name} = $entry->value;
        return $out;
    }
}
