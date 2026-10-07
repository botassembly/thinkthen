// The deck's sample and every verb, typed: a wrong branch is a compile
// error. `tsc --noEmit --strict` over this file is the test; nothing runs.
import * as tt from '../index.js';

export async function sample(text: string, message: string, inbox: string[], reviews: string[], signal: AbortSignal) {
  const options = ['billing', 'shipping', 'account'];
  const team: string | null = (await tt.choose('Which team owns it?', text, { options })).value;
  const labels = ['billing', 'shipping', 'urgent', 'praise'];
  const topics: string[] = (await tt.tag('Which topics?', message, { labels })).value;
  const stop = new AbortController();
  const urgent: tt.Ranked[] = (await tt.rank('Is this urgent?', inbox, { top: 5, signal: stop.signal })).value;
  const first: tt.Ranked | undefined = urgent[0];

  const answeredCall: tt.Call<tt.Answer> = await tt.decide('Does the customer ask for a refund?', text);
  const answered: tt.Answer = answeredCall.value;
  const callFacts: tt.Facts = answeredCall.facts;
  const firstDetail: tt.QuestionObservation | undefined = answeredCall.details[0];
  void callFacts; void firstDetail;
  const refund = tt.question({ decide: 'Does the customer ask for a refund?', threshold: [0.2, 0.8] });
  const second: boolean | null = (await tt.decide(refund, 'I was charged twice. Can you fix this?')).value;
  const fromFile: tt.Question = tt.questionFile('question.json');
  const recognition = tt.recognize('Ana Bob', { file: 'recognize.json' });
  const relation = tt.relate([{ name: 'Ana', kind: 'person' }], { file: 'relate.json' });
  void recognition;
  void relation;
  const fileAnswer: tt.Answer = (await tt.decide(fromFile, text)).value;
  void fileAnswer;
  const level: number = (await tt.score('How urgent?', text, { levels: ['low', 'mid', 'high'] })).value;
  const complaints: string[] = (await tt.filter('Is this a complaint?', reviews, { batch: 2, context: 'Shared evidence.' })).value;
  const rows: tt.AnnotatedRow[] = (await tt.annotate('form.json', reviews, { signal, deadlineMs: null, batch: 2 })).value;
  const found: tt.Found | null = (await tt.find('Which unit answers best?', reviews)).value;
  const audit: tt.Details = (await tt.details('Refund?', text, { deadlineMs: 5_000 })).value;
  const calibrated: tt.QuestionSpec[] = [
    { decide: 'Refund?', profile: 'old' },
    { choose: 'Which?', options: ['one', 'two'], profile: 'old' },
    { tag: 'Which?', labels: ['one'], profile: 'old' },
    { score: 'How?', levels: ['low', 'high'], profile: 'old' },
  ];
  const warning: { tuned_for: string; running: string } | undefined = audit.meta.profile_warning;
  void calibrated;
  void warning;
  const digests: string[] = audit.meta.requests;
  const confidence: number | undefined = audit.answer.confidence;
  const url: string = audit.meta.url;
  const many: tt.Answer[] = (await tt.decide_many('Refund?', reviews)).value;
  const described: tt.LabelSet = { '2': ['nested', { active: true, count: 3 }], '1': { what: 'first', other: [null, false] } };
  const tuple: readonly ['low', 'high'] = ['low', 'high'];
  const choices: tt.Call<(string | null)[]> = await tt.choose_many('Which?', reviews, { options: described, batch: 2, context: 'Shared.' });
  const scored: tt.Call<number[]> = await tt.score_many('How?', reviews, { levels: { low: null, high: { nested: [true, 2] } } });
  const tagged: tt.Call<string[][]> = await tt.tag_many('Which?', reviews, { labels: tuple });
  const meaning: tt.Question = tt.question({ decide: ['nested', { ready: false }], true: null, false: { nested: [1, true] } });
  void choices; void scored; void tagged; void meaning;
  const names: tt.Recognized = (await tt.recognize(text, { kinds: ['person'], relations: { works_for: ['person', '*'] } })).value;
  const edges: tt.Edge[] = (await tt.relate([['Ann', 'person'], { name: 'Acme', kind: 'organization' }], { relations: ['works_for'] })).value;
  const related: tt.Edge[] = (await tt.relate(names.entities, { relations: ['works_for'] })).value;
  const counters: tt.Usage = tt.usage();

  const engine = new tt.Engine({ throttle: 4, baseUrl: 'http://127.0.0.1:1/v1', cache: false, maxRequests: 10, maxRequestBytes: 20_000 });
  const fromEngine: tt.Answer = (await engine.decide('Refund?', text)).value;
  const sent: number = engine.usage().requests_sent;
  const retries: number = engine.usage().retries;
  void retries;

  const planned: tt.Plan = new tt.Engine({ maxRequestsTotal: 0 }).plan('Refund?', reviews, { batch: 'max' });
  const band: number = planned.estimated_input_tokens.upper + tt.plan({ tag: 'Which?', labels: ['a'] }, reviews).requests;
  const code: tt.Outcome = tt.outcome(answered);
  const failure = tt.failed(rows[0]?.['team']);
  const why: tt.FailureCause | undefined = failure?.cause;
  void band; void code; void why;
  const field = rows[0]?.['team'];
  if (field !== null && typeof field === 'object' && 'failed' in field) {
    const cause: tt.FailureCause = field.failed.cause;
    void cause;
  }
  try {
    await tt.decide('Refund?', text);
  } catch (error) {
    if (error instanceof tt.ThinkThenError && error.kind === 'cancelled' && error.code === 5) {
      const receipt: tt.Completion<unknown> | undefined = error.completion;
      const report = await receipt?.wait();
      if (report && 'ok' in report) { const final: tt.Facts = report.ok.facts; void final; }
      return null;
    }
  }
  return { team, topics, first, answered, second, level, complaints, found, digests, confidence, url, many, names, edges, counters, fromEngine, sent };
}

async function literalRankFindTypes() {
  const records = ['a', 'b'];
  await tt.rank({ decide: 'Q?' }, records);
  await tt.find({ decide: 'Q?' }, records, { none: true });
  // @ts-expect-error native find does not admit a score question.
  await tt.find({ score: 'Q?', levels: ['low', 'high'] }, records);
  // @ts-expect-error native find reads text alone, not a threshold.
  await tt.find({ decide: 'Q?', threshold: 0.8 }, records);
  // @ts-expect-error rich rank execution still requires native adoption.
  await tt.rank({ decide: 'Q?', true: 'wanted' }, records);
}
void literalRankFindTypes;
