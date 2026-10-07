// Compiled and executed through both installed public module faces.
import { CompleteTypes as C, complete, type CompleteInput } from 'thinkthen';
const expected = 'a'.repeat(64);
const id: C.AnswerId = C.AnswerId(expected);
if (id !== expected) throw new Error('the public AnswerId factory lost the identity');
const usage: C.Usage = C.decode('Usage', { input_tokens: 2 });
if (usage.input_tokens !== 2 || usage.output_tokens !== undefined) {
  throw new Error('the public decoder lost partial usage');
}

type Item = Extract<CompleteInput, { kind: 'records' }>['records'][number];
type Controls = NonNullable<Parameters<typeof complete.decide>[2]>;
const shared: Controls = { context: 'shared text' };
const text: Item = { content: { kind: 'text', value: 'record' }, context: { kind: 'text', value: '' } };
const json: Item = { content: { kind: 'json', value: false }, context: { kind: 'json', value: { body: 'private', ready: false } } };
const nullContext: Item = { content: { kind: 'text', value: 'record' }, context: { kind: 'json', value: null } };
const image: Item = { content: { kind: 'images' }, images: [{ media: 'image/png', bytes: [1] }] };
// @ts-expect-error shared native context is text, not a JSON object.
const objectShared: Controls = { context: { shared: 'text' } };
// @ts-expect-error shared native context is text, not a JSON array.
const arrayShared: Controls = { context: ['shared text'] };
// @ts-expect-error native per-record context cannot contain images.
const imageContext: Item = { content: { kind: 'text', value: 'record' }, context: { kind: 'images' } };
// @ts-expect-error text context retains a string; structural values use JSON context.
const objectText: Item = { content: { kind: 'text', value: 'record' }, context: { kind: 'text', value: {} } };
void [shared, text, json, nullContext, image, objectShared, arrayShared, imageContext, objectText];
