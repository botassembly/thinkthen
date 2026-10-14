<?php
declare(strict_types=1);
namespace ThinkThen\Internal;
use ThinkThen\Internal\Requests as Input;
use ThinkThen\Files;

/** Own request buffers across one native admission call. */
final class Request
{
    public readonly \FFI\CData $value;
    private array $buffers = [];
    public function __construct(private readonly \FFI $ffi, string $function, mixed $question, mixed $input, mixed $options)
    {
        if ($input instanceof Files) {
            $source = new Input\InputSource(new Input\Source($input->paths,
                ['reading' => new Input\Reader($input->reading), ...($input->framing === null ? [] : ['framing' => $input->framing]), ...($input->media === null ? [] : ['media' => $input->media])]));
        } elseif ($input instanceof \Traversable) {
            $source = new Input\InputFeed('php');
        } else {
            $records = is_array($input) && array_is_list($input) ? $input : [$input];
            $source = new Input\InputRecords(array_map(fn($item) => new Input\Item(\ThinkThen\Internal\descriptor($item)), $records));
        }
        $question = $question instanceof \ThinkThen\Inputs\QuestionSource
            ? $question
            : (is_string($question) ? new Input\QuestionText($question) : new Input\QuestionDefinition($question));
        $request = $this->ffi->new('struct thinkthen_named_request_v1');
        foreach (['function' => $function, 'question_json' => \ThinkThen\Internal\Encoder::encode($question),
            'input_json' => \ThinkThen\Internal\Encoder::encode($source), 'options_json' => \ThinkThen\Internal\Encoder::encode(($options instanceof \ThinkThen\Inputs\Options ? $options : (object)$options))] as $field => $text) {
            $request->$field = $this->utf8($text, $this->buffers);
        }
        $this->value = $request;
    }
    public function utf8(string $text, array &$buffers): \FFI\CData
    {
        $bytes = $this->ffi->new('char['.(strlen($text) + 1).']');
        \FFI::memcpy($bytes, $text, strlen($text));
        $buffers[] = $bytes;
        $view = $this->ffi->new('struct thinkthen_complete_utf8_v1');
        $view->data = \FFI::addr($bytes[0]);
        $view->len = strlen($text);
        return $view;
    }
}
