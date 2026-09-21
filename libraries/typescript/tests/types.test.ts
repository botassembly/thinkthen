// The slide sample as a compile-time check: a wrong branch is a compile
// error here, which is this surface's goal. No assertions run; tsc's exit
// code is the test.
import * as tt from '../index.js';

export async function sample(text: string, message: string, inbox: string[], reviews: string[], signal: AbortSignal) {
  // the deck's TypeScript sample, typed
  const options = ['billing', 'shipping', 'account'];
  const team: string | null = await tt.choose('Which team owns it?', text, { options });

  const labels = ['billing', 'shipping', 'urgent', 'praise'];
  const topics: string[] = await tt.tag('Which topics?', message, { labels });

  const stop = new AbortController();
  const urgent: tt.RankedAnswer[] = await tt.rank('Is this urgent?', inbox, {
    top: 5,
    signal: stop.signal,
  });
  const first: tt.RankedAnswer | undefined = urgent[0];

  // the rest of the verbs stay typed
  const answered: boolean | null = await tt.decide('Does the customer ask for a refund?', text); // true

  const refund = tt.question({
    decide: 'Does the customer ask for a refund?',
    threshold: [0.2, 0.8],
  });
  const second: boolean | null = await tt.decide(refund, 'I was charged twice. Can you fix this?'); // null

  const complaints = await tt.filter('Is this a complaint?', reviews);
  const rows = await tt.annotate('form.json', reviews, { signal });
  const held = await tt.choose({ choose: 'Which team owns this?', options: ['billing', 'shipping'] }, text);
  const urgency = await tt.score({ score: 'How urgent?', levels: ['Routine.', 'Soon.', 'Immediate.'] }, text);
  const alsoHeld: string[] = await tt.tag({ tag: 'Name what applies', labels: ['refund'] }, text);
  const ranked = await tt.rank('Is this a complaint?', reviews, { top: 2, deadlineMs: 5_000 });
  const found = await tt.find('Which unit answers best?', reviews);
  const audit = await tt.details('Refund?', text);
  const many = await tt.decide_many('Refund?', reviews);
  const counters = tt.usage();
  tt.reset_usage();

  return {
    team,
    topics,
    first,
    answered,
    second,
    complaints,
    rows,
    held,
    urgency,
    alsoHeld,
    ranked,
    found,
    audit,
    many,
    counters,
  };
}
