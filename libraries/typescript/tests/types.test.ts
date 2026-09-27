// The deck's sample and every verb, typed: a wrong branch is a compile
// error. `tsc --noEmit --strict` over this file is the test; nothing runs.
import * as tt from '../index.js';

export async function sample(text: string, message: string, inbox: string[], reviews: string[], signal: AbortSignal) {
  const options = ['billing', 'shipping', 'account'];
  const team: string | null = await tt.choose('Which team owns it?', text, { options });
  const labels = ['billing', 'shipping', 'urgent', 'praise'];
  const topics: string[] = await tt.tag('Which topics?', message, { labels });
  const stop = new AbortController();
  const urgent: tt.Ranked[] = await tt.rank('Is this urgent?', inbox, { top: 5, signal: stop.signal });
  const first: tt.Ranked | undefined = urgent[0];

  const answered: tt.Answer = await tt.decide('Does the customer ask for a refund?', text);
  const refund = tt.question({ decide: 'Does the customer ask for a refund?', threshold: [0.2, 0.8] });
  const second: boolean | null = await tt.decide(refund, 'I was charged twice. Can you fix this?');
  const level: number = await tt.score('How urgent?', text, { levels: ['low', 'mid', 'high'] });
  const complaints: string[] = await tt.filter('Is this a complaint?', reviews);
  const rows: tt.AnnotatedRow[] = await tt.annotate('form.json', reviews, { signal, deadlineMs: null });
  const found: tt.Found | null = await tt.find('Which unit answers best?', reviews);
  const audit: tt.Details = await tt.details('Refund?', text, { deadlineMs: 5_000 });
  const digests: string[] = audit.meta.requests;
  const confidence: number | undefined = audit.answer.confidence;
  const url: string = audit.meta.url;
  const many: tt.Answer[] = await tt.decide_many('Refund?', reviews);
  const names: tt.Recognized = await tt.recognize(text, { kinds: ['person'], relations: { works_for: ['person', '*'] } });
  const edges: tt.Edge[] = await tt.relate([['Ann', 'person'], { name: 'Acme', kind: 'organization' }], { relations: ['works_for'] });
  const related: tt.Edge[] = await tt.relate(names.entities, { relations: ['works_for'] });
  const counters: tt.Usage = tt.usage();

  const engine = new tt.Engine({ throttle: 4, baseUrl: 'http://127.0.0.1:1/v1', cache: false, maxRequests: 10 });
  const fromEngine: tt.Answer = await engine.decide('Refund?', text);
  const sent: number = engine.usage().requests_sent;

  const field = rows[0]?.['team'];
  if (field !== null && typeof field === 'object' && 'failed' in field) {
    const cause: tt.FailureCause = field.failed.cause;
    void cause;
  }
  try {
    await tt.decide('Refund?', text);
  } catch (error) {
    if (error instanceof tt.ThinkThenError && error.kind === 'cancelled') return null;
  }
  return { team, topics, first, answered, second, level, complaints, found, digests, confidence, url, many, names, edges, counters, fromEngine, sent };
}
