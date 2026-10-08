// Compile consumer for private carrier integration; runtime is checked separately.
import { decode, AnswerId, CallId, type Result, type AnnotatedValue } from '../_complete.js';
import * as requests from '../_requests.js';
const input: unknown = {};
const decide = decode('DecideResult', input);
const meaning: boolean | import('../_complete.js').Description = decide.value;
// @ts-expect-error complete decisions may carry an authored reading.
const bareMeaning: boolean | null = decide.value;
const probability: number = decide.answer.probability;
const id = decide.answer_id;
const facts = decode('Facts', input);
const call = facts.call_id;
const attempt = decode('Attempt', input);
const requestId = attempt.sdk_request_id;
const server: number | undefined = attempt.server_ms;
const annotation = decode('AnnotateResult', input);
for (const member of Object.values(annotation.answers)) {
  if ('failure' in member) { const cause: string = member.failure.cause; void cause; }
  else { const value: AnnotatedValue = member.value; const answer = member.answer; void value; void answer; }
}
const recognition = decode('RecognizeResult', input);
const offsets: number = recognition.value.entities[0]!.start;
const line: number | undefined = recognition.value.entities[0]!.first_line;
const pieces: number = recognition.answer.pieces[0]!.tags.SINGLE!;
const kinds: number | undefined = recognition.answer.names[0]!.kinds?.person;
const relations = decode('RelateResult', input);
const source: string | undefined = relations.value[0]!.source.file;
const target: string | undefined = relations.value[0]!.target.file;
const aggregate: Result = relations;
const choose = decode('ChooseResult', input);
const images: readonly string[] | undefined = choose.position?.images;
// @ts-expect-error call identities and stable answer identities are distinct.
const wrongId: typeof id = call;
// @ts-expect-error answer identities are not SDK request identities.
const wrongRequest: typeof requestId = AnswerId('a'.repeat(64));
const records = { records: [false, null, { original: [] }] } as const;
requests.decide({ decide: ['Q', { active: false }], false: null }, { images: [{ data: new Uint8Array([1]) }] });
requests.choose({ choose: 'Q', options: { a: { nested: [false] }, b: null } }, records);
requests.score({ score: 'Q', levels: ['low', 'high'] }, records);
requests.tag({ tag: 'Q', labels: ['a'] }, records);
requests.filter({ decide: 'Q' }, records);
requests.rank({ score: 'Q', levels: ['low', 'high'] }, records, { top: 2 });
requests.find({ find: 'Q', none: true }, { units: [false, null] });
requests.annotate({ version: 1, questions: { ready: { decide: 'Q', on: ['/body'] } } }, records);
requests.recognize({ version: 1, recognize: { kinds: { person: { nested: [] } } } }, records);
requests.relate({ version: 1, relate: { relations: [{ name: 'knows', source: '*', target: '*' }] } }, records);
// @ts-expect-error find cannot read a score question.
requests.find({ score: 'Q', levels: ['low', 'high'] }, records);
// @ts-expect-error explicit images are refused on tag.
requests.tag({ tag: 'Q', labels: ['a'] }, { images: [{ data: new Uint8Array([1]) }] });
// @ts-expect-error annotation is a record operation.
requests.annotate({ version: 1, questions: {} }, { text: 'literal' });
void [meaning, bareMeaning, probability, server, offsets, line, pieces, kinds, source, target, aggregate, images, wrongId, wrongRequest, CallId];

// @ts-expect-error descriptions have a structured/string/null root.
requests.choose({ choose: 'Q', options: { a: true } }, records);
